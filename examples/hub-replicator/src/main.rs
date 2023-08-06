use hub_rs::hub_service_client::HubServiceClient;
use hub_rs::HubInfoRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = HubServiceClient::connect("https://mainnet.useportals.app:2283").await?;

    let info = client.get_info(HubInfoRequest { db_stats: true }).await?;


    println!("{:?}", info.into_inner());

    Ok(())
}
