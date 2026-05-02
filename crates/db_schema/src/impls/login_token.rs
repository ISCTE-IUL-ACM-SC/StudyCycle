use crate::{
  diesel::{ExpressionMethods, QueryDsl},
  newtypes::LocalUserId,
  source::login_token::{LoginToken, LoginTokenCreateForm},
};
use diesel::{delete, dsl::exists, insert_into, select};
use diesel_async::RunQueryDsl;
use studycycle_db_schema_file::schema::login_token::{dsl::login_token, user_id};
use studycycle_diesel_utils::connection::{DbPool, get_conn};
use studycycle_utils::error::{StudyCycleErrorExt, StudyCycleErrorType, StudyCycleResult};

impl LoginToken {
  pub async fn create(pool: &mut DbPool<'_>, form: LoginTokenCreateForm) -> StudyCycleResult<Self> {
    let conn = &mut get_conn(pool).await?;
    insert_into(login_token)
      .values(form)
      .get_result::<Self>(conn)
      .await
      .with_studycycle_type(StudyCycleErrorType::CouldntCreate)
  }

  /// Check if the given token is valid for user.
  pub async fn validate(
    pool: &mut DbPool<'_>,
    user_id_: LocalUserId,
    token_: &str,
  ) -> StudyCycleResult<()> {
    let conn = &mut get_conn(pool).await?;
    select(exists(
      login_token.find(token_).filter(user_id.eq(user_id_)),
    ))
    .get_result::<bool>(conn)
    .await?
    .then_some(())
    .ok_or(StudyCycleErrorType::NotLoggedIn.into())
  }

  pub async fn list(pool: &mut DbPool<'_>, user_id_: LocalUserId) -> StudyCycleResult<Vec<LoginToken>> {
    let conn = &mut get_conn(pool).await?;

    login_token
      .filter(user_id.eq(user_id_))
      .get_results(conn)
      .await
      .with_studycycle_type(StudyCycleErrorType::NotFound)
  }

  /// Invalidate specific token on user logout.
  pub async fn invalidate(pool: &mut DbPool<'_>, token_: &str) -> StudyCycleResult<usize> {
    let conn = &mut get_conn(pool).await?;
    delete(login_token.find(token_))
      .execute(conn)
      .await
      .with_studycycle_type(StudyCycleErrorType::Deleted)
  }

  /// Invalidate all logins of given user on password reset/change, or account deletion.
  pub async fn invalidate_all(pool: &mut DbPool<'_>, user_id_: LocalUserId) -> StudyCycleResult<usize> {
    let conn = &mut get_conn(pool).await?;
    delete(login_token.filter(user_id.eq(user_id_)))
      .execute(conn)
      .await
      .with_studycycle_type(StudyCycleErrorType::Deleted)
  }
}
