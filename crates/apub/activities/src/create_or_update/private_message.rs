use crate::{
  generate_activity_id,
  protocol::{CreateOrUpdateType, create_or_update::private_message::CreateOrUpdatePrivateMessage},
  send_studycycle_activity,
  verify_person,
};
use activitypub_federation::{
  config::Data,
  protocol::verification::{verify_domains_match, verify_urls_match},
  traits::{Activity, Actor, Object},
};
use studycycle_api_utils::context::StudyCycleContext;
use studycycle_apub_objects::objects::{person::ApubPerson, private_message::ApubPrivateMessage};
use studycycle_db_schema::source::activity::ActivitySendTargets;
use studycycle_db_views_private_message::PrivateMessageView;
use studycycle_utils::error::{StudyCycleError, StudyCycleResult};
use url::Url;

pub(crate) async fn send_create_or_update_pm(
  pm_view: PrivateMessageView,
  kind: CreateOrUpdateType,
  context: Data<StudyCycleContext>,
) -> StudyCycleResult<()> {
  let actor: ApubPerson = pm_view.creator.into();
  let recipient: ApubPerson = pm_view.recipient.into();

  let id = generate_activity_id(kind.clone(), &context)?;
  let create_or_update = CreateOrUpdatePrivateMessage {
    id: id.clone(),
    actor: actor.id().clone().into(),
    to: [recipient.id().clone().into()],
    object: ApubPrivateMessage(pm_view.private_message.clone())
      .into_json(&context)
      .await?,
    kind,
  };
  let inbox = ActivitySendTargets::to_inbox(recipient.shared_inbox_or_inbox());
  send_studycycle_activity(&context, create_or_update, &actor, inbox, true).await
}

#[async_trait::async_trait]
impl Activity for CreateOrUpdatePrivateMessage {
  type DataType = StudyCycleContext;
  type Error = StudyCycleError;

  fn id(&self) -> &Url {
    &self.id
  }

  fn actor(&self) -> &Url {
    self.actor.inner()
  }

  async fn verify(&self, context: &Data<Self::DataType>) -> StudyCycleResult<()> {
    verify_person(&self.actor, context).await?;
    verify_domains_match(self.actor.inner(), self.object.id.inner())?;
    verify_domains_match(self.to[0].inner(), self.object.to[0].inner())?;
    verify_urls_match(self.actor.inner(), self.object.attributed_to.inner())?;
    ApubPrivateMessage::verify(&self.object, self.actor.inner(), context).await?;
    Ok(())
  }

  async fn receive(self, context: &Data<Self::DataType>) -> StudyCycleResult<()> {
    ApubPrivateMessage::from_json(self.object, context).await?;
    Ok(())
  }
}
