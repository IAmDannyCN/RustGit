use std::process;

use crate::utils::{hash, serialize, storage};
use super::object::*;

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

    /// `read_blob`: read file and update `self.data` based on `self.hash`
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

    /// `write_blob`: calculate `self.hash` if necessary, then write file based on `self.data`
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

    /// calculate the hash for `blob.data`
    fn calculate_hash(&mut self) {
        assert!(self.data.is_none() == false);
        self.hash = Some(hash::sha1(self.data.as_ref().unwrap()));
    }
}

pub fn get_blob_from_file(file_path: &str) -> Blob {

    let blob_data = match storage::read_file(file_path) {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Error when reading blob file {} : {}", file_path, e);
            process::exit(1)
        }
    };

    Blob { hash: Some(hash::sha1(&blob_data)), data: Some(blob_data)}
}