use actix_web::web::{Data, Json};
use studycycle_api_utils::context::StudyCycleContext;
use studycycle_db_schema::source::notification::Notification;
use studycycle_db_views_local_user::LocalUserView;
use studycycle_db_views_notification::api::MarkNotificationAsRead;
use studycycle_db_views_site::api::SuccessResponse;
use studycycle_utils::error::StudyCycleResult;

pub async fn mark_notification_as_read(
  Json(data): Json<MarkNotificationAsRead>,
  context: Data<StudyCycleContext>,
  local_user_view: LocalUserView,
) -> StudyCycleResult<Json<SuccessResponse>> {
  Notification::mark_read_by_id_and_person(
    &mut context.pool(),
    data.notification_id,
    local_user_view.person.id,
    data.read,
  )
  .await?;

  Ok(Json(SuccessResponse::default()))
}
