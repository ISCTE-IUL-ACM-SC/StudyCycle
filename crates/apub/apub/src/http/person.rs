use crate::protocol::collections::url_collection::UrlCollection;
use activitypub_federation::{config::Data, traits::Object};
use actix_web::{HttpResponse, web::Path};
use studycycle_api_utils::{context::StudyCycleContext, utils::generate_outbox_url};
use studycycle_apub_objects::objects::person::ApubPerson;
use studycycle_db_schema::{source::person::Person, traits::ApubActor};
use studycycle_utils::{
  FEDERATION_CONTEXT,
  error::{StudyCycleErrorType, StudyCycleResult},
};
use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct PersonQuery {
  user_name: String,
}

/// Return the ActivityPub json representation of a local person over HTTP.
pub(crate) async fn get_apub_person_http(
  info: Path<PersonQuery>,
  context: Data<StudyCycleContext>,
) -> StudyCycleResult<HttpResponse> {
  let user_name = info.into_inner().user_name;
  // This needs to be able to read deleted persons, so that it can send tombstones
  let person: ApubPerson = Person::read_from_name(&mut context.pool(), &user_name, None, true)
    .await?
    .ok_or(StudyCycleErrorType::NotFound)?
    .into();

  person.http_response(&FEDERATION_CONTEXT, &context).await
}

pub(crate) async fn get_apub_person_outbox(
  info: Path<PersonQuery>,
  context: Data<StudyCycleContext>,
) -> StudyCycleResult<HttpResponse> {
  let person = Person::read_from_name(&mut context.pool(), &info.user_name, None, false)
    .await?
    .ok_or(StudyCycleErrorType::NotFound)?;
  let outbox_id = generate_outbox_url(&person.ap_id)?.to_string();
  UrlCollection::new_empty_response(outbox_id)
}
