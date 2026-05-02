use clap::Parser;
use studycycle_server::{CmdArgs, start_studycycle_server};
use studycycle_utils::{error::StudyCycleResult, settings::SETTINGS};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

#[tokio::main]
pub async fn main() -> StudyCycleResult<()> {
  let filter = EnvFilter::builder()
    .with_default_directive(LevelFilter::INFO.into())
    .from_env_lossy();
  if SETTINGS.json_logging {
    tracing_subscriber::fmt()
      .with_env_filter(filter)
      .json()
      .init();
  } else {
    tracing_subscriber::fmt().with_env_filter(filter).init();
  }

  let args = CmdArgs::parse();

  start_studycycle_server(args).await?;
  Ok(())
}
