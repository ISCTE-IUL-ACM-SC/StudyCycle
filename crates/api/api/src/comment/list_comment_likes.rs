use actix_web::web::{Data, Json, Query};
use studycycle_api_utils::{context::StudyCycleContext, utils::is_mod_or_admin};
use studycycle_db_views_comment::{CommentView, api::ListCommentLikes};
use studycycle_db_views_local_user::LocalUserView;
use studycycle_db_views_vote::VoteView;
use studycycle_diesel_utils::pagination::PagedResponse;
use studycycle_utils::error::StudyCycleResult;

/// Lists likes for a comment
pub async fn list_comment_likes(
  Query(data): Query<ListCommentLikes>,
  context: Data<StudyCycleContext>,
  local_user_view: LocalUserView,
) -> StudyCycleResult<Json<PagedResponse<VoteView>>> {
  let local_instance_id = local_user_view.person.instance_id;

  let comment_view = CommentView::read(
    &mut context.pool(),
    data.comment_id,
    Some(&local_user_view.local_user),
    local_instance_id,
  )
  .await?;

  is_mod_or_admin(
    &mut context.pool(),
    &local_user_view,
    comment_view.community.id,
  )
  .await?;

  let comment_likes = VoteView::list_for_comment(
    &mut context.pool(),
    data.comment_id,
    data.page_cursor,
    data.limit,
    local_instance_id,
  )
  .await?;

  Ok(Json(comment_likes))
}
