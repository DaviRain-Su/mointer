use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PythProductRequest {
    pub product_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PythProductResponse {
    pub product_id: String,
    pub description: String,
    pub symbol: String,
    pub asset_type: String,
    pub quote: String,
    pub base: String,
    pub schedule: String,
    pub generic_symbol: String,
}
