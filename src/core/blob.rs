use crate::utils::serialize;
use super::object::*;

pub struct Blob {
    pub name: String,
    pub data: String
}

pub trait BlobTrait {
    fn read_blob(&mut self);
    fn write_blob(&self);
}

impl BlobTrait for Blob {
    fn read_blob(&mut self) {
        let raw_content = read_object_file(&self.name);
        let full_content = serialize::deserialize(&raw_content);
        assert!(full_content.len() >= 4);
        assert!(full_content[..4] == "BLOB"[..]);
        self.data = full_content[4..].to_string();
    }
    fn write_blob(&self) {
        let full_content = "BLOB".to_string() + &self.data.to_string();
        let raw_content = serialize::serialize(&full_content);
        write_object_file(&self.name, &raw_content);
    }
}