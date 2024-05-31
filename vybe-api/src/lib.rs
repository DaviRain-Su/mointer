pub mod account;
pub mod borrow_lend;
pub mod nft;
pub mod perps;
pub mod prices;
pub mod programs;
pub mod tokens;

pub(crate) const BASE_URL: &str = "https://api.vybenetwork.xyz";

use reqwest::header;
use reqwest::ClientBuilder;

pub struct VybeClient {
    client: ClientBuilder,
    api_key: String,
}

impl VybeClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: ClientBuilder::new(),
            api_key,
        }
    }

    /// build with api_key and header
    pub fn build(self) -> anyhow::Result<ClientBuilder> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "X-API-KEY",
            header::HeaderValue::from_str(&self.api_key.as_str())?,
        );
        headers.insert("accept", header::HeaderValue::from_str("application/json")?);

        Ok(self.client.user_agent("reqwest").default_headers(headers))
    }
}
