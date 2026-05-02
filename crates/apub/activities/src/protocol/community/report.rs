use activitypub_federation::{
  config::Data,
  fetch::object_id::ObjectId,
  kinds::activity::FlagType,
  protocol::helpers::deserialize_one,
};
use either::Either;
use studycycle_api_utils::context::StudyCycleContext;
use studycycle_apub_objects::{
  objects::{ReportableObjects, community::ApubCommunity, instance::ApubSite, person::ApubPerson},
  utils::protocol::InCommunity,
};
use studycycle_utils::error::{StudyCycleErrorType, StudyCycleResult};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Report {
  pub(crate) actor: ObjectId<ApubPerson>,
  #[serde(deserialize_with = "deserialize_one")]
  pub(crate) to: [ObjectId<Either<ApubSite, ApubCommunity>>; 1],
  pub(crate) object: ReportObject,
  /// Report reason as sent by StudyCycle
  pub(crate) summary: Option<String>,
  /// Report reason as sent by Mastodon
  pub(crate) content: Option<String>,
  #[serde(rename = "type")]
  pub(crate) kind: FlagType,
  pub(crate) id: Url,
  pub(crate) audience: Option<ObjectId<ApubCommunity>>,
}

impl Report {
  pub fn reason(&self) -> StudyCycleResult<String> {
    self
      .summary
      .clone()
      .or(self.content.clone())
      .ok_or(StudyCycleErrorType::NotFound.into())
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub(crate) enum ReportObject {
  StudyCycle(ObjectId<ReportableObjects>),
  /// Mastodon sends an array containing user id and one or more post ids
  Mastodon(Vec<Url>),
}

impl ReportObject {
  pub(crate) async fn dereference(
    &self,
    context: &Data<StudyCycleContext>,
  ) -> StudyCycleResult<ReportableObjects> {
    match self {
      ReportObject::StudyCycle(l) => l.dereference(context).await,
      ReportObject::Mastodon(objects) => {
        for o in objects {
          // Find the first reported item which can be dereferenced as post or comment (StudyCycle can
          // only handle one item per report).
          let deref = ObjectId::from(o.clone()).dereference(context).await;
          if deref.is_ok() {
            return deref;
          }
        }
        Err(StudyCycleErrorType::NotFound.into())
      }
    }
  }

  pub(crate) async fn object_id(
    &self,
    context: &Data<StudyCycleContext>,
  ) -> StudyCycleResult<ObjectId<ReportableObjects>> {
    match self {
      ReportObject::StudyCycle(l) => Ok(l.clone()),
      ReportObject::Mastodon(objects) => {
        for o in objects {
          // Same logic as above, but return the ID and not the object itself.
          let deref = ObjectId::<ReportableObjects>::from(o.clone())
            .dereference(context)
            .await;
          if deref.is_ok() {
            return Ok(o.clone().into());
          }
        }
        Err(StudyCycleErrorType::NotFound.into())
      }
    }
  }
}

impl InCommunity for Report {
  async fn community(&self, context: &Data<StudyCycleContext>) -> StudyCycleResult<ApubCommunity> {
    if let Some(audience) = &self.audience {
      return audience.dereference(context).await;
    }
    match self.to[0].dereference(context).await? {
      Either::Left(_) => Err(StudyCycleErrorType::NotFound.into()),
      Either::Right(c) => Ok(c),
    }
  }
}
