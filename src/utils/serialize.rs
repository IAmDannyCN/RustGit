//! Module: serialize
//!
//! Provides utilities for base64 encoding and decoding of data.
//! These functions are used throughout the Git implementation for serializing and deserializing
//! object contents when interacting with storage.

use std::process;

use base64::{engine::general_purpose, Engine as _};


/// Serializes a byte slice into a base64-encoded string.
///
/// # Arguments
/// * `data` - Raw binary data to encode.
///
/// # Returns
/// * A `String` containing the base64 representation of the input data.
pub fn serialize(data: &[u8]) -> String {
    general_purpose::STANDARD.encode(data)
}


/// Deserializes a base64-encoded string back into raw bytes.
///
/// # Arguments
/// * `text` - Base64-encoded string to decode.
///
/// # Returns
/// * A `Vec<u8>` containing the decoded binary data.
///
/// # Exits
/// * If the input is not valid base64, prints an error and exits the process.
pub fn deserialize(text: &str) -> Vec<u8> {
    match general_purpose::STANDARD.decode(text) {
        Ok(decoded_bytes) => decoded_bytes,
        Err(e) => {
            eprintln!("Deserialize error when decoding base64: {}", e);
            process::exit(1)
        }
    }
}