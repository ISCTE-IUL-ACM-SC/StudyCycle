use crate::community::do_follow_community;
use activitypub_federation::config::Data;
use actix_web::web::Json;
use studycycle_api_utils::context::StudyCycleContext;
use studycycle_db_schema::source::{
  community::Community,
  post::{Post, PostActions},
};
use studycycle_db_schema_file::enums::PostNotificationsMode;
use studycycle_db_views_local_user::LocalUserView;
use studycycle_db_views_post::api::EditPostNotifications;
use studycycle_db_views_site::api::SuccessResponse;
use studycycle_diesel_utils::traits::Crud;
use studycycle_utils::error::StudyCycleResult;

pub async fn edit_post_notifications(
  Json(data): Json<EditPostNotifications>,
  context: Data<StudyCycleContext>,
  local_user_view: LocalUserView,
) -> StudyCycleResult<Json<SuccessResponse>> {
  PostActions::update_notification_state(
    data.post_id,
    local_user_view.person.id,
    data.mode,
    &mut context.pool(),
  )
  .await?;
  let post = Post::read(&mut context.pool(), data.post_id).await?;

  // To get notifications for a remote community, the user needs to follow it over federation.
  // Do this automatically here to avoid confusion.
  if data.mode == PostNotificationsMode::AllComments {
    let community = Community::read(&mut context.pool(), post.community_id).await?;
    if !community.local {
      do_follow_community(community, &local_user_view.person, true, &context).await?;
    }
  }
  Ok(Json(SuccessResponse::default()))
}
