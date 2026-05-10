use actix_web::web::{Data, Json, Query};
use studycycle_api_utils::context::StudyCycleContext;
use studycycle_db_schema::source::tagline::Tagline;
use studycycle_db_views_site::api::ListTaglines;
use studycycle_diesel_utils::pagination::PagedResponse;
use studycycle_utils::error::StudyCycleError;

pub async fn list_taglines(
  Query(data): Query<ListTaglines>,
  context: Data<StudyCycleContext>,
) -> Result<Json<PagedResponse<Tagline>>, StudyCycleError> {
  let taglines = Tagline::list(&mut context.pool(), data.page_cursor, data.limit).await?;

  Ok(Json(taglines))
}
