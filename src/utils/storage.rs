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
use crate::core::tree::Tree;
use crate::core::tree::TreeTrait;
use crate::core::tree::TreeEntryType;

use super::utils;

pub fn read_file(file_name: &str) -> Result<Vec<u8>, io::Error> {
    fs::read(file_name)
}

pub fn write_file(file_name: &str, contents: &[u8]) -> io::Result<()> {
    let path = Path::new(file_name);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("write_file: failed to create parent folder.");
    }
    fs::write(path, contents)
}

pub fn read_text_file(file_name: &str) -> Result<String, io::Error> {
    fs::read_to_string(file_name)
}

pub fn write_text_file(file_name: &str, contents: &str) -> io::Result<()> {
    let path = Path::new(file_name);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("write_file: failed to create parent folder.");
    }
    fs::write(path, contents)
}

pub fn create_directory(directory_name: &str) {
    if let Err(e) = fs::create_dir_all(directory_name) {
        eprintln!("Failed to create directory '{}': {}.", directory_name, e);
        process::exit(1);
    }
}

pub fn create_file(file_name: &str) {
    if let Err(e) = fs::File::create(file_name) {
        eprintln!("Failed to create file '{}': {}.", file_name, e);
        process::exit(1);
    }
}

pub fn remove_file(file_name: &str) {
    if let Err(e) = fs::remove_file(file_name) {
        eprintln!("Failed to remove file '{}': {}.", file_name, e);
        process::exit(1);
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

fn restore_tree(tree_hash: &str, tree_path: &str) {

    // tree_path is real path, e.g. /mnt/repo/A/B/C
    // before calling `restore_tree(tree_hash, tree_path)`, this tree itself has been built.

    let mut tree = Tree { hash: Some(tree_hash.to_owned()), data: None};
    tree.read_tree();

    let entries = tree.data.as_ref().unwrap();
    for entry in entries {
        let son_path = format!("{}/{}", tree_path, entry.name);
        if Path::new(&son_path).exists() {
            continue;
        }
        
        match &entry.entry_type {
            TreeEntryType::Tree => {
                create_nonexist_directory(&son_path);
                restore_tree(&entry.hash, &son_path);
            }
            TreeEntryType::Blob => {
                create_nonexist_file(&son_path);
                let mut blob = Blob { hash: Some(entry.hash.clone()), data: None };
                blob.read_blob();
                if let Err(e) = write_file(&son_path, &blob.data.unwrap()) {
                    eprintln!("Error when restoring blob {} : {}", son_path, e);
                    process::exit(1);
                }
            }
            TreeEntryType::Bexe => {
                create_nonexist_file(&son_path);
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

pub fn restore_working_area(commit_hash: &str) {

    if commit_hash == "" {
        return ;
    }

    let mut commit = Commit { hash: Some(commit_hash.to_owned()), data: None };
    commit.read_commit();
    let root_hash = commit.data.unwrap().tree_hash;

    restore_tree(&root_hash, &utils::pwd());
}