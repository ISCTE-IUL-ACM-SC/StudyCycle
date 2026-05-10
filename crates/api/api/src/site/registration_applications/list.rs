use activitypub_federation::config::Data;
use actix_web::web::{Json, Query};
use studycycle_api_utils::{context::StudyCycleContext, utils::is_admin};
use studycycle_db_views_local_user::LocalUserView;
use studycycle_db_views_registration_applications::{
  RegistrationApplicationView,
  api::ListRegistrationApplications,
  impls::RegistrationApplicationQuery,
};
use studycycle_db_views_site::SiteView;
use studycycle_diesel_utils::pagination::PagedResponse;
use studycycle_utils::error::StudyCycleResult;

/// Lists registration applications, filterable by undenied only.
pub async fn list_registration_applications(
  Query(data): Query<ListRegistrationApplications>,
  context: Data<StudyCycleContext>,
  local_user_view: LocalUserView,
) -> StudyCycleResult<Json<PagedResponse<RegistrationApplicationView>>> {
  let local_site = SiteView::read_local(&mut context.pool()).await?.local_site;

  // Make sure user is an admin
  is_admin(&local_user_view)?;

  let registration_applications = RegistrationApplicationQuery {
    unread_only: data.unread_only,
    verified_email_only: Some(local_site.email_verification_required),
    page_cursor: data.page_cursor,
    limit: data.limit,
  }
  .list(&mut context.pool())
  .await?;

  Ok(Json(registration_applications))
}
