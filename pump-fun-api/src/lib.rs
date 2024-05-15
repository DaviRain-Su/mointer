use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NewsMintsResponse {
    pub mint: Vec<String>,
}

/// Get the newly minted tokens
pub async fn get_newes_mints(limit: Option<u8>) -> anyhow::Result<NewsMintsResponse> {
    let client = Client::new();
    let url = "https://pumpapi.fun/api/get_newer_mints";
    let params = if let Some(v) = limit {
        if v > 20 {
            return Err(anyhow::anyhow!("Limit should be less than 20"));
        } else {
            vec![("limit", v)]
        }
    } else {
        vec![("limit", 1)]
    };
    let response = client.get(url).query(&params).send().await?;

    if response.status().is_success() {
        let data: NewsMintsResponse = response.json().await?;
        println!("{:#?}", data); // Process the JSON response
        Ok(data)
    } else {
        println!("Error retrieving data: {}", response.status());
        Err(anyhow::anyhow!(
            "Error retrieving data: {}",
            response.status()
        ))
    }
}

// ERROR: Error retrieving data: 400 Bad Request
/// To fetch the token metadata about a Pump token such as name, ticker, image etc.
pub async fn get_token_metadata(token_id: &str) -> anyhow::Result<()> {
    let response = reqwest::get(format!("https://pumpapi.fun/api/get_metadata/{}", token_id))
        .await?
        .text()
        .await?;
    println!("{:#?}", response);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_newes_mints() {
        let _ = get_newes_mints(None).await.unwrap();
    }

    #[tokio::test]
    async fn test_get_token_metadata() {
        let _ = get_token_metadata("9QUYvUGiqCALxrMCyJrVYXtJSpt4BYzPRv5ZRjsdqzkh")
            .await
            .unwrap();
    }
}
