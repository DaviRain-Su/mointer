use serde::{Deserialize, Serialize};
use serde_json::Value;
use solana_program::clock::Slot;
use solana_program::clock::UnixTimestamp;
use solana_program::message::MessageHeader;
use solana_sdk::transaction::Result as TransactionResult;
use solana_sdk::transaction::TransactionError;
use solana_sdk::transaction::TransactionVersion;
use solana_transaction_status::option_serializer::OptionSerializer;
use solana_transaction_status::parse_accounts::ParsedAccount;
use solana_transaction_status::Rewards;
use solana_transaction_status::TransactionBinaryEncoding;
use solana_transaction_status::UiLoadedAddresses;
use solana_transaction_status::UiTransactionReturnData;
use solana_transaction_status::UiTransactionTokenBalance;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ConfirmedBlock {
    pub previous_blockhash: String,
    pub blockhash: String,
    pub parent_slot: Slot,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transactions: Option<Vec<DecodeTransactionWithStatusMeta>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signatures: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rewards: Option<Rewards>,
    pub block_time: Option<UnixTimestamp>,
    pub block_height: Option<u64>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DecodeConfirmedTransactionWithStatusMeta {
    pub slot: Slot,
    #[serde(flatten)]
    pub transaction: DecodeTransactionWithStatusMeta,
    pub block_time: Option<UnixTimestamp>,
}

impl From<solana_transaction_status::EncodedConfirmedTransactionWithStatusMeta>
    for DecodeConfirmedTransactionWithStatusMeta
{
    fn from(value: solana_transaction_status::EncodedConfirmedTransactionWithStatusMeta) -> Self {
        Self {
            slot: value.slot,
            transaction: value.transaction.into(),
            block_time: value.block_time,
        }
    }
}

impl From<solana_transaction_status::UiConfirmedBlock> for ConfirmedBlock {
    fn from(value: solana_transaction_status::UiConfirmedBlock) -> Self {
        Self {
            previous_blockhash: value.previous_blockhash,
            blockhash: value.blockhash,
            parent_slot: value.parent_slot,
            transactions: value.transactions.map(|txs| {
                txs.into_iter()
                    .map(DecodeTransactionWithStatusMeta::from)
                    .collect()
            }),
            signatures: value.signatures,
            rewards: value.rewards,
            block_time: value.block_time,
            block_height: value.block_height,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DecodeTransactionWithStatusMeta {
    pub transaction: DecodeTransaction,
    pub meta: Option<DecodeTransactionStatusMeta>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<TransactionVersion>,
}

impl From<solana_transaction_status::EncodedTransactionWithStatusMeta>
    for DecodeTransactionWithStatusMeta
{
    fn from(value: solana_transaction_status::EncodedTransactionWithStatusMeta) -> Self {
        Self {
            transaction: value.transaction.into(),
            meta: value.meta.map(Into::into),
            version: value.version,
        }
    }
}

/// A duplicate representation of TransactionStatusMeta with `err` field
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DecodeTransactionStatusMeta {
    pub err: Option<TransactionError>,
    pub status: TransactionResult<()>, // This field is deprecated.  See https://github.com/solana-labs/solana/issues/9302
    pub fee: u64,
    pub pre_balances: Vec<u64>,
    pub post_balances: Vec<u64>,
    #[serde(
        default = "OptionSerializer::none",
        skip_serializing_if = "OptionSerializer::should_skip"
    )]
    pub inner_instructions: OptionSerializer<Vec<DecodeInnerInstructions>>,
    #[serde(
        default = "OptionSerializer::none",
        skip_serializing_if = "OptionSerializer::should_skip"
    )]
    pub log_messages: OptionSerializer<Vec<String>>,
    #[serde(
        default = "OptionSerializer::none",
        skip_serializing_if = "OptionSerializer::should_skip"
    )]
    pub pre_token_balances: OptionSerializer<Vec<UiTransactionTokenBalance>>,
    #[serde(
        default = "OptionSerializer::none",
        skip_serializing_if = "OptionSerializer::should_skip"
    )]
    pub post_token_balances: OptionSerializer<Vec<UiTransactionTokenBalance>>,
    #[serde(
        default = "OptionSerializer::none",
        skip_serializing_if = "OptionSerializer::should_skip"
    )]
    pub rewards: OptionSerializer<Rewards>,
    #[serde(
        default = "OptionSerializer::skip",
        skip_serializing_if = "OptionSerializer::should_skip"
    )]
    pub loaded_addresses: OptionSerializer<UiLoadedAddresses>,
    #[serde(
        default = "OptionSerializer::skip",
        skip_serializing_if = "OptionSerializer::should_skip"
    )]
    pub return_data: OptionSerializer<UiTransactionReturnData>,
    #[serde(
        default = "OptionSerializer::skip",
        skip_serializing_if = "OptionSerializer::should_skip"
    )]
    pub compute_units_consumed: OptionSerializer<u64>,
}

