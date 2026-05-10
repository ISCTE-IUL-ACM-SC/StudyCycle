use crate::{
  diesel::OptionalExtension,
  source::local_site_rate_limit::{
    LocalSiteRateLimit,
    LocalSiteRateLimitInsertForm,
    LocalSiteRateLimitUpdateForm,
  },
};
use diesel::dsl::insert_into;
use diesel_async::RunQueryDsl;
use studycycle_db_schema_file::schema::local_site_rate_limit;
use studycycle_diesel_utils::connection::{DbPool, get_conn};
use studycycle_utils::error::{StudyCycleErrorExt, StudyCycleErrorType, StudyCycleResult};

impl LocalSiteRateLimit {
  pub async fn read(pool: &mut DbPool<'_>) -> StudyCycleResult<Option<Self>> {
    let conn = &mut get_conn(pool).await?;
    local_site_rate_limit::table
      .first(conn)
      .await
      .optional()
      .with_studycycle_type(StudyCycleErrorType::NotFound)
  }

  pub async fn create(
    pool: &mut DbPool<'_>,
    form: &LocalSiteRateLimitInsertForm,
  ) -> StudyCycleResult<Self> {
    let conn = &mut get_conn(pool).await?;
    insert_into(local_site_rate_limit::table)
      .values(form)
      .get_result::<Self>(conn)
      .await
      .with_studycycle_type(StudyCycleErrorType::CouldntCreate)
  }
  pub async fn update(
    pool: &mut DbPool<'_>,
    form: &LocalSiteRateLimitUpdateForm,
  ) -> StudyCycleResult<()> {
    // avoid error "There are no changes to save. This query cannot be built"
    if form.is_empty() {
      return Ok(());
    }
    let conn = &mut get_conn(pool).await?;
    diesel::update(local_site_rate_limit::table)
      .set(form)
      .get_result::<Self>(conn)
      .await?;
    Ok(())
  }
}

impl LocalSiteRateLimitUpdateForm {
  fn is_empty(&self) -> bool {
    self.message_max_requests.is_none()
      && self.message_interval_seconds.is_none()
      && self.post_max_requests.is_none()
      && self.post_interval_seconds.is_none()
      && self.register_max_requests.is_none()
      && self.register_interval_seconds.is_none()
      && self.image_max_requests.is_none()
      && self.image_interval_seconds.is_none()
      && self.comment_max_requests.is_none()
      && self.comment_interval_seconds.is_none()
      && self.search_max_requests.is_none()
      && self.search_interval_seconds.is_none()
      && self.updated_at.is_none()
  }
}
