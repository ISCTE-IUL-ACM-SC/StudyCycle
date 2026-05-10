use actix_web::web::{Data, Json};
use chrono::Utc;
use studycycle_api_utils::context::StudyCycleContext;
use studycycle_db_schema::source::local_user::{LocalUser, LocalUserUpdateForm};
use studycycle_db_views_local_user::LocalUserView;
use studycycle_db_views_site::api::SuccessResponse;
use studycycle_utils::error::StudyCycleResult;

pub async fn mark_donation_dialog_shown(
  context: Data<StudyCycleContext>,
  local_user_view: LocalUserView,
) -> StudyCycleResult<Json<SuccessResponse>> {
  let form = LocalUserUpdateForm {
    last_donation_notification_at: Some(Utc::now()),
    ..Default::default()
  };
  LocalUser::update(&mut context.pool(), local_user_view.local_user.id, &form).await?;

  Ok(Json(SuccessResponse::default()))
}
