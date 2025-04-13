use std::process;

use base64::{engine::general_purpose, Engine as _};

pub fn serialize(data: &[u8]) -> String {
    general_purpose::STANDARD.encode(data)
}

pub fn deserialize(text: &str) -> Vec<u8> {
    match general_purpose::STANDARD.decode(text) {
        Ok(decoded_bytes) => decoded_bytes,
        Err(e) => {
            eprintln!("Deserialize error when decoding base64: {}", e);
            process::exit(1)
        }
    }
}