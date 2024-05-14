//! Standard Package:
//! - Standard package is a collection of modules that are part of the Rust standard library.
//!   - Price - Multiple : /defi/multi_price
//!   - Price - Historical : /defi/history_price
//!   - Token List : /defi/tokenlist
//!   - Price : /defi/price

use crate::primitives::response::PriceResponse;
use crate::XChain;

const BASE_URL: &str = "https://public-api.birdeye.so";

// TODO: miss check_liquidity parameter
/// Get token prices
pub async fn get_price(
    api_key: &str,
    address: &str,
    include_liquidity: Option<bool>,
    x_chian: Option<XChain>,
) -> anyhow::Result<PriceResponse> {
    let url = if let Some(include_liquidity) = include_liquidity {
        if include_liquidity {
            format!(
                "{}/defi/price?include_liquidity=true&address={}",
                BASE_URL, address
            )
        } else {
            format!("{}/defi/price?address={}", BASE_URL, address)
        }
    } else {
        format!("{}/defi/price?address={}", BASE_URL, address)
    };

    let request_build = reqwest::Client::new()
        .get(&url)
        .header("X-API-KEY", api_key)
        .header("x-chain", x_chian.unwrap_or(XChain::default()).to_string());

    let response = request_build.send().await?.text().await?;
    println!("response: {}", response);

    let price_response: PriceResponse = serde_json::from_str(&response)?;
    println!("price_response: {:#?}", price_response);

    Ok(price_response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_x_chain_default_value() {
        let x_chian = XChain::default();
        println!("x_chain: {}", x_chian);
        assert_eq!(x_chian, XChain::Solana);
    }

    #[tokio::test]
    async fn test_get_price() {
        let api_key = "67ee84080cc74239b63b716fe1d76ad7";
        let address = "ukHH6c7mMyiWCf1b9pnWe25TSpkDDt3H5pQZgZ74J82";
        let result = get_price(api_key, address, None, None).await;
        println!("result: {:?}", result);
        assert!(result.is_ok());
    }
}
