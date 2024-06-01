//! 获取所提供账户地址的 NFT 余额。该端点整合 NFT 持有量并提供美元和 SOL 的投资组合价值，从而促进全面了解账户的 NFT 资产。
use crate::VybeClient;
use vybe_api_types::account::nft_balances::{NftBalancesRequest, NftBalancesResponse};

impl VybeClient {
    pub async fn nft_balances(
        &self,
        address: NftBalancesRequest,
    ) -> anyhow::Result<NftBalancesResponse> {
        let client = self.client();
        let url = format!(
            "{}/account/nft-balance/{}",
            VybeClient::BASE_URL,
            address.address
        );
        let resp = client
            .get(url)
            .send()
            .await?
            .json::<NftBalancesResponse>()
            .await?;

        Ok(resp)
    }
}

#[cfg(test)]
mod test {
    use crate::VybeClient;
    use vybe_api_types::account::nft_balances::NftBalancesRequest;

    #[tokio::test]
    async fn test_nft_balances() {
        let client =
            VybeClient::new("8U8BxtAQVbrGzxWxaVdAkSPXtq4CPCExEbaBFQxwX4uPeByd".to_string())
                .unwrap();
        let resp = client
            .nft_balances(NftBalancesRequest {
                address: "sb3bUnVTEcKusCEu8nxVnUZu1NPyCx5xP9pxV8RNa2y".to_string(),
            })
            .await
            .unwrap();
        println!("{:#?}", resp);
    }
}
