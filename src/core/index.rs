use std::{collections::HashMap, process};

use crate::utils::*;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct IndexEntry {
    pub path: String,
    pub hash: String,
}

pub type Index = HashMap<String, IndexEntry>;

/// read and parse .git/index
pub fn read_index() -> Index {
    
    let index_path = utils::get_git_directory() + "/index";
    let raw_content = match storage::read_text_file(&index_path) {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Failed to read index: {}", e);
            process::exit(1)
        }
    };

    let mut entries: Index = Default::default();
    let vecu8_content = serialize::deserialize(&raw_content);
    let full_content = std::str::from_utf8(&vecu8_content).expect("Invalid UTF-8");

    if full_content.len() == 0 {
        return entries;
    }

    assert!(full_content.len() >= 4);
    assert!(full_content[..4] == "DIRC"[..]);

    for line in full_content[4..].lines() {
        let parts: Vec<&str> = line.split('\0').collect();
        assert!(parts.len() == 2);

        let path = parts[0].to_string();
        let hash = parts[1].to_string();

        entries.insert(path.clone(), IndexEntry { path, hash });
    }

    entries
}

pub fn write_index(index: &Index) {

    let index_path = utils::get_git_directory() + "/index";
    let mut data: String = Default::default();

    data.push_str("DIRC");

    for entry in index {
        data.push_str(&format!("{}\0{}\n", entry.1.path, entry.1.hash));
    }

    let raw_content = serialize::serialize(&data.as_bytes());

    if let Err(e) = storage::write_text_file(&index_path, &raw_content) {
        eprintln!("Failed to write index: {}", e);
        process::exit(1)
    }
}