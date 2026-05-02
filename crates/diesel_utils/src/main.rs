/// Very minimal wrapper around `studycycle_diesel_utils::run` to allow running migrations without
/// compiling everything.
fn main() -> anyhow::Result<()> {
  if std::env::args().len() > 1 {
    anyhow::bail!("To set parameters for running migrations, use the studycycle_server command.");
  }

  studycycle_diesel_utils::schema_setup::run(
    studycycle_diesel_utils::schema_setup::Options::default().run(),
    &std::env::var("STUDYCYCLE_DATABASE_URL")?,
  )?;

  Ok(())
}
