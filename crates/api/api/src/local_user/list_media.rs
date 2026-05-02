use actix_web::web::{Data, Json, Query};
use studycycle_api_utils::context::StudyCycleContext;
use studycycle_db_views_local_image::{LocalImageView, api::ListMedia};
use studycycle_db_views_local_user::LocalUserView;
use studycycle_diesel_utils::pagination::PagedResponse;
use studycycle_utils::error::StudyCycleResult;

pub async fn list_media(
  Query(data): Query<ListMedia>,
  context: Data<StudyCycleContext>,
  local_user_view: LocalUserView,
) -> StudyCycleResult<Json<PagedResponse<LocalImageView>>> {
  let images = LocalImageView::get_all_paged_by_person_id(
    &mut context.pool(),
    local_user_view.person.id,
    data.page_cursor,
    data.limit,
  )
  .await?;
  Ok(Json(images))
}
