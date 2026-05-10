pub mod delete;
pub mod delete_user;
pub mod undo_delete;

#[cfg(test)]
mod tests {
  use crate::protocol::deletion::{
    delete::Delete,
    delete_user::DeleteUser,
    undo_delete::UndoDelete,
  };
  use studycycle_apub_objects::utils::test::test_parse_studycycle_item;
  use studycycle_utils::error::StudyCycleResult;

  #[test]
  fn test_parse_studycycle_deletion() -> StudyCycleResult<()> {
    test_parse_studycycle_item::<Delete>("../apub/assets/studycycle/activities/deletion/remove_note.json")?;
    test_parse_studycycle_item::<Delete>("../apub/assets/studycycle/activities/deletion/delete_page.json")?;

    test_parse_studycycle_item::<UndoDelete>(
      "../apub/assets/studycycle/activities/deletion/undo_remove_note.json",
    )?;
    test_parse_studycycle_item::<UndoDelete>(
      "../apub/assets/studycycle/activities/deletion/undo_delete_page.json",
    )?;
    test_parse_studycycle_item::<Delete>(
      "../apub/assets/studycycle/activities/deletion/delete_private_message.json",
    )?;
    test_parse_studycycle_item::<UndoDelete>(
      "../apub/assets/studycycle/activities/deletion/undo_delete_private_message.json",
    )?;

    test_parse_studycycle_item::<DeleteUser>(
      "../apub/assets/studycycle/activities/deletion/delete_user.json",
    )?;
    Ok(())
  }
}
