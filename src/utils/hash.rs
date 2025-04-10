use sha1::{Sha1, Digest};
use hex;

pub const HASH_LENGTH: usize = 40;
pub const FOLDER_LENGTH: usize = 2;

pub fn sha1(text: &str) -> String {
    hex::encode(Sha1::digest(text))
}