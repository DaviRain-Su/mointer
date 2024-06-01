use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashSet;
use std::fs::write;
use tokio;

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
async fn main() {
    let client = Client::new();
    let url = "https://mainnet.helius-rpc.com/?api-key=945100da-05f7-4e19-9c1c-ce0778de6ef7";
    let mut all_owners: HashSet<String> = HashSet::new();
    let mut cursor: Option<String> = None;

    loop {
        let params = json!({
            "limit": 1000,
            "mint": "ukHH6c7mMyiWCf1b9pnWe25TSpkDDt3H5pQZgZ74J82",
            "cursor": cursor,
        });
        println!("Params: {:?}", params);
        println!("Requesting with cursor {:?}", cursor);

        let res = client
            .post(url)
            .json(&json!({
                "jsonrpc": "2.0",
                "id": "helius-test",
                "method": "getTokenAccounts",
                "params": params,
            }))
            .send()
            .await
            .expect("Failed to send request");

        let data: ResponseData = res.json().await.expect("Failed to parse JSON");
        println!(
            "Got {} accounts",
            data.result.as_ref().unwrap().token_accounts.len()
        );

        match data.result {
            Some(result) => {
                if result.token_accounts.is_empty() {
                    println!("No more results");
                    break;
                }

                for account in result.token_accounts {
                    all_owners.insert(account.owner);
                }

                cursor = result.cursor;
            }
            None => break,
        }
        println!("Current owners: {}", all_owners.len());
    }

    write(
        "output.json",
        serde_json::to_string_pretty(&all_owners).unwrap(),
    )
    .expect("Failed to write to file");
}
