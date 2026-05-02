use activitypub_federation::config::Data;
use actix_web::web::Json;
use studycycle_api_utils::{
  build_response::build_community_response,
  context::StudyCycleContext,
  notify::notify_mod_action,
  send_activity::{ActivityChannel, SendActivityData},
  utils::{check_community_mod_action, is_admin},
};
use studycycle_db_schema::{
  source::{
    community::{Community, CommunityUpdateForm},
    community_report::CommunityReport,
    modlog::{Modlog, ModlogInsertForm},
  },
  traits::Reportable,
};
use studycycle_db_views_community::api::{CommunityResponse, RemoveCommunity};
use studycycle_db_views_community_moderator::CommunityModeratorView;
use studycycle_db_views_local_user::LocalUserView;
use studycycle_diesel_utils::traits::Crud;
use studycycle_utils::error::StudyCycleResult;

pub async fn remove_community(
  Json(data): Json<RemoveCommunity>,
  context: Data<StudyCycleContext>,
  local_user_view: LocalUserView,
) -> StudyCycleResult<Json<CommunityResponse>> {
  let community = Community::read(&mut context.pool(), data.community_id).await?;
  check_community_mod_action(&local_user_view, &community, true, &mut context.pool()).await?;

  // Verify its an admin (only an admin can remove a community)
  is_admin(&local_user_view)?;

  // Do the remove
  let community_id = data.community_id;
  let removed = data.removed;
  let community = Community::update(
    &mut context.pool(),
    community_id,
    &CommunityUpdateForm {
      removed: Some(removed),
      ..Default::default()
    },
  )
  .await?;

  CommunityReport::resolve_all_for_object(
    &mut context.pool(),
    community_id,
    local_user_view.person.id,
  )
  .await?;

  // Mod
  let community_owner =
    CommunityModeratorView::top_mod_for_community(&mut context.pool(), data.community_id).await?;
  let form = ModlogInsertForm::admin_remove_community(
    &local_user_view.person,
    data.community_id,
    community_owner,
    removed,
    &data.reason,
  );
  let action = Modlog::create(&mut context.pool(), &[form]).await?;
  notify_mod_action(action.clone(), context.app_data());

  ActivityChannel::submit_activity(
    SendActivityData::RemoveCommunity {
      moderator: local_user_view.person.clone(),
      community,
      reason: data.reason.clone(),
      removed: data.removed,
    },
    &context,
  )?;

  build_community_response(&context, local_user_view, community_id).await
}
