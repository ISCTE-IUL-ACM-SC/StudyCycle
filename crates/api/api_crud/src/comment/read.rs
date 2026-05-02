use actix_web::web::{Data, Json, Query};
use studycycle_api_utils::{
  build_response::build_comment_response,
  context::StudyCycleContext,
  utils::check_private_instance,
};
use studycycle_db_views_comment::api::{CommentResponse, GetComment};
use studycycle_db_views_local_user::LocalUserView;
use studycycle_db_views_site::SiteView;
use studycycle_utils::error::StudyCycleResult;

pub async fn get_comment(
  Query(data): Query<GetComment>,
  context: Data<StudyCycleContext>,
  local_user_view: Option<LocalUserView>,
) -> StudyCycleResult<Json<CommentResponse>> {
  let site_view = SiteView::read_local(&mut context.pool()).await?;
  let local_site = site_view.local_site;
  let local_instance_id = site_view.site.instance_id;

  check_private_instance(&local_user_view, &local_site)?;

  Ok(Json(
    build_comment_response(&context, data.id, local_user_view, local_instance_id).await?,
  ))
}
