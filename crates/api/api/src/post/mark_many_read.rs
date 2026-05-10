use actix_web::web::{Data, Json};
use studycycle_api_utils::context::StudyCycleContext;
use studycycle_db_schema::source::post::PostActions;
use studycycle_db_views_local_user::LocalUserView;
use studycycle_db_views_post::api::MarkManyPostsAsRead;
use studycycle_db_views_site::api::SuccessResponse;
use studycycle_utils::{error::StudyCycleResult, utils::validation::check_api_elements_count};

pub async fn mark_posts_as_read(
  Json(data): Json<MarkManyPostsAsRead>,
  context: Data<StudyCycleContext>,
  local_user_view: LocalUserView,
) -> StudyCycleResult<Json<SuccessResponse>> {
  let post_ids = &data.post_ids;
  check_api_elements_count(post_ids.len())?;

  let person_id = local_user_view.person.id;

  // Mark the posts as read / unread
  if data.read {
    PostActions::mark_as_read(&mut context.pool(), person_id, post_ids).await?;
  } else {
    PostActions::mark_as_unread(&mut context.pool(), person_id, post_ids).await?;
  }

  Ok(Json(SuccessResponse::default()))
}
