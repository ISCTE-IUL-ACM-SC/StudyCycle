use crate::source::local_site_url_blocklist::{LocalSiteUrlBlocklist, LocalSiteUrlBlocklistForm};
use diesel::dsl::insert_into;
use diesel_async::{AsyncPgConnection, RunQueryDsl, scoped_futures::ScopedFutureExt};
use studycycle_db_schema_file::schema::local_site_url_blocklist;
use studycycle_diesel_utils::connection::{DbPool, get_conn};
use studycycle_utils::error::{StudyCycleErrorExt, StudyCycleErrorType, StudyCycleResult};

impl LocalSiteUrlBlocklist {
  pub async fn replace(pool: &mut DbPool<'_>, url_blocklist: Vec<String>) -> StudyCycleResult<usize> {
    let conn = &mut get_conn(pool).await?;

    conn
      .run_transaction(|conn| {
        async move {
          Self::clear(conn).await?;

          let forms = url_blocklist
            .into_iter()
            .map(|url| LocalSiteUrlBlocklistForm {
              url,
              updated_at: None,
            })
            .collect::<Vec<_>>();

          insert_into(local_site_url_blocklist::table)
            .values(forms)
            .execute(conn)
            .await
            .with_studycycle_type(StudyCycleErrorType::CouldntUpdate)
        }
        .scope_boxed()
      })
      .await
  }

  async fn clear(conn: &mut AsyncPgConnection) -> StudyCycleResult<usize> {
    diesel::delete(local_site_url_blocklist::table)
      .execute(conn)
      .await
      .with_studycycle_type(StudyCycleErrorType::Deleted)
  }

  pub async fn get_all(pool: &mut DbPool<'_>) -> StudyCycleResult<Vec<Self>> {
    let conn = &mut get_conn(pool).await?;
    local_site_url_blocklist::table
      .get_results::<Self>(conn)
      .await
      .with_studycycle_type(StudyCycleErrorType::NotFound)
  }
}