// DecodeTransactionStatusMeta
impl From<solana_transaction_status::UiTransactionStatusMeta> for DecodeTransactionStatusMeta {
    fn from(
        ui_transaction_status_meta: solana_transaction_status::UiTransactionStatusMeta,
    ) -> Self {
        let inner_instructions = match ui_transaction_status_meta.inner_instructions {
            OptionSerializer::Some(instructions) => OptionSerializer::Some(
                instructions
                    .into_iter()
                    .map(Into::into)
                    .collect::<Vec<DecodeInnerInstructions>>(),
            ),
            OptionSerializer::None => OptionSerializer::None,
            OptionSerializer::Skip => OptionSerializer::skip(),
        };
        DecodeTransactionStatusMeta {
            err: ui_transaction_status_meta.err,
            status: ui_transaction_status_meta.status,
            fee: ui_transaction_status_meta.fee,
            pre_balances: ui_transaction_status_meta.pre_balances,
            post_balances: ui_transaction_status_meta.post_balances,
            inner_instructions,
            log_messages: ui_transaction_status_meta.log_messages,
            pre_token_balances: ui_transaction_status_meta.pre_token_balances,

            post_token_balances: ui_transaction_status_meta.post_token_balances,
            rewards: ui_transaction_status_meta.rewards,
            loaded_addresses: ui_transaction_status_meta.loaded_addresses,
            return_data: ui_transaction_status_meta.return_data,
            compute_units_consumed: ui_transaction_status_meta.compute_units_consumed,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DecodeInnerInstructions {
    /// Transaction instruction index
    pub index: u8,
    /// List of inner instructions
    pub instructions: Vec<DecodeInstruction>,
}

impl From<solana_transaction_status::UiInnerInstructions> for DecodeInnerInstructions {
    fn from(ui_inner_instructions: solana_transaction_status::UiInnerInstructions) -> Self {
        DecodeInnerInstructions {
            index: ui_inner_instructions.index,
            instructions: ui_inner_instructions
                .instructions
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum DecodeTransaction {
    LegacyBinary(String), // Old way of expressing base-58, retained for RPC backwards compatibility
    Binary(String, TransactionBinaryEncoding),
    Json(Transaction),
    Accounts(AccountsList),
}

impl From<solana_transaction_status::EncodedTransaction> for DecodeTransaction {
    fn from(encoded_transaction: solana_transaction_status::EncodedTransaction) -> Self {
        match encoded_transaction {
            solana_transaction_status::EncodedTransaction::LegacyBinary(encoded_transaction) => {
                DecodeTransaction::LegacyBinary(encoded_transaction)
            }
            solana_transaction_status::EncodedTransaction::Binary(
                encoded_transaction,
                encoding,
            ) => DecodeTransaction::Binary(encoded_transaction, encoding),
            solana_transaction_status::EncodedTransaction::Json(transaction) => {
                DecodeTransaction::Json(transaction.into())
            }
            solana_transaction_status::EncodedTransaction::Accounts(accounts) => {
                DecodeTransaction::Accounts(accounts.into())
            }
        }
    }
}

/// A duplicate representation of a Transaction for pretty JSON serialization
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub signatures: Vec<String>,
    pub message: UiMessage,
}

impl From<solana_transaction_status::UiTransaction> for Transaction {
    fn from(ui_transaction: solana_transaction_status::UiTransaction) -> Self {
        Transaction {
            signatures: ui_transaction.signatures,
            message: ui_transaction.message.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountsList {
    pub signatures: Vec<String>,
    pub account_keys: Vec<ParsedAccount>,
}

impl From<solana_transaction_status::UiAccountsList> for AccountsList {
    fn from(ui_accounts_list: solana_transaction_status::UiAccountsList) -> Self {
        AccountsList {
            signatures: ui_accounts_list.signatures,
            account_keys: ui_accounts_list
                .account_keys
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum UiMessage {
    Parsed(ParsedMessage),
    Raw(RawMessage),
}

impl From<solana_transaction_status::UiMessage> for UiMessage {
    fn from(ui_message: solana_transaction_status::UiMessage) -> Self {
        match ui_message {
            solana_transaction_status::UiMessage::Parsed(parsed_message) => {
                UiMessage::Parsed(parsed_message.into())
            }
            solana_transaction_status::UiMessage::Raw(raw_message) => {
                UiMessage::Raw(raw_message.into())
            }
        }
    }
}

/// A duplicate representation of a Message, in parsed format, for pretty JSON serialization
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedMessage {
    pub account_keys: Vec<ParsedAccount>,
    pub recent_blockhash: String,
    pub instructions: Vec<DecodeInstruction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_table_lookups: Option<Vec<AddressTableLookup>>,
}

impl From<solana_transaction_status::UiParsedMessage> for ParsedMessage {
    fn from(parsed_message: solana_transaction_status::UiParsedMessage) -> Self {
        ParsedMessage {
            account_keys: parsed_message.account_keys,
            recent_blockhash: parsed_message.recent_blockhash,
            instructions: parsed_message
                .instructions
                .into_iter()
                .map(Into::into)
                .collect(),
            address_table_lookups: parsed_message
                .address_table_lookups
                .map(|v| v.into_iter().map(Into::into).collect()),
        }
    }
}

/// A duplicate representation of a Message, in raw format, for pretty JSON serialization
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawMessage {
    pub header: MessageHeader,
    pub account_keys: Vec<String>,
    pub recent_blockhash: String,
    pub instructions: Vec<CompiledInstruction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_table_lookups: Option<Vec<AddressTableLookup>>,
}

impl From<solana_transaction_status::UiRawMessage> for RawMessage {
    fn from(raw_message: solana_transaction_status::UiRawMessage) -> Self {
        RawMessage {
            header: raw_message.header,
            account_keys: raw_message.account_keys,
            recent_blockhash: raw_message.recent_blockhash,
            instructions: raw_message
                .instructions
                .into_iter()
                .map(CompiledInstruction::from)
                .collect(),
            address_table_lookups: raw_message
                .address_table_lookups
                .map(|address_table_lookups| {
                    address_table_lookups
                        .into_iter()
                        .map(AddressTableLookup::from)
                        .collect()
                }),
        }
    }
}

/// A duplicate representation of a MessageAddressTableLookup, in raw format, for pretty JSON serialization
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddressTableLookup {
    pub account_key: String,
    pub writable_indexes: Vec<u8>,
    pub readonly_indexes: Vec<u8>,
}

impl From<solana_transaction_status::UiAddressTableLookup> for AddressTableLookup {
    fn from(address_table_lookup: solana_transaction_status::UiAddressTableLookup) -> Self {
        AddressTableLookup {
            account_key: address_table_lookup.account_key,
            writable_indexes: address_table_lookup.writable_indexes,
            readonly_indexes: address_table_lookup.readonly_indexes,
        }
    }
}

/// A duplicate representation of an Instruction for pretty JSON serialization
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum DecodeInstruction {
    Compiled(CompiledInstruction),
    Parsed(DecodeParsedInstruction),
}

impl From<solana_transaction_status::UiInstruction> for DecodeInstruction {
    fn from(instruction: solana_transaction_status::UiInstruction) -> Self {
        match instruction {
            solana_transaction_status::UiInstruction::Parsed(instruction) => {
                DecodeInstruction::Parsed(instruction.into())
            }
            solana_transaction_status::UiInstruction::Compiled(instruction) => {
                DecodeInstruction::Compiled(instruction.into())
            }
        }
    }
}

/// A duplicate representation of a CompiledInstruction for pretty JSON serialization
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompiledInstruction {
    pub program_id_index: u8,
    pub accounts: Vec<u8>,
    pub data: Vec<u8>,
    pub stack_height: Option<u32>,
}

impl From<solana_transaction_status::UiCompiledInstruction> for CompiledInstruction {
    fn from(instruction: solana_transaction_status::UiCompiledInstruction) -> Self {
        CompiledInstruction {
            program_id_index: instruction.program_id_index,
            accounts: instruction.accounts,
            data: bs58::decode(&instruction.data).into_vec().unwrap(),
            stack_height: instruction.stack_height,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum DecodeParsedInstruction {
    Parsed(ParsedInstruction),
    PartiallyDecoded(PartiallyDecodedInstruction),
}

impl From<solana_transaction_status::UiParsedInstruction> for DecodeParsedInstruction {
    fn from(instruction: solana_transaction_status::UiParsedInstruction) -> Self {
        match instruction {
            solana_transaction_status::UiParsedInstruction::Parsed(instruction) => {
                DecodeParsedInstruction::Parsed(instruction.into())
            }
            solana_transaction_status::UiParsedInstruction::PartiallyDecoded(instruction) => {
                DecodeParsedInstruction::PartiallyDecoded(instruction.into())
            }
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ParsedInstruction {
    pub program: String,
    pub program_id: String,
    pub parsed: Value,
    pub stack_height: Option<u32>,
}

impl From<solana_transaction_status::parse_instruction::ParsedInstruction> for ParsedInstruction {
    fn from(instruction: solana_transaction_status::parse_instruction::ParsedInstruction) -> Self {
        Self {
            program: instruction.program,
            program_id: instruction.program_id,
            parsed: instruction.parsed,
            stack_height: instruction.stack_height,
        }
    }
}

/// A partially decoded CompiledInstruction that includes explicit account addresses
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PartiallyDecodedInstruction {
    pub program_id: String,
    pub accounts: Vec<String>,
    pub data: String,
    pub stack_height: Option<u32>,
}

impl From<solana_transaction_status::UiPartiallyDecodedInstruction>
    for PartiallyDecodedInstruction
{
    fn from(instruction: solana_transaction_status::UiPartiallyDecodedInstruction) -> Self {
        Self {
            program_id: instruction.program_id,
            accounts: instruction.accounts,
            data: instruction.data,
            stack_height: instruction.stack_height,
        }
    }
}
