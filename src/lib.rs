use hex::FromHexError;
use chrono::{Utc, TimeZone, LocalResult, Duration};
pub use client_definitions::*;

pub mod client_definitions {
    tonic::include_proto!("_");
}


/// Converts a sequence of bytes into its hexadecimal string representation.
///
/// # Parameters
/// - `bytes`: The bytes to be converted. Accepts anything that can be referenced as a byte slice
///   due to the `AsRef<[u8]>` bound.
///
/// # Returns
/// - A `String` containing the hexadecimal representation of the input bytes.
///
/// # Examples
/// ```
/// use hub_rs::bytes_to_hex_string;
///
/// assert_eq!(
///     bytes_to_hex_string(&[0xDE, 0xAD, 0xBE, 0xEF]),
///     "deadbeef"
/// );
/// ```
pub fn bytes_to_hex_string<B: AsRef<[u8]>>(bytes: B) -> String {
    hex::encode(bytes.as_ref())
}

/// Converts a hexadecimal string representation into a sequence of bytes.
///
/// # Parameters
/// - `string`: The hexadecimal string to be converted.
///
/// # Returns
/// - A `Result` that contains a `Vec<u8>` if the conversion is successful, or a `FromHexError` if
///   the conversion fails.
///
/// # Examples
/// ```
/// use hub_rs::hex_string_to_bytes;
///
/// assert_eq!(
///     hex_string_to_bytes("deadbeef".to_string()).unwrap(),
///     vec![0xDE, 0xAD, 0xBE, 0xEF]
/// );
/// ```
pub fn hex_string_to_bytes(string: String) -> Result<Vec<u8>, FromHexError> {
    hex::decode(string)
}

/// Returns the number of seconds elapsed since the "Farcaster Epoch" (2021-01-01 00:00:00 UTC)
/// to the current moment.
///
/// The "Farcaster Epoch" is arbitrarily defined here as the starting reference point.
///
/// # Returns
/// - An `Option<i64>` that contains the number of seconds if the date conversion is successful.
///   Returns `None` if there's an error determining the epoch or calculating the duration.
///
/// # Examples
/// ```
/// use hub_rs::get_farcaster_time;
///
/// let seconds_since_epoch = get_farcaster_time();
/// assert!(seconds_since_epoch.is_some());
/// ```
pub fn get_farcaster_time() -> Option<i64> {
    match Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0) {
        LocalResult::Single(farcaster_epoch) => {
            Some(Utc::now().signed_duration_since(farcaster_epoch).num_seconds())
        }
        _ => None,
    }
}


/// Converts from a Unix timestamp to a Farcaster timestamp.
///
/// # Arguments
///
/// * `time` - A i64 value specifying unix milliseconds
///
/// # Returns
///
/// * `Option<i64>` - An optional i64 value for seconds since the Farcaster Epoch. If the given Unix timestamp
///    cannot be converted into a valid DateTime object, the function will return None.
pub fn to_farcaster_time(time: i64) -> Option<i64> {
    let farcaster_epoch = Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0);

    match farcaster_epoch {
        LocalResult::Single(farcaster_epoch) => {
            let unix_timestamp = Utc.timestamp_opt(time / 1000, ((time % 1000) as u32) * 1_000_000);

            match unix_timestamp {
                LocalResult::Single(unix_timestamp) => {
                    Some(unix_timestamp.signed_duration_since(farcaster_epoch).num_seconds())
                },
                _ => None
            }
        },
        _ => None
    }
}

/// Converts from a Farcaster timestamp to a Unix timestamp.
///
/// # Arguments
///
/// * `time` - A i64 value specifying seconds since the Farcaster Epoch
///
/// # Returns
///
/// * `Option<i64>` - An optional i64 value for Unix timestamp in milliseconds. If the Farcaster epoch
///    cannot be converted into a valid DateTime object, the function will return None.
pub fn from_farcaster_time(time: i64) -> Option<i64> {
    let farcaster_epoch = Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0);

    match farcaster_epoch {
        LocalResult::Single(farcaster_epoch) => {
            let farcaster_timestamp = farcaster_epoch + Duration::seconds(time);
            Some(farcaster_timestamp.timestamp() * 1000 + (farcaster_timestamp.timestamp_subsec_millis() as i64))
        },
        _ => None
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_info() -> Result<(), Box<dyn std::error::Error>> {
        let mut client = hub_service_client::HubServiceClient::connect("https://mainnet.useportals.app:2283").await?;

        let response = client.get_info(HubInfoRequest { db_stats: true }).await;

        assert!(response.is_ok(), "Failed to get info from the HubService");

        Ok(())
    }

    #[tokio::test]
    async fn test_farcaster_time() -> Result<(), Box<dyn std::error::Error>> {
        let time = get_farcaster_time();
        let to_unix_time = from_farcaster_time(time.unwrap());
        let from_unix_time = to_farcaster_time(to_unix_time.unwrap());

        println!("{}", time.unwrap());
        println!("{}", to_unix_time.unwrap());
        println!("{}", from_unix_time.unwrap());

        assert!(time.is_some(), "Failed to get farcaster time");
        assert!(to_unix_time.is_some(), "Failed to convert the current time into a unix time");
        assert!(from_unix_time.is_some(), "Failed to convert the Unix time into a Farcaster time");
        Ok(())
    }
}
