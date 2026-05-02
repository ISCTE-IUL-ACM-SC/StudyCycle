use activitypub_federation::{config::Data, traits::Actor};
use studycycle_api_utils::context::StudyCycleContext;
use studycycle_apub_objects::protocol::tags::ApubTag;
use studycycle_db_schema::source::{activity::ActivitySendTargets, person::Person};
use studycycle_utils::error::StudyCycleResult;

pub mod comment;
pub(crate) mod note_wrapper;
pub mod post;
pub mod private_message;

/// From Activitypub `tag` field extract the mentions, and return the inboxes for these users.
/// Used when sending out activity to ensure the mentioned users see it.
async fn tagged_user_inboxes(
  tagged_users: &[ApubTag],
  context: &Data<StudyCycleContext>,
) -> StudyCycleResult<ActivitySendTargets> {
  let tagged_users: Vec<_> = tagged_users.iter().flat_map(ApubTag::mention_id).collect();
  let mut inboxes = ActivitySendTargets::empty();
  for t in tagged_users {
    let person = t.dereference(context).await?;
    inboxes.add_inbox(person.shared_inbox_or_inbox());
  }
  Ok(inboxes)
}

/// Extracts the users who are mentioned in a received, federated post.
async fn parse_apub_mentions(
  tags: &[ApubTag],
  context: &Data<StudyCycleContext>,
) -> StudyCycleResult<Vec<Person>> {
  let mentions: Vec<_> = tags.iter().filter_map(ApubTag::mention_id).collect();
  let mut res = vec![];
  for m in mentions {
    let Some(person) = m.dereference(context).await?.left() else {
      continue;
    };
    if person.local {
      res.push(person.0);
    }
  }
  Ok(res)
}
