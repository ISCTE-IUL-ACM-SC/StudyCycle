pub(crate) mod accept;
pub mod follow;
pub(crate) mod reject;
pub mod undo_follow;

#[cfg(test)]
mod tests {
  use crate::protocol::following::{accept::AcceptFollow, follow::Follow, undo_follow::UndoFollow};
  use studycycle_apub_objects::utils::test::test_parse_studycycle_item;
  use studycycle_utils::error::StudyCycleResult;

  #[test]
  fn test_parse_studycycle_accept_follow() -> StudyCycleResult<()> {
    test_parse_studycycle_item::<Follow>("../apub/assets/studycycle/activities/following/follow.json")?;
    test_parse_studycycle_item::<AcceptFollow>("../apub/assets/studycycle/activities/following/accept.json")?;
    test_parse_studycycle_item::<UndoFollow>(
      "../apub/assets/studycycle/activities/following/undo_follow.json",
    )?;
    Ok(())
  }
}
