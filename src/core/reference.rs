//! Module: reference
//!
//! Provides functions for working with Git references, including reading and writing branch heads,
//! resolving the current branch or commit, and managing symbolic references like `.git/HEAD`.

use std::{io, path::PathBuf, process};

use crate::{core::*, utils::*};


/// Reads the commit hash pointed to by a specific branch head.
///
/// # Arguments
/// * `head_name` - The name of the branch (e.g., "main", "dev").
///
/// # Returns
/// * `String` - The SHA-1 hash of the commit that the branch points to.
///
/// # Exits
/// * If the ref file cannot be read or is not valid UTF-8.
pub fn get_head(head_name: &str) -> String {
    let ref_path = utils::get_git_directory() + "/refs/heads/" + head_name;
    match storage::read_file(&ref_path) {
        Ok(content) => {
            std::str::from_utf8(&content).expect("Not valid UTF-8").to_string()
        },
        Err(e) => {
            eprintln!("Error when reading head file {} : {}", ref_path, e);
            process::exit(1)
        }
    }
}


/// Attempts to read the commit hash from a branch head without exiting on error.
///
/// # Arguments
/// * `head_name` - Name of the branch.
///
/// # Returns
/// * `Result<String, io::Error>` - Either the commit hash or an I/O error.
pub fn try_get_head(head_name: &str) -> Result<String, io::Error> {
    let ref_path = utils::get_git_directory() + "/refs/heads/" + head_name;
    match storage::read_file(&ref_path) {
        Ok(content) => {
            Ok(std::str::from_utf8(&content).expect("Not valid UTF-8").to_string())
        },
        Err(e) => Err(e)
    }
}


/// Stores a commit hash into a branch reference file.
///
/// # Arguments
/// * `ref_name` - Name of the branch.
/// * `hash` - SHA-1 hash of the commit to store.
///
/// # Exits
/// * If the write operation fails.
pub fn store_head(ref_name: &str, hash: &str) {
    let ref_path = utils::get_git_directory() + "/refs/heads/" + ref_name;
    if let Err(e) = storage::write_file(&ref_path, hash.as_bytes()) {
        eprintln!("Error when writing head file {} : {}", ref_path, e);
        process::exit(1)
    }
}


/// Gets the name of the currently checked-out branch, if any.
///
/// # Returns
/// * `Option<String>` - Some(branch_name) if HEAD is on a branch, None if in detached HEAD state.
///
/// # Exits
/// * If `.git/HEAD` is malformed or refers to an invalid object.
pub fn get_current_branch() -> Option<String> {
    let head_path = utils::get_git_directory() + "/HEAD";
    match storage::read_file(&head_path) {
        Ok(content) => {
            let content = std::str::from_utf8(&content).expect("Not valid UTF-8").to_string();
            if content.starts_with("ref: refs/heads/") {
                return Some(content[16..].to_string());
            }
            // checks if content is a CMIT_hash
            match object::get_object_type(&content) {
                object::ObjectType::Commit => {
                    return None
                }
                _ => {
                    eprintln!(".git/HEAD: {} does not refer to a commit.", content);
                    process::exit(1);
                }
            }
        },
        Err(e) => {
            eprintln!("Error when reading HEAD {} : {}", head_path, e);
            process::exit(1);
        }
    }
}


/// Resolves the current commit hash pointed to by `.git/HEAD`.
///
/// # Returns
/// * `String` - SHA-1 hash of the current commit.
///
/// # Exits
/// * If `.git/HEAD` is malformed or refers to an invalid object.
pub fn get_current_commit() -> String {
    let head_path = utils::get_git_directory() + "/HEAD";
    match storage::read_file(&head_path) {
        Ok(content) => {
            let content = std::str::from_utf8(&content).expect("Not valid UTF-8").to_string();
            if content.starts_with("ref: refs/heads/") {
                return get_head(&content[16..].to_string());
            }
            // checks if content is a CMIT_hash
            match object::get_object_type(&content) {
                object::ObjectType::Commit => {
                    return content;
                }
                _ => {
                    eprintln!(".git/HEAD: {} does not refer to a commit.", content);
                    process::exit(1);
                }
            }
        },
        Err(e) => {
            eprintln!("Error when reading HEAD {} : {}", head_path, e);
            process::exit(1);
        }
    }
}


/// Updates `.git/HEAD` to point to a specific branch.
///
/// # Arguments
/// * `ref_name` - Name of the branch to reference.
///
/// # Exits
/// * If the write operation fails.
pub fn store_current_branch_ref(ref_name: &str) {
    let head_path = utils::get_git_directory() + "/HEAD";
    if let Err(e) = storage::write_text_file(&head_path, &format!("ref: refs/heads/{}", ref_name)) {
        eprintln!("Error writing to HEAD file {} : {}", head_path, e);
        process::exit(1);
    }
}


/// Sets `.git/HEAD` directly to a commit hash (detached HEAD state).
///
/// # Arguments
/// * `commit_hash` - SHA-1 hash of the commit to set as current.
///
/// # Exits
/// * If the write operation fails.
pub fn store_current_branch_commit(commit_hash: &str) {
    let head_path = utils::get_git_directory() + "/HEAD";
    if let Err(e) = storage::write_text_file(&head_path, commit_hash) {
        eprintln!("Error writing to HEAD file {} : {}", head_path, e);
        process::exit(1);
    }
}


/// Lists all available local branch names.
///
/// # Returns
/// * `Vec<String>` - A vector containing the names of all local branches.
///
/// # Panics
/// * If the `refs/heads/` directory cannot be read.
pub fn get_all_heads() -> Vec<String> {
    let ref_path = PathBuf::from(utils::get_git_directory()).join("refs").join("heads");

    let mut res = Vec::new();

    for entry in std::fs::read_dir(ref_path).unwrap() {
        let entry = entry.unwrap();
        let file_name = entry.file_name();
        res.push(file_name.to_string_lossy().to_string());
    }

    res
}


/// Creates a new branch reference pointing to a given commit.
///
/// # Arguments
/// * `head_name` - Name of the new branch.
/// * `content` - SHA-1 hash of the commit to point to.
///
/// # Exits
/// * If the file cannot be created or written to.
pub fn create_head(head_name: &str, content: &str) {
    let ref_path = utils::get_git_directory() + "/refs/heads/" + head_name;
    storage::create_nonexist_file(&ref_path);
    if let Err(e) = storage::write_text_file(&ref_path, content) {
        eprintln!("Error writing to {} : {}", ref_path, e);
        process::exit(1);
    }
}


/// Checks whether one branch is an ancestor of another.
///
/// # Arguments
/// * `prev_branch` - Name of the potential ancestor branch.
/// * `post_branch` - Name of the potential descendant branch.
///
/// # Returns
/// * `bool` - True if `prev_branch` is an ancestor of `post_branch`.
///
/// # Exits
/// * If either branch does not exist or points to an invalid commit.
pub fn is_prev_branch(prev_branch: &str, post_branch: &str) -> bool {
    let prev_commit = get_head(prev_branch);
    let post_commit = get_head(post_branch);
    commit::is_prev_commit(&prev_commit, &post_commit)
}