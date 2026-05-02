use activitypub_federation::config::Data;
use actix_web::web::Json;
use studycycle_api_utils::{
  context::StudyCycleContext,
  send_activity::{ActivityChannel, SendActivityData},
  utils::check_local_user_valid,
};
use studycycle_db_schema::source::private_message::{PrivateMessage, PrivateMessageUpdateForm};
use studycycle_db_views_local_user::LocalUserView;
use studycycle_db_views_private_message::{
  PrivateMessageView,
  api::{DeletePrivateMessage, PrivateMessageResponse},
};
use studycycle_diesel_utils::traits::Crud;
use studycycle_utils::error::{StudyCycleErrorType, StudyCycleResult};

pub async fn delete_private_message(
  Json(data): Json<DeletePrivateMessage>,
  context: Data<StudyCycleContext>,
  local_user_view: LocalUserView,
) -> StudyCycleResult<Json<PrivateMessageResponse>> {
  check_local_user_valid(&local_user_view)?;
  // Checking permissions
  let private_message_id = data.private_message_id;
  let orig_private_message = PrivateMessage::read(&mut context.pool(), private_message_id).await?;

  let deleted = data.deleted;
  let form = if local_user_view.person.id == orig_private_message.recipient_id {
    PrivateMessageUpdateForm {
      deleted_by_recipient: Some(deleted),
      ..Default::default()
    }
  } else if local_user_view.person.id == orig_private_message.creator_id {
    PrivateMessageUpdateForm {
      deleted: Some(deleted),
      ..Default::default()
    }
  } else {
    return Err(StudyCycleErrorType::EditPrivateMessageNotAllowed.into());
  };

  // Doing the update
  let private_message =
    PrivateMessage::update(&mut context.pool(), private_message_id, &form).await?;

  let view = PrivateMessageView::read(
    &mut context.pool(),
    private_message_id,
    Some(&local_user_view.person),
  )
  .await?;

  ActivityChannel::submit_activity(
    SendActivityData::DeletePrivateMessage(local_user_view.person, private_message, data.deleted),
    &context,
  )?;

  Ok(Json(PrivateMessageResponse {
    private_message_view: view,
  }))
}
