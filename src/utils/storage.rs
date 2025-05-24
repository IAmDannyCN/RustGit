//! Module: storage
//!
//! Provides low-level file and directory operations used throughout the Git implementation.
//! These functions handle reading/writing files, creating/removing directories,
//! and restoring working areas based on Git objects like trees and commits.

use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::process;
use std::os::unix::fs::PermissionsExt;
use std::os::unix::fs::symlink;

use crate::core::blob::Blob;
use crate::core::blob::BlobTrait;
use crate::core::commit::Commit;
use crate::core::commit::CommitTrait;
use crate::core::index::IndexEntry;
use crate::core::tree::Tree;
use crate::core::tree::TreeTrait;
use crate::core::tree::TreeEntryType;
use crate::core::index;
use crate::core::index::Index;

use super::utils;


/// Reads the contents of a file into a byte vector.
///
/// # Arguments
/// * `file_name` - Path to the file to read.
///
/// # Returns
/// * `Result<Vec<u8>, io::Error>` - The content of the file or an I/O error.
pub fn read_file(file_name: &str) -> Result<Vec<u8>, io::Error> {
    fs::read(file_name)
}


/// Writes a byte slice to a file, creating parent directories if necessary.
///
/// # Arguments
/// * `file_name` - Path to the file to write.
/// * `contents` - Raw bytes to write to the file.
///
/// # Returns
/// * `io::Result<()>` - Ok(()) if successful, or an I/O error.
pub fn write_file(file_name: &str, contents: &[u8]) -> io::Result<()> {
    let path = Path::new(file_name);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("write_file: failed to create parent folder.");
    }
    fs::write(path, contents)
}


/// Reads the contents of a text file into a String.
///
/// # Arguments
/// * `file_name` - Path to the file to read.
///
/// # Returns
/// * `Result<String, io::Error>` - The content of the file as a UTF-8 string or an I/O error.
pub fn read_text_file(file_name: &str) -> Result<String, io::Error> {
    fs::read_to_string(file_name)
}


/// Writes a string to a text file, creating parent directories if necessary.
///
/// # Arguments
/// * `file_name` - Path to the file to write.
/// * `contents` - Text to write to the file.
///
/// # Returns
/// * `io::Result<()>` - Ok(()) if successful, or an I/O error.
pub fn write_text_file(file_name: &str, contents: &str) -> io::Result<()> {
    let path = Path::new(file_name);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("write_file: failed to create parent folder.");
    }
    fs::write(path, contents)
}


/// Creates a directory and all required parent directories.
///
/// # Arguments
/// * `directory_name` - Path of the directory to create.
///
/// # Exits
/// * If directory creation fails, prints an error and exits.
pub fn create_directory(directory_name: &str) {
    if let Err(e) = fs::create_dir_all(directory_name) {
        eprintln!("Failed to create directory '{}': {}.", directory_name, e);
        process::exit(1);
    }
}


/// Creates a new file at the specified path.
///
/// # Arguments
/// * `file_name` - Path of the file to create.
///
/// # Exits
/// * If file creation fails, prints an error and exits.
pub fn create_file(file_name: &str) {
    if let Err(e) = fs::File::create(file_name) {
        eprintln!("Failed to create file '{}': {}.", file_name, e);
        process::exit(1);
    }
}


/// Deletes a file at the specified path.
///
/// # Arguments
/// * `file_name` - Path of the file to remove.
///
/// # Exits
/// * If file removal fails, prints an error and exits.
pub fn remove_file(file_name: &str) {
    if let Err(e) = fs::remove_file(file_name) {
        eprintln!("Failed to remove file '{}': {}.", file_name, e);
        process::exit(1);
    }
}


