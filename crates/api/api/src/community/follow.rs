use crate::community::do_follow_community;
use activitypub_federation::config::Data;
use actix_web::web::Json;
use studycycle_api_utils::{context::StudyCycleContext, utils::check_local_user_valid};
use studycycle_db_schema::source::{actor_language::CommunityLanguage, community::Community};
use studycycle_db_views_community::{
  CommunityView,
  api::{CommunityResponse, FollowCommunity},
};
use studycycle_db_views_local_user::LocalUserView;
use studycycle_diesel_utils::traits::Crud;
use studycycle_utils::error::StudyCycleResult;

pub async fn follow_community(
  Json(data): Json<FollowCommunity>,
  context: Data<StudyCycleContext>,
  local_user_view: LocalUserView,
) -> StudyCycleResult<Json<CommunityResponse>> {
  check_local_user_valid(&local_user_view)?;
  let community_id = data.community_id;
  let community = Community::read(&mut context.pool(), community_id).await?;

  do_follow_community(community, &local_user_view.person, data.follow, &context).await?;

  let community_view = CommunityView::read(
    &mut context.pool(),
    community_id,
    Some(&local_user_view.local_user),
    false,
  )
  .await?;

  let discussion_languages = CommunityLanguage::read(&mut context.pool(), community_id).await?;

  Ok(Json(CommunityResponse {
    community_view,
    discussion_languages,
  }))
}
