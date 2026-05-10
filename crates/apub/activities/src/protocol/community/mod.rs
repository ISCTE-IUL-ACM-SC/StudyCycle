pub mod announce;
pub mod collection_add;
pub mod collection_remove;
pub mod lock;
pub mod report;
pub mod resolve_report;
pub mod update;

#[cfg(test)]
mod tests {
  use super::resolve_report::ResolveReport;
  use crate::protocol::community::{
    announce::AnnounceActivity,
    collection_add::CollectionAdd,
    collection_remove::CollectionRemove,
    lock::{LockPageOrNote, UndoLockPageOrNote},
    report::Report,
    update::Update,
  };
  use studycycle_apub_objects::utils::test::test_parse_studycycle_item;
  use studycycle_utils::error::StudyCycleResult;

  #[test]
  fn test_parse_studycycle_community_activities() -> StudyCycleResult<()> {
    test_parse_studycycle_item::<AnnounceActivity>(
      "../apub/assets/studycycle/activities/community/announce_create_page.json",
    )?;

    test_parse_studycycle_item::<CollectionAdd>(
      "../apub/assets/studycycle/activities/community/add_mod.json",
    )?;
    test_parse_studycycle_item::<CollectionRemove>(
      "../apub/assets/studycycle/activities/community/remove_mod.json",
    )?;

    test_parse_studycycle_item::<CollectionAdd>(
      "../apub/assets/studycycle/activities/community/add_featured_post.json",
    )?;
    test_parse_studycycle_item::<CollectionRemove>(
      "../apub/assets/studycycle/activities/community/remove_featured_post.json",
    )?;

    test_parse_studycycle_item::<LockPageOrNote>(
      "../apub/assets/studycycle/activities/community/lock_page.json",
    )?;
    test_parse_studycycle_item::<UndoLockPageOrNote>(
      "../apub/assets/studycycle/activities/community/undo_lock_page.json",
    )?;

    test_parse_studycycle_item::<LockPageOrNote>(
      "../apub/assets/studycycle/activities/community/lock_note.json",
    )?;
    test_parse_studycycle_item::<UndoLockPageOrNote>(
      "../apub/assets/studycycle/activities/community/undo_lock_note.json",
    )?;

    test_parse_studycycle_item::<Update>(
      "../apub/assets/studycycle/activities/community/update_community.json",
    )?;

    test_parse_studycycle_item::<Report>("../apub/assets/studycycle/activities/community/report_page.json")?;
    test_parse_studycycle_item::<ResolveReport>(
      "../apub/assets/studycycle/activities/community/resolve_report_page.json",
    )?;

    Ok(())
  }
}
