pub mod note;
pub(crate) mod note_wrapper;
pub mod page;
pub mod private_message;

#[cfg(test)]
mod tests {
  use super::note_wrapper::{CreateOrUpdateNoteWrapper, NoteWrapper};
  use crate::protocol::create_or_update::{
    note::CreateOrUpdateNote,
    page::CreateOrUpdatePage,
    private_message::CreateOrUpdatePrivateMessage,
  };
  use studycycle_apub_objects::utils::test::test_parse_studycycle_item;
  use studycycle_utils::error::StudyCycleResult;

  #[test]
  fn test_parse_studycycle_create_or_update() -> StudyCycleResult<()> {
    test_parse_studycycle_item::<CreateOrUpdatePage>(
      "../apub/assets/studycycle/activities/create_or_update/create_page.json",
    )?;
    test_parse_studycycle_item::<CreateOrUpdatePage>(
      "../apub/assets/studycycle/activities/create_or_update/update_page.json",
    )?;
    test_parse_studycycle_item::<CreateOrUpdateNote>(
      "../apub/assets/studycycle/activities/create_or_update/create_comment.json",
    )?;
    test_parse_studycycle_item::<CreateOrUpdatePrivateMessage>(
      "../apub/assets/studycycle/activities/create_or_update/create_private_message.json",
    )?;
    test_parse_studycycle_item::<CreateOrUpdateNoteWrapper>(
      "../apub/assets/studycycle/activities/create_or_update/create_comment.json",
    )?;
    test_parse_studycycle_item::<CreateOrUpdateNoteWrapper>(
      "../apub/assets/studycycle/activities/create_or_update/create_private_message.json",
    )?;
    test_parse_studycycle_item::<NoteWrapper>("../apub/assets/studycycle/objects/comment.json")?;
    test_parse_studycycle_item::<NoteWrapper>("../apub/assets/studycycle/objects/private_message.json")?;
    Ok(())
  }
}
