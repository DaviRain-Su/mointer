use crate::VybeClient;
use vybe_api_types::account::known_accounts::{KnownAccountsRequest, KnownAccountsResponse};

impl VybeClient {
    /// 检索带标签的 Solana 账户的分类列表，包括 CEX 钱包、项目金库、AMM 流动性池、做市商和影响者。
    pub async fn known_accounts(
        &self,
        request: Option<KnownAccountsRequest>,
    ) -> anyhow::Result<KnownAccountsResponse> {
        let client = self.client();
        let url = if let Some(request) = request {
            println!("{:?}", request.to_query());
            let url = format!(
                "{}/account/known-accounts?{}",
                Self::BASE_URL,
                request.to_query()
            );
            url
        } else {
            format!("{}/account/known-accounts", Self::BASE_URL)
        };
        let resp = client
            .get(url)
            .send()
            .await?
            .json::<KnownAccountsResponse>()
            .await?;

        Ok(resp)
    }
}

#[cfg(test)]
mod test {
    use crate::VybeClient;
    use vybe_api_types::account::known_accounts::{KnownAccountsRequest, Lable};

    #[tokio::test]
    async fn test_known_accounts() {
        let client =
            VybeClient::new("8U8BxtAQVbrGzxWxaVdAkSPXtq4CPCExEbaBFQxwX4uPeByd".to_string())
                .unwrap();
        let resp = client.known_accounts(None).await.unwrap();
        println!("{:#?}", resp);
    }

    #[tokio::test]
    async fn test_known_accounts_with_request() {
        let client =
            VybeClient::new("8U8BxtAQVbrGzxWxaVdAkSPXtq4CPCExEbaBFQxwX4uPeByd".to_string())
                .unwrap();
        let resp = client
            .known_accounts(Some(KnownAccountsRequest {
                labels: Some(vec![Lable::Dao]),
                ..Default::default()
            }))
            .await
            .unwrap();
        println!("{:#?}", resp);
    }
}
