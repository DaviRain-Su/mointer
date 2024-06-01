use crate::VybeClient;
use vybe_api_types::tokens::token::{TokenRequest, TokenResponse};

impl VybeClient {
    pub async fn tokens(&self, request: Option<TokenRequest>) -> anyhow::Result<TokenResponse> {
        let client = self.client();
        let url = if let Some(request) = request {
            println!("{:?}", request.to_query());
            let url = format!("{}/tokens?{}", Self::BASE_URL, request.to_query());
            url
        } else {
            format!("{}/tokens", Self::BASE_URL)
        };
        let resp = client
            .get(url)
            .send()
            .await?
            .json::<TokenResponse>()
            .await?;

        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use crate::VybeClient;
    use vybe_api_types::tokens::token::SimpleToken;
    #[tokio::test]
    async fn test_tokens() {
        let client =
            VybeClient::new("8U8BxtAQVbrGzxWxaVdAkSPXtq4CPCExEbaBFQxwX4uPeByd".to_string())
                .unwrap();
        let resp = client.tokens(None).await.unwrap();
        let tokens = resp
            .data
            .into_iter()
            .map(SimpleToken::from)
            .collect::<Vec<SimpleToken>>();

        println!("{:#?}", tokens);
    }
}
