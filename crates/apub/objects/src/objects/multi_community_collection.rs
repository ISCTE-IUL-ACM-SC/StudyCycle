use super::multi_community::ApubMultiCommunity;
use crate::protocol::multi_community::FeedCollection;
use activitypub_federation::{
  config::Data,
  protocol::verification::verify_domains_match,
  traits::Collection,
};
use futures::future::join_all;
use studycycle_api_utils::{
  context::StudyCycleContext,
  send_activity::{ActivityChannel, SendActivityData},
};
use studycycle_db_schema::{
  newtypes::CommunityId,
  source::{
    community::{CommunityActions, CommunityFollowerForm},
    multi_community::MultiCommunity,
  },
  traits::Followable,
};
use studycycle_db_schema_file::enums::CommunityFollowerState;
use studycycle_db_views_site::SiteView;
use studycycle_utils::error::{StudyCycleError, StudyCycleResult};
use tracing::info;
use url::Url;

pub struct ApubFeedCollection;

#[async_trait::async_trait]
impl Collection for ApubFeedCollection {
  type DataType = StudyCycleContext;
  type Kind = FeedCollection;
  type Owner = ApubMultiCommunity;
  type Error = StudyCycleError;

  async fn read_local(
    owner: &Self::Owner,
    context: &Data<Self::DataType>,
  ) -> Result<Self::Kind, Self::Error> {
    let entries = MultiCommunity::read_community_ap_ids(&mut context.pool(), &owner.name).await?;
    Ok(Self::Kind {
      r#type: Default::default(),
      id: owner.following_url.clone().into(),
      total_items: entries.len().try_into()?,
      items: entries.into_iter().map(Into::into).collect(),
    })
  }

  async fn verify(
    json: &Self::Kind,
    expected_domain: &Url,
    _context: &Data<StudyCycleContext>,
  ) -> StudyCycleResult<()> {
    verify_domains_match(expected_domain, &json.id.clone().into())?;
    Ok(())
  }

  async fn from_json(
    json: Self::Kind,
    owner: &Self::Owner,
    context: &Data<StudyCycleContext>,
  ) -> StudyCycleResult<Self> {
    let communities = join_all(
      json
        .items
        .into_iter()
        .map(|ap_id| async move { Ok(ap_id.dereference(context).await?.id) }),
    )
    .await
    .into_iter()
    .flat_map(|c: StudyCycleResult<CommunityId>| match c {
      Ok(c) => Some(c),
      Err(e) => {
        info!("Failed to fetch multi-community item: {e}");
        None
      }
    })
    .collect();

    let (remote_added, remote_removed, has_local_followers) =
      MultiCommunity::update_entries(&mut context.pool(), owner.id, &communities).await?;

    // Have multi-comm follower bot follow all communities which were added to multi-comm,
    // and unfollow those that were removed.
    // If the multi-comm has no local followers its ignored.
    // TODO: This means there will be posts missing in multi-comm without local followers.
    if has_local_followers {
      let system_account = SiteView::read_system_account(&mut context.pool()).await?;
      for community in remote_added {
        let form = CommunityFollowerForm::new(
          community.id,
          system_account.id,
          CommunityFollowerState::Pending,
        );
        CommunityActions::follow(&mut context.pool(), &form).await?;
        ActivityChannel::submit_activity(
          SendActivityData::FollowCommunity(community.clone(), system_account.clone(), true),
          context,
        )?;
      }
      for community in remote_removed {
        CommunityActions::unfollow(&mut context.pool(), system_account.id, community.id).await?;
        ActivityChannel::submit_activity(
          SendActivityData::FollowCommunity(community.clone(), system_account.clone(), false),
          context,
        )?;
      }
    }

    Ok(ApubFeedCollection)
  }
}
