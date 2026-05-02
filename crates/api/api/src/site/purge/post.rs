use activitypub_federation::config::Data;
use actix_web::web::Json;
use studycycle_api_utils::{
  context::StudyCycleContext,
  send_activity::{ActivityChannel, SendActivityData},
  utils::{is_admin, purge_post_images},
};
use studycycle_db_schema::source::{
  local_user::LocalUser,
  modlog::{Modlog, ModlogInsertForm},
  post::Post,
};
use studycycle_db_views_local_user::LocalUserView;
use studycycle_db_views_post::api::PurgePost;
use studycycle_db_views_site::api::SuccessResponse;
use studycycle_diesel_utils::traits::Crud;
use studycycle_utils::error::StudyCycleResult;

pub async fn purge_post(
  Json(data): Json<PurgePost>,
  context: Data<StudyCycleContext>,
  local_user_view: LocalUserView,
) -> StudyCycleResult<Json<SuccessResponse>> {
  // Only let admin purge an item
  is_admin(&local_user_view)?;

  // Read the post to get the community_id
  let post = Post::read(&mut context.pool(), data.post_id).await?;

  // Also check that you're a higher admin
  LocalUser::is_higher_admin_check(
    &mut context.pool(),
    local_user_view.person.id,
    vec![post.creator_id],
  )
  .await?;

  purge_post_images(post.url.clone(), post.thumbnail_url.clone(), &context).await;

  Post::delete(&mut context.pool(), data.post_id).await?;

  // Mod tables
  let form =
    ModlogInsertForm::admin_purge_post(local_user_view.person.id, post.community_id, &data.reason);
  Modlog::create(&mut context.pool(), &[form]).await?;

  ActivityChannel::submit_activity(
    SendActivityData::RemovePost {
      post,
      moderator: local_user_view.person.clone(),
      reason: data.reason.clone(),
      removed: true,
      with_replies: false,
    },
    &context,
  )?;

  Ok(Json(SuccessResponse::default()))
}
