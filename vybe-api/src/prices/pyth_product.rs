use crate::VybeClient;
use vybe_api_types::prices::pyth_product::{PythProductRequest, PythProductResponse};

impl VybeClient {
    pub async fn pyth_product(
        &self,
        request: &PythProductRequest,
    ) -> anyhow::Result<PythProductResponse> {
        let client = self.client();
        let url = format!(
            "{}/price/{}/pyth-product",
            Self::BASE_URL,
            request.product_id
        );
        let resp = client
            .get(url)
            .send()
            .await?
            .json::<PythProductResponse>()
            .await?;
        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use crate::VybeClient;
    use vybe_api_types::prices::pyth_product::PythProductRequest;

    #[tokio::test]
    async fn test_pyth_product() {
        let client =
            VybeClient::new("8U8BxtAQVbrGzxWxaVdAkSPXtq4CPCExEbaBFQxwX4uPeByd".to_string())
                .unwrap();
        let resp = client
            .pyth_product(&PythProductRequest {
                product_id: "5t8evHxBNRbBkC8Q2jGCs3K435n25FjJMJQ1oWCnhrWW".to_string(),
            })
            .await
            .unwrap();
        println!("{:#?}", resp);
    }
}
