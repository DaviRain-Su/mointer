use clap::Parser;
use dexscreener_api::get_pairs_by_chain_and_pair_address;
use dexscreener_api::primitives::ChainId;
use tracing::info;

#[derive(Parser, Debug)]
pub enum Dexscreener {
    #[command(name = "pair-address", about = "Get pair address info")]
    PairAddress(PairAddress),
}

#[derive(Parser, Debug)]
pub struct PairAddress {
    #[arg(short, long, default_value = "solana")]
    pub chain_id: ChainId,
    #[arg(short, long)]
    pub pair_address: String,
}

impl Dexscreener {
    pub async fn run(&self) -> anyhow::Result<()> {
        match self {
            Dexscreener::PairAddress(pair_address) => {
                info!(
                    "chain_id: {}, pair_address: {}",
                    pair_address.chain_id, pair_address.pair_address
                );
                let result = get_pairs_by_chain_and_pair_address(
                    pair_address.chain_id,
                    vec![&pair_address.pair_address],
                )
                .await?;
                println!("{:#?}", result);
            }
        }
        Ok(())
    }
}
