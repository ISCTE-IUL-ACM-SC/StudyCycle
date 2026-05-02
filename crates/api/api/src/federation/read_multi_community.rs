use crate::federation::fetcher::resolve_multi_community_identifier;
use activitypub_federation::config::Data;
use actix_web::web::{Json, Query};
use studycycle_api_utils::context::StudyCycleContext;
use studycycle_db_views_community::{
  MultiCommunityView,
  api::{GetMultiCommunity, GetMultiCommunityResponse},
  impls::CommunityQuery,
};
use studycycle_db_views_local_user::LocalUserView;
use studycycle_db_views_site::SiteView;
use studycycle_utils::error::{StudyCycleErrorType, StudyCycleResult};

pub async fn read_multi_community(
  Query(data): Query<GetMultiCommunity>,
  context: Data<StudyCycleContext>,
  local_user_view: Option<LocalUserView>,
) -> StudyCycleResult<Json<GetMultiCommunityResponse>> {
  let my_person_id = local_user_view.as_ref().map(|l| l.person.id);
  let id = resolve_multi_community_identifier(&data.name, data.id, &context, &local_user_view)
    .await?
    .ok_or(StudyCycleErrorType::NoIdGiven)?;
  let multi_community_view =
    MultiCommunityView::read(&mut context.pool(), id, my_person_id).await?;

  let local_site = SiteView::read_local(&mut context.pool()).await?;
  let communities = CommunityQuery {
    multi_community_id: Some(id),
    ..Default::default()
  }
  .list(&local_site.site, &mut context.pool())
  .await?
  .items;

  Ok(Json(GetMultiCommunityResponse {
    multi_community_view,
    communities,
  }))
}
