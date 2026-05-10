use activitypub_federation::config::Data;
use actix_web::web::{Json, Query};
use studycycle_api_utils::context::StudyCycleContext;
use studycycle_db_views_community::{
  MultiCommunityView,
  api::ListMultiCommunities,
  impls::MultiCommunityQuery,
};
use studycycle_db_views_local_user::LocalUserView;
use studycycle_diesel_utils::pagination::PagedResponse;
use studycycle_utils::error::StudyCycleResult;

pub async fn list_multi_communities(
  Query(data): Query<ListMultiCommunities>,
  context: Data<StudyCycleContext>,
  local_user_view: Option<LocalUserView>,
) -> StudyCycleResult<Json<PagedResponse<MultiCommunityView>>> {
  let res = MultiCommunityQuery {
    listing_type: data.type_,
    sort: data.sort,
    creator_id: data.creator_id,
    local_user: local_user_view.map(|l| l.local_user).as_ref(),
    time_range_seconds: data.time_range_seconds,
    search_term: data.search_term,
    search_title_only: data.search_title_only,
    page_cursor: data.page_cursor,
    limit: data.limit,
    no_limit: None,
  }
  .list(&mut context.pool())
  .await?;

  Ok(Json(res))
}
