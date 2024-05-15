//! Standard Package:
//! - Standard package is a collection of modules that are part of the Rust standard library.
//!   - Price - Multiple : /defi/multi_price
//!   - Price - Historical : /defi/history_price
//!   - Token List : /defi/tokenlist
//!   - Price : /defi/price

use crate::primitives::response::{MultiplePriceResponse, PriceResponse, TokenListResponse};
use crate::XChain;

const BASE_URL: &str = "https://public-api.birdeye.so";

// TODO: miss check_liquidity parameter
/// Get token prices
pub async fn get_token_price(
    include_liquidity: Option<bool>,
    token_address: &str,
    api_key: &str,
    x_chain: Option<XChain>,
) -> anyhow::Result<PriceResponse> {
    let url = if let Some(include_liquidity) = include_liquidity {
        if include_liquidity {
            format!(
                "{}/defi/price?include_liquidity=true&address={}",
                BASE_URL, token_address
            )
        } else {
            format!("{}/defi/price?address={}", BASE_URL, token_address)
        }
    } else {
        format!("{}/defi/price?address={}", BASE_URL, token_address)
    };

    let request_build = reqwest::Client::new()
        .get(&url)
        .header("X-API-KEY", api_key)
        .header("x-chain", x_chain.unwrap_or(XChain::default()).to_string());

    let response = request_build.send().await?.text().await?;

    let price_response: PriceResponse = serde_json::from_str(&response)?;

    Ok(price_response)
}

#[derive(Debug, Default)]
pub enum SortBy {
    MarketCap,
    #[default]
    Volume24hUSD,
    Volume24hChangePercent,
}

impl std::fmt::Display for SortBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortBy::MarketCap => write!(f, "mc"),
            SortBy::Volume24hUSD => write!(f, "v24hUSD"),
            SortBy::Volume24hChangePercent => write!(f, "v24hChangePercent"),
        }
    }
}

#[derive(Debug, Default)]
pub enum SortType {
    Asc,
    #[default]
    Desc,
}

impl std::fmt::Display for SortType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortType::Asc => write!(f, "asc"),
            SortType::Desc => write!(f, "desc"),
        }
    }
}

pub async fn get_token_list(
    sort_by: Option<SortBy>,
    sort_type: Option<SortType>,
    _offset: Option<i32>,
    _limit: Option<i32>,
    api_key: &str,
    x_chain: Option<XChain>,
) -> anyhow::Result<TokenListResponse> {
    let url = format!(
        "{}/defi/tokenlist?sort_by={}&sort_type={}",
        BASE_URL,
        sort_by.unwrap_or(SortBy::default()),
        sort_type.unwrap_or(SortType::default())
    );

    let request_build = reqwest::Client::new()
        .get(&url)
        .header("X-API-KEY", api_key)
        .header("x-chain", x_chain.unwrap_or(XChain::default()).to_string());

    let response = request_build.send().await?.text().await?;

    let response: TokenListResponse = serde_json::from_str(&response)?;

    Ok(response)
}

pub async fn get_multiple_price(
    _check_liquidity: Option<f64>,
    include_liquidity: Option<bool>,
    list_address: Vec<&str>,
    api_key: &str,
    x_chain: Option<XChain>,
) -> anyhow::Result<MultiplePriceResponse> {
    let list_address = list_address.join("%2C");
    let url = if let Some(include_liquidity) = include_liquidity {
        if include_liquidity {
            format!(
                "{}/defi/multi_price?include_liquidity=true&list_address={}",
                BASE_URL, list_address
            )
        } else {
            format!(
                "{}/defi/multi_price?list_address={}",
                BASE_URL, list_address
            )
        }
    } else {
        format!(
            "{}/defi/multi_price?list_address={}",
            BASE_URL, list_address
        )
    };

    let request_build = reqwest::Client::new()
        .get(&url)
        .header("X-API-KEY", api_key)
        .header("x-chain", x_chain.unwrap_or(XChain::default()).to_string());

    let response = request_build.send().await?.text().await?;

    let response: MultiplePriceResponse = serde_json::from_str(&response)?;

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_x_chain_default_value() {
        let x_chian = XChain::default();
        assert_eq!(x_chian, XChain::Solana);
    }

    #[tokio::test]
    async fn test_get_price() {
        let api_key = "67ee84080cc74239b63b716fe1d76ad7";
        let address = "ukHH6c7mMyiWCf1b9pnWe25TSpkDDt3H5pQZgZ74J82";
        let result = get_token_price(Some(true), address, api_key, None).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_token_list() -> anyhow::Result<()> {
        let api_key = "67ee84080cc74239b63b716fe1d76ad7";
        let result = get_token_list(
            Some(SortBy::Volume24hChangePercent),
            None,
            None,
            None,
            api_key,
            None,
        )
        .await;
        assert!(result.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_multiple_price() {
        let api_key = "67ee84080cc74239b63b716fe1d76ad7";
        let list_address = vec![
            "So11111111111111111111111111111111111111112",
            "mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So",
        ];
        let result = get_multiple_price(None, None, list_address, api_key, None).await;
        assert!(result.is_ok());
    }
}
