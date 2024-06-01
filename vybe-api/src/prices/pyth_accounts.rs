use crate::VybeClient;
use vybe_api_types::prices::pyth_accounts::{PythAccountsRequest, PythAccountsResponse};

impl VybeClient {
    pub async fn pyth_accounts(
        &self,
        request: &PythAccountsRequest,
    ) -> anyhow::Result<PythAccountsResponse> {
        let client = self.client();
        let url = format!(
            "{}/price/pyth-accounts?{}",
            Self::BASE_URL,
            request.to_query()
        );
        let resp = client
            .get(url)
            .send()
            .await?
            .json::<PythAccountsResponse>()
            .await?;
        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use crate::VybeClient;
    use vybe_api_types::prices::pyth_accounts::PythAccountsRequest;

    #[tokio::test]
    async fn test_pyth_accounts() {
        let client =
            VybeClient::new("8U8BxtAQVbrGzxWxaVdAkSPXtq4CPCExEbaBFQxwX4uPeByd".to_string())
                .unwrap();
        let resp = client
            .pyth_accounts(&PythAccountsRequest {
                product_id: Some("5t8evHxBNRbBkC8Q2jGCs3K435n25FjJMJQ1oWCnhrWW".to_string()),
                ..Default::default()
            })
            .await
            .unwrap();
        println!("{:#?}", resp);
    }
}
