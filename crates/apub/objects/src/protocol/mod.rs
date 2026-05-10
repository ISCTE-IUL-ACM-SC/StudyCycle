pub mod group;
pub mod instance;
pub mod multi_community;
pub mod note;
pub mod page;
pub mod person;
pub mod private_message;
pub mod tags;

#[cfg(test)]
mod tests {
  use super::{
    group::Group,
    instance::Instance,
    note::Note,
    page::Page,
    person::Person,
    private_message::PrivateMessage,
  };
  use crate::utils::test::{test_json, test_parse_studycycle_item};
  use activitypub_federation::protocol::tombstone::Tombstone;
  use studycycle_utils::error::StudyCycleResult;

  #[test]
  fn test_parse_objects_studycycle() -> StudyCycleResult<()> {
    test_parse_studycycle_item::<Instance>("../apub/assets/studycycle/objects/instance.json")?;
    test_parse_studycycle_item::<Group>("../apub/assets/studycycle/objects/group.json")?;
    test_parse_studycycle_item::<Person>("../apub/assets/studycycle/objects/person.json")?;
    test_parse_studycycle_item::<Page>("../apub/assets/studycycle/objects/page.json")?;
    test_parse_studycycle_item::<Note>("../apub/assets/studycycle/objects/comment.json")?;
    test_parse_studycycle_item::<PrivateMessage>("../apub/assets/studycycle/objects/private_message.json")?;
    test_parse_studycycle_item::<Tombstone>("../apub/assets/studycycle/objects/tombstone.json")?;
    Ok(())
  }

  #[test]
  fn test_parse_objects_pleroma() -> StudyCycleResult<()> {
    test_json::<Person>("../apub/assets/pleroma/objects/person.json")?;
    test_json::<Note>("../apub/assets/pleroma/objects/note.json")?;
    Ok(())
  }

  #[test]
  fn test_parse_objects_smithereen() -> StudyCycleResult<()> {
    test_json::<Person>("../apub/assets/smithereen/objects/person.json")?;
    test_json::<Note>("../apub/assets/smithereen/objects/note.json")?;
    Ok(())
  }

  #[test]
  fn test_parse_objects_mastodon() -> StudyCycleResult<()> {
    test_json::<Person>("../apub/assets/mastodon/objects/person.json")?;
    test_json::<Note>("../apub/assets/mastodon/objects/note_1.json")?;
    test_json::<Note>("../apub/assets/mastodon/objects/note_2.json")?;
    test_json::<Page>("../apub/assets/mastodon/objects/page.json")?;
    Ok(())
  }

  #[test]
  fn test_parse_objects_lotide() -> StudyCycleResult<()> {
    test_json::<Group>("../apub/assets/lotide/objects/group.json")?;
    test_json::<Person>("../apub/assets/lotide/objects/person.json")?;
    test_json::<Note>("../apub/assets/lotide/objects/note.json")?;
    test_json::<Page>("../apub/assets/lotide/objects/page.json")?;
    test_json::<Tombstone>("../apub/assets/lotide/objects/tombstone.json")?;
    Ok(())
  }

  #[test]
  fn test_parse_object_friendica() -> StudyCycleResult<()> {
    test_json::<Person>("../apub/assets/friendica/objects/person_1.json")?;
    test_json::<Person>("../apub/assets/friendica/objects/person_2.json")?;
    test_json::<Page>("../apub/assets/friendica/objects/page_1.json")?;
    test_json::<Page>("../apub/assets/friendica/objects/page_2.json")?;
    test_json::<Note>("../apub/assets/friendica/objects/note_1.json")?;
    test_json::<Note>("../apub/assets/friendica/objects/note_2.json")?;
    Ok(())
  }

  #[test]
  fn test_parse_object_gnusocial() -> StudyCycleResult<()> {
    test_json::<Person>("../apub/assets/gnusocial/objects/person.json")?;
    test_json::<Group>("../apub/assets/gnusocial/objects/group.json")?;
    test_json::<Page>("../apub/assets/gnusocial/objects/page.json")?;
    test_json::<Note>("../apub/assets/gnusocial/objects/note.json")?;
    Ok(())
  }

  #[test]
  fn test_parse_object_peertube() -> StudyCycleResult<()> {
    test_json::<Person>("../apub/assets/peertube/objects/person.json")?;
    test_json::<Group>("../apub/assets/peertube/objects/group.json")?;
    test_json::<Page>("../apub/assets/peertube/objects/video.json")?;
    test_json::<Note>("../apub/assets/peertube/objects/note.json")?;
    Ok(())
  }

  #[test]
  fn test_parse_object_mobilizon() -> StudyCycleResult<()> {
    test_json::<Group>("../apub/assets/mobilizon/objects/group.json")?;
    test_json::<Page>("../apub/assets/mobilizon/objects/event.json")?;
    test_json::<Person>("../apub/assets/mobilizon/objects/person.json")?;
    Ok(())
  }

  #[test]
  fn test_parse_object_discourse() -> StudyCycleResult<()> {
    test_json::<Group>("../apub/assets/discourse/objects/group.json")?;
    test_json::<Page>("../apub/assets/discourse/objects/page.json")?;
    test_json::<Person>("../apub/assets/discourse/objects/person.json")?;
    Ok(())
  }

  #[test]
  fn test_parse_object_nodebb() -> StudyCycleResult<()> {
    test_json::<Group>("../apub/assets/nodebb/objects/group.json")?;
    test_json::<Page>("../apub/assets/nodebb/objects/page.json")?;
    test_json::<Person>("../apub/assets/nodebb/objects/person.json")?;
    Ok(())
  }

  #[test]
  fn test_parse_object_wordpress() -> StudyCycleResult<()> {
    test_json::<Group>("../apub/assets/wordpress/objects/group.json")?;
    test_json::<Page>("../apub/assets/wordpress/objects/page.json")?;
    test_json::<Person>("../apub/assets/wordpress/objects/person.json")?;
    test_json::<Note>("../apub/assets/wordpress/objects/note.json")?;
    Ok(())
  }

  #[test]
  fn test_parse_object_mbin() -> StudyCycleResult<()> {
    test_json::<Instance>("../apub/assets/mbin/objects/instance.json")?;
    Ok(())
  }
}
