use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug)]
pub enum SortedBy {
    Asc,
    Desc,
}

impl Default for SortedBy {
    fn default() -> Self {
        SortedBy::Asc
    }
}

impl SortedBy {
    pub fn as_str(&self) -> &str {
        match self {
            SortedBy::Asc => "sorted_by_asc",
            SortedBy::Desc => "sorted_by_desc",
        }
    }
}

impl Display for SortedBy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenRequest {
    pub sorted_by: Option<SortedBy>,
    pub limit: Option<i32>,
    pub page: Option<i32>,
}

impl TokenRequest {
    pub fn new() -> Self {
        TokenRequest {
            sorted_by: None,
            limit: None,
            page: None,
        }
    }

    pub fn to_query(&self) -> String {
        let mut query = String::new();
        if let Some(sorted_by) = &self.sorted_by {
            match sorted_by {
                SortedBy::Asc => query.push_str(&format!("sortByAsc={}&", sorted_by)),
                SortedBy::Desc => query.push_str(&format!("sortByDesc={}&", sorted_by)),
            }
        }
        if let Some(limit) = &self.limit {
            query.push_str(&format!("limit={}&", limit));
        }
        if let Some(page) = &self.page {
            query.push_str(&format!("page={}&", page));
        }
        query
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenResponse {
    pub data: Vec<Token>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub symbol: String,
    pub name: String,
    pub mint_address: String,
    pub price: f64,
    pub decimal: i32,
    pub logo_url: Option<String>,
    pub update_time: i64,
    pub current_supply: f64,
    pub market_cap: f64,
    pub token_amount_volume_24h: Option<f64>,
    pub usd_value_volume_24h: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SimpleToken {
    pub symbol: String,
    pub mint_address: String,
}

impl From<Token> for SimpleToken {
    fn from(token: Token) -> Self {
        SimpleToken {
            symbol: token.symbol,
            mint_address: token.mint_address,
        }
    }
}
