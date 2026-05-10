use actix_web::web::{Data, Json};
use studycycle_api_utils::context::StudyCycleContext;
use studycycle_db_schema::source::post::PostActions;
use studycycle_db_views_local_user::LocalUserView;
use studycycle_db_views_post::{
  PostView,
  api::{MarkPostAsRead, PostResponse},
};
use studycycle_utils::error::StudyCycleResult;

pub async fn mark_post_as_read(
  Json(data): Json<MarkPostAsRead>,
  context: Data<StudyCycleContext>,
  local_user_view: LocalUserView,
) -> StudyCycleResult<Json<PostResponse>> {
  let person_id = local_user_view.person.id;
  let local_instance_id = local_user_view.person.instance_id;
  let post_id = data.post_id;

  // Mark the post as read / unread
  if data.read {
    PostActions::mark_as_read(&mut context.pool(), person_id, &[post_id]).await?;
  } else {
    PostActions::mark_as_unread(&mut context.pool(), person_id, &[post_id]).await?;
  }
  let post_view = PostView::read(
    &mut context.pool(),
    post_id,
    Some(&local_user_view.local_user),
    local_instance_id,
    false,
  )
  .await?;

  Ok(Json(PostResponse { post_view }))
}
