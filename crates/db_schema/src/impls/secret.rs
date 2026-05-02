use crate::source::secret::Secret;
use diesel_async::RunQueryDsl;
use studycycle_db_schema_file::schema::secret::dsl::secret;
use studycycle_diesel_utils::connection::{DbPool, get_conn};
use studycycle_utils::error::{StudyCycleErrorExt, StudyCycleErrorType, StudyCycleResult};

impl Secret {
  /// Initialize the Secrets from the DB.
  /// Warning: You should only call this once.
  pub async fn init(pool: &mut DbPool<'_>) -> StudyCycleResult<Secret> {
    Self::read_secrets(pool).await
  }

  async fn read_secrets(pool: &mut DbPool<'_>) -> StudyCycleResult<Self> {
    let conn = &mut get_conn(pool).await?;
    secret
      .first(conn)
      .await
      .with_studycycle_type(StudyCycleErrorType::NotFound)
  }
}
