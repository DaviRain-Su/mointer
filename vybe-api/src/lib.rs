mod account;
mod borrow_lend;
mod nft;
mod perps;
mod prices;
mod programs;
mod tokens;

pub use account::*;
pub use borrow_lend::*;
pub use nft::*;
pub use perps::*;
pub use prices::*;
pub use programs::*;
pub use tokens::*;

use reqwest::header;
use reqwest::{Client, ClientBuilder};

pub struct VybeClient {
    pub client: Client,
    pub api_key: String,
}

impl VybeClient {
    pub const BASE_URL: &'static str = "https://api.vybenetwork.xyz";

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
