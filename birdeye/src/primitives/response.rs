use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PriceResponse {
    pub data: PriceData,
    pub success: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceData {
    #[serde(rename = "value")]
    price: f64,
    update_unix_time: u64,
    update_human_time: String,
    price_change_24h: Option<f64>,
    liquidity: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenListResponse {
    pub data: TokenListData,
    pub success: bool,
}

impl TokenListResponse {
    pub fn tokens(&self) -> &[Token] {
        &self.data.tokens
    }

    pub fn tokens_len(&self) -> usize {
        self.data.tokens.len()
    }

    pub fn tokens_name(&self) -> Vec<String> {
        self.data
            .tokens
            .iter()
            .map(|token| token.name.clone())
            .collect()
    }

    pub fn tokens_symbol(&self) -> Vec<String> {
        self.data
            .tokens
            .iter()
            .map(|token| token.symbol.clone())
            .collect()
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenListData {
    pub update_unix_time: u64,
    pub update_time: String,
    pub tokens: Vec<Token>,
    pub total: u64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub address: String,
    pub decimals: u64,
    pub last_trade_unix_time: u64,
    pub liquidity: f64,
    pub logo_uri: Option<String>,
    pub mc: Option<f64>,
    pub name: String,
    pub symbol: String,
    pub v24h_change_percent: Option<f64>,
    pub v24h_usd: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MultiplePriceResponse {
    pub data: HashMap<String, PriceData>,
    pub success: bool,
}
