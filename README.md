## ⚠️🚧 hub-rs is currently in active development and should NOT be assumed production-ready 🚧⚠️

# hub-rs

hub-rs is a barebones Rust library for interacting with [Farcaster](https://farcaster.xyz) hubs

All definitions and functions are auto-generated with [tonic](https://github.com/hyperium/tonic)

Protobufs are provided from the [Farcaster monorepo](https://github.com/farcasterxyz/hub-monorepo)

## How to use
First, install the crate:

`hub-rs = { version = "0.1.3" }`  OR `cargo add hub-rs`

Then, you can use it in your project. To create a client, and get info about the connected hub, you might write something like this:
```rust
use hub_rs::hub_service_client::HubServiceClient;
use hub_rs::HubInfoRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Secure or insecure connections are defined by the protocol you provide, either http or https
    let mut client = HubServiceClient::connect("https://mainnet.useportals.app:2283").await?;

    let info = client.get_info(HubInfoRequest { db_stats: true }).await?;


    println!("{:?}", info.into_inner());

    Ok(())
}
```
## Write Operations
Currently, writing to Hubs is not supported here. I have a messy start on the [landon-write-operations](https://github.com/withportals/hub-rs/tree/landon-write-operations) branch if you'd like to take a look / build on it. (Under src/builders/make_cast_add.rs)

## Additional Functions Provided
Currently, two extra functions are provided for QOL:
- `bytes_to_hex_string` - Converts `Vec<u8>` or `[u8; 32/64]` types to a hex string
- `hex_string_to_bytes` - Converts a hex string into `Vec<u8>`

## FAQ

Q:  Why?

A: 🦀🦀🦀 🚀🚀🚀

Q: Is this faster than hub-nodejs?

A: Theoretically, yes, just due to the nature of Rust. However, the difference is probably minimal, since its majorly just requests, real bottleneck is I/O & internet speed. 

If you're interested in doing some benchmarking, though, either open a PR or contact me on telegram: @landon_xyz


