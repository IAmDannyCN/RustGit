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

use super::utils;

/// Reads the contents of a file into a byte vector.
pub fn read_file(file_name: &str) -> Result<Vec<u8>, io::Error> {
    fs::read(file_name)
}

/// Writes the contents of a byte vector to a file.
pub fn write_file(file_name: &str, contents: &[u8]) -> io::Result<()> {
    let path = Path::new(file_name);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("write_file: failed to create parent folder.");
    }
    fs::write(path, contents)
}

/// Reads the contents of a text file into a string.
pub fn read_text_file(file_name: &str) -> Result<String, io::Error> {
    fs::read_to_string(file_name)
}

/// Writes the contents of a string to a text file.
pub fn write_text_file(file_name: &str, contents: &str) -> io::Result<()> {
    let path = Path::new(file_name);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("write_file: failed to create parent folder.");
    }
    fs::write(path, contents)
}

/// Creates a directory and all its parent directories if they do not exist.
pub fn create_directory(directory_name: &str) {
    if let Err(e) = fs::create_dir_all(directory_name) {
        eprintln!("Failed to create directory '{}': {}.", directory_name, e);
        process::exit(1);
    }
}

/// Creates a file. 
pub fn create_file(file_name: &str) {
    if let Err(e) = fs::File::create(file_name) {
        eprintln!("Failed to create file '{}': {}.", file_name, e);
        process::exit(1);
    }
}

/// Delete a target file.
pub fn remove_file(file_name: &str) {
    if let Err(e) = fs::remove_file(file_name) {
        eprintln!("Failed to remove file '{}': {}.", file_name, e);
        process::exit(1);
    }
}

/// Delete a target file/directory, when path == utils::pwd(), only remove its contents
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

pub fn clear_working_area() {
    let working_path = utils::pwd();
    let git_dir = PathBuf::from(utils::get_git_directory());

    for entry in fs::read_dir(&working_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path == git_dir {
            continue;
        }

        if path.is_dir() {
            fs::remove_dir_all(&path).unwrap_or_else(|e| {
                eprintln!("Failed to remove directory {}: {}", path.display(), e);
                std::process::exit(1);
            });
        } else {
            fs::remove_file(&path).unwrap_or_else(|e| {
                eprintln!("Failed to remove file {}: {}", path.display(), e);
                std::process::exit(1);
            });
        }
    }
}

pub fn clear_index() {
    let index_path = format!("{}/index", utils::get_git_directory());
    if let Err(e) = write_text_file(&index_path, "") {
        eprintln!("Error clearing index file {} : {}", index_path, e);
        process::exit(1);
    }
}

/// Restores the working area from a tree hash.
fn restore_tree(tree_hash: &str, tree_path: &str) {

    // tree_path is real path, e.g. /mnt/repo/A/B/C
    // before calling `restore_tree(tree_hash, tree_path)`, this tree itself has been built.

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

/// Restores the working area from a tree hash and updates the index entries.
pub fn restore_index_by_tree(tree_hash: &str, tree_path: &str, all_entries: &mut HashMap<String, IndexEntry>) {
    // tree_path is real path, e.g. /mnt/repo/A/B/C
    // before calling `restore_tree(tree_hash, tree_path)`, this tree itself has been built.

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


/// Restores the working area from a commit hash.
pub fn restore_working_area(commit_hash: &str) {

    if commit_hash == "" {
        return ;
    }

    let mut commit = Commit { hash: Some(commit_hash.to_owned()), data: None };
    commit.read_commit();
    let root_hash = commit.data.unwrap().tree_hash;

    restore_tree(&root_hash, &utils::pwd());
}