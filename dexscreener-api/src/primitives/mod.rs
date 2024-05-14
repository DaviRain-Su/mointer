use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PairsResponse {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    pub pairs: Vec<Pair>,
    pub pair: Option<Pair>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokensResponse {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    pub pairs: Vec<Pair>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    pub pairs: Vec<Pair>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pair {
    #[serde(rename = "chainId")]
    chain_id: String,
    #[serde(rename = "dexId")]
    dex_id: String,
    url: String,
    #[serde(rename = "pairAddress")]
    pair_address: String,
    labels: Option<Vec<String>>,
    #[serde(rename = "baseToken")]
    base_token: Token,
    #[serde(rename = "quoteToken")]
    quote_token: Token,
    #[serde(rename = "priceNative")]
    price_native: String,
    #[serde(rename = "priceUsd")]
    price_usd: String,
    txns: Txns,
    volume: Volume,
    #[serde(rename = "priceChange")]
    price_change: PriceChange,
    liquidity: Liquidity,
    fdv: usize,
    #[serde(rename = "pairCreatedAt")]
    pair_createed_at: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    address: String,
    name: String,
    symbol: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Txns {
    m5: BuysAndSells,
    h1: BuysAndSells,
    h6: BuysAndSells,
    h24: BuysAndSells,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BuysAndSells {
    pub buys: usize,
    pub sells: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrimitiveTime {
    pub m5: f64,
    pub h1: f64,
    pub h6: f64,
    pub h24: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Volume(pub PrimitiveTime);

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceChange(pub PrimitiveTime);

#[derive(Debug, Serialize, Deserialize)]
pub struct Liquidity {
    usd: f64,
    base: f64,
    quote: f64,
}
