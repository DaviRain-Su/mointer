use clap::Parser;

pub mod base64;
pub mod bs58;
pub mod dexscreener;
pub mod server;
pub mod solana_rpc;

use dexscreener::Dexscreener;
use server::Server;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct MointerCommand {
    #[command(subcommand)]
    subcommand: SubCommand,
}

impl MointerCommand {
    pub async fn run(&self) -> anyhow::Result<()> {
        self.subcommand.run().await
    }
}

#[derive(Parser, Debug)]
pub enum SubCommand {
    #[command(name = "server")]
    Server(Server),
    #[command(subcommand, name = "dexscreener", about = "dexscreener commands")]
    Dexscreener(Dexscreener),
    #[command(subcommand, name = "solana-rpc")]
    SolanaRpc(solana_rpc::SolanaRpc),
    #[command(subcommand, name = "base64")]
    Base64(base64::Base64),
    #[command(subcommand, name = "bs58")]
    Bs58(bs58::Bs58),
}

impl SubCommand {
    pub async fn run(&self) -> anyhow::Result<()> {
        match self {
            SubCommand::Server(server) => server.run().await,
            SubCommand::Dexscreener(dexscreener) => dexscreener.run().await,
            SubCommand::SolanaRpc(solana_rpc) => solana_rpc.run().await,
            SubCommand::Base64(base64) => base64.run(),
            SubCommand::Bs58(bs58) => bs58.run(),
        }
    }
}
