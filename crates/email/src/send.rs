use studycycle_diesel_utils::sensitive::SensitiveString;
use studycycle_utils::{
  error::{StudyCycleErrorExt, StudyCycleErrorType},
  settings::structs::Settings,
  spawn_try_task,
};
use lettre::{
  Address,
  AsyncTransport,
  Message,
  message::{Mailbox, MultiPart},
  transport::smtp::extension::ClientId,
};
use std::{str::FromStr, sync::OnceLock};
use uuid::Uuid;

type AsyncSmtpTransport = lettre::AsyncSmtpTransport<lettre::Tokio1Executor>;

pub(crate) fn send_email(
  subject: String,
  to_email: SensitiveString,
  to_username: String,
  html: String,
  settings: &'static Settings,
) {
  spawn_try_task(async move {
    static MAILER: OnceLock<AsyncSmtpTransport> = OnceLock::new();
    let email_config = settings.email.clone().ok_or(StudyCycleErrorType::NoEmailSetup)?;

    #[expect(clippy::expect_used)]
    let mailer = MAILER.get_or_init(|| {
      AsyncSmtpTransport::from_url(&email_config.connection)
        .expect("init email transport")
        .hello_name(ClientId::Domain(settings.hostname.clone()))
        .build()
    });

    // use usize::MAX as the line wrap length, since lettre handles the wrapping for us
    let plain_text = html2text::from_read(html.as_bytes(), usize::MAX)?;

    let smtp_from_address = &email_config.smtp_from_address;

    let email = Message::builder()
      .from(
        smtp_from_address
          .parse()
          .with_studycycle_type(StudyCycleErrorType::InvalidEmailAddress(
            smtp_from_address.into(),
          ))?,
      )
      .to(Mailbox::new(
        Some(to_username.clone()),
        Address::from_str(&to_email)
          .with_studycycle_type(StudyCycleErrorType::InvalidEmailAddress(to_email.into_inner()))?,
      ))
      .message_id(Some(format!("<{}@{}>", Uuid::new_v4(), settings.hostname)))
      .subject(subject)
      .multipart(MultiPart::alternative_plain_html(plain_text, html.clone()))
      .with_studycycle_type(StudyCycleErrorType::EmailSendFailed)?;

    mailer
      .send(email)
      .await
      .with_studycycle_type(StudyCycleErrorType::EmailSendFailed)?;

    Ok(())
  })
}
