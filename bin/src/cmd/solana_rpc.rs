//! # Solana program ID
//!
//! 1. Vote - Vote111111111111111111111111111111111111111
//! 2. Comput Budget - ComputeBudget111111111111111111111111111111
//! 3. Drift v2 - dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH
//! 4. System program - 11111111111111111111111111111111
//! 5. Sequence Enforcer - GDDMwNyyx8uB6zrqwBFHjLLG3TBYk2F8Az4yrQC5RzMp
//! 6. Phoenix - PhoeNiXZ8ByJGLkxNfZRnkUfjvmuYqLR89jjFHGqdXY
//! 7. Pyth Orcale - FsJ3A3u2vn5cTVofAjvy6y5kwABJAqYWpe4975bi2epH
//! 8. Token program - TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
//! 9. Associated token account program - ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL
//! 10. Jupyter Aggregator v6 - JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4
//! 11. Saber Stable swap - SSwpkEEcbUqx4vtoEByFjSkhKdCT862DNVb52nZg1UZ
//! 12. Meteora DLMM program - LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo
//! 13. Orca - whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc
//! 14. Invariant Swap - HyaB3W9q6XdA5xwpU4XnSZV94htfmbmqJXZcEbRaJutt
//! 15. Mercurial Stable swap - MERLuDFBMmsHnsBPZw2sDQZHvXFMwp8EdjudcU2HKky
//! 16. Raydium Liquidity Pool v4 - 675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8
//!
//!
//!
//! token address:
//!
//! 1. Sol token address - So11111111111111111111111111111111111111112
//! 2. USDT token address - Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB
//!
use clap::Parser;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::RpcBlockConfig;
use solana_transaction_status::{EncodedTransaction, UiMessage};
use tracing::info;

#[derive(Parser, Debug)]
pub enum SolanaRpc {
    #[command(name = "get-block", about = "Get block info")]
    GetBlock { solt: u64 },
}

impl SolanaRpc {
    pub async fn run(&self) -> anyhow::Result<()> {
        let client = RpcClient::new("https://gayleen-v43l6p-fast-mainnet.helius-rpc.com");
        match self {
            SolanaRpc::GetBlock { solt } => {
                info!("solt: {}", solt);
                // 配置请求参数，包含 maxSupportedTransactionVersion
                let config = RpcBlockConfig {
                    max_supported_transaction_version: Some(0),
                    ..RpcBlockConfig::default()
                };
                let result = client.get_block_with_config(*solt, config)?;
                let tx = result.transactions.unwrap_or_default();
                println!("tx len all have {}", tx.len());

                // filter success tx
                let txs_success = tx
                    .iter()
                    .filter(|tx| {
                        if let Some(meta) = &tx.meta {
                            meta.err.is_none()
                        } else {
                            true
                        }
                    })
                    .collect::<Vec<_>>();
                //println!("txs_success: {:?}", txs_success);
                println!("tx_success length: {}", txs_success.len());

                // filter vote program instruction Vote111111111111111111111111111111111111111
                let mut filter_vote_program = Vec::new();
                for tx1 in txs_success.iter() {
                    match &tx1.transaction {
                        EncodedTransaction::Json(tx) => match &tx.message {
                            UiMessage::Raw(message) => {
                                for instruction in message.instructions.iter() {
                                    if message.account_keys[instruction.program_id_index as usize]
                                        != "Vote111111111111111111111111111111111111111"
                                    {
                                        filter_vote_program.push(tx1);
                                    }
                                }
                            }
                            _ => unimplemented!(),
                        },
                        _ => unimplemented!(),
                    }
                }

                //println!("filter_vote_program: {:#?}", filter_vote_program);
                println!("filter_vote_program length: {}", filter_vote_program.len());
            }
        }
        Ok(())
    }
}
