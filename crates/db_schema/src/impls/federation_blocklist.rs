use crate::source::federation_blocklist::{FederationBlockList, FederationBlockListForm};
use diesel::{ExpressionMethods, QueryDsl, delete, dsl::insert_into};
use diesel_async::RunQueryDsl;
use studycycle_db_schema_file::{InstanceId, schema::federation_blocklist};
use studycycle_diesel_utils::connection::{DbPool, get_conn};
use studycycle_utils::error::{StudyCycleErrorExt, StudyCycleErrorType, StudyCycleResult};

impl FederationBlockList {
  pub async fn block(pool: &mut DbPool<'_>, form: &FederationBlockListForm) -> StudyCycleResult<Self> {
    let conn = &mut get_conn(pool).await?;
    insert_into(federation_blocklist::table)
      .values(form)
      .get_result::<Self>(conn)
      .await
      .with_studycycle_type(StudyCycleErrorType::CouldntCreate)
  }
  pub async fn unblock(pool: &mut DbPool<'_>, instance_id_: InstanceId) -> StudyCycleResult<usize> {
    let conn = &mut get_conn(pool).await?;
    delete(federation_blocklist::table.filter(federation_blocklist::instance_id.eq(instance_id_)))
      .execute(conn)
      .await
      .with_studycycle_type(StudyCycleErrorType::Deleted)
  }
}
