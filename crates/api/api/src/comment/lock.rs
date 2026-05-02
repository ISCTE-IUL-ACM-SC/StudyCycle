use activitypub_federation::config::Data;
use actix_web::web::Json;
use studycycle_api_utils::{
  build_response::build_comment_response,
  context::StudyCycleContext,
  notify::notify_mod_action,
  send_activity::{ActivityChannel, SendActivityData},
  utils::check_community_mod_action,
};
use studycycle_db_schema::source::{
  comment::Comment,
  modlog::{Modlog, ModlogInsertForm},
};
use studycycle_db_views_comment::{
  CommentView,
  api::{CommentResponse, LockComment},
};
use studycycle_db_views_local_user::LocalUserView;
use studycycle_utils::error::{StudyCycleErrorType, StudyCycleResult};

pub async fn lock_comment(
  Json(data): Json<LockComment>,
  context: Data<StudyCycleContext>,
  local_user_view: LocalUserView,
) -> StudyCycleResult<Json<CommentResponse>> {
  let comment_id = data.comment_id;
  let local_instance_id = local_user_view.person.instance_id;
  let locked = data.locked;

  let orig_comment =
    CommentView::read(&mut context.pool(), comment_id, None, local_instance_id).await?;

  check_community_mod_action(
    &local_user_view,
    &orig_comment.community,
    false,
    &mut context.pool(),
  )
  .await?;

  let comments = Comment::update_locked_for_comment_and_children(
    &mut context.pool(),
    &orig_comment.comment.path,
    locked,
  )
  .await?;
  let comment = comments.first().ok_or(StudyCycleErrorType::NotFound)?;

  let form = ModlogInsertForm::mod_lock_comment(
    local_user_view.person.id,
    comment,
    orig_comment.community.id,
    locked,
    &data.reason,
  );
  let action = Modlog::create(&mut context.pool(), &[form]).await?;
  notify_mod_action(action.clone(), &context);

  ActivityChannel::submit_activity(
    SendActivityData::LockComment(
      comment.clone(),
      local_user_view.person.clone(),
      data.locked,
      data.reason.clone(),
    ),
    &context,
  )?;

  build_comment_response(
    &context,
    comment_id,
    local_user_view.into(),
    local_instance_id,
  )
  .await
  .map(Json)
}
