pub mod undo_vote;
pub mod vote;

#[cfg(test)]
mod tests {
  use crate::protocol::voting::{undo_vote::UndoVote, vote::Vote};
  use studycycle_apub_objects::utils::test::test_parse_studycycle_item;
  use studycycle_utils::error::StudyCycleResult;

  #[test]
  fn test_parse_studycycle_voting() -> StudyCycleResult<()> {
    test_parse_studycycle_item::<Vote>("../apub/assets/studycycle/activities/voting/like_note.json")?;
    test_parse_studycycle_item::<Vote>("../apub/assets/studycycle/activities/voting/dislike_page.json")?;

    test_parse_studycycle_item::<UndoVote>(
      "../apub/assets/studycycle/activities/voting/undo_like_note.json",
    )?;
    test_parse_studycycle_item::<UndoVote>(
      "../apub/assets/studycycle/activities/voting/undo_dislike_page.json",
    )?;
    Ok(())
  }
}
