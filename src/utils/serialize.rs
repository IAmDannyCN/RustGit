use std::process;

use base64::{engine::general_purpose, Engine as _};

pub fn serialize(text: &str) -> String {
    general_purpose::STANDARD.encode(text)
}

pub fn deserialize(text: &str) -> String {
    match general_purpose::STANDARD.decode(text) {
        Ok(decoded_bytes) => {
            match String::from_utf8(decoded_bytes) {
                Ok(res) => res,
                Err(e) => {
                    eprintln!("Deserialize error when converting Vec<u8> to String: {}", e);
                    process::exit(1)
                }
            }
        }
        Err(e) => {
            eprintln!("Deserialize error when decoding base64: {}", e);
            process::exit(1)
        }
    }
}