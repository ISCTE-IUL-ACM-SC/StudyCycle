use actix_web::{
  HttpRequest,
  web::{Data, Json},
};
use bcrypt::verify;
use studycycle_api_utils::{
  claims::Claims,
  context::StudyCycleContext,
  utils::{check_local_user_valid, password_length_check},
};
use studycycle_db_schema::source::{local_user::LocalUser, login_token::LoginToken};
use studycycle_db_views_local_user::LocalUserView;
use studycycle_db_views_site::api::{ChangePassword, LoginResponse};
use studycycle_utils::error::{StudyCycleErrorType, StudyCycleResult};

pub async fn change_password(
  Json(data): Json<ChangePassword>,
  req: HttpRequest,
  context: Data<StudyCycleContext>,
  local_user_view: LocalUserView,
) -> StudyCycleResult<Json<LoginResponse>> {
  check_local_user_valid(&local_user_view)?;
  password_length_check(&data.new_password)?;

  // Make sure passwords match
  if data.new_password != data.new_password_verify {
    return Err(StudyCycleErrorType::PasswordsDoNotMatch.into());
  }

  // Check the old password
  let valid: bool = if let Some(password_encrypted) = &local_user_view.local_user.password_encrypted
  {
    verify(&data.old_password, password_encrypted).unwrap_or(false)
  } else {
    data.old_password.is_empty()
  };

  if !valid {
    return Err(StudyCycleErrorType::IncorrectLogin.into());
  }

  let local_user_id = local_user_view.local_user.id;
  let new_password = data.new_password.clone();
  let updated_local_user =
    LocalUser::update_password(&mut context.pool(), local_user_id, &new_password).await?;

  LoginToken::invalidate_all(&mut context.pool(), local_user_view.local_user.id).await?;

  // Return the jwt
  Ok(Json(LoginResponse {
    jwt: Some(Claims::generate(updated_local_user.id, data.stay_logged_in, req, &context).await?),
    verify_email_sent: false,
    registration_created: false,
  }))
}
