use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PriceResponse {
    pub data: PriceData,
    pub success: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PriceData {
    #[serde(rename = "value")]
    price: f64,
    #[serde(rename = "updateUnixTime")]
    update_unix_time: u64,
    #[serde(rename = "updateHumanTime")]
    update_human_time: String,
    liquidity: Option<f64>,
}
