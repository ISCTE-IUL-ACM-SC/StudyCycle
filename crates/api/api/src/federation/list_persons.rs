use activitypub_federation::config::Data;
use actix_web::web::{Json, Query};
use studycycle_api_utils::{context::StudyCycleContext, utils::check_private_instance};
use studycycle_db_views_local_user::LocalUserView;
use studycycle_db_views_person::{PersonView, api::ListPersons, impls::PersonQuery};
use studycycle_db_views_site::SiteView;
use studycycle_diesel_utils::pagination::PagedResponse;
use studycycle_utils::error::StudyCycleResult;

pub async fn list_persons(
  Query(data): Query<ListPersons>,
  context: Data<StudyCycleContext>,
  local_user_view: Option<LocalUserView>,
) -> StudyCycleResult<Json<PagedResponse<PersonView>>> {
  let SiteView {
    site, local_site, ..
  } = SiteView::read_local(&mut context.pool()).await?;

  check_private_instance(&local_user_view, &local_site)?;

  let res = PersonQuery {
    local_user: local_user_view.map(|l| l.local_user).as_ref(),
    sort: data.sort,
    listing_type: data.type_,
    search_term: data.search_term,
    search_title_only: data.search_title_only,
    limit: data.limit,
    page_cursor: data.page_cursor,
  }
  .list(&site, &mut context.pool())
  .await?;

  Ok(Json(res))
}
