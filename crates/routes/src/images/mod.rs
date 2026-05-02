use actix_web::web::*;
use studycycle_api_utils::context::StudyCycleContext;
use studycycle_db_views_site::api::SuccessResponse;
use studycycle_utils::error::StudyCycleResult;

pub mod delete;
pub mod download;
pub mod upload;
mod utils;

pub async fn pictrs_health(context: Data<StudyCycleContext>) -> StudyCycleResult<Json<SuccessResponse>> {
  let pictrs_config = context.settings().pictrs()?;
  let url = format!("{}healthz", pictrs_config.url);

  context
    .pictrs_client()
    .get(url)
    .send()
    .await?
    .error_for_status()?;

  Ok(Json(SuccessResponse::default()))
}
