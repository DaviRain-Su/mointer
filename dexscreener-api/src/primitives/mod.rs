use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum ChainId {
    Solana,
    Ethereum,
    BinanceSmartChain,
    Polygon,
    Avalanche,
    Fantom,
    Arbitrum,
    Optimism,
    Celo,
    Moonriver,
    Moonbeam,
    Near,
    Harmony,
}

impl Display for ChainId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ChainId::Solana => write!(f, "solana"),
            ChainId::Ethereum => write!(f, "ethereum"),
            ChainId::BinanceSmartChain => write!(f, "bsc"),
            ChainId::Polygon => write!(f, "polygon"),
            ChainId::Avalanche => write!(f, "avalanche"),
            ChainId::Fantom => write!(f, "fantom"),
            ChainId::Arbitrum => write!(f, "arbitrum"),
            ChainId::Optimism => write!(f, "optimism"),
            ChainId::Celo => write!(f, "celo"),
            ChainId::Moonriver => write!(f, "moonriver"),
            ChainId::Moonbeam => write!(f, "moonbeam"),
            ChainId::Near => write!(f, "near"),
            ChainId::Harmony => write!(f, "harmony"),
        }
    }
}

impl FromStr for ChainId {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "solana" => Ok(ChainId::Solana),
            "ethereum" => Ok(ChainId::Ethereum),
            "bsc" => Ok(ChainId::BinanceSmartChain),
            "polygon" => Ok(ChainId::Polygon),
            "avalanche" => Ok(ChainId::Avalanche),
            "fantom" => Ok(ChainId::Fantom),
            "arbitrum" => Ok(ChainId::Arbitrum),
            "optimism" => Ok(ChainId::Optimism),
            "celo" => Ok(ChainId::Celo),
            "moonriver" => Ok(ChainId::Moonriver),
            "moonbeam" => Ok(ChainId::Moonbeam),
            "near" => Ok(ChainId::Near),
            "harmony" => Ok(ChainId::Harmony),
            _ => Err(format!("Unknown chain id: {}", s)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PairsResponse {
    pub schema_version: String,
    pub pairs: Vec<Pair>,
    pub pair: Option<Pair>,
}

impl PairsResponse {
    pub fn pairs(&self) -> Vec<Pair> {
        self.pairs.clone()
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokensResponse {
    pub schema_version: String,
    pub pairs: Vec<Pair>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResponse {
    pub schema_version: String,
    pub pairs: Vec<Pair>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Pair {
    chain_id: String,
    dex_id: String,
    url: String,
    pair_address: String,
    labels: Option<Vec<String>>,
    base_token: Token,
    quote_token: Token,
    price_native: String,
    price_usd: Option<String>,
    txns: Txns,
    volume: Volume,
    price_change: PriceChange,
    liquidity: Option<Liquidity>,
    fdv: Option<usize>,
    pair_createed_at: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Token {
    address: String,
    name: String,
    symbol: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Txns {
    m5: BuysAndSells,
    h1: BuysAndSells,
    h6: BuysAndSells,
    h24: BuysAndSells,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BuysAndSells {
    pub buys: usize,
    pub sells: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrimitiveTime {
    pub m5: f64,
    pub h1: f64,
    pub h6: f64,
    pub h24: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Volume(pub PrimitiveTime);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PriceChange(pub PrimitiveTime);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Liquidity {
    usd: f64,
    base: f64,
    quote: f64,
}
