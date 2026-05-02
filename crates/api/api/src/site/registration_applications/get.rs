use actix_web::web::{Data, Json, Query};
use studycycle_api_utils::{context::StudyCycleContext, utils::is_admin};
use studycycle_db_views_local_user::LocalUserView;
use studycycle_db_views_registration_applications::{
  RegistrationApplicationView,
  api::{GetRegistrationApplication, RegistrationApplicationResponse},
};
use studycycle_utils::error::StudyCycleResult;

/// Lists registration applications, filterable by undenied only.
pub async fn get_registration_application(
  Query(data): Query<GetRegistrationApplication>,
  context: Data<StudyCycleContext>,
  local_user_view: LocalUserView,
) -> StudyCycleResult<Json<RegistrationApplicationResponse>> {
  // Make sure user is an admin
  is_admin(&local_user_view)?;

  // Read the view
  let registration_application =
    RegistrationApplicationView::read_by_person(&mut context.pool(), data.person_id).await?;

  Ok(Json(RegistrationApplicationResponse {
    registration_application,
  }))
}
