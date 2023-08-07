use std::sync::Arc;
use types::EnvConfig;
use sqlx::postgres::PgPoolOptions;
use tokio::sync::Semaphore;
use hub_rs::hub_service_client::HubServiceClient;
use tonic::transport::Channel;
use hub_rs::{FidRequest, FidsRequest};
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
    let total_success = Arc::new(tokio::sync::Mutex::new(0));
    let total_error = Arc::new(tokio::sync::Mutex::new(0));
    let start_time = Instant::now();

    let sem = Arc::new(Semaphore::new(3));
    let mut handles = Vec::new();

    for fid in 1..=max_fid {
        let sem = Arc::clone(&sem);
        let total_processed = total_processed.clone();
        let total_success = total_success.clone();
        let total_error = total_error.clone();

        let mut hub_client = hub_client.clone();
        let handle = tokio::spawn(async move {
            let _permit = sem.acquire().await;

            // process all messages for the current fid
            let last_cast = hub_client.get_casts_by_fid(FidRequest {
                fid,
                page_size: Some(1),
                page_token: None,
                reverse: Some(true),
            }).await;

            let mut total_success = total_success.lock().await;
            let mut total_error = total_error.lock().await;

            match last_cast {
                Ok(_) => *total_success += 1,
                Err(err) => {
                    *total_error += 1;
                    println!("{:?}", fid);
                    println!("{:#?}", err);
                }
            }
            let mut total_processed = total_processed.lock().await;
            *total_processed += 1;

            let elapsed = start_time.elapsed();
            let millis_remaining = (elapsed.as_millis() / *total_processed as u128) * (max_fid as u128 - *total_processed as u128);
            let remaining_duration = Duration::from_millis(millis_remaining as u64);

            println!("[Backfill] Completed FID {}/{}. Estimated time remaining: {:?}", fid, max_fid, remaining_duration);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await?;
    }

    println!("[Backfill] Completed in {:?}", start_time.elapsed());
    println!("[Backfill] Total # Error'd: {:?}", total_error);
    Ok(())
}

