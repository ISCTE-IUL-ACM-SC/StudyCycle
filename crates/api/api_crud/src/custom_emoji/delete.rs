use activitypub_federation::config::Data;
use actix_web::web::Json;
use studycycle_api_utils::{context::StudyCycleContext, utils::is_admin};
use studycycle_db_schema::source::custom_emoji::CustomEmoji;
use studycycle_db_views_custom_emoji::api::DeleteCustomEmoji;
use studycycle_db_views_local_user::LocalUserView;
use studycycle_db_views_site::api::SuccessResponse;
use studycycle_diesel_utils::traits::Crud;
use studycycle_utils::error::StudyCycleResult;

pub async fn delete_custom_emoji(
  Json(data): Json<DeleteCustomEmoji>,
  context: Data<StudyCycleContext>,
  local_user_view: LocalUserView,
) -> StudyCycleResult<Json<SuccessResponse>> {
  // Make sure user is an admin
  is_admin(&local_user_view)?;

  CustomEmoji::delete(&mut context.pool(), data.id).await?;

  Ok(Json(SuccessResponse::default()))
}
