use crate::community::do_follow_community;
use activitypub_federation::config::Data;
use actix_web::web::Json;
use studycycle_api_utils::context::StudyCycleContext;
use studycycle_db_schema::source::community::{Community, CommunityActions};
use studycycle_db_schema_file::enums::CommunityNotificationsMode;
use studycycle_db_views_community::api::EditCommunityNotifications;
use studycycle_db_views_local_user::LocalUserView;
use studycycle_db_views_site::api::SuccessResponse;
use studycycle_diesel_utils::traits::Crud;
use studycycle_utils::error::StudyCycleResult;

pub async fn edit_community_notifications(
  Json(data): Json<EditCommunityNotifications>,
  context: Data<StudyCycleContext>,
  local_user_view: LocalUserView,
) -> StudyCycleResult<Json<SuccessResponse>> {
  CommunityActions::update_notification_state(
    data.community_id,
    local_user_view.person.id,
    data.mode,
    &mut context.pool(),
  )
  .await?;

  // To get notifications for a remote community, the user needs to follow it over federation.
  // Do this automatically here to avoid confusion.
  if data.mode == CommunityNotificationsMode::AllPostsAndComments
    || data.mode == CommunityNotificationsMode::AllPosts
  {
    let community = Community::read(&mut context.pool(), data.community_id).await?;
    if !community.local {
      do_follow_community(community, &local_user_view.person, true, &context).await?;
    }
  }

  Ok(Json(SuccessResponse::default()))
}
