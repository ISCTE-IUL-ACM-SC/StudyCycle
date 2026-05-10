use activitypub_federation::config::Data;
use actix_web::web::Json;
use studycycle_api_utils::{
  context::StudyCycleContext,
  send_activity::{ActivityChannel, SendActivityData},
  utils::is_mod_or_admin,
};
use studycycle_db_schema::source::community::CommunityActions;
use studycycle_db_schema_file::enums::CommunityFollowerState;
use studycycle_db_views_community::api::ApproveCommunityPendingFollower;
use studycycle_db_views_local_user::LocalUserView;
use studycycle_db_views_site::api::SuccessResponse;
use studycycle_utils::error::StudyCycleResult;

pub async fn post_pending_follows_approve(
  Json(data): Json<ApproveCommunityPendingFollower>,
  context: Data<StudyCycleContext>,
  local_user_view: LocalUserView,
) -> StudyCycleResult<Json<SuccessResponse>> {
  is_mod_or_admin(&mut context.pool(), &local_user_view, data.community_id).await?;

  let (state, activity_data) = if data.approve {
    (
      CommunityFollowerState::Accepted,
      SendActivityData::AcceptFollower(data.community_id, data.follower_id),
    )
  } else {
    (
      CommunityFollowerState::Denied,
      SendActivityData::RejectFollower(data.community_id, data.follower_id),
    )
  };
  CommunityActions::approve_private_community_follower(
    &mut context.pool(),
    data.community_id,
    data.follower_id,
    local_user_view.person.id,
    state,
  )
  .await?;
  ActivityChannel::submit_activity(activity_data, &context)?;

  Ok(Json(SuccessResponse::default()))
}
