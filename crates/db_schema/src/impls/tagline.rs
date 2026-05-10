use crate::{
  newtypes::TaglineId,
  source::tagline::{Tagline, TaglineInsertForm, TaglineUpdateForm, tagline_keys as key},
  utils::limit_fetch,
};
use diesel::{QueryDsl, insert_into};
use diesel_async::RunQueryDsl;
use i_love_jesus::SortDirection;
use studycycle_db_schema_file::schema::tagline;
use studycycle_diesel_utils::{
  connection::{DbPool, get_conn},
  pagination::{
    CursorData,
    PagedResponse,
    PaginationCursor,
    PaginationCursorConversion,
    paginate_response,
  },
  traits::Crud,
  utils::functions::random,
};
use studycycle_utils::error::{StudyCycleErrorExt, StudyCycleErrorType, StudyCycleResult};

impl Crud for Tagline {
  type InsertForm = TaglineInsertForm;
  type UpdateForm = TaglineUpdateForm;
  type IdType = TaglineId;

  async fn create(pool: &mut DbPool<'_>, form: &Self::InsertForm) -> StudyCycleResult<Self> {
    let conn = &mut get_conn(pool).await?;
    insert_into(tagline::table)
      .values(form)
      .get_result::<Self>(conn)
      .await
      .with_studycycle_type(StudyCycleErrorType::CouldntCreate)
  }

  async fn update(
    pool: &mut DbPool<'_>,
    tagline_id: TaglineId,
    form: &Self::UpdateForm,
  ) -> StudyCycleResult<Self> {
    let conn = &mut get_conn(pool).await?;
    diesel::update(tagline::table.find(tagline_id))
      .set(form)
      .get_result::<Self>(conn)
      .await
      .with_studycycle_type(StudyCycleErrorType::CouldntUpdate)
  }
}

impl PaginationCursorConversion for Tagline {
  type PaginatedType = Tagline;

  fn to_cursor(&self) -> CursorData {
    CursorData::new_id(self.id.0)
  }

  async fn from_cursor(
    cursor: CursorData,
    pool: &mut DbPool<'_>,
  ) -> StudyCycleResult<Self::PaginatedType> {
    Tagline::read(pool, TaglineId(cursor.id()?)).await
  }
}

impl Tagline {
  pub async fn list(
    pool: &mut DbPool<'_>,
    page_cursor: Option<PaginationCursor>,
    limit: Option<i64>,
  ) -> StudyCycleResult<PagedResponse<Self>> {
    let limit = limit_fetch(limit, None)?;
    let query = tagline::table.limit(limit).into_boxed();
    let paginated_query = Self::paginate(query, &page_cursor, SortDirection::Desc, pool)
      .await?
      .then_order_by(key::published_at)
      .then_order_by(key::id);

    let conn = &mut get_conn(pool).await?;
    let res = paginated_query
      .load::<Self>(conn)
      .await
      .with_studycycle_type(StudyCycleErrorType::NotFound)?;
    paginate_response(res, limit, page_cursor)
  }

  pub async fn get_random(pool: &mut DbPool<'_>) -> StudyCycleResult<Self> {
    let conn = &mut get_conn(pool).await?;
    tagline::table
      .order(random())
      .limit(1)
      .first::<Self>(conn)
      .await
      .with_studycycle_type(StudyCycleErrorType::NotFound)
  }
}
