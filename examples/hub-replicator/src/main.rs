use std::sync::Arc;
use types::EnvConfig;
use sqlx::postgres::PgPoolOptions;
use tokio::sync::Semaphore;
use hub_rs::hub_service_client::HubServiceClient;
use tonic::transport::Channel;
use hub_rs::FidsRequest;
use tokio::time::{Duration, Instant, sleep};
pub mod types;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let env = match envy::from_env::<EnvConfig>() {
        Ok(c) => c,
        Err(err) => panic!("{:#?}", err)
    };


    let hub = HubServiceClient::connect("https://mainnet.useportals.app:2283").await?;
    let db = PgPoolOptions::new().max_connections(5).connect(&env.database_url).await?;

    // Backfill
    let _backfill_result = backfill(hub).await?;
    // Get new events

    println!("Hello, world!");

    Ok(())
}

async fn backfill(mut hub_client: HubServiceClient<Channel>) -> Result<(), Box<dyn std::error::Error>> {
    let max_fid_result = hub_client.get_fids(FidsRequest { page_size: Some(1), page_token: None, reverse: Some(true) }).await?;
    let max_fid = max_fid_result.into_inner().fids[0];
    let total_processed = Arc::new(tokio::sync::Mutex::new(0));
    let start_time = Instant::now();

    let semaphore = Arc::new(Semaphore::new(5));

    let mut handles = Vec::new();

    for fid in 1..=max_fid {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let total_processed = total_processed.clone();
        let handle = tokio::spawn(async move {
            // process all messages for the current fid
            // process_all_messages_for_fid(fid).await;

            let mut total_processed = total_processed.lock().await;
            *total_processed += 1;

            let elapsed = start_time.elapsed();
            let millis_remaining = (elapsed.as_millis() / *total_processed as u128) * (max_fid as u128 - *total_processed as u128);
            let remaining_duration = Duration::from_millis(millis_remaining as u64);

            println!("[Backfill] Completed FID {}/{}. Estimated time remaining: {:?}", fid, max_fid, remaining_duration);

            sleep(Duration::from_secs(20)).await;
        });
        handles.push(handle);
    }

    // wait for all tasks to complete
    for handle in handles {
        handle.await?;
    }

    println!("[Backfill] Completed in {:?}", start_time.elapsed());

    Ok(())
}
