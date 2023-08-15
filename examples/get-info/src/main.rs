use hub_rs::hub_service_client::HubServiceClient;
use hub_rs::{CastAddBody, FarcasterNetwork, HubInfoRequest};
use hub_rs::{from_farcaster_time, get_farcaster_time, to_farcaster_time};
use hub_rs::builders::make_cast_add::{make_cast_add, MessageDataOptions};
use rand::rngs::OsRng;

// Aug 15 TODO:
// Figure out signer stuff and like the wallets and stuff ??
// https://github.com/farcasterxyz/hub-monorepo/blob/main/packages/hub-nodejs/examples/make-cast/index.ts
// TODO: this ^^

// How this needs to work:
// I need to be able to construct a new ethers wallet w/ my mnemonic
// Then I need to construct an EIP712 signer from said wallet
// Then I need to be able to construct a new signer message from that signer
// Then I need to be able to publish that message
// Then I need to be able to create a new cast_add
// Then I need to be able to create submit that message and publish it

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = HubServiceClient::connect("https://mainnet.useportals.app:2283").await?;

    let mut csprng = OsRng;
    let signing_key = ed25519_dalek::SigningKey::generate(&mut csprng);
    let curr_ts = get_farcaster_time().unwrap();
    let test = make_cast_add(CastAddBody {
        embeds_deprecated: vec![],
        mentions: vec![],
        text: "hello from hub-rs!".to_string(),
        mentions_positions: vec![],
        embeds: vec![],
        parent: None,
    }, MessageDataOptions {
        fid: 1117,
        network: FarcasterNetwork::Mainnet,
        timestamp: Some(curr_ts as u32),
    }, signing_key).unwrap();

    let submit = client.submit_message(test).await?;

    println!("{:#?}", submit);
    Ok(())
}
