use actix_web::web::{Data, Json};
use studycycle_api_utils::context::StudyCycleContext;
use studycycle_db_schema::source::notification::Notification;
use studycycle_db_views_local_user::LocalUserView;
use studycycle_db_views_site::api::SuccessResponse;
use studycycle_utils::error::StudyCycleResult;

pub async fn mark_all_notifications_read(
  context: Data<StudyCycleContext>,
  local_user_view: LocalUserView,
) -> StudyCycleResult<Json<SuccessResponse>> {
  Notification::mark_all_as_read(&mut context.pool(), local_user_view.person.id).await?;

  Ok(Json(SuccessResponse::default()))
}