/// Removes a file or directory (optionally recursively).
///
/// # Arguments
/// * `path` - Path to remove.
/// * `recursive` - Whether to remove directories and their contents recursively.
///
/// # Notes
/// * If path is current working directory and recursive is true, only clears contents, not the directory itself.
pub fn remove_path(path: &str, recursive: bool) {
    let p = Path::new(path);

    if !p.exists() {
        eprintln!("Error: Path '{}' does not exist.", path);
        return;
    }

    if p.is_file() {
        if let Err(e) = fs::remove_file(p) {
            eprintln!("Error removing file '{}': {}", path, e);
        }
    } else if p.is_dir() {
        if recursive {
            if path == utils::pwd() {
                match fs::read_dir(p) {
                    Ok(entries) => {
                        for entry in entries {
                            if let Ok(entry) = entry {
                                let sub_path = entry.path();
                                let result = if sub_path.is_dir() {
                                    fs::remove_dir_all(&sub_path)
                                } else {
                                    fs::remove_file(&sub_path)
                                };
                                if let Err(e) = result {
                                    eprintln!("Error removing '{}': {}", sub_path.display(), e);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error reading directory '{}': {}", path, e);
                    }
                }
            } else {
                if let Err(e) = fs::remove_dir_all(p) {
                    eprintln!("Error recursively removing directory '{}': {}", path, e);
                }
            }
        } else {
            eprintln!("Error: '{}' is a directory. Set recursive = true to remove directories.", path);
        }
    } else {
        eprintln!("Error: '{}' is neither a file nor a directory.", path);
    }
}


/// Creates a directory only if it does not already exist.
///
/// # Arguments
/// * `directory_name` - Path of the directory to create.
///
/// # Exits
/// * If the directory already exists or creation fails.
pub fn create_nonexist_directory(directory_name: &str) {
    match fs::exists(&directory_name) {
        Ok(res) => if res == true {
            eprintln!("Directory {} already exists.", directory_name);
            process::exit(1);
        }
        Err(e) => {
            eprintln!("Error checking the exsitence of directory {}: {}.", directory_name, e);
            process::exit(1);
        }
    }

    create_directory(&directory_name);
}


/// Creates a file only if it does not already exist.
///
/// # Arguments
/// * `file_name` - Path of the file to create.
///
/// # Exits
/// * If the file already exists or creation fails.
pub fn create_nonexist_file(file_name: &str) {
    match fs::exists(&file_name) {
        Ok(res) => if res == true {
            eprintln!("File {} already exists", file_name);
            process::exit(1);
        }
        Err(e) => {
            eprintln!("Error checking the exsitence of file {}: {}.", file_name, e);
            process::exit(1);
        }
    }

    create_file(&file_name);
}


/// Clears the working area by removing all tracked files from the index.
///
/// # Notes
/// * Does not remove untracked files or directories.
pub fn clear_working_area() {
    let repo_path = utils::pwd();

    let index_entries: Index = index::read_index();
    for entry in &index_entries {
        let path = PathBuf::from(format!("{}/{}", repo_path, entry.0));
        fs::remove_file(&path).unwrap_or_else(|e| {
            eprintln!("Failed to remove directory {}: {}", path.display(), e);
            std::process::exit(1);
        });
    } 
}


/// Restores the working directory from a given commit's tree.
///
/// # Arguments
/// * `commit_hash` - SHA-1 hash of the commit whose tree to restore.
pub fn restore_working_area(commit_hash: &str) {

    if commit_hash == "" {
        return ;
    }

    let mut commit = Commit { hash: Some(commit_hash.to_owned()), data: None };
    commit.read_commit();
    let root_hash = commit.data.unwrap().tree_hash;

    restore_tree(&root_hash, &utils::pwd());
}


/// Recursively restores a tree object into the file system.
///
/// # Arguments
/// * `tree_hash` - SHA-1 hash of the tree to restore.
/// * `tree_path` - Filesystem (absolute) path where the tree should be restored.
/// 
/// # Notes
/// * before calling `restore_tree(tree_hash, tree_path)`, this tree itself must have been built.
fn restore_tree(tree_hash: &str, tree_path: &str) {

    // tree_path is real path, e.g. /mnt/repo/A/B/C

    let mut tree = Tree { hash: Some(tree_hash.to_owned()), data: None};
    tree.read_tree();

    let entries = tree.data.as_ref().unwrap();
    for entry in entries {
        let son_path = format!("{}/{}", tree_path, entry.name);
        // if Path::new(&son_path).exists() {
        //     continue;
        // }
        
        match &entry.entry_type {
            TreeEntryType::Tree => {
                create_directory(&son_path);
                restore_tree(&entry.hash, &son_path);
            }
            TreeEntryType::Blob => {
                create_file(&son_path);
                let mut blob = Blob { hash: Some(entry.hash.clone()), data: None };
                blob.read_blob();
                if let Err(e) = write_file(&son_path, &blob.data.unwrap()) {
                    eprintln!("Error when restoring blob {} : {}", son_path, e);
                    process::exit(1);
                }
            }
            TreeEntryType::Bexe => {
                create_file(&son_path);
                let mut blob = Blob { hash: Some(entry.hash.clone()), data: None };
                blob.read_blob();
                if let Err(e) = write_file(&son_path, &blob.data.unwrap()) {
                    eprintln!("Error when restoring bexe {} : {}", son_path, e);
                    process::exit(1);
                }
                
                // add +x permission
                let mut perms = fs::metadata(&son_path).unwrap().permissions();
                perms.set_mode(0o755);  // rwxr-xr-x
                fs::set_permissions(&son_path, perms).unwrap();
            }
            TreeEntryType::Bsym => {
                let mut blob = Blob { hash: Some(entry.hash.clone()), data: None };
                blob.read_blob();
                let target = String::from_utf8(blob.data.unwrap()).unwrap();  // symlink target
                if let Err(e) = symlink(&target, &son_path) {
                    eprintln!("Error when restoring symlink {} -> {} : {}", son_path, target, e);
                    process::exit(1);
                }
            }
        }
    }

}


/// Recursively builds an index by restoring a tree and collecting entries.
///
/// # Arguments
/// * `tree_hash` - SHA-1 hash of the tree to restore.
/// * `tree_path` - Filesystem (absolute) path where the tree would be restored.
/// * `all_entries` - A map to collect index entries during restoration.
pub fn restore_index_by_tree(tree_hash: &str, tree_path: &str, all_entries: &mut HashMap<String, IndexEntry>) {
    
    // tree_path is real path, e.g. /mnt/repo/A/B/C

    let mut tree = Tree { hash: Some(tree_hash.to_owned()), data: None};
    tree.read_tree();

    let repo_path = utils::pwd();

    let entries = tree.data.as_ref().unwrap();
    for entry in entries {
        let son_path = format!("{}/{}", tree_path, entry.name);
        // if Path::new(&son_path).exists() {
        //     continue;
        // }
        
        match &entry.entry_type {
            TreeEntryType::Tree => {
                restore_index_by_tree(&entry.hash, &son_path, all_entries);
            }
            _ => {
                let this_entry = IndexEntry {
                    path: utils::get_relative_path(&repo_path, &son_path),
                    hash: entry.hash.clone()
                };
                all_entries.insert(son_path, this_entry);
            }
        }
    }
}