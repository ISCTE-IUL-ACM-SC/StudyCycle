use activitypub_federation::config::Data;
use actix_web::web::Json;
use either::Either;
use studycycle_api_utils::{
  context::StudyCycleContext,
  send_activity::{ActivityChannel, SendActivityData},
  utils::is_admin,
};
use studycycle_db_schema::{
  source::{community_report::CommunityReport, site::Site},
  traits::Reportable,
};
use studycycle_db_views_local_user::LocalUserView;
use studycycle_db_views_report_combined::{
  ReportCombinedViewInternal,
  api::{CommunityReportResponse, ResolveCommunityReport},
};
use studycycle_utils::error::StudyCycleResult;

pub async fn resolve_community_report(
  Json(data): Json<ResolveCommunityReport>,
  context: Data<StudyCycleContext>,
  local_user_view: LocalUserView,
) -> StudyCycleResult<Json<CommunityReportResponse>> {
  is_admin(&local_user_view)?;

  let report_id = data.report_id;
  let person = &local_user_view.person;
  CommunityReport::update_resolved(&mut context.pool(), report_id, person.id, data.resolved)
    .await?;

  let community_report_view =
    ReportCombinedViewInternal::read_community_report(&mut context.pool(), report_id, person)
      .await?;
  let site = Site::read_from_instance_id(
    &mut context.pool(),
    community_report_view.community.instance_id,
  )
  .await?;

  ActivityChannel::submit_activity(
    SendActivityData::SendResolveReport {
      object_id: community_report_view.community.ap_id.inner().clone(),
      actor: local_user_view.person,
      report_creator: community_report_view.creator.clone(),
      receiver: Either::Left(site),
    },
    &context,
  )?;

  Ok(Json(CommunityReportResponse {
    community_report_view,
  }))
}
