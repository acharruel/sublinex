use anyhow::{Context, Result};
use clap::Parser;
use sublinex::Cli;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter};

fn main() -> Result<()> {
    let cli = Cli::parse();
    setup_logging(&cli.log_level)?;
    sublinex::run(cli)?;
    Ok(())
}

pub fn setup_logging(log_level: &str) -> Result<()> {
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::OFF.into())
        .from_env()?
        .add_directive(log_level.parse()?);

    let layer = tracing_subscriber::fmt::layer().without_time();
    let tracer = tracing_subscriber::registry().with(layer).with(filter);
    tracing::subscriber::set_global_default(tracer).context("Failed to set subscriber")?;

    Ok(())
}
