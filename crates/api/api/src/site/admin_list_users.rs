use actix_web::web::{Data, Json, Query};
use studycycle_api_utils::{context::StudyCycleContext, utils::is_admin};
use studycycle_db_views_local_user::{LocalUserView, api::AdminListUsers, impls::LocalUserQuery};
use studycycle_diesel_utils::pagination::PagedResponse;
use studycycle_utils::error::StudyCycleResult;

pub async fn admin_list_users(
  Query(data): Query<AdminListUsers>,
  context: Data<StudyCycleContext>,
  local_user_view: LocalUserView,
) -> StudyCycleResult<Json<PagedResponse<LocalUserView>>> {
  // Make sure user is an admin
  is_admin(&local_user_view)?;

  let users = LocalUserQuery {
    banned_only: data.banned_only,
    page_cursor: data.page_cursor,
    limit: data.limit,
    sort: data.sort,
  }
  .list(&mut context.pool())
  .await?;

  Ok(Json(users))
}
