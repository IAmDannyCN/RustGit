use crate::utils::{hash, serialize};
use super::object::*;

pub struct Blob {
    pub hash: Option<String>,
    pub data: Option<String>,
}

pub trait BlobTrait {
    fn read_blob(&mut self);
    fn write_blob(&mut self);
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
        assert!(full_content[..4] == "BLOB"[..]);
        
        self.data = Some(full_content[4..].to_string());
    }

    /// `write_blob`: calculate `self.hash` and write file based on `self.data`
    fn write_blob(&mut self) {

        assert!(self.hash.is_none() == true);
        assert!(self.data.is_none() == false);

        let data = self.data.as_ref().unwrap();

        let full_content = "BLOB".to_string() + data;
        let hash = hash::sha1(&full_content);
        let raw_content = serialize::serialize(&full_content);

        write_object_file(&hash, &raw_content);
        self.hash = Some(hash);
    }
}