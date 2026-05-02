use activitypub_federation::config::Data;
use actix_web::web::{Json, Query};
use studycycle_api_utils::{context::StudyCycleContext, utils::check_private_instance};
use studycycle_db_views_local_user::LocalUserView;
use studycycle_db_views_person_saved_combined::{ListPersonSaved, impls::PersonSavedCombinedQuery};
use studycycle_db_views_post_comment_combined::PostCommentCombinedView;
use studycycle_db_views_site::SiteView;
use studycycle_diesel_utils::pagination::PagedResponse;
use studycycle_utils::error::StudyCycleResult;

pub async fn list_person_saved(
  Query(data): Query<ListPersonSaved>,
  context: Data<StudyCycleContext>,
  local_user_view: LocalUserView,
) -> StudyCycleResult<Json<PagedResponse<PostCommentCombinedView>>> {
  let local_site = SiteView::read_local(&mut context.pool()).await?;

  check_private_instance(&Some(local_user_view.clone()), &local_site.local_site)?;

  let saved = PersonSavedCombinedQuery {
    type_: data.type_,
    page_cursor: data.page_cursor,
    limit: data.limit,
    no_limit: None,
  }
  .list(&mut context.pool(), &local_user_view)
  .await?;

  Ok(Json(saved))
}
