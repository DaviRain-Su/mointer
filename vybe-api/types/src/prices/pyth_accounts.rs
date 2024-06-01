use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PythAccountsRequest {
    /// Pyth product ID to filter over.
    pub product_id: Option<String>,
    /// Pyth price feed ID to filter over.
    pub price_feed_id: Option<String>,
}

impl PythAccountsRequest {
    pub fn to_query(&self) -> String {
        let mut query = vec![];
        if let Some(product_id) = &self.product_id {
            query.push(format!("productId={}", product_id));
        }
        if let Some(price_feed_id) = &self.price_feed_id {
            query.push(format!("priceFeedId={}", price_feed_id));
        }
        query.join("&")
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PythAccountsResponse {
    pub data: Vec<PythAccount>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PythAccount {
    pub product_id: String,
    pub price_feed_id: String,
    pub symbol: String,
}
