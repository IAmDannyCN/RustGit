use std::process;

use crate::utils::{hash, serialize};
use super::object::*;

pub enum TreeEntryType {
    Blob,
    Tree,
}

pub struct TreeEntry {
    pub entry_type: TreeEntryType,
    pub name: String,
    pub hash: String,
}

pub struct Tree {
    pub hash: Option<String>,
    pub data: Option<Vec<TreeEntry>>,
}

pub trait TreeTrait {
    fn read_tree(&mut self);
    fn write_tree(&mut self);
}

impl TreeTrait for Tree {
    
    /// `read_tree`: read file and update `self.data` based on `self.hash`
    fn read_tree(&mut self) {

        assert!(self.hash.is_none() == false);
        assert!(self.data.is_none() == true);

        let hash = self.hash.as_ref().unwrap();
        
        let raw_content = read_object_file(hash);
        let full_content = serialize::deserialize(&raw_content);
        assert!(full_content.len() >= 4);
        assert!(full_content[..4] == "TREE"[..]);

        let mut entries = Vec::new();

        for line in full_content[4..].lines() {
            let parts: Vec<&str> = line.split('\0').collect();
            assert!(parts.len() == 3);

            let kind = parts[0];
            let name = parts[1].to_string();
            let hash = parts[2].to_string();

            match kind {
                "BLOB" => entries.push(TreeEntry { entry_type: TreeEntryType::Blob, name, hash }),
                "TREE" => entries.push(TreeEntry { entry_type: TreeEntryType::Tree, name, hash }),
                _ => {
                    eprintln!("read_tree: invalid entry_type: {}", kind);
                    process::exit(1);
                }
            }
        }

        self.data = Some(entries);
    }

    /// `write_tree`: calculate `self.hash` and write file based on `self.data`
    fn write_tree(&mut self) {

        assert!(self.hash.is_none() == true);
        assert!(self.data.is_none() == false);

        let entries = self.data.as_ref().unwrap();
        let mut data: String = Default::default();
        for entry in entries {
            data.push_str(&format!("{}\0{}\0{}\n",
                        match entry.entry_type {
                            TreeEntryType::Blob => "BLOB",
                            TreeEntryType::Tree => "TREE",
                        },
                        entry.name,
                        entry.hash));
        }

        let full_content = "BLOB".to_string() + &data;
        let hash = hash::sha1(&full_content);
        let raw_content = serialize::serialize(&full_content);

        write_object_file(&hash, &raw_content);
        self.hash = Some(hash);
    }
}