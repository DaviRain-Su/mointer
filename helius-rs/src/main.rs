use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::{HashMap, HashSet};
use std::fs::write;
use std::time::Duration;
use tokio;
use tokio::time::sleep;
use vybe_api::VybeClient;
use vybe_api_types::tokens::token::SimpleToken;

// 定义最大重试次数和重试间的等待时间
const MAX_RETRIES: u32 = 3;
const RETRY_DELAY: Duration = Duration::from_secs(2);

#[derive(Serialize, Deserialize, Debug)]
struct TokenAccount {
    owner: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ResultData {
    token_accounts: Vec<TokenAccount>,
    cursor: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ResponseData {
    result: Option<ResultData>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new();
    let base_url = "https://mainnet.helius-rpc.com/?api-key=945100da-05f7-4e19-9c1c-ce0778de6ef7";
    let vp_client = VybeClient::new("8U8BxtAQVbrGzxWxaVdAkSPXtq4CPCExEbaBFQxwX4uPeByd".to_string())
        .map_err(|e| anyhow::anyhow!("Failed to create VybeClient: {}", e))?;
    let resp = vp_client
        .tokens(None)
        .await
        .map_err(|e| anyhow::anyhow!("feild call to fetch tokens: {}", e))?;

    let tokens = resp
        .data
        .into_iter()
        .map(SimpleToken::from)
        .collect::<Vec<SimpleToken>>();

    println!("Found {} tokens", tokens.len());

    let mut token_owners: HashMap<String, HashSet<String>> = HashMap::new();

    for (idx, token) in tokens.iter().enumerate() {
        println!("Index: {}, Fetching owners for {}", idx, token.symbol);
        let mut owners: HashSet<String> = HashSet::new();
        let mut cursor: Option<String> = None;
        loop {
            let params = json!({
                "limit": 1000,
                "mint": token.mint_address,
                "cursor": cursor,
            });

            let data = fetch_token_accounts(&client, &base_url, &params)
                .await
                .map_err(|e| anyhow::anyhow!("Failed to fetch token accounts: {}", e))?;
            if let Some(result) = data.result {
                if result.token_accounts.is_empty() {
                    println!("No more results for {}", token.symbol);
                    break;
                }

                for account in result.token_accounts {
                    owners.insert(account.owner);
                }

                cursor = result.cursor;
                println!("token: {}, Cursor: {:?}", token.symbol, cursor);
            } else {
                println!("No result for {}", token.symbol);
                break;
            }
        }
        write(
            format!("../holders/{}_holder.json", token.symbol),
            serde_json::to_string_pretty(&owners)
                .map_err(|e| anyhow::anyhow!("Failed to serde owners to file: {}", e))?,
        )
        .map_err(|e| anyhow::anyhow!("Failed to write to file: {}", e))?;

        println!("Found {} owners for {}", owners.len(), token.symbol);
        token_owners.insert(token.symbol.clone(), owners);
    }

    println!("Found {} tokens", token_owners.len());
    println!("Calculating common owners...");

    let common_owners = token_owners.values().skip(1).fold(
        token_owners.values().next().cloned().unwrap_or_default(),
        |acc, set| acc.intersection(set).cloned().collect(),
    );

    println!("Found {} common owners", common_owners.len());

    // Optionally, write common owners to a file
    write(
        "../holders/common_owners.json",
        serde_json::to_string_pretty(&common_owners)
            .map_err(|e| anyhow::anyhow!("Failed : {} to serde common owner to file", e))?,
    )
    .map_err(|e| anyhow::anyhow!("Failed : {} to write to file", e))?;

    println!("Common owners written to common_owners.json");
    Ok(())
}

async fn fetch_token_accounts(
    client: &Client,
    base_url: &str,
    params: &serde_json::Value,
) -> Result<ResponseData, String> {
    let mut retries = 0;

    while retries < MAX_RETRIES {
        let res = client
            .post(base_url)
            .json(&json!({
                "jsonrpc": "2.0",
                "id": "helius-test",
                "method": "getTokenAccounts",
                "params": params,
            }))
            .send()
            .await;

        match res {
            Ok(response) => match response.json::<ResponseData>().await {
                Ok(data) => return Ok(data),
                Err(e) => {
                    eprintln!("Failed to parse JSON: {}", e);
                    retries += 1;
                    sleep(RETRY_DELAY).await;
                }
            },
            Err(e) => {
                eprintln!("Failed to send request: {}", e);
                retries += 1;
                sleep(RETRY_DELAY).await;
            }
        }
    }

    Err("Failed to fetch token accounts after several retries".to_string())
}
