use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use std::fmt;
use std::fmt::Display;

#[derive(Deserialize, Serialize, Debug)]
pub enum Lable {
    Defi,
    Pool,
    Bridge,
    Treasury,
    DApp,
    Hacker,
    Dao,
    Cex,
    Clmm,
    Marketplace,
    Clob,
    Nft,
}

impl Display for Lable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Lable::Defi => write!(f, "DeFi"),
            Lable::Pool => write!(f, "Pool"),
            Lable::Bridge => write!(f, "Bridge"),
            Lable::Treasury => write!(f, "Treasury"),
            Lable::DApp => write!(f, "dApp"),
            Lable::Hacker => write!(f, "Hacker"),
            Lable::Dao => write!(f, "DAO"),
            Lable::Cex => write!(f, "CEX"),
            Lable::Clmm => write!(f, "CLMM"),
            Lable::Clob => write!(f, "CLOB"),
            Lable::Marketplace => write!(f, "Marketplace"),
            Lable::Nft => write!(f, "NFT"),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub enum SortedBy {
    Asc,
    Desc,
}

impl Display for SortedBy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SortedBy::Asc => write!(f, "sort_by_asc"),
            SortedBy::Desc => write!(f, "sort_by_desc"),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KnownAccountsRequest {
    /// The public key (pubKey) associated with the Solana account
    #[serde(skip_serializing_if = "Option::is_none")]
    owner_address: Option<Pubkey>,
    /// Friendly name of account
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    /// Category of account. Only accounts matching all labels will be returned (eg. labels=DEFI,NFT)
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<Vec<Lable>>,
    /// Name of the business or entity that controls this account
    #[serde(skip_serializing_if = "Option::is_none")]
    entity_name: Option<String>,
    /// Entity id to filter with (including as empty or null, such as "entity_id=" will filter programs without an entity_id)
    #[serde(skip_serializing_if = "Option::is_none")]
    entity_id: Option<isize>,
    /// Sort list ascending. Only one of sort_by_asc or sort_by_desc can be used
    #[serde(skip_serializing_if = "Option::is_none")]
    sort_by_asc: Option<String>,
    /// Sort by descending. Only one of sort_by_asc or sort_by_desc can be used
    #[serde(skip_serializing_if = "Option::is_none")]
    sort_by_desc: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KnownAccountsResponse {
    accounts: Vec<KnownAccountsResponseInner>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KnownAccountsResponseInner {
    #[serde(skip_serializing_if = "Option::is_none")]
    owner_address: Option<Pubkey>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logo_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<Vec<Lable>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entity_id: Option<isize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_added: Option<String>,
}
