//! Module: add
//!
//! Implements add functionality for staging files into the index.
//! Supports recursive directory addition, blob hashing, and updates to the index.
//!
//! Handles:
//! - Adding new files or directories
//! - Updating modified files
//! - Removing deleted files from the index
//! - Verbose output of changes

use std::collections::HashSet;
use std::process;
use std::path::Path;

use crate::core::blob::BlobTrait;
use crate::{core::*, utils::*};
use crate::core::index::{Index, IndexEntry};


/// Core logic for adding files to the index.
///
/// # Arguments
/// * `files` - List of file/directory paths to add.
///
/// # Returns
/// A tuple containing:
/// 1. Updated index with new hashes
/// 2. Set of files added
/// 3. Set of files removed
/// 4. Set of files modified
///
/// # Behavior
/// 1. Registers all files recursively using `register_entries`.
/// 2. Reads current index.
/// 3. Hashes each file and updates the index accordingly.
/// 4. Detects and logs additions, modifications, and deletions.
///
/// # Exits
/// * If any path cannot be canonicalized.
/// * If any path is outside the repository root.
pub fn add_core(files: &Vec<String>) -> (
    Index,
    HashSet<IndexEntry>,
    HashSet<IndexEntry>,
    HashSet<IndexEntry>,
) {

    let (new_entries, base_entries) = register_entries(files);

    // After registration, ALL entry.hash ARE EMPTY in new_entries !!!

    let mut index = index::read_index();
    let repo_path = utils::pwd();

    let mut add_log: HashSet<IndexEntry> = Default::default();
    let mut remove_log: HashSet<IndexEntry> = Default::default();
    let mut modify_log: HashSet<IndexEntry> = Default::default();

    for entry in &new_entries  {

        // entry.hash is empty !!!
        let file_path = format!("{}/{}", repo_path, entry.path);
        let mut blob = blob::get_blob_from_file(&file_path);
        
        let path = entry.path.clone();
        let hash = blob.hash.clone().unwrap();

        match index.get(&path) {
            None => {
                // A new entry
                let new_entry = IndexEntry { path: path.clone(), hash: hash.clone() };

                add_log.insert(new_entry.clone());

                index.insert(path, new_entry);
                blob.write_blob();
                
                continue;
            }
            Some(old_entry) => {
                // An old entry
                if old_entry.hash == hash {
                    continue;
                } else {
                    let new_entry = IndexEntry { path: path.clone(), hash: hash.clone() };

                    modify_log.insert(new_entry.clone());

                    index.remove(&path);
                    index.insert(path, new_entry);
                    blob.write_blob();
                    
                    continue;
                }
            }
        }
    }

    let mut new_entry_paths: HashSet<String> = Default::default();
    for entry in &new_entries {
        new_entry_paths.insert(entry.path.clone());
    }

    index.retain(|_, old_entry| {
        if !new_entry_paths.contains(&old_entry.path) {
            for base_entry in &base_entries {
                let old_entry_full = format!("{}/{}", repo_path, &old_entry.path);
                if utils::is_subpath(&base_entry, &old_entry_full) {
                    remove_log.insert(old_entry.clone());
                    return false;
                }
            }
            true
        } else {
            true
        }
    });

    (index, add_log, remove_log, modify_log)
}


/// Adds the specified files or directories to the index.
///
/// # Arguments
/// * `files` - List of file/directory paths to add.
/// * `verbose` - If true, displays beautified output instead of just hashes.
///
/// # Behavior
/// 1. Calls `add_core` to compute index updates.
/// 2. Writes updated index back to disk.
/// 3. Outputs detailed change log if `verbose` is enabled.
pub fn add(files: Vec<String>, verbose: bool) {

    let (
        index,
        add_log,
        remove_log,
        modify_log
    ) = add_core(&files);

    index::write_index(&index);

    if !verbose {
        for entry in &add_log {
            eprintln!("{}", entry.hash);
        }
    } else {
        eprintln!(
            "Added {} file(s), Removed {} file(s), Modified {} file(s).", 
            add_log.len(), remove_log.len(), modify_log.len()
        );
        if !(add_log.len() == 0 && remove_log.len() == 0 && modify_log.len() == 0) {
            eprintln!();
            for entry in &add_log {
                eprintln!("    \x1b[32mAdd:\x1b[0m    {} ({})", entry.path, entry.hash);
            }
            for entry in &remove_log {
                eprintln!("    \x1b[31mRemove:\x1b[0m {} ({})", entry.path, entry.hash);
            }
            for entry in &modify_log {
                eprintln!("    \x1b[33mModify:\x1b[0m {} ({})", entry.path, entry.hash);
            }
            eprintln!();
        }
    }
    
}


/// Recursively registers files from the given paths into a set of index entries.
///
/// # Arguments
/// * `files` - List of file/directory paths to register.
///
/// # Returns
/// A tuple containing:
/// 1. Set of unhashed index entries (to be hashed later)
/// 2. Set of base file paths used for filtering removals
///
/// # Exits
/// * If any path cannot be canonicalized.
/// * If any path is outside the repository root.
fn register_entries(files: &Vec<String>) -> (HashSet<IndexEntry>, HashSet<String>) {

    let repo_path = utils::pwd();

    let mut new_entries: HashSet<IndexEntry> = Default::default();
    let mut base_entries: HashSet<String> = Default::default();
    
    for file in files {
        let file_path = match Path::new(&file).canonicalize() {
            Ok(p) => p.to_string_lossy().into_owned(),
            Err(e) => {
                eprintln!("Error when canonicalizing path {}: {}", file, e);
                process::exit(1)
            }
        };
        if !utils::is_subpath(&repo_path, &file_path) {
            eprintln!("File {} does not belong to repository {}.", file_path, repo_path);
            process::exit(1);
        }

        index::register_files(&file_path, &utils::relative_path(&repo_path, &file_path), &mut new_entries, &true);
        base_entries.insert(file_path);
    }

    (new_entries, base_entries)
}