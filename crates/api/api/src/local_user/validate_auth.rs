use actix_web::{
  HttpRequest,
  web::{Data, Json},
};
use studycycle_api_utils::{
  context::StudyCycleContext,
  utils::{local_user_view_from_jwt, read_auth_token},
};
use studycycle_db_views_site::api::SuccessResponse;
use studycycle_utils::error::{StudyCycleErrorType, StudyCycleResult};

/// Returns an error message if the auth token is invalid for any reason. Necessary because other
/// endpoints silently treat any call with invalid auth as unauthenticated.
pub async fn validate_auth(
  req: HttpRequest,
  context: Data<StudyCycleContext>,
) -> StudyCycleResult<Json<SuccessResponse>> {
  let jwt = read_auth_token(&req)?;
  if let Some(jwt) = jwt {
    local_user_view_from_jwt(&jwt, &context).await?;
  } else {
    return Err(StudyCycleErrorType::NotLoggedIn.into());
  }
  Ok(Json(SuccessResponse::default()))
}
