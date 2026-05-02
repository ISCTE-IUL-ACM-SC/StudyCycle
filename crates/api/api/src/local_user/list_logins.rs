use actix_web::web::{Data, Json};
use studycycle_api_utils::context::StudyCycleContext;
use studycycle_db_schema::source::login_token::LoginToken;
use studycycle_db_views_local_user::LocalUserView;
use studycycle_db_views_site::api::ListLoginsResponse;
use studycycle_utils::error::StudyCycleResult;

pub async fn list_logins(
  context: Data<StudyCycleContext>,
  local_user_view: LocalUserView,
) -> StudyCycleResult<Json<ListLoginsResponse>> {
  let logins = LoginToken::list(&mut context.pool(), local_user_view.local_user.id).await?;

  Ok(Json(ListLoginsResponse { logins }))
}
