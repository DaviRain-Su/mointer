use clap::Parser;
use helius::error::Result;
use helius::types::*;
use helius::Helius;

#[derive(Parser, Debug)]
pub enum Command {
    #[command(name = "px", about = "Parse Tx")]
    ParseTx(ParseTx),
}

#[derive(Parser, Debug)]
pub struct ParseTx {}

impl Command {
    pub async fn run(&self) -> anyhow::Result<()> {
        match self {
            Command::ParseTx(_parse_tx) => {
                let api_key: &str = "945100da-05f7-4e19-9c1c-ce0778de6ef7";
                let cluster: Cluster = Cluster::MainnetBeta;

                let helius: Helius = Helius::new(api_key, cluster).unwrap();

                let request: ParseTransactionsRequest = ParseTransactionsRequest {
                        transactions: vec![
                            "2sShYqqcWAcJiGc3oK74iFsYKgLCNiY2DsivMbaJGQT8pRzR8z5iBcdmTMXRobH8cZNZgeV9Ur9VjvLsykfFE2Li".to_string(),
                        ],
                    };

                let response: Result<Vec<EnhancedTransaction>> =
                    helius.parse_transactions(request).await;
                println!("Assets: {:#?}", response);
            }
        }
        Ok(())
    }
}
