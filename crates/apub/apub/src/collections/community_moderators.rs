use crate::{is_new_instance, protocol::collections::group_moderators::GroupModerators};
use activitypub_federation::{
  config::Data,
  fetch::object_id::ObjectId,
  kinds::collection::OrderedCollectionType,
  protocol::verification::verify_domains_match,
  traits::Collection,
};
use studycycle_api_utils::{context::StudyCycleContext, utils::generate_moderators_url};
use studycycle_apub_objects::objects::{community::ApubCommunity, person::ApubPerson};
use studycycle_db_schema::source::community::{CommunityActions, CommunityModeratorForm};
use studycycle_db_views_community_moderator::CommunityModeratorView;
use studycycle_utils::error::{StudyCycleError, StudyCycleResult};
use url::Url;

#[derive(Clone, Debug)]
pub(crate) struct ApubCommunityModerators(());

#[async_trait::async_trait]
impl Collection for ApubCommunityModerators {
  type Owner = ApubCommunity;
  type DataType = StudyCycleContext;
  type Kind = GroupModerators;
  type Error = StudyCycleError;

  async fn read_local(owner: &Self::Owner, data: &Data<Self::DataType>) -> StudyCycleResult<Self::Kind> {
    let moderators = CommunityModeratorView::for_community(&mut data.pool(), owner.id).await?;
    let ordered_items = moderators
      .into_iter()
      .map(|m| ObjectId::<ApubPerson>::from(m.moderator.ap_id))
      .collect();
    Ok(GroupModerators {
      r#type: OrderedCollectionType::OrderedCollection,
      id: generate_moderators_url(&owner.ap_id)?.into(),
      ordered_items,
    })
  }

  async fn verify(
    group_moderators: &GroupModerators,
    expected_domain: &Url,
    _data: &Data<Self::DataType>,
  ) -> StudyCycleResult<()> {
    verify_domains_match(&group_moderators.id, expected_domain)?;
    Ok(())
  }

  async fn from_json(
    apub: Self::Kind,
    owner: &Self::Owner,
    data: &Data<Self::DataType>,
  ) -> StudyCycleResult<Self> {
    handle_community_moderators(&apub.ordered_items, owner, data).await?;

    // This return value is unused, so just set an empty vec
    Ok(ApubCommunityModerators(()))
  }
}

pub(super) async fn handle_community_moderators(
  new_mods: &Vec<ObjectId<ApubPerson>>,
  community: &ApubCommunity,
  context: &Data<StudyCycleContext>,
) -> StudyCycleResult<()> {
  let community_id = community.id;
  let current_moderators =
    CommunityModeratorView::for_community(&mut context.pool(), community_id).await?;
  // Remove old mods from database which arent in the moderators collection anymore
  for mod_user in &current_moderators {
    let mod_id = ObjectId::from(mod_user.moderator.ap_id.clone());
    if !new_mods.contains(&mod_id) {
      let community_moderator_form =
        CommunityModeratorForm::new(mod_user.community.id, mod_user.moderator.id);
      CommunityActions::leave(&mut context.pool(), &community_moderator_form).await?;
    }
  }

  // Add new mods to database which have been added to moderators collection
  for mod_id in new_mods {
    // Ignore errors as mod accounts might be deleted or instances unavailable.
    let mod_user: Option<ApubPerson> = mod_id.dereference(context).await.ok();
    if let Some(mod_user) = mod_user
      && !current_moderators
        .iter()
        .any(|x| x.moderator.ap_id == mod_user.ap_id)
    {
      let community_moderator_form = CommunityModeratorForm::new(community.id, mod_user.id);
      CommunityActions::join(&mut context.pool(), &community_moderator_form).await?;
    }

    // Only add the top mod in case of new instance
    if is_new_instance(context).await? {
      return Ok(());
    }
  }
  Ok(())
}

#[cfg(test)]
#[expect(clippy::indexing_slicing)]
mod tests {

  use super::*;
  use studycycle_apub_objects::utils::test::{
    file_to_json_object,
    parse_studycycle_community,
    parse_studycycle_person,
  };
  use studycycle_db_schema::{
    source::community::{CommunityActions, CommunityModeratorForm},
    test_data::TestData,
  };
  use pretty_assertions::assert_eq;
  use serial_test::serial;

  #[tokio::test]
  #[serial]
  async fn test_parse_studycycle_community_moderators() -> StudyCycleResult<()> {
    let context = StudyCycleContext::init_test_context().await;
    let data = TestData::create(&mut context.pool()).await?;
    let (new_mod, site) = parse_studycycle_person(&context).await?;
    let community = parse_studycycle_community(&context).await?;
    let community_id = community.id;

    let community_moderator_form = CommunityModeratorForm::new(community.id, data.person.id);

    CommunityActions::join(&mut context.pool(), &community_moderator_form).await?;

    assert_eq!(site.ap_id.to_string(), "https://enterprise.lemmy.ml/");

    let json: GroupModerators =
      file_to_json_object("assets/studycycle/collections/group_moderators.json")?;
    let url = Url::parse("https://enterprise.lemmy.ml/c/tenforward")?;
    ApubCommunityModerators::verify(&json, &url, &context).await?;
    ApubCommunityModerators::from_json(json, &community, &context).await?;
    assert_eq!(context.request_count(), 0);

    let current_moderators =
      CommunityModeratorView::for_community(&mut context.pool(), community_id).await?;

    assert_eq!(current_moderators.len(), 1);
    assert_eq!(current_moderators[0].moderator.id, new_mod.id);

    data.delete(&mut context.pool()).await?;
    Ok(())
  }
}
