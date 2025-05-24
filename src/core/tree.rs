//! Module: tree
//!
//! Provides structures and methods for representing and manipulating Git tree objects.
//! Tree objects in Git are used to represent directory hierarchies, linking file names
//! to blob hashes or other trees (subdirectories), with support for symbolic links and executable bits.

use std::process;

use crate::utils::{hash, serialize};
use super::object::*;

/// Represents the type of a tree entry in Git.
#[derive(PartialEq, Eq, Hash, Clone)]
pub enum TreeEntryType {
    Blob,
    Tree,
    Bsym,
    Bexe,
}

/// Represents a single entry within a Git tree object.
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct TreeEntry {
    pub entry_type: TreeEntryType,
    pub name: String,
    pub hash: String,
}

/// Represents a Git tree object.
///
/// A tree may either be loaded from disk (`read_tree`) or built in memory and written out (`write_tree`).
pub struct Tree {
    pub hash: Option<String>,
    pub data: Option<Vec<TreeEntry>>,
}

pub trait TreeTrait {
    fn read_tree(&mut self);
    fn write_tree(&mut self);
    fn calculate_hash(&mut self);
}

impl TreeTrait for Tree {
    
    /// Reads the tree content from the Git object store into memory.
    ///
    /// # Panics
    /// * If `self.hash` is `None` (i.e., no hash is set for reading).
    /// * If `self.data` is already `Some` (i.e., data is already present).
    /// * If the content does not start with `"TREE"` magic string.
    /// * If any line is malformed and doesn't contain three null-separated fields.
    ///
    /// # Exits
    /// * If an unknown entry type is encountered, prints an error and exits.
    fn read_tree(&mut self) {

        assert!(self.hash.is_none() == false);
        assert!(self.data.is_none() == true);

        let hash = self.hash.as_ref().unwrap();

        if hash == "" {
            self.data = Some(Default::default());
            return ;
        }
        
        let raw_content = read_object_file(hash);
        let vecu8_content = serialize::deserialize(&raw_content);
        let full_content = std::str::from_utf8(&vecu8_content).expect("Invalid UTF-8");

        assert!(full_content.len() >= 4);
        assert!(full_content.starts_with("TREE"));

        let mut entries = Vec::new();

        for line in full_content[4..].lines() {
            let parts: Vec<&str> = line.split('\0').collect();
            assert!(parts.len() == 3);

            let kind = parts[0];
            let name = parts[1].to_string();
            let hash = parts[2].to_string();

            match kind {
                "BLOB" => entries.push(TreeEntry { entry_type: TreeEntryType::Blob, name, hash }),
                "BSYM" => entries.push(TreeEntry { entry_type: TreeEntryType::Bsym, name, hash }),
                "BEXE" => entries.push(TreeEntry { entry_type: TreeEntryType::Bexe, name, hash }),
                "TREE" => entries.push(TreeEntry { entry_type: TreeEntryType::Tree, name, hash }),
                _ => {
                    eprintln!("read_tree: invalid entry_type: {}", kind);
                    process::exit(1);
                }
            }
        }

        self.data = Some(entries);
    }


    /// Serializes and writes the current tree's data to the Git object store.
    ///
    /// # Panics
    /// * If `self.data` is `None` (i.e., data must exist before writing).
    ///
    /// # Notes
    /// If `self.hash` is `None`, it will be calculated first via `calculate_hash`.
    fn write_tree(&mut self) {

        assert!(self.data.is_none() == false);

        if self.hash.is_none() {
            self.calculate_hash();
        }

        let entries = self.data.as_ref().unwrap();
        let mut data: String = Default::default();
        for entry in entries {
            data.push_str(&format!("{}\0{}\0{}\n",
                        match entry.entry_type {
                            TreeEntryType::Blob => "BLOB",
                            TreeEntryType::Bsym => "BSYM",
                            TreeEntryType::Bexe => "BEXE",
                            TreeEntryType::Tree => "TREE",
                        },
                        entry.name,
                        entry.hash));
        }

        let full_content = "TREE".to_string() + &data;
        let raw_content = serialize::serialize(&full_content.as_bytes());

        write_object_file(self.hash.as_ref().unwrap(), &raw_content);
    }


    /// Computes and sets the SHA-1 hash of the tree based on its contents.
    ///
    /// # Panics
    /// * If `self.data` is `None` (i.e., data must exist before hashing).
    fn calculate_hash(&mut self) {
        assert!(self.data.is_none() == false);
        let entries = self.data.as_ref().unwrap();
        let mut data: String = Default::default();
        for entry in entries {
            data.push_str(&format!("{}\0{}\0{}\n",
                        match entry.entry_type {
                            TreeEntryType::Blob => "BLOB",
                            TreeEntryType::Bsym => "BSYM",
                            TreeEntryType::Bexe => "BEXE",
                            TreeEntryType::Tree => "TREE",
                        },
                        entry.name,
                        entry.hash));
        }
        
        self.hash = Some(hash::sha1(&data.as_bytes()));
    }
}