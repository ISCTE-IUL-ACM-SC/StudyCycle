use actix_web::web::{Data, Json, Query};
use studycycle_api_utils::context::StudyCycleContext;
use studycycle_db_views_site::{FederatedInstanceView, api::GetFederatedInstances};
use studycycle_diesel_utils::pagination::PagedResponse;
use studycycle_utils::error::StudyCycleResult;

pub async fn get_federated_instances(
  Query(data): Query<GetFederatedInstances>,
  context: Data<StudyCycleContext>,
) -> StudyCycleResult<Json<PagedResponse<FederatedInstanceView>>> {
  let federated_instances = FederatedInstanceView::list(&mut context.pool(), data).await?;

  // Return the jwt
  Ok(Json(federated_instances))
}
