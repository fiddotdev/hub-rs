use hub_rs::{from_farcaster_time, get_farcaster_time, to_farcaster_time};
use hub_rs::hub_service_client::HubServiceClient;
use hub_rs::HubInfoRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let to_fc_time = to_farcaster_time(1691407877000);

    let from_fc_time = from_farcaster_time(81948677);

    println!("{:#?}", to_fc_time.unwrap());

    println!("{:#?}", from_fc_time.unwrap());

    Ok(())
}
