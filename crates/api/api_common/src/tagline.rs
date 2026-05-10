pub use studycycle_db_schema::{newtypes::TaglineId, source::tagline::Tagline};
pub use studycycle_db_views_site::api::{ListTaglines, TaglineResponse};

pub mod administration {
  pub use studycycle_db_views_site::api::{CreateTagline, DeleteTagline, EditTagline};
}
