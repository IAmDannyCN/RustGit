//! Module: hash
//!
//! Provides hashing utilities used throughout the Git implementation.
//! Currently, it uses SHA-1 as the hashing algorithm to compute object identifiers,
//! consistent with Git's internal use of SHA-1 for commit, tree, and blob hashes.

use sha1::{Sha1, Digest};
use hex;

/// The length (in characters) of a hexadecimal-encoded SHA-1 hash string.
pub const HASH_LENGTH: usize = 40;
/// The length (in characters) used for the first level of folder sharding in the object store.
pub const FOLDER_LENGTH: usize = 2;


/// Computes the SHA-1 hash of the provided byte slice and returns the hexadecimal representation as a string.
///
/// # Arguments
/// * `text` - A byte slice containing the data to be hashed.
///
/// # Returns
/// * A `String` representing the SHA-1 hash in hexadecimal format (40 lowercase hex characters).
pub fn sha1(text: &[u8]) -> String {
    hex::encode(Sha1::digest(text))
}