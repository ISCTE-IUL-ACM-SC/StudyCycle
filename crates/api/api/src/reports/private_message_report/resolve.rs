use actix_web::web::{Data, Json};
use studycycle_api_utils::{context::StudyCycleContext, utils::is_admin};
use studycycle_db_schema::{source::private_message_report::PrivateMessageReport, traits::Reportable};
use studycycle_db_views_local_user::LocalUserView;
use studycycle_db_views_report_combined::{
  ReportCombinedViewInternal,
  api::{PrivateMessageReportResponse, ResolvePrivateMessageReport},
};
use studycycle_utils::error::StudyCycleResult;

pub async fn resolve_pm_report(
  Json(data): Json<ResolvePrivateMessageReport>,
  context: Data<StudyCycleContext>,
  local_user_view: LocalUserView,
) -> StudyCycleResult<Json<PrivateMessageReportResponse>> {
  is_admin(&local_user_view)?;

  let report_id = data.report_id;
  let person = &local_user_view.person;
  PrivateMessageReport::update_resolved(&mut context.pool(), report_id, person.id, data.resolved)
    .await?;

  let private_message_report_view =
    ReportCombinedViewInternal::read_private_message_report(&mut context.pool(), report_id, person)
      .await?;

  Ok(Json(PrivateMessageReportResponse {
    private_message_report_view,
  }))
}
