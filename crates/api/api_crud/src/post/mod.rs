use chrono::{DateTime, TimeZone, Utc};
use studycycle_api_utils::context::StudyCycleContext;
use studycycle_db_schema::source::post::Post;
use studycycle_db_views_local_user::LocalUserView;
use studycycle_utils::error::{StudyCycleErrorType, StudyCycleResult};

pub mod create;
pub mod delete;
pub mod read;
pub mod remove;
pub mod update;

async fn convert_published_time(
  scheduled_publish_time: Option<i64>,
  local_user_view: &LocalUserView,
  context: &StudyCycleContext,
) -> StudyCycleResult<Option<DateTime<Utc>>> {
  const MAX_SCHEDULED_POSTS: i64 = 10;
  if let Some(scheduled_publish_time) = scheduled_publish_time {
    let converted = Utc
      .timestamp_opt(scheduled_publish_time, 0)
      .single()
      .ok_or(StudyCycleErrorType::InvalidUnixTime)?;
    if converted < Utc::now() {
      return Err(StudyCycleErrorType::PostScheduleTimeMustBeInFuture.into());
    }
    if !local_user_view.local_user.admin {
      let count =
        Post::user_scheduled_post_count(local_user_view.person.id, &mut context.pool()).await?;
      if count >= MAX_SCHEDULED_POSTS {
        return Err(StudyCycleErrorType::TooManyScheduledPosts.into());
      }
    }
    Ok(Some(converted))
  } else {
    Ok(None)
  }
}
