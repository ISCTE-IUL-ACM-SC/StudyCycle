use crate::{
  newtypes::LocalUserId,
  source::oauth_account::{OAuthAccount, OAuthAccountInsertForm},
};
use diesel::{ExpressionMethods, QueryDsl, insert_into};
use diesel_async::RunQueryDsl;
use studycycle_db_schema_file::schema::{oauth_account, oauth_account::dsl::local_user_id};
use studycycle_diesel_utils::connection::{DbPool, get_conn};
use studycycle_utils::error::{StudyCycleErrorExt, StudyCycleErrorType, StudyCycleResult};

impl OAuthAccount {
  pub async fn create(pool: &mut DbPool<'_>, form: &OAuthAccountInsertForm) -> StudyCycleResult<Self> {
    let conn = &mut get_conn(pool).await?;
    insert_into(oauth_account::table)
      .values(form)
      .get_result::<Self>(conn)
      .await
      .with_studycycle_type(StudyCycleErrorType::CouldntCreate)
  }

  pub async fn delete_user_accounts(
    pool: &mut DbPool<'_>,
    for_local_user_id: LocalUserId,
  ) -> StudyCycleResult<usize> {
    let conn = &mut get_conn(pool).await?;

    diesel::delete(oauth_account::table.filter(local_user_id.eq(for_local_user_id)))
      .execute(conn)
      .await
      .with_studycycle_type(StudyCycleErrorType::Deleted)
  }
}
