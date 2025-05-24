//! Module: index
//!
//! Provides structures and functions for managing the Git index (staging area),
//! including reading and writing the `.git/index` file and recursively registering
//! files and directories into the index.

use std::{collections::{HashMap, HashSet}, fs, path::Path, process};

use crate::utils::*;

/// Represents a file entry in the staging index.
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct IndexEntry {
    /// Relative file path
    pub path: String,
    /// Hash (blob ID) of the file content
    pub hash: String,
}

/// Type alias for the index (staging area), mapping file path → `IndexEntry`
pub type Index = HashMap<String, IndexEntry>;


/// Reads and parses the `.git/index` file into an in-memory `Index`.
///
/// # Returns
/// * `Index` - A map of file paths to their corresponding index entries.
///
/// # Panics
/// * If the index content is invalid UTF-8.
/// * If index format is not prefixed with `"DIRC"` magic string.
/// * If any line is malformed and doesn't contain two null-separated fields.
///
/// # Exits
/// * If the index file cannot be read, prints an error and exits the process.
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


/// Serializes and writes the given `Index` into `.git/index`.
///
/// # Arguments
/// * `index` - The staging index to write.
///
/// # Format
/// Each line after the `"DIRC"` magic string consists of:
/// `path\0hash\n`
///
/// # Exits
/// * If the write operation fails, prints an error and exits the process.
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


/// Recursively (if enabled) registers files into the index set from a given path.
///
/// This function is typically used for preparing a list of files to be added,
/// including symlinks and regular files. Directories are handled recursively if specified.
///
/// # Arguments
/// * `path` - Absolute path of the file or directory to process.
/// * `rel_path` - Relative path for storing in the index.
/// * `index` - A mutable set collecting index entries (`IndexEntry`) found under the given path.
/// * `recursive` - If `true`, subdirectories will be traversed recursively.
///
/// # Exits
/// * If a directory is encountered but `recursive` is `false`, prints an error and exits.
pub fn register_files(path: &str, rel_path: &str, index: &mut HashSet<IndexEntry>, recursive: &bool) {

    if &path == &utils::get_git_directory() {
        return ;
    }

    let path = Path::new(path);

    if let Ok(metadata) = fs::symlink_metadata(path) {
        if metadata.file_type().is_symlink() {
            index.insert(
                IndexEntry {
                path: rel_path.to_string(),
                hash: String::default(),
                }
            );
        } else if metadata.is_file() {
            index.insert(
                IndexEntry {
                path: rel_path.to_string(),
                hash: String::default(),
                }
            );
        } else if metadata.is_dir() {
            if !recursive {
                eprintln!("{} is a directory, use --recursive or -r to handle.", path.to_string_lossy());
                process::exit(1);
            }
            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries.flatten() {
                    let file_name = entry.file_name();
                    let file_name_str = file_name.to_string_lossy();
                    let new_abs_path = entry.path();
                    let new_rel_path = if rel_path.is_empty() {
                        file_name_str.to_string()
                    } else {
                        format!("{}/{}", rel_path, file_name_str)
                    };
                    register_files(new_abs_path.to_str().unwrap(), &new_rel_path, index, recursive);
                }
            }
        }
    }
}