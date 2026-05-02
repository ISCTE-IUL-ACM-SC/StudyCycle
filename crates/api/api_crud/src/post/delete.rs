use activitypub_federation::config::Data;
use actix_web::web::Json;
use studycycle_api_utils::{
  build_response::build_post_response,
  context::StudyCycleContext,
  send_activity::{ActivityChannel, SendActivityData},
  utils::check_community_user_action,
};
use studycycle_db_schema::source::{
  community::Community,
  post::{Post, PostUpdateForm},
};
use studycycle_db_views_local_user::LocalUserView;
use studycycle_db_views_post::api::{DeletePost, PostResponse};
use studycycle_diesel_utils::traits::Crud;
use studycycle_utils::error::{StudyCycleErrorType, StudyCycleResult};

pub async fn delete_post(
  Json(data): Json<DeletePost>,
  context: Data<StudyCycleContext>,
  local_user_view: LocalUserView,
) -> StudyCycleResult<Json<PostResponse>> {
  let post_id = data.post_id;
  let orig_post = Post::read(&mut context.pool(), post_id).await?;

  // Dont delete it if its already been deleted.
  if orig_post.deleted == data.deleted {
    return Err(StudyCycleErrorType::CouldntUpdate.into());
  }

  let community = Community::read(&mut context.pool(), orig_post.community_id).await?;
  check_community_user_action(&local_user_view, &community, &mut context.pool()).await?;

  // Verify that only the creator can delete
  if !Post::is_post_creator(local_user_view.person.id, orig_post.creator_id) {
    return Err(StudyCycleErrorType::NoPostEditAllowed.into());
  }

  // Update the post
  let post = Post::update(
    &mut context.pool(),
    post_id,
    &PostUpdateForm {
      deleted: Some(data.deleted),
      ..Default::default()
    },
  )
  .await?;

  ActivityChannel::submit_activity(
    SendActivityData::DeletePost(post, local_user_view.person.clone(), community),
    &context,
  )?;

  build_post_response(&context, orig_post.community_id, local_user_view, post_id).await
}
