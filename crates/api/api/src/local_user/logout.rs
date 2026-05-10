use activitypub_federation::config::Data;
use actix_web::{HttpRequest, HttpResponse, cookie::Cookie};
use studycycle_api_utils::{
  context::StudyCycleContext,
  utils::{AUTH_COOKIE_NAME, read_auth_token},
};
use studycycle_db_schema::source::login_token::LoginToken;
use studycycle_db_views_local_user::LocalUserView;
use studycycle_db_views_site::api::SuccessResponse;
use studycycle_utils::error::{StudyCycleErrorType, StudyCycleResult};

pub async fn logout(
  req: HttpRequest,
  // require login
  _local_user_view: LocalUserView,
  context: Data<StudyCycleContext>,
) -> StudyCycleResult<HttpResponse> {
  let jwt = read_auth_token(&req)?.ok_or(StudyCycleErrorType::NotLoggedIn)?;
  LoginToken::invalidate(&mut context.pool(), &jwt).await?;

  let mut res = HttpResponse::Ok().json(SuccessResponse::default());
  let cookie = Cookie::new(AUTH_COOKIE_NAME, "");
  res.add_removal_cookie(&cookie)?;
  Ok(res)
}
