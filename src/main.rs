use chrono_h::cli::{run, Commands};
use chrono_h::Result;
use clap::Parser;
use tracing::info;

#[derive(Parser)]
#[command(name = "chrono")]
#[command(about = "Time-aware harness for AI agents")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("ChronoH starting...");

    let cli = Cli::parse();
    run(cli.command).await
}
