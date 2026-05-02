use activitypub_federation::config::Data;
use actix_web::web::Json;
use studycycle_api_utils::{
  context::StudyCycleContext,
  utils::{check_community_mod_of_any_or_admin_action, is_admin},
};
use studycycle_db_views_community_follower_approval::PendingFollowerView;
use studycycle_db_views_local_user::LocalUserView;
use studycycle_db_views_notification::NotificationView;
use studycycle_db_views_registration_applications::RegistrationApplicationView;
use studycycle_db_views_report_combined::ReportCombinedViewInternal;
use studycycle_db_views_site::{SiteView, api::UnreadCountsResponse};
use studycycle_utils::error::StudyCycleResult;

pub async fn get_unread_counts(
  context: Data<StudyCycleContext>,
  local_user_view: LocalUserView,
) -> StudyCycleResult<Json<UnreadCountsResponse>> {
  let person = &local_user_view.person;
  let show_bot_accounts = local_user_view.local_user.show_bot_accounts;

  let notification_count =
    NotificationView::get_unread_count(&mut context.pool(), person, show_bot_accounts).await?;

  // Community mods get additional counts for reports and pending follows for private communities.
  let (report_count, pending_follow_count) =
    if check_community_mod_of_any_or_admin_action(&local_user_view, &mut context.pool())
      .await
      .is_ok()
    {
      (
        Some(
          ReportCombinedViewInternal::get_report_count(&mut context.pool(), &local_user_view)
            .await?,
        ),
        Some(PendingFollowerView::count_approval_required(&mut context.pool(), person.id).await?),
      )
    } else {
      (None, None)
    };

  // Admins also get the number of unread registration applications.
  let registration_application_count = if is_admin(&local_user_view).is_ok() {
    let local_site = SiteView::read_local(&mut context.pool()).await?.local_site;
    let verified_email_only = local_site.email_verification_required;
    Some(
      RegistrationApplicationView::get_unread_count(&mut context.pool(), verified_email_only)
        .await?,
    )
  } else {
    None
  };

  Ok(Json(UnreadCountsResponse {
    notification_count,
    report_count,
    pending_follow_count,
    registration_application_count,
  }))
}
