use crate::{
  check_community_deleted_or_removed,
  generate_activity_id,
  protocol::voting::{undo_vote::UndoVote, vote::Vote},
  voting::{undo_vote_comment, undo_vote_post},
};
use activitypub_federation::{
  config::Data,
  kinds::activity::UndoType,
  protocol::verification::verify_urls_match,
  traits::{Activity, Object},
};
use studycycle_api_utils::context::StudyCycleContext;
use studycycle_apub_objects::{
  objects::{PostOrComment, community::ApubCommunity, person::ApubPerson},
  utils::{functions::verify_person_in_community, protocol::InCommunity},
};
use studycycle_utils::error::{StudyCycleError, StudyCycleResult};
use url::Url;

impl UndoVote {
  pub(in crate::voting) fn new(
    vote: Vote,
    actor: &ApubPerson,
    community: &ApubCommunity,
    context: &Data<StudyCycleContext>,
  ) -> StudyCycleResult<Self> {
    Ok(UndoVote {
      actor: actor.id().clone().into(),
      object: vote,
      kind: UndoType::Undo,
      id: generate_activity_id(UndoType::Undo, context)?,
      audience: Some(community.ap_id.clone().into()),
    })
  }
}

#[async_trait::async_trait]
impl Activity for UndoVote {
  type DataType = StudyCycleContext;
  type Error = StudyCycleError;

  fn id(&self) -> &Url {
    &self.id
  }

  fn actor(&self) -> &Url {
    self.actor.inner()
  }

  async fn verify(&self, context: &Data<StudyCycleContext>) -> StudyCycleResult<()> {
    let community = self.object.community(context).await?;
    check_community_deleted_or_removed(&community)?;
    verify_person_in_community(&self.actor, &community, context).await?;
    verify_urls_match(self.actor.inner(), self.object.actor.inner())?;
    self.object.verify(context).await?;
    Ok(())
  }

  async fn receive(self, context: &Data<StudyCycleContext>) -> StudyCycleResult<()> {
    let actor = self.actor.dereference(context).await?;
    let object = self.object.object.dereference(context).await?;
    match object {
      PostOrComment::Left(p) => undo_vote_post(actor, &p, context).await,
      PostOrComment::Right(c) => undo_vote_comment(actor, &c, context).await,
    }
  }
}
