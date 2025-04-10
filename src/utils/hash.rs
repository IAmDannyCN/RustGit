use sha1::{Sha1, Digest};
use hex;

pub fn sha1(text: &str) -> String {
    hex::encode(Sha1::digest(text))
}