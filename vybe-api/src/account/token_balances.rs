use crate::VybeClient;
use vybe_api_types::account::token_balances::{TokenBalancesRequest, TokenBalancesResponse};

impl VybeClient {
    /// 该端点整合了 SPL 代币持有量，并提供美元和 SOL 的投资组合价值，从而有助于全面了解账户的代币余额。
    pub async fn token_balances(
        &self,
        address: TokenBalancesRequest,
    ) -> anyhow::Result<TokenBalancesResponse> {
        let client = self.client();
        let url = format!(
            "{}/account/token-balance/{}",
            VybeClient::BASE_URL,
            address.address
        );
        let resp = client
            .get(url)
            .send()
            .await?
            .json::<TokenBalancesResponse>()
            .await?;

        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use crate::VybeClient;
    use vybe_api_types::account::token_balances::TokenBalancesRequest;

    #[tokio::test]
    async fn test_token_balances() {
        let client =
            VybeClient::new("8U8BxtAQVbrGzxWxaVdAkSPXtq4CPCExEbaBFQxwX4uPeByd".to_string())
                .unwrap();
        let resp = client
            .token_balances(TokenBalancesRequest {
                address: "AVAZvHLR2PcWpDf8BXY4rVxNHYRBytycHkcB5z5QNXYm".to_string(),
            })
            .await
            .unwrap();
        println!("{}", resp);
    }
}
