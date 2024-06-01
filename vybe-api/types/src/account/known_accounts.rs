use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;
use std::fmt::Display;

#[derive(Serialize, Debug)]
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
    Influencer,
    Whale,
    Vc,
    Mm,
    None,
}

impl<'de> Deserialize<'de> for Lable {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "DeFi" => Ok(Lable::Defi),
            "Pool" => Ok(Lable::Pool),
            "Bridge" => Ok(Lable::Bridge),
            "Treasury" => Ok(Lable::Treasury),
            "dApp" => Ok(Lable::DApp),
            "Hacker" => Ok(Lable::Hacker),
            "DAO" => Ok(Lable::Dao),
            "CEX" => Ok(Lable::Cex),
            "CLMM" => Ok(Lable::Clmm),
            "Marketplace" => Ok(Lable::Marketplace),
            "CLOB" => Ok(Lable::Clob),
            "NFT" => Ok(Lable::Nft),
            "Influencer" => Ok(Lable::Influencer),
            "Whale" => Ok(Lable::Whale),
            "VC" => Ok(Lable::Vc),
            "MM" => Ok(Lable::Mm),
            "" => Ok(Lable::None),
            s => {
                println!("Unknown lable: {}", s);
                Err(serde::de::Error::custom(format!("Unknown lable: {}", s)))
            }
        }
    }
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
            Lable::Influencer => write!(f, "Influencer"),
            Lable::Whale => write!(f, "Whale"),
            Lable::None => write!(f, ""),
            Lable::Vc => write!(f, "VC"),
            Lable::Mm => write!(f, "MM"),
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

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct KnownAccountsRequest {
    /// The public key (pubKey) associated with the Solana account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_address: Option<String>,
    /// Friendly name of account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Category of account. Only accounts matching all labels will be returned (eg. labels=DEFI,NFT)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<Lable>>,
    /// Name of the business or entity that controls this account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_name: Option<String>,
    /// Entity id to filter with (including as empty or null, such as "entity_id=" will filter programs without an entity_id)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<isize>,
    /// Sort list ascending. Only one of sort_by_asc or sort_by_desc can be used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by_asc: Option<SortedBy>,
    /// Sort by descending. Only one of sort_by_asc or sort_by_desc can be used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by_desc: Option<SortedBy>,
}

impl KnownAccountsRequest {
    pub fn to_query(&self) -> String {
        let mut query = vec![];
        if let Some(owner_address) = &self.owner_address {
            query.push(format!("owner_address={}", owner_address));
        }
        if let Some(name) = &self.name {
            query.push(format!("name={}", name));
        }
        if let Some(labels) = &self.labels {
            for label in labels {
                query.push(format!("labels={}", label));
            }
        }
        if let Some(entity_name) = &self.entity_name {
            query.push(format!("entity_name={}", entity_name));
        }
        if let Some(entity_id) = &self.entity_id {
            query.push(format!("entity_id={}", entity_id));
        }
        if let Some(sort_by_asc) = &self.sort_by_asc {
            query.push(format!("sort_by_asc={}", sort_by_asc));
        }
        if let Some(sort_by_desc) = &self.sort_by_desc {
            query.push(format!("sort_by_desc={}", sort_by_desc));
        }
        query.join("&")
    }
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
    owner_address: Option<String>,
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
