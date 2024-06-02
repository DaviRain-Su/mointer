use anyhow::Result;
use glob::glob;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use tokio::io::AsyncWriteExt;

#[derive(Serialize, Deserialize, Debug)]
struct Holder {
    owners: Vec<String>,
}

async fn find_common_owners() -> Result<()> {
    let pattern = "../holders/*_holder.json";
    let mut token_owners: Vec<HashSet<String>> = vec![];

    // 读取所有符合模式的文件
    for entry in glob(pattern)? {
        let path = entry?.display().to_string();
        let data = fs::read_to_string(&path)?;
        let holder: Vec<String> = serde_json::from_str(&data)?;
        println!("file: {} have {} holder", path, holder.len());
        token_owners.push(holder.into_iter().collect());
    }

    // 确保至少有一个集合来进行交集操作
    let common_owners = token_owners.iter().fold(
        token_owners.first().cloned().unwrap_or_default(),
        |acc, set| acc.intersection(set).cloned().collect(),
    );

    println!("Found {} common owners", common_owners.len());

    // 将公共地址写入文件
    let mut file = tokio::fs::File::create("../holders/common_owners.json").await?;
    let json_data = serde_json::to_string_pretty(&common_owners)?;
    file.write_all(json_data.as_bytes()).await?;
    file.sync_all().await?;

    println!("Common owners written to common_owners.json");

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = find_common_owners().await {
        eprintln!("Error: {}", e);
    }
}
