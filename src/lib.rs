use hex::FromHexError;
use client_definitions::{hub_service_client::HubServiceClient, HubInfoRequest};
pub mod client_definitions {
    tonic::include_proto!("_");
}

pub use client_definitions::*;

pub fn bytes_to_hex_string<B: AsRef<[u8]>>(bytes: B) -> String {
    hex::encode(bytes.as_ref())
}

pub fn hex_string_to_bytes(string: String) -> Result<Vec<u8>, FromHexError> {
    hex::decode(string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_info() -> Result<(), Box<dyn std::error::Error>> {
        let mut client = HubServiceClient::connect("https://mainnet.useportals.app:2283").await?;

        let response = client.get_info(HubInfoRequest { db_stats: true }).await;

        assert!(response.is_ok(), "Failed to get info from the HubService");

        Ok(())
    }
}
