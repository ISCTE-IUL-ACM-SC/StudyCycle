use activitypub_federation::config::Data;
use actix_web::web::Json;
use studycycle_api_utils::{
  context::StudyCycleContext,
  send_activity::{ActivityChannel, SendActivityData},
  utils::check_local_user_valid,
};
use studycycle_db_schema::source::multi_community::{MultiCommunity, MultiCommunityFollowForm};
use studycycle_db_schema_file::enums::CommunityFollowerState;
use studycycle_db_views_community::{
  MultiCommunityView,
  api::{FollowMultiCommunity, MultiCommunityResponse},
};
use studycycle_db_views_local_user::LocalUserView;
use studycycle_diesel_utils::traits::Crud;
use studycycle_utils::error::StudyCycleResult;

pub async fn follow_multi_community(
  Json(data): Json<FollowMultiCommunity>,
  context: Data<StudyCycleContext>,
  local_user_view: LocalUserView,
) -> StudyCycleResult<Json<MultiCommunityResponse>> {
  check_local_user_valid(&local_user_view)?;
  let multi_community_id = data.multi_community_id;
  let my_person_id = local_user_view.person.id;
  let multi = MultiCommunity::read(&mut context.pool(), multi_community_id).await?;

  let follow_state = if multi.local {
    CommunityFollowerState::Accepted
  } else {
    CommunityFollowerState::Pending
  };
  let form = MultiCommunityFollowForm {
    multi_community_id,
    person_id: my_person_id,
    follow_state,
  };

  if data.follow {
    MultiCommunity::follow(&mut context.pool(), &form).await?;
  } else {
    MultiCommunity::unfollow(&mut context.pool(), my_person_id, multi_community_id).await?;
  }

  if !multi.local {
    ActivityChannel::submit_activity(
      SendActivityData::FollowMultiCommunity(multi, local_user_view.person.clone(), data.follow),
      &context,
    )?;
  }

  let multi_community_view =
    MultiCommunityView::read(&mut context.pool(), multi_community_id, Some(my_person_id)).await?;

  Ok(Json(MultiCommunityResponse {
    multi_community_view,
  }))
}
