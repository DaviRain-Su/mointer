use std::fmt::Display;

use serde::{Deserialize, Serialize};

pub struct TokenBalancesRequest {
    pub address: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TokenBalancesResponse {
    pub date: usize,
    pub owner_address: String,
    pub fetcher_reports: Vec<FetcherReport>,
    pub staked_sol_usd: String,
    pub total_token_value_usd: String,
    pub data: Vec<TokenData>,
}

impl Display for TokenBalancesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "🧍Owner Address: {}, 🪙Staked SOL: {}, 🪙Total Token Value: {}",
            self.owner_address, self.staked_sol_usd, self.total_token_value_usd
        )?;
        for token in &self.data {
            writeln!(f, "{}", token)?;
        }
        write!(f, "")
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FetcherReport {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TokenData {
    pub symbol: String,
    pub name: String,
    pub mint_address: String,
    pub amount: String,
    pub price_usd: String,
    pub value_usd: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
}

impl Display for TokenData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "🌾{}-🌲{}-🪞{}",
            self.symbol, self.amount, self.value_usd
        )
    }
}
