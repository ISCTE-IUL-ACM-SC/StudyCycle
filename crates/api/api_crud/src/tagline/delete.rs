use activitypub_federation::config::Data;
use actix_web::web::Json;
use studycycle_api_utils::{context::StudyCycleContext, utils::is_admin};
use studycycle_db_schema::source::tagline::Tagline;
use studycycle_db_views_local_user::LocalUserView;
use studycycle_db_views_site::api::{DeleteTagline, SuccessResponse};
use studycycle_diesel_utils::traits::Crud;
use studycycle_utils::error::StudyCycleError;

pub async fn delete_tagline(
  Json(data): Json<DeleteTagline>,
  context: Data<StudyCycleContext>,
  local_user_view: LocalUserView,
) -> Result<Json<SuccessResponse>, StudyCycleError> {
  // Make sure user is an admin
  is_admin(&local_user_view)?;

  Tagline::delete(&mut context.pool(), data.id).await?;

  Ok(Json(SuccessResponse::default()))
}
