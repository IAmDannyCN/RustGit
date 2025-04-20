use sha1::{Sha1, Digest};
use hex;

pub const HASH_LENGTH: usize = 40;
pub const FOLDER_LENGTH: usize = 2;

/// Computes the SHA-1 hash of the given byte slice and returns it as a hexadecimal string.
pub fn sha1(text: &[u8]) -> String {
    hex::encode(Sha1::digest(text))
}