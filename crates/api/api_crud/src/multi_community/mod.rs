use activitypub_federation::config::Data;
use studycycle_api_utils::{
  context::StudyCycleContext,
  send_activity::{ActivityChannel, SendActivityData},
};
use studycycle_db_schema::source::{multi_community::MultiCommunity, person::Person};
use studycycle_db_views_local_user::LocalUserView;
use studycycle_utils::error::{StudyCycleErrorType, StudyCycleResult};

pub mod create;
pub mod create_entry;
pub mod delete_entry;
pub mod list;
pub mod update;

/// Check that current user is creator of multi-comm and can modify it.
fn check_multi_community_creator(
  multi: &MultiCommunity,
  local_user_view: &LocalUserView,
) -> StudyCycleResult<()> {
  if multi.local && local_user_view.local_user.admin {
    Ok(())
  } else if multi.creator_id != local_user_view.person.id {
    Err(StudyCycleErrorType::MultiCommunityUpdateWrongUser.into())
  } else {
    Ok(())
  }
}

fn send_federation_update(
  multi: MultiCommunity,
  person: Person,
  context: &Data<StudyCycleContext>,
) -> StudyCycleResult<()> {
  ActivityChannel::submit_activity(
    SendActivityData::UpdateMultiCommunity(multi, person),
    context,
  )
}
