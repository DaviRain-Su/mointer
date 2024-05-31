pub mod account;
pub mod borrow_lend;
pub mod nft;
pub mod perps;
pub mod prices;
pub mod programs;
pub mod tokens;

pub(crate) const BASE_URL: &str = "https://api.vybenetwork.xyz";

use reqwest::header;
use reqwest::{Client, ClientBuilder};

pub struct VybeClient {
    client: Client,
    api_key: String,
}

impl VybeClient {
    pub fn new(api_key: String) -> anyhow::Result<Self> {
        let client_builder = ClientBuilder::new();
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "X-API-KEY",
            header::HeaderValue::from_str(api_key.as_str())?,
        );
        headers.insert("accept", header::HeaderValue::from_str("application/json")?);

        let client = client_builder
            .user_agent("reqwest")
            .default_headers(headers)
            .build()?;

        Ok(Self { client, api_key })
    }

    pub fn client(&self) -> &Client {
        &self.client
    }
}
