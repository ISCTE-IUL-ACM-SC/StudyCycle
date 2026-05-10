use crate::{context::StudyCycleContext, utils::is_mod_or_admin};
use actix_web::web::Json;
use studycycle_db_schema::{
  newtypes::{CommentId, CommunityId, PostId},
  source::actor_language::CommunityLanguage,
};
use studycycle_db_schema_file::InstanceId;
use studycycle_db_views_comment::{CommentView, api::CommentResponse};
use studycycle_db_views_community::{CommunityView, api::CommunityResponse};
use studycycle_db_views_local_user::LocalUserView;
use studycycle_db_views_post::{PostView, api::PostResponse};
use studycycle_utils::error::StudyCycleResult;

pub async fn build_comment_response(
  context: &StudyCycleContext,
  comment_id: CommentId,
  local_user_view: Option<LocalUserView>,
  local_instance_id: InstanceId,
) -> StudyCycleResult<CommentResponse> {
  let local_user = local_user_view.map(|l| l.local_user);
  let comment_view = CommentView::read(
    &mut context.pool(),
    comment_id,
    local_user.as_ref(),
    local_instance_id,
  )
  .await?;
  Ok(CommentResponse { comment_view })
}

pub async fn build_community_response(
  context: &StudyCycleContext,
  local_user_view: LocalUserView,
  community_id: CommunityId,
) -> StudyCycleResult<Json<CommunityResponse>> {
  let is_mod_or_admin = is_mod_or_admin(&mut context.pool(), &local_user_view, community_id)
    .await
    .is_ok();
  let local_user = local_user_view.local_user;
  let community_view = CommunityView::read(
    &mut context.pool(),
    community_id,
    Some(&local_user),
    is_mod_or_admin,
  )
  .await?;
  let discussion_languages = CommunityLanguage::read(&mut context.pool(), community_id).await?;

  Ok(Json(CommunityResponse {
    community_view,
    discussion_languages,
  }))
}

pub async fn build_post_response(
  context: &StudyCycleContext,
  community_id: CommunityId,
  local_user_view: LocalUserView,
  post_id: PostId,
) -> StudyCycleResult<Json<PostResponse>> {
  let is_mod_or_admin = is_mod_or_admin(&mut context.pool(), &local_user_view, community_id)
    .await
    .is_ok();
  let local_user = local_user_view.local_user;
  let post_view = PostView::read(
    &mut context.pool(),
    post_id,
    Some(&local_user),
    local_user_view.person.instance_id,
    is_mod_or_admin,
  )
  .await?;
  Ok(Json(PostResponse { post_view }))
}
