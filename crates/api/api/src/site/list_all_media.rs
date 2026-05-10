use actix_web::web::{Data, Json, Query};
use studycycle_api_utils::{context::StudyCycleContext, utils::is_admin};
use studycycle_db_views_local_image::{LocalImageView, api::ListMedia};
use studycycle_db_views_local_user::LocalUserView;
use studycycle_diesel_utils::pagination::PagedResponse;
use studycycle_utils::error::StudyCycleResult;

pub async fn list_all_media(
  Query(data): Query<ListMedia>,
  context: Data<StudyCycleContext>,
  local_user_view: LocalUserView,
) -> StudyCycleResult<Json<PagedResponse<LocalImageView>>> {
  // Only let admins view all media
  is_admin(&local_user_view)?;

  let images =
    LocalImageView::get_all_paged(&mut context.pool(), data.page_cursor, data.limit).await?;

  Ok(Json(images))
}
