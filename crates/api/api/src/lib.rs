use studycycle_api_utils::{context::StudyCycleContext, utils::is_mod_or_admin_opt};
use studycycle_db_schema::newtypes::CommunityId;
use studycycle_db_views_local_user::LocalUserView;
use studycycle_utils::{
  error::{StudyCycleErrorExt, StudyCycleErrorType, StudyCycleResult},
  utils::slurs::check_slurs,
};
use regex::Regex;
use totp_rs::{Secret, TOTP};

pub mod comment;
pub mod community;
pub mod federation;
pub mod local_user;
pub mod post;
pub mod reports;
pub mod site;
pub mod sitemap;

/// Check size of report
pub(crate) fn check_report_reason(reason: &str, slur_regex: &Regex) -> StudyCycleResult<()> {
  check_slurs(reason, slur_regex)?;
  if reason.is_empty() {
    Err(StudyCycleErrorType::ReportReasonRequired.into())
  } else if reason.chars().count() > 1000 {
    Err(StudyCycleErrorType::ReportTooLong.into())
  } else {
    Ok(())
  }
}

pub(crate) fn check_totp_2fa_valid(
  local_user_view: &LocalUserView,
  totp_token: &Option<String>,
  site_name: &str,
) -> StudyCycleResult<()> {
  // Throw an error if their token is missing
  let token = totp_token
    .as_deref()
    .ok_or(StudyCycleErrorType::MissingTotpToken)?;
  let secret = local_user_view
    .local_user
    .totp_2fa_secret
    .as_deref()
    .ok_or(StudyCycleErrorType::MissingTotpSecret)?;

  let totp = build_totp_2fa(site_name, &local_user_view.person.name, secret)?;

  let check_passed = totp.check_current(token)?;
  if !check_passed {
    return Err(StudyCycleErrorType::IncorrectTotpToken.into());
  }

  Ok(())
}

pub(crate) fn generate_totp_2fa_secret() -> String {
  Secret::generate_secret().to_string()
}

fn build_totp_2fa(hostname: &str, username: &str, secret: &str) -> StudyCycleResult<TOTP> {
  let sec = Secret::Raw(secret.as_bytes().to_vec());
  let sec_bytes = sec
    .to_bytes()
    .with_studycycle_type(StudyCycleErrorType::CouldntParseTotpSecret)?;

  TOTP::new(
    totp_rs::Algorithm::SHA1,
    6,
    1,
    30,
    sec_bytes,
    Some(hostname.to_string()),
    username.to_string(),
  )
  .with_studycycle_type(StudyCycleErrorType::CouldntGenerateTotp)
}

/// Only show the modlog names if:
/// You're an admin or
/// You're fetching the modlog for a single community, and you're a mod
/// (Alternatively !admin/mod)
async fn hide_modlog_names(
  local_user_view: Option<&LocalUserView>,
  community_id: Option<CommunityId>,
  context: &StudyCycleContext,
) -> bool {
  if let Some(community_id) = community_id {
    is_mod_or_admin_opt(&mut context.pool(), local_user_view, Some(community_id))
      .await
      .is_err()
  } else {
    !local_user_view
      .map(|l| l.local_user.admin)
      .unwrap_or_default()
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_build_totp() {
    let generated_secret = generate_totp_2fa_secret();
    let totp = build_totp_2fa("lemmy.ml", "my_name", &generated_secret);
    assert!(totp.is_ok());
  }
}
