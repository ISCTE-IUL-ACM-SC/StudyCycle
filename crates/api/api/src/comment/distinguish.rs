use activitypub_federation::config::Data;
use actix_web::web::Json;
use studycycle_api_utils::{
  context::StudyCycleContext,
  send_activity::{ActivityChannel, SendActivityData},
  utils::{check_community_mod_action, check_community_user_action},
};
use studycycle_db_schema::source::comment::{Comment, CommentUpdateForm};
use studycycle_db_views_comment::{
  CommentView,
  api::{CommentResponse, DistinguishComment},
};
use studycycle_db_views_local_user::LocalUserView;
use studycycle_diesel_utils::traits::Crud;
use studycycle_utils::error::{StudyCycleErrorType, StudyCycleResult};

pub async fn distinguish_comment(
  Json(data): Json<DistinguishComment>,
  context: Data<StudyCycleContext>,
  local_user_view: LocalUserView,
) -> StudyCycleResult<Json<CommentResponse>> {
  let local_instance_id = local_user_view.person.instance_id;

  let orig_comment = CommentView::read(
    &mut context.pool(),
    data.comment_id,
    Some(&local_user_view.local_user),
    local_instance_id,
  )
  .await?;

  check_community_user_action(
    &local_user_view,
    &orig_comment.community,
    &mut context.pool(),
  )
  .await?;

  // Verify that only the creator can distinguish
  if local_user_view.person.id != orig_comment.creator.id {
    return Err(StudyCycleErrorType::NoCommentEditAllowed.into());
  }

  // Verify that only a mod or admin can distinguish a comment
  check_community_mod_action(
    &local_user_view,
    &orig_comment.community,
    false,
    &mut context.pool(),
  )
  .await?;

  // Update the Comment
  let form = CommentUpdateForm {
    distinguished: Some(data.distinguished),
    ..Default::default()
  };

  let comment = Comment::update(&mut context.pool(), data.comment_id, &form).await?;
  ActivityChannel::submit_activity(SendActivityData::UpdateComment(comment), &context)?;

  let comment_view = CommentView::read(
    &mut context.pool(),
    data.comment_id,
    Some(&local_user_view.local_user),
    local_instance_id,
  )
  .await?;

  Ok(Json(CommentResponse { comment_view }))
}
