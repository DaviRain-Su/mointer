use crate::VybeClient;
use vybe_api_types::prices::base_or_quote_ohlcv::{
    BaseOrQuoteOhlcvRequest, BaseOrQuoteOhlcvResponse,
};
impl VybeClient {
    /// 根据我们支持的 DEX 和 AMM 的交易和掉期，检索基础/报价对的交易价格。
    /// 交易价格是指两方之间执行交易（买入或卖出）的具体价格。
    pub async fn get_base_or_quote_ohlcv(
        &self,
        reqwest: &BaseOrQuoteOhlcvRequest,
    ) -> anyhow::Result<BaseOrQuoteOhlcvResponse> {
        let client = self.client();
        let url = format!(
            "{}/price/{}/token-trades-ohlcv?{}",
            Self::BASE_URL,
            reqwest.program_id.program_id(),
            reqwest.to_query()
        );

        let resp = client
            .get(url)
            .send()
            .await?
            .json::<BaseOrQuoteOhlcvResponse>()
            .await?;
        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use crate::VybeClient;
    use vybe_api_types::prices::base_or_quote_ohlcv::{BaseOrQuoteOhlcvRequest, SupportProgramId};

    #[tokio::test]
    async fn test_get_base_or_quote_ohlcv() {
        let client =
            VybeClient::new("8U8BxtAQVbrGzxWxaVdAkSPXtq4CPCExEbaBFQxwX4uPeByd".to_string())
                .unwrap();
        let resp = client
            .get_base_or_quote_ohlcv(&BaseOrQuoteOhlcvRequest {
                program_id: SupportProgramId::Raydiumv4,
                base_mint_address: Some("EKpQGSJtjMFqKZ9KQanSqYXRcF8fBopzLHYxdM65zcjm".to_string()),
                quote_mint_address: Some("So11111111111111111111111111111111111111112".to_string()),
                ..Default::default()
            })
            .await
            .unwrap();
        println!("{:#?}", resp);
    }
}
