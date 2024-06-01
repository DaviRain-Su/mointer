use serde::{Deserialize, Serialize};

pub struct NftBalancesRequest {
    /// 与 Solana 帐户关联的公钥 (pubKey)。
    pub address: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NftBalancesResponse {
    pub date: usize,
    pub owner_address: String,
    pub fetcher_reports: Vec<FetcherReport>,
    pub total_sol: String,
    pub total_usd: String,
    pub data: Vec<NftData>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FetcherReport {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NftData {
    pub name: String,
    pub collection_address: String,
    pub total_items: usize,
    pub value_sol: String,
    pub price_sol: String,
    pub value_usd: String,
    pub price_usd: String,
    pub logo_url: String,
}
