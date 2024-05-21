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
//! 17. Memo Program v2 - MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr
//! 18. Jupiter DCA Program : DCA265Vj8a9CEuX1eb1LWRnDT7uK6q1xMipnNyatn23M
//!
//! token address:
//!
//! 1. Sol token address - So11111111111111111111111111111111111111112
//! 2. USDT token address - Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB
//!
use bincode::deserialize;
use clap::Parser;
use solana_client::rpc_client::GetConfirmedSignaturesForAddress2Config;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::RpcBlockConfig;
use solana_client::rpc_config::RpcTransactionConfig;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::compute_budget::ComputeBudgetInstruction;
use solana_sdk::signature::Signature;
use solana_sdk::system_instruction::SystemInstruction;
use solana_transaction_status::UiTransactionEncoding;
use solana_transaction_status::{EncodedTransaction, UiMessage};
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;
use tracing::info;
use tx_parse_types::DecodeConfirmedTransactionWithStatusMeta;
use tx_parse_types::DecodeTransaction;

#[derive(Parser, Debug)]
pub enum SolanaRpc {
    #[command(name = "get-block", about = "Get block info")]
    GetBlock { solt: u64 },
    #[command(name = "get-transaction", about = "Get transaction info")]
    GetTransaction { signature: String },
    #[command(
        name = "get-transaction-by-address",
        about = "Get transaction info by address"
    )]
    GetTransactionByAddress { address: String },
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
                                    // vovte program
                                    {
                                        filter_vote_program.push(tx1);
                                    } else if message.account_keys
                                        [instruction.program_id_index as usize]
                                        == "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8"
                                    {
                                        let data = bs58::decode(&instruction.data).into_vec()?;
                                        let decode_data =
                                            raydium_amm_types::AmmInstruction::unpack(&data)?;
                                        println!("{:?}", decode_data);
                                    }
                                }
                            }
                            _ => unimplemented!(),
                        },
                        _ => unimplemented!(),
                    }
                }

                println!("filter_vote_program: {:?}", filter_vote_program);
                println!("filter_vote_program length: {}", filter_vote_program.len());
            }
            SolanaRpc::GetTransaction { signature } => {
                let config = RpcTransactionConfig {
                    encoding: Some(UiTransactionEncoding::Json),
                    commitment: Some(CommitmentConfig::finalized()),
                    max_supported_transaction_version: Some(0),
                };
                let result =
                    client.get_transaction_with_config(&Signature::from_str(signature)?, config)?;
                // let result = DecodeConfirmedTransactionWithStatusMeta::from(result);
                match &result.transaction.transaction {
                    EncodedTransaction::Json(tx) => match &tx.message {
                        UiMessage::Raw(message) => {
                            for instruction in message.instructions.iter() {
                                if message.account_keys[instruction.program_id_index as usize]
                                    == "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8"
                                {
                                    let data = bs58::decode(&instruction.data).into_vec()?;
                                    let decode_data =
                                        raydium_amm_types::AmmInstruction::unpack(&data)?;
                                    println!("{:?}", decode_data);
                                } else if message.account_keys
                                    [instruction.program_id_index as usize]
                                    == "11111111111111111111111111111111"
                                {
                                    let data = bs58::decode(&instruction.data).into_vec()?;
                                    // use bincode to deserialize
                                    let system_instruction =
                                        deserialize::<SystemInstruction>(&data)?;
                                    println!("system_instruction: {:?}", system_instruction);
                                } else if message.account_keys
                                    [instruction.program_id_index as usize]
                                    == "ComputeBudget111111111111111111111111111111"
                                {
                                    let data = bs58::decode(&instruction.data).into_vec()?;
                                    let compute_budget_instruction =
                                        borsh::from_slice::<ComputeBudgetInstruction>(&data)?;
                                    println!("{:?}", compute_budget_instruction);
                                } else if message.account_keys
                                    [instruction.program_id_index as usize]
                                    == "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4"
                                {
                                    for account_idx in instruction.accounts.iter() {
                                        println!(
                                            "account: {}",
                                            message.account_keys[*account_idx as usize]
                                        );
                                    }
                                }
                            }
                        }
                        _ => unimplemented!(),
                    },
                    _ => unimplemented!(),
                }
                println!("{:?}", result);
            }
            SolanaRpc::GetTransactionByAddress { address } => {
                let address = solana_sdk::pubkey::Pubkey::from_str(address)?;
                let config = GetConfirmedSignaturesForAddress2Config {
                    before: Some(Signature::from_str("5a5Xbz1kjs69gesYpJ9HkeDWGkVrj7o1qf8D9YHTYREvszuNAmsSsM6LJoi4wKiGMwJSTgHfvKQh7HgdThayM5FB").unwrap()),
                    until: None,
                    limit: Some(1000),
                    commitment: Some(CommitmentConfig::confirmed()),
                };
                let result = client
                    .get_signatures_for_address_with_config(&address, config)?
                    .into_iter()
                    .map(|value| Signature::from_str(&value.signature).unwrap())
                    .collect::<Vec<_>>();
                println!("Address {} have {} transacition", address, result.len());
                for (idx, signature) in result.iter().enumerate() {
                    sleep(Duration::from_secs(1));
                    println!("process {}: {}", idx, signature);
                }
            }
        }
        Ok(())
    }
}
