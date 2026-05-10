use crate::{
  newtypes::LocalUserId,
  source::email_verification::{EmailVerification, EmailVerificationForm},
};
use diesel::{ExpressionMethods, QueryDsl, dsl::IntervalDsl, insert_into};
use diesel_async::RunQueryDsl;
use studycycle_db_schema_file::schema::email_verification;
use studycycle_diesel_utils::{
  connection::{DbPool, get_conn},
  utils::now,
};
use studycycle_utils::error::{StudyCycleErrorExt, StudyCycleErrorType, StudyCycleResult};

impl EmailVerification {
  pub async fn create(pool: &mut DbPool<'_>, form: &EmailVerificationForm) -> StudyCycleResult<Self> {
    let conn = &mut get_conn(pool).await?;
    insert_into(email_verification::table)
      .values(form)
      .get_result(conn)
      .await
      .with_studycycle_type(StudyCycleErrorType::CouldntCreate)
  }

  pub async fn read_for_token(pool: &mut DbPool<'_>, token: &str) -> StudyCycleResult<Self> {
    let conn = &mut get_conn(pool).await?;
    email_verification::table
      .filter(email_verification::verification_token.eq(token))
      .filter(email_verification::published_at.gt(now() - 7.days()))
      .first(conn)
      .await
      .with_studycycle_type(StudyCycleErrorType::NotFound)
  }
  pub async fn delete_old_tokens_for_local_user(
    pool: &mut DbPool<'_>,
    local_user_id_: LocalUserId,
  ) -> StudyCycleResult<usize> {
    let conn = &mut get_conn(pool).await?;
    diesel::delete(
      email_verification::table.filter(email_verification::local_user_id.eq(local_user_id_)),
    )
    .execute(conn)
    .await
    .with_studycycle_type(StudyCycleErrorType::Deleted)
  }
}
