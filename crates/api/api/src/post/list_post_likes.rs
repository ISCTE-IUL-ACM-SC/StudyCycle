use actix_web::web::{Data, Json, Query};
use studycycle_api_utils::{context::StudyCycleContext, utils::is_mod_or_admin};
use studycycle_db_schema::source::post::Post;
use studycycle_db_views_local_user::LocalUserView;
use studycycle_db_views_post::api::ListPostLikes;
use studycycle_db_views_vote::VoteView;
use studycycle_diesel_utils::{pagination::PagedResponse, traits::Crud};
use studycycle_utils::error::StudyCycleResult;

/// Lists likes for a post
pub async fn list_post_likes(
  Query(data): Query<ListPostLikes>,
  context: Data<StudyCycleContext>,
  local_user_view: LocalUserView,
) -> StudyCycleResult<Json<PagedResponse<VoteView>>> {
  let post = Post::read(&mut context.pool(), data.post_id).await?;
  is_mod_or_admin(&mut context.pool(), &local_user_view, post.community_id).await?;

  let post_likes = VoteView::list_for_post(
    &mut context.pool(),
    data.post_id,
    data.page_cursor,
    data.limit,
    local_user_view.person.instance_id,
  )
  .await?;

  Ok(Json(post_likes))
}
