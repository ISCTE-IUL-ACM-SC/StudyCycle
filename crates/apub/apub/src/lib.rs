use activitypub_federation::{config::UrlVerifier, error::Error as ActivityPubError};
use async_trait::async_trait;
use chrono::{Days, Utc};
use studycycle_api_utils::context::StudyCycleContext;
use studycycle_apub_objects::utils::functions::{check_apub_id_valid, local_site_data_cached};
use studycycle_db_views_site::SiteView;
use studycycle_diesel_utils::connection::ActualDbPool;
use studycycle_utils::error::{StudyCycleError, StudyCycleErrorType, StudyCycleResult, UntranslatedError};
use url::Url;

pub mod collections;
pub mod http;
pub mod protocol;

/// Maximum number of outgoing HTTP requests to fetch a single object. Needs to be high enough
/// to fetch a new community with posts, moderators and featured posts.
pub const FEDERATION_HTTP_FETCH_LIMIT: u32 = 100;

#[derive(Clone)]
pub struct VerifyUrlData(pub ActualDbPool);

#[async_trait]
impl UrlVerifier for VerifyUrlData {
  async fn verify(&self, url: &Url) -> Result<(), ActivityPubError> {
    use UntranslatedError::*;
    let local_site_data = local_site_data_cached(&mut (&self.0).into())
      .await
      .map_err(|e| ActivityPubError::Other(format!("Cant read local site data: {e}")))?;

    check_apub_id_valid(url, &local_site_data).map_err(|err| match err {
      StudyCycleError {
        error_type: StudyCycleErrorType::UntranslatedError(Some(FederationDisabled)),
        ..
      } => ActivityPubError::Other("Federation disabled".into()),
      StudyCycleError {
        error_type: StudyCycleErrorType::UntranslatedError(Some(DomainBlocked(domain))),
        ..
      } => ActivityPubError::Other(format!("Domain {domain:?} is blocked")),
      StudyCycleError {
        error_type: StudyCycleErrorType::UntranslatedError(Some(DomainNotInAllowList(domain))),
        ..
      } => ActivityPubError::Other(format!("Domain {domain:?} is not in allowlist")),
      _ => ActivityPubError::Other("Failed validating apub id".into()),
    })?;
    Ok(())
  }
}

/// Returns true if the local instance was created in the last 24 hours. In this case StudyCycle should
/// fetch less data over federation, because the setup task fetches a lot of communities.
async fn is_new_instance(context: &StudyCycleContext) -> StudyCycleResult<bool> {
  let local_site = SiteView::read_local(&mut context.pool()).await?.local_site;
  Ok(local_site.published_at - Days::new(1) < Utc::now())
}
