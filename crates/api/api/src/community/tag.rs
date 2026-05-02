use activitypub_federation::config::Data;
use actix_web::web::Json;
use chrono::Utc;
use studycycle_api_utils::{
  context::StudyCycleContext,
  send_activity::{ActivityChannel, SendActivityData},
  utils::{check_community_mod_action, slur_regex},
};
use studycycle_db_schema::source::{
  community::Community,
  community_tag::{CommunityTag, CommunityTagInsertForm, CommunityTagUpdateForm},
};
use studycycle_db_views_community::{
  CommunityView,
  api::{CreateCommunityTag, DeleteCommunityTag, EditCommunityTag},
};
use studycycle_db_views_local_user::LocalUserView;
use studycycle_diesel_utils::{traits::Crud, utils::diesel_string_update};
use studycycle_utils::{
  error::StudyCycleResult,
  utils::{
    slurs::check_slurs,
    validation::{check_api_elements_count, is_valid_actor_name, summary_length_check},
  },
};
use url::Url;

pub async fn create_community_tag(
  Json(data): Json<CreateCommunityTag>,
  context: Data<StudyCycleContext>,
  local_user_view: LocalUserView,
) -> StudyCycleResult<Json<CommunityTag>> {
  is_valid_actor_name(&data.name)?;

  let community_view =
    CommunityView::read(&mut context.pool(), data.community_id, None, false).await?;
  let community = community_view.community;

  // Verify that only mods can create tags
  check_community_mod_action(&local_user_view, &community, false, &mut context.pool()).await?;

  check_api_elements_count(community_view.tags.0.len())?;
  if let Some(summary) = &data.summary {
    summary_length_check(summary)?;
    check_slurs(summary, &slur_regex(&context).await?)?;
  }

  let ap_id = Url::parse(&format!("{}/tag/{}", community.ap_id, &data.name))?;

  // Create the tag
  let tag_form = CommunityTagInsertForm {
    name: data.name.clone(),
    display_name: data.display_name.clone(),
    summary: data.summary.clone(),
    community_id: data.community_id,
    ap_id: ap_id.into(),
    deleted: Some(false),
    color: data.color,
  };

  let tag = CommunityTag::create(&mut context.pool(), &tag_form).await?;

  ActivityChannel::submit_activity(
    SendActivityData::UpdateCommunity(local_user_view.person.clone(), community),
    &context,
  )?;

  Ok(Json(tag))
}

pub async fn edit_community_tag(
  Json(data): Json<EditCommunityTag>,
  context: Data<StudyCycleContext>,
  local_user_view: LocalUserView,
) -> StudyCycleResult<Json<CommunityTag>> {
  let tag = CommunityTag::read(&mut context.pool(), data.tag_id).await?;
  let community = Community::read(&mut context.pool(), tag.community_id).await?;

  // Verify that only mods can update tags
  check_community_mod_action(&local_user_view, &community, false, &mut context.pool()).await?;

  if let Some(summary) = &data.summary {
    summary_length_check(summary)?;
    check_slurs(summary, &slur_regex(&context).await?)?;
  }

  // Update the tag
  let tag_form = CommunityTagUpdateForm {
    display_name: diesel_string_update(data.display_name.as_deref()),
    summary: diesel_string_update(data.summary.as_deref()),
    updated_at: Some(Some(Utc::now())),
    color: data.color,
    ..Default::default()
  };

  let tag = CommunityTag::update(&mut context.pool(), data.tag_id, &tag_form).await?;
  Ok(Json(tag))
}

pub async fn delete_community_tag(
  Json(data): Json<DeleteCommunityTag>,
  context: Data<StudyCycleContext>,
  local_user_view: LocalUserView,
) -> StudyCycleResult<Json<CommunityTag>> {
  let tag = CommunityTag::read(&mut context.pool(), data.tag_id).await?;
  let community = Community::read(&mut context.pool(), tag.community_id).await?;

  // Verify that only mods can delete tags
  check_community_mod_action(&local_user_view, &community, false, &mut context.pool()).await?;

  // Soft delete the tag
  let tag_form = CommunityTagUpdateForm {
    updated_at: Some(Some(Utc::now())),
    deleted: Some(data.delete),
    ..Default::default()
  };

  let tag = CommunityTag::update(&mut context.pool(), data.tag_id, &tag_form).await?;

  ActivityChannel::submit_activity(
    SendActivityData::UpdateCommunity(local_user_view.person.clone(), community),
    &context,
  )?;

  Ok(Json(tag))
}
