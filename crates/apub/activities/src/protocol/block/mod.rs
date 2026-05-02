pub mod block_user;
pub mod undo_block_user;

#[cfg(test)]
mod tests {
  use crate::protocol::block::{block_user::BlockUser, undo_block_user::UndoBlockUser};
  use studycycle_apub_objects::utils::test::test_parse_studycycle_item;
  use studycycle_utils::error::StudyCycleResult;

  #[test]
  fn test_parse_studycycle_block() -> StudyCycleResult<()> {
    test_parse_studycycle_item::<BlockUser>("../apub/assets/studycycle/activities/block/block_user.json")?;
    test_parse_studycycle_item::<UndoBlockUser>(
      "../apub/assets/studycycle/activities/block/undo_block_user.json",
    )?;
    Ok(())
  }
}
