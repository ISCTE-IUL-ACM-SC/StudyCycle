use activitypub_federation::config::Data;
use actix_web::web::Json;
use studycycle_api_utils::{
  build_response::build_post_response,
  context::StudyCycleContext,
  notify::notify_mod_action,
  send_activity::{ActivityChannel, SendActivityData},
  utils::check_community_mod_action,
};
use studycycle_db_schema::source::{
  modlog::{Modlog, ModlogInsertForm},
  post::{Post, PostUpdateForm},
};
use studycycle_db_views_local_user::LocalUserView;
use studycycle_db_views_post::{
  PostView,
  api::{LockPost, PostResponse},
};
use studycycle_diesel_utils::traits::Crud;
use studycycle_utils::error::StudyCycleResult;

pub async fn lock_post(
  Json(data): Json<LockPost>,
  context: Data<StudyCycleContext>,
  local_user_view: LocalUserView,
) -> StudyCycleResult<Json<PostResponse>> {
  let post_id = data.post_id;
  let local_instance_id = local_user_view.person.instance_id;

  let orig_post = PostView::read(
    &mut context.pool(),
    post_id,
    Some(&local_user_view.local_user),
    local_instance_id,
    false,
  )
  .await?;

  check_community_mod_action(
    &local_user_view,
    &orig_post.community,
    false,
    &mut context.pool(),
  )
  .await?;

  // Update the post
  let post_id = data.post_id;
  let locked = data.locked;
  let post = Post::update(
    &mut context.pool(),
    post_id,
    &PostUpdateForm {
      locked: Some(locked),
      ..Default::default()
    },
  )
  .await?;

  // Mod tables
  let form = ModlogInsertForm::mod_lock_post(
    local_user_view.person.id,
    &orig_post.post,
    locked,
    &data.reason,
  );
  let action = Modlog::create(&mut context.pool(), &[form]).await?;
  notify_mod_action(action.clone(), &context);

  ActivityChannel::submit_activity(
    SendActivityData::LockPost(
      post,
      local_user_view.person.clone(),
      data.locked,
      data.reason.clone(),
    ),
    &context,
  )?;

  build_post_response(&context, orig_post.community.id, local_user_view, post_id).await
}
