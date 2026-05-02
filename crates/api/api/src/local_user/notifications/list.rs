use crate::hide_modlog_names;
use actix_web::web::{Data, Json, Query};
use studycycle_api_utils::context::StudyCycleContext;
use studycycle_db_views_local_user::LocalUserView;
use studycycle_db_views_notification::{ListNotifications, NotificationView, impls::NotificationQuery};
use studycycle_diesel_utils::pagination::PagedResponse;
use studycycle_utils::error::StudyCycleResult;

pub async fn list_notifications(
  Query(data): Query<ListNotifications>,
  context: Data<StudyCycleContext>,
  local_user_view: LocalUserView,
) -> StudyCycleResult<Json<PagedResponse<NotificationView>>> {
  let hide_modlog_names = hide_modlog_names(Some(&local_user_view), None, &context).await;
  let notifications = NotificationQuery {
    type_: data.type_,
    unread_only: data.unread_only,
    show_bot_accounts: Some(local_user_view.local_user.show_bot_accounts),
    page_cursor: data.page_cursor,
    hide_modlog_names: Some(hide_modlog_names),
    creator_id: data.creator_id,
    limit: data.limit,
    no_limit: None,
  }
  .list(&mut context.pool(), &local_user_view.person)
  .await?;

  Ok(Json(notifications))
}
