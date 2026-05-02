use actix_web::web::{Data, Json};
use studycycle_api_utils::{
  context::StudyCycleContext,
  utils::{check_email_verified, check_local_user_valid},
};
use studycycle_db_views_local_user::LocalUserView;
use studycycle_db_views_site::{
  SiteView,
  api::{ResetPassword, SuccessResponse},
};
use studycycle_email::account::send_password_reset_email;
use studycycle_utils::error::StudyCycleResult;
use tracing::error;

pub async fn reset_password(
  Json(data): Json<ResetPassword>,
  context: Data<StudyCycleContext>,
) -> StudyCycleResult<Json<SuccessResponse>> {
  let email = data.email.to_lowercase();
  // For security, errors are not returned.
  // https://github.com/LemmyNet/lemmy/issues/5277
  let _ = try_reset_password(&email, &context).await;
  Ok(Json(SuccessResponse::default()))
}

async fn try_reset_password(email: &str, context: &StudyCycleContext) -> StudyCycleResult<()> {
  let local_user_view = LocalUserView::find_by_email(&mut context.pool(), email).await?;
  check_local_user_valid(&local_user_view)?;
  let site_view = SiteView::read_local(&mut context.pool()).await?;

  check_email_verified(&local_user_view, &site_view)?;
  if let Err(e) =
    send_password_reset_email(&local_user_view, &mut context.pool(), context.settings()).await
  {
    error!("Failed to send password reset email: {}", e);
  }

  Ok(())
}
