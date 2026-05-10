use actix_web::web::{Data, Json};
use studycycle_api_utils::{context::StudyCycleContext, utils::check_local_user_valid};
use studycycle_db_views_local_user::LocalUserView;
use studycycle_db_views_site::{
  SiteView,
  api::{ResendVerificationEmail, SuccessResponse},
};
use studycycle_email::account::send_verification_email_if_required;
use studycycle_utils::error::StudyCycleResult;

pub async fn resend_verification_email(
  Json(data): Json<ResendVerificationEmail>,
  context: Data<StudyCycleContext>,
) -> StudyCycleResult<Json<SuccessResponse>> {
  let site_view = SiteView::read_local(&mut context.pool()).await?;
  let email = data.email.to_string();

  // Fetch that email
  let local_user_view = LocalUserView::find_by_email(&mut context.pool(), &email).await?;
  check_local_user_valid(&local_user_view)?;

  send_verification_email_if_required(
    &site_view.local_site,
    &local_user_view,
    &mut context.pool(),
    context.settings(),
  )
  .await?;

  Ok(Json(SuccessResponse::default()))
}
