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
