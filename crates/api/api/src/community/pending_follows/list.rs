use actix_web::web::{Data, Json, Query};
use studycycle_api_utils::{context::StudyCycleContext, utils::check_community_mod_of_any_or_admin_action};
use studycycle_db_views_community_follower_approval::{
  PendingFollowerView,
  api::ListCommunityPendingFollows,
};
use studycycle_db_views_local_user::LocalUserView;
use studycycle_diesel_utils::pagination::PagedResponse;
use studycycle_utils::error::StudyCycleResult;

pub async fn get_pending_follows_list(
  Query(data): Query<ListCommunityPendingFollows>,
  context: Data<StudyCycleContext>,
  local_user_view: LocalUserView,
) -> StudyCycleResult<Json<PagedResponse<PendingFollowerView>>> {
  check_community_mod_of_any_or_admin_action(&local_user_view, &mut context.pool()).await?;
  let all_communities =
    data.all_communities.unwrap_or_default() && local_user_view.local_user.admin;

  let items = PendingFollowerView::list_approval_required(
    &mut context.pool(),
    local_user_view.person.id,
    all_communities,
    data.unread_only.unwrap_or_default(),
    data.page_cursor,
    data.limit,
  )
  .await?;

  Ok(Json(items))
}
