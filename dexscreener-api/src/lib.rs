//! Dex screener DOC: https://docs.dexscreener.com/api/reference
//!
use tracing::info;
pub mod primitives;

use primitives::{PairsResponse, SearchResponse, TokensResponse};

const DEXSCREENER_API_URL: &str = "https://api.dexscreener.com/latest/dex";

/// Get one or multiple pairs by chain and pair address
/// chain_id: &str, ethereum | bsc | polygon| etc
/// pair_address: Vec<&str>, One or multiple, comma-separated pair addresses (up to 30 addresses)
pub async fn get_pairs_by_chain_and_pair_address(
    chain_id: &str,
    pair_address: Vec<&str>,
) -> anyhow::Result<PairsResponse> {
    if pair_address.len() > 30 || pair_address.len() == 0 {
        return Err(anyhow::anyhow!(
            "pair_address length should be less than 30 or greater than 0. the current pair_address length({})", pair_address.len()
        ));
    }
    let pair_address = if pair_address.len() == 1 {
        pair_address.first().unwrap().to_string()
    } else {
        pair_address.join(",")
    };
    info!("pairs_address: {}", pair_address);
    let url = format!(
        "{}/pairs/{}/{}",
        DEXSCREENER_API_URL, chain_id, pair_address
    );
    info!("url: {}", url);
    let body = reqwest::get(url).await?.text().await?;

    let pairs_response: PairsResponse = serde_json::from_str(&body)?;
    info!("pairs_response: {:#?}", pairs_response);
    Ok(pairs_response)
}

// Get one or multiple pairs by token address
// token_address: One or multiple, comma-separated token addresses (up to 30 addresses)
pub async fn get_tokens_by_token_address(
    pair_address: Vec<&str>,
) -> anyhow::Result<TokensResponse> {
    if pair_address.len() > 30 || pair_address.len() == 0 {
        return Err(anyhow::anyhow!(
            "pair_address length should be less than 30 or greater than 0. the current pair_address length({})", pair_address.len()
        ));
    }
    let pair_address = if pair_address.len() == 1 {
        pair_address.first().unwrap().to_string()
    } else {
        pair_address.join(",")
    };
    info!("pairs_address: {}", pair_address);
    let url = format!("{}/tokens/{}", DEXSCREENER_API_URL, pair_address);
    info!("url: {}", url);

    let body = reqwest::get(url).await?.text().await?;
    let token_response: TokensResponse = serde_json::from_str(&body)?;
    info!("token_response: {:#?}", token_response);

    Ok(token_response)
}

// Search for pairs matching query
// Query may include pair address, token address, token name or token symbol.
pub async fn search_pairs_by_query(query: &str) -> anyhow::Result<SearchResponse> {
    let split_query: Vec<&str> = query.split("/").collect();
    let collect_query = split_query.join("%20");
    info!("collect query: {}", collect_query);

    let url = format!("{}/search?q={}", DEXSCREENER_API_URL, collect_query);
    info!("url: {}", url);

    let body = reqwest::get(url).await?.text().await?;
    let pairs_response: SearchResponse = serde_json::from_str(&body)?;
    info!("pairs_response: {:?}", pairs_response);

    Ok(pairs_response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_pari_by_chain_and_pair_address() {
        let chain_id = "bsc";
        let pair_address = vec![
            "0x7213a321F1855CF1779f42c0CD85d3D95291D34C",
            "0x16b9a82891338f9ba80e2d6970fdda79d1eb0dae",
        ];
        let result = get_pairs_by_chain_and_pair_address(chain_id, pair_address).await;
        assert!(result.is_ok());
        println!("{:#?}", result);
    }

    #[tokio::test]
    async fn test_get_token_pairs_by_token_address() {
        let pair_address = vec![
            "0x2170Ed0880ac9A755fd29B2688956BD959F933F8",
            "0x16b9a82891338f9ba80e2d6970fdda79d1eb0dae",
        ];
        let result = get_tokens_by_token_address(pair_address).await;
        assert!(result.is_ok());
        println!("{:#?}", result);
    }

    #[tokio::test]
    async fn test_search_pairs_by_query() {
        let query = "WBNB/USDC";
        let result = search_pairs_by_query(query).await;
        assert!(result.is_ok());
        println!("{:?}", result);
    }
}
