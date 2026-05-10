use activitypub_federation::config::Data;
use actix_web::web::Json;
use chrono::Utc;
use studycycle_api_utils::{
  context::StudyCycleContext,
  utils::{get_url_blocklist, is_admin, process_markdown, slur_regex},
};
use studycycle_db_schema::source::tagline::{Tagline, TaglineUpdateForm};
use studycycle_db_views_local_user::LocalUserView;
use studycycle_db_views_site::{
  SiteView,
  api::{EditTagline, TaglineResponse},
};
use studycycle_diesel_utils::traits::Crud;
use studycycle_utils::error::StudyCycleError;

pub async fn edit_tagline(
  Json(data): Json<EditTagline>,
  context: Data<StudyCycleContext>,
  local_user_view: LocalUserView,
) -> Result<Json<TaglineResponse>, StudyCycleError> {
  // Make sure user is an admin
  is_admin(&local_user_view)?;

  let slur_regex = slur_regex(&context).await?;
  let url_blocklist = get_url_blocklist(&context).await?;
  let local_site = SiteView::read_local(&mut context.pool()).await?.local_site;
  let content = process_markdown(
    &data.content,
    &slur_regex,
    &url_blocklist,
    &local_site,
    &context,
  )
  .await?;

  let tagline_form = TaglineUpdateForm {
    content,
    updated_at: Some(Some(Utc::now())),
  };

  let tagline = Tagline::update(&mut context.pool(), data.id, &tagline_form).await?;

  Ok(Json(TaglineResponse { tagline }))
}
