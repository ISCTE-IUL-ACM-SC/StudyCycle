use super::{generate_activity_id, send_studycycle_activity};
use crate::protocol::following::{
  accept::AcceptFollow,
  follow::Follow,
  reject::RejectFollow,
  undo_follow::UndoFollow,
};
use activitypub_federation::{config::Data, kinds::activity::FollowType, traits::Activity};
use either::Either::*;
use studycycle_api_utils::context::StudyCycleContext;
use studycycle_apub_objects::objects::{CommunityOrMulti, UserOrCommunityOrMulti, person::ApubPerson};
use studycycle_db_schema::{
  newtypes::CommunityId,
  source::{activity::ActivitySendTargets, community::Community, person::Person},
};
use studycycle_db_schema_file::PersonId;
use studycycle_diesel_utils::traits::Crud;
use studycycle_utils::error::{StudyCycleError, StudyCycleResult};
use serde::Serialize;

pub(crate) mod accept;
pub(crate) mod follow;
pub(crate) mod reject;
pub(crate) mod undo_follow;

pub async fn send_follow(
  target: CommunityOrMulti,
  person: Person,
  follow: bool,
  context: &Data<StudyCycleContext>,
) -> StudyCycleResult<()> {
  let actor: ApubPerson = person.into();
  if follow {
    Follow::send(&actor, &target, context).await
  } else {
    UndoFollow::send(&actor, &target, context).await
  }
}

pub async fn send_accept_or_reject_follow(
  community_id: CommunityId,
  person_id: PersonId,
  accepted: bool,
  context: &Data<StudyCycleContext>,
) -> StudyCycleResult<()> {
  let community = Community::read(&mut context.pool(), community_id).await?;
  let person = Person::read(&mut context.pool(), person_id).await?;

  let follow = Follow {
    actor: person.ap_id.into(),
    to: Some([community.ap_id.clone().into()]),
    object: community.ap_id.into(),
    kind: FollowType::Follow,
    id: generate_activity_id(FollowType::Follow, context)?,
  };
  if accepted {
    AcceptFollow::send(follow, context).await
  } else {
    RejectFollow::send(follow, context).await
  }
}

/// Wrapper type which is needed because we cant implement ActorT for Either.
async fn send_activity_from_user_or_community_or_multi<A>(
  context: &Data<StudyCycleContext>,
  activity: A,
  target: UserOrCommunityOrMulti,
  send_targets: ActivitySendTargets,
) -> StudyCycleResult<()>
where
  A: Activity + Serialize + Send + Sync + Clone + Activity<Error = StudyCycleError>,
{
  match target {
    Left(user) => send_studycycle_activity(context, activity, &user, send_targets, true).await,
    Right(Left(community)) => {
      send_studycycle_activity(context, activity, &community, send_targets, true).await
    }
    Right(Right(multi)) => send_studycycle_activity(context, activity, &multi, send_targets, true).await,
  }
}
