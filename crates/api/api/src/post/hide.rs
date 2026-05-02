use actix_web::web::{Data, Json};
use studycycle_api_utils::{context::StudyCycleContext, utils::check_local_user_valid};
use studycycle_db_schema::source::post::{PostActions, PostHideForm};
use studycycle_db_views_local_user::LocalUserView;
use studycycle_db_views_post::{
  PostView,
  api::{HidePost, PostResponse},
};
use studycycle_utils::error::StudyCycleResult;

pub async fn hide_post(
  Json(data): Json<HidePost>,
  context: Data<StudyCycleContext>,
  local_user_view: LocalUserView,
) -> StudyCycleResult<Json<PostResponse>> {
  check_local_user_valid(&local_user_view)?;
  let person_id = local_user_view.person.id;
  let local_instance_id = local_user_view.person.instance_id;
  let post_id = data.post_id;

  let hide_form = PostHideForm::new(post_id, person_id);

  // Mark the post as hidden / unhidden
  if data.hide {
    PostActions::hide(&mut context.pool(), &hide_form).await?;
  } else {
    PostActions::unhide(&mut context.pool(), &hide_form).await?;
  }

  let post_view = PostView::read(
    &mut context.pool(),
    post_id,
    Some(&local_user_view.local_user),
    local_instance_id,
    false,
  )
  .await?;

  Ok(Json(PostResponse { post_view }))
}
