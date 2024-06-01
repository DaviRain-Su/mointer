use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;

#[derive(Serialize, Deserialize, Default)]
pub enum SupportProgramId {
    #[default]
    Raydiumv4,
    RaydiumCLMM,
    Phoenix,
    OrcaWhirlpool,
}

impl Display for SupportProgramId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SupportProgramId::Raydiumv4 => write!(f, "Raydiumv4"),
            SupportProgramId::RaydiumCLMM => write!(f, "RaydiumCLMM"),
            SupportProgramId::Phoenix => write!(f, "Phoenix"),
            SupportProgramId::OrcaWhirlpool => write!(f, "OrcaWhirlpool"),
        }
    }
}

impl SupportProgramId {
    pub fn program_id(&self) -> String {
        match self {
            SupportProgramId::Raydiumv4 => {
                "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8".to_string()
            }
            SupportProgramId::RaydiumCLMM => {
                "CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK".to_string()
            }
            SupportProgramId::Phoenix => "PhoeNiXZ8ByJGLkxNfZRnkUfjvmuYqLR89jjFHGqdXY".to_string(),
            SupportProgramId::OrcaWhirlpool => {
                "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc".to_string()
            }
        }
    }
}

#[derive(Default)]
pub struct BaseOrQuoteOhlcvRequest {
    pub program_id: SupportProgramId,
    ///The public key of the base token
    pub base_mint_address: Option<String>,
    /// The public key of the quote token
    pub quote_mint_address: Option<String>,
    /// Market id to filter with. If provided, the baseMintAddress and quoteMintAddress fields are ignored
    pub market_id: Option<String>,
    /// Resolution of the data. Possible values: 1h, 1d, 1w, 1m, 1y, or a string parseable to seconds
    pub resolution: Option<String>,
    /// Start time of the data to return (unix timestamp)
    pub time_start: Option<i64>,
    /// End time of the data to return (unix timestamp)
    pub time_end: Option<i64>,
    /// Page selection, 0-indexed.
    pub page: Option<i32>,
}

impl BaseOrQuoteOhlcvRequest {
    pub fn to_query(&self) -> String {
        let mut query = String::new();
        if self.base_mint_address.is_none() && self.quote_mint_address.is_none() {
            if let Some(market_id) = &self.market_id {
                query.push_str(&format!("&marketId={}", market_id));
            }
        } else {
            if let Some(base_mint_address) = &self.base_mint_address {
                query.push_str(&format!("&baseMintAddress={}", base_mint_address));
            }
            if let Some(quote_mint_address) = &self.quote_mint_address {
                query.push_str(&format!("&quoteMintAddress={}", quote_mint_address));
            }
        }

        if let Some(resolution) = &self.resolution {
            query.push_str(&format!("&resolution={}", resolution));
        }
        if let Some(time_start) = &self.time_start {
            query.push_str(&format!("&timeStart={}", time_start));
        }
        if let Some(time_end) = &self.time_end {
            query.push_str(&format!("&timeEnd={}", time_end));
        }
        if let Some(page) = &self.page {
            query.push_str(&format!("&page={}", page));
        }
        query
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseOrQuoteOhlcvResponse {
    pub data: Vec<OhlcvData>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OhlcvData {
    pub time_bucket_start: u64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub count: i64,
}
