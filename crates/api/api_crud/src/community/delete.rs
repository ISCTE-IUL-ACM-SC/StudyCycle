use activitypub_federation::config::Data;
use actix_web::web::Json;
use studycycle_api_utils::{
  build_response::build_community_response,
  context::StudyCycleContext,
  send_activity::{ActivityChannel, SendActivityData},
  utils::{check_community_mod_action, check_local_user_valid, is_top_mod},
};
use studycycle_db_schema::source::community::{Community, CommunityUpdateForm};
use studycycle_db_views_community::api::{CommunityResponse, DeleteCommunity};
use studycycle_db_views_community_moderator::CommunityModeratorView;
use studycycle_db_views_local_user::LocalUserView;
use studycycle_diesel_utils::traits::Crud;
use studycycle_utils::error::StudyCycleResult;

pub async fn delete_community(
  Json(data): Json<DeleteCommunity>,
  context: Data<StudyCycleContext>,
  local_user_view: LocalUserView,
) -> StudyCycleResult<Json<CommunityResponse>> {
  check_local_user_valid(&local_user_view)?;
  // Fetch the community mods
  let community_mods =
    CommunityModeratorView::for_community(&mut context.pool(), data.community_id).await?;

  let community = Community::read(&mut context.pool(), data.community_id).await?;
  check_community_mod_action(&local_user_view, &community, true, &mut context.pool()).await?;

  // Make sure deleter is the top mod
  is_top_mod(&local_user_view, &community_mods)?;

  // Do the delete
  let community_id = data.community_id;
  let deleted = data.deleted;
  let community = Community::update(
    &mut context.pool(),
    community_id,
    &CommunityUpdateForm {
      deleted: Some(deleted),
      ..Default::default()
    },
  )
  .await?;

  ActivityChannel::submit_activity(
    SendActivityData::DeleteCommunity(local_user_view.person.clone(), community, data.deleted),
    &context,
  )?;

  build_community_response(&context, local_user_view, community_id).await
}
