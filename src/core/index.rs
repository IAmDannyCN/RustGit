use std::process;

use crate::utils::*;

pub struct IndexEntry {
    pub path: String,
    pub hash: String,
}

type Index = Vec<IndexEntry>;

/// read and parse .git/index
pub fn read_index() -> Index {
    
    let index_path = utils::get_git_directory() + "/index";
    let raw_content = match storage::read_file(&index_path) {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Failed to read index: {}", e);
            process::exit(1)
        }
    };

    let full_content = serialize::deserialize(&raw_content);
    assert!(full_content.len() >= 4);
    assert!(full_content[..4] == "DIRC"[..]);

    let mut entries: Index = Default::default();

    for line in full_content[4..].lines() {
        let parts: Vec<&str> = line.split('\0').collect();
        assert!(parts.len() == 2);

        let path = parts[0].to_string();
        let hash = parts[1].to_string();

        entries.push(IndexEntry { path, hash });
    }

    entries
}

pub fn write_index(index: &Index) {

    let index_path = utils::get_git_directory() + "/index";
    let mut data: String = Default::default();

    data.push_str("DIRC");

    for entry in index {
        data.push_str(&format!("{}\0{}\n", entry.path, entry.hash));
    }

    let raw_content = serialize::serialize(&data);

    if let Err(e) = storage::write_file(&index_path, &raw_content) {
        eprintln!("Failed to write index: {}", e);
        process::exit(1)
    }
}