//! Module: rm
//!
//! Implements file removal functionality, including support for:
//! - Removing files from the working directory and/or index
//! - Recursive deletion of directories
//! - Beautified verbose output

use std::collections::HashSet;
use std::path::Path;
use std::process;

use crate::{core::*, utils::*};
use crate::core::index::IndexEntry;


/// Removes files from the working directory and/or index based on user-specified options.
///
/// # Arguments
/// * `files` - List of file/directory paths to remove.
/// * `recursive` - If true, recursively removes directories and their contents.
/// * `cached` - If true, only removes files from the index, not from the filesystem.
/// * `verbose` - If true, displays detailed information about removed files.
///
/// # Behavior
/// 1. Reads the current index.
/// 2. Registers the files/directories to be removed.
/// 3. Removes matching entries from the index.
/// 4. Optionally removes files from the working area.
/// 5. Writes the updated index back to disk.
///
/// # Exits
/// * If any path cannot be canonicalized.
/// * If any path is outside the repository root.
pub fn remove(files: Vec<String>, recursive: bool, cached: bool, verbose: bool) {
    let mut index = index::read_index();
    let repo_path = utils::pwd();

    let mut rm_entries: HashSet<IndexEntry> = Default::default();

    for file in &files {
        let file_path = match Path::new(file).canonicalize() {
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

        index::register_files(&file_path, &utils::relative_path(&repo_path, &file_path), &mut rm_entries, &recursive);
    }

    let mut remove_log: HashSet<IndexEntry> = Default::default();
    let mut remove_working_log: HashSet<String> = Default::default();

    for entry in rm_entries {
        let path = entry.path.clone();
        if let Some(index_entry) = index.get(&path) {
            remove_log.insert(index_entry.clone());
        }
    }

    for entry in &remove_log {
        index.remove(&entry.path);
    }

    if !cached {
        for file in &files {
            if file == &utils::get_git_directory() {
                continue;
            }
            storage::remove_path(file, recursive);
            remove_working_log.insert(file.clone());
        }
    }

    index::write_index(&index);

    if !verbose {
        for entry in &remove_log {
            eprintln!("{}", entry.hash);
        }
    } else {
        eprintln!("Removed {} file(s) from staging area.", remove_log.len());
        if remove_log.len() != 0 {
            eprintln!();
            for entry in &remove_log {
                eprintln!("    \x1b[31mRemove:\x1b[0m {} ({})", entry.path, entry.hash);
            }
            eprintln!();
        }
        if !cached {
            eprintln!("Removed {} file(s) from working area.", remove_working_log.len());
            if remove_working_log.len() != 0 {
                eprintln!();
                for entry in &remove_working_log {
                    eprintln!("    \x1b[31mRemove:\x1b[0m {}", entry);
                }
                eprintln!();
            }
        }
    }

}
