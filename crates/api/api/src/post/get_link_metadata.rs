use actix_web::web::{Data, Json, Query};
use studycycle_api_utils::{context::StudyCycleContext, request::fetch_link_metadata};
use studycycle_db_views_local_user::LocalUserView;
use studycycle_db_views_post::api::{GetSiteMetadata, GetSiteMetadataResponse};
use studycycle_utils::error::{StudyCycleErrorExt, StudyCycleErrorType, StudyCycleResult};
use url::Url;

pub async fn get_link_metadata(
  Query(data): Query<GetSiteMetadata>,
  context: Data<StudyCycleContext>,
  // Require an account for this API
  _local_user_view: LocalUserView,
) -> StudyCycleResult<Json<GetSiteMetadataResponse>> {
  let url = Url::parse(&data.url).with_studycycle_type(StudyCycleErrorType::InvalidUrl)?;
  let metadata = fetch_link_metadata(&url, &context, false).await?;

  Ok(Json(GetSiteMetadataResponse { metadata }))
}
