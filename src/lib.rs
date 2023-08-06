use hex::FromHexError;

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
