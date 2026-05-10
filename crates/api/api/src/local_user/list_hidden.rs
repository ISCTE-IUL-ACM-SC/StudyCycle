use activitypub_federation::config::Data;
use actix_web::web::{Json, Query};
use studycycle_api_utils::context::StudyCycleContext;
use studycycle_db_views_local_user::LocalUserView;
use studycycle_db_views_person_content_combined::api::ListPersonHidden;
use studycycle_db_views_post::PostView;
use studycycle_diesel_utils::pagination::PagedResponse;
use studycycle_utils::error::StudyCycleResult;

pub async fn list_person_hidden(
  Query(data): Query<ListPersonHidden>,
  context: Data<StudyCycleContext>,
  local_user_view: LocalUserView,
) -> StudyCycleResult<Json<PagedResponse<PostView>>> {
  let hidden = PostView::list_hidden(
    &mut context.pool(),
    &local_user_view.person,
    data.page_cursor,
    data.limit,
    None,
  )
  .await?;

  Ok(Json(hidden))
}
