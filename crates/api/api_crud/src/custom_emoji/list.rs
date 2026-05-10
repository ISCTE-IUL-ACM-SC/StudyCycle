use actix_web::web::{Data, Json, Query};
use studycycle_api_utils::context::StudyCycleContext;
use studycycle_db_views_custom_emoji::{
  CustomEmojiView,
  api::{ListCustomEmojis, ListCustomEmojisResponse},
};
use studycycle_utils::error::StudyCycleError;

pub async fn list_custom_emojis(
  Query(data): Query<ListCustomEmojis>,
  context: Data<StudyCycleContext>,
) -> Result<Json<ListCustomEmojisResponse>, StudyCycleError> {
  let custom_emojis = CustomEmojiView::list(&mut context.pool(), &data.category).await?;

  Ok(Json(ListCustomEmojisResponse { custom_emojis }))
}
