pub mod cmd;
pub mod error;

use clap::Parser;
use cmd::MointerCommand;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let cmd = MointerCommand::parse();
    cmd.run().await?;
    Ok(())
}
