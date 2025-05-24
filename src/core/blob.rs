//! Module: blob
//!
//! This module defines the `Blob` structure and operations for interacting with
//! file blobs in a Git-like object storage system. Blobs represent file content,
//! and can be regular files or symbolic links.

use std::{fs, process};
use std::os::unix::fs::MetadataExt;

use crate::utils::{hash, serialize, storage};
use super::{object::*, tree::TreeEntryType};

/// Represents a Git blob object, which stores file content.
///
/// - `hash`: Optional SHA-1 hash of the blob content.
/// - `data`: Optional byte content of the blob.
pub struct Blob {
    pub hash: Option<String>,
    pub data: Option<Vec<u8>>,
}

pub trait BlobTrait {
    fn read_blob(&mut self);
    fn write_blob(&mut self);
    fn calculate_hash(&mut self);
}


impl BlobTrait for Blob {

    /// Reads the blob object from storage and populates `self.data`.
    ///
    /// Requires that `self.hash` is set and `self.data` is empty.
    fn read_blob(&mut self) {

        assert!(self.hash.is_none() == false);
        assert!(self.data.is_none() == true);

        let hash = self.hash.as_ref().unwrap();
        
        let raw_content = read_object_file(hash);
        let full_content = serialize::deserialize(&raw_content);

        assert!(full_content.len() >= 4);
        assert!(full_content.starts_with(b"BLOB"));
        
        self.data = Some(full_content[4..].to_vec());
    }


    /// Serializes and writes the blob to storage.
    ///
    /// Calculates the hash if not already present. Prepends "BLOB" as a type header.
    fn write_blob(&mut self) {

        assert!(self.data.is_none() == false);

        if self.hash.is_none() {
            self.calculate_hash();
        }

        let data = self.data.as_ref().unwrap();
        let mut full_content = b"BLOB".to_vec();
        full_content.extend(data);
        let raw_content = serialize::serialize(&full_content);

        write_object_file(self.hash.as_ref().unwrap(), &raw_content);
    }

    
    /// Computes the SHA-1 hash for the blob's content.
    fn calculate_hash(&mut self) {
        assert!(self.data.is_none() == false);
        self.hash = Some(hash::sha1(self.data.as_ref().unwrap()));
    }
}


/// Constructs a `Blob` object from a file path.
///
/// This function reads the file's content (or symlink target if it's a symlink),
/// computes its SHA-1 hash, and returns a populated `Blob`.
///
/// # Arguments
///
/// * `file_path` - The path to the file or symlink.
///
/// # Panics / Exits
///
/// This function prints an error and exits if the file cannot be read or stat-ed.
pub fn get_blob_from_file(file_path: &str) -> Blob {
    let meta = fs::symlink_metadata(file_path).unwrap_or_else(|e| {
        eprintln!("Error when stat file {} : {}", file_path, e);
        process::exit(1)
    });

    let blob_data = if meta.file_type().is_symlink() {
        // This is a SymLink
        match fs::read_link(file_path) {
            Ok(target_path) => target_path.to_string_lossy().to_string().into_bytes(),
            Err(e) => {
                eprintln!("Error when reading symlink {} : {}", file_path, e);
                process::exit(1)
            }
        }
    } else {
        // This is an ordinary file
        match storage::read_file(file_path) {
            Ok(res) => res,
            Err(e) => {
                eprintln!("Error when reading blob file {} : {}", file_path, e);
                process::exit(1)
            }
        }
    };

    Blob {
        hash: Some(hash::sha1(&blob_data)),
        data: Some(blob_data),
    }
}


/// Determines the type of the blob based on the file system metadata.
///
/// Returns one of the following `TreeEntryType` values:
/// - `Bsym` for symlinks
/// - `Bexe` for executable files
/// - `Blob` for regular files
///
/// # Arguments
///
/// * `blob_path` - The path to the file or symlink.
///
/// # Panics / Exits
///
/// Prints an error and exits the process for unsupported file types or on I/O failure.
pub fn get_blob_type(blob_path: &str) -> TreeEntryType {
    let meta = fs::symlink_metadata(blob_path).unwrap_or_else(|e| {
        eprintln!("Failed to stat {}: {}", blob_path, e);
        std::process::exit(1);
    });

    let file_type = meta.file_type();

    if file_type.is_symlink() {
        TreeEntryType::Bsym
    } else if file_type.is_file() {
        let mode = meta.mode();
        if (mode & 0o111) != 0 {
            TreeEntryType::Bexe
        } else {
            TreeEntryType::Blob
        }
    } else {
        eprintln!("Unsupported file type: {}", blob_path);
        std::process::exit(1);
    }
}