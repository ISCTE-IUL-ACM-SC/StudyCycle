use activitypub_federation::config::Data;
use actix_web::web::{Json, Query};
use studycycle_api_utils::context::StudyCycleContext;
use studycycle_db_views_local_user::LocalUserView;
use studycycle_db_views_person_liked_combined::{ListPersonLiked, impls::PersonLikedCombinedQuery};
use studycycle_db_views_post_comment_combined::PostCommentCombinedView;
use studycycle_diesel_utils::pagination::PagedResponse;
use studycycle_utils::error::StudyCycleResult;

pub async fn list_person_liked(
  Query(data): Query<ListPersonLiked>,
  context: Data<StudyCycleContext>,
  local_user_view: LocalUserView,
) -> StudyCycleResult<Json<PagedResponse<PostCommentCombinedView>>> {
  let liked = PersonLikedCombinedQuery {
    type_: data.type_,
    like_type: data.like_type,
    page_cursor: data.page_cursor,
    limit: data.limit,
    no_limit: None,
  }
  .list(&mut context.pool(), &local_user_view)
  .await?;

  Ok(Json(liked))
}
