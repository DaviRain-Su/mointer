use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub mod response;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum XChain {
    #[default]
    Solana,
    Ethereum,
    Arbitrum,
    Avalanche,
    Bsc,
    Optimism,
    Polygon,
    Base,
    ZkSync,
}

impl Display for XChain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            XChain::Solana => write!(f, "solana"),
            XChain::Ethereum => write!(f, "ethereum"),
            XChain::Arbitrum => write!(f, "arbitrum"),
            XChain::Avalanche => write!(f, "avalanche"),
            XChain::Bsc => write!(f, "bsc"),
            XChain::Optimism => write!(f, "optimism"),
            XChain::Polygon => write!(f, "polygon"),
            XChain::Base => write!(f, "base"),
            XChain::ZkSync => write!(f, "zksync"),
        }
    }
}
