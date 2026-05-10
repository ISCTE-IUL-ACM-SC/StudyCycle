use actix_web::web::{Data, Json};
use studycycle_api_utils::{
  context::StudyCycleContext,
  plugins::{is_captcha_plugin_loaded, plugin_metadata},
};
use studycycle_db_schema::source::{
  actor_language::SiteLanguage,
  language::Language,
  local_site_url_blocklist::LocalSiteUrlBlocklist,
  oauth_provider::AdminOAuthProvider,
  registration_application::RegistrationApplication,
  tagline::Tagline,
};
use studycycle_db_views_local_user::LocalUserView;
use studycycle_db_views_person::PersonView;
use studycycle_db_views_site::{SiteView, api::GetSiteResponse};
use studycycle_utils::{CacheLock, VERSION, build_cache, error::StudyCycleResult};
use std::sync::LazyLock;

pub async fn get_site(
  local_user_view: Option<LocalUserView>,
  context: Data<StudyCycleContext>,
) -> StudyCycleResult<Json<GetSiteResponse>> {
  // This data is independent from the user account so we can cache it across requests
  static CACHE: CacheLock<GetSiteResponse> = LazyLock::new(build_cache);
  let mut site_response = Box::pin(CACHE.try_get_with((), read_site(&context)))
    .await
    .map_err(|e| anyhow::anyhow!("Failed to construct site response: {e}"))?;

  // filter oauth_providers for public access
  if !local_user_view
    .map(|l| l.local_user.admin)
    .unwrap_or_default()
  {
    site_response.admin_oauth_providers = vec![];
  }

  Ok(Json(site_response))
}

async fn read_site(context: &StudyCycleContext) -> StudyCycleResult<GetSiteResponse> {
  let site_view = SiteView::read_local(&mut context.pool()).await?;
  let admins = PersonView::list_admins(None, site_view.instance.id, &mut context.pool()).await?;
  let all_languages = Language::read_all(&mut context.pool()).await?;
  let discussion_languages = SiteLanguage::read_local_raw(&mut context.pool()).await?;
  let blocked_urls = LocalSiteUrlBlocklist::get_all(&mut context.pool()).await?;
  let tagline = Tagline::get_random(&mut context.pool()).await.ok();
  let admin_oauth_providers = AdminOAuthProvider::get_all(&mut context.pool()).await?;
  let oauth_providers =
    AdminOAuthProvider::convert_providers_to_public(admin_oauth_providers.clone());
  let last_application_duration_seconds =
    RegistrationApplication::last_updated(&mut context.pool())
      .await
      .ok()
      .and_then(|u| u.updated_published_duration());

  Ok(GetSiteResponse {
    site_view,
    admins,
    version: VERSION.to_string(),
    all_languages,
    discussion_languages,
    blocked_urls,
    tagline,
    oauth_providers,
    admin_oauth_providers,
    active_plugins: plugin_metadata(),
    last_application_duration_seconds,
    captcha_enabled: is_captcha_plugin_loaded(),
  })
}
