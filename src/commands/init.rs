//! Module: init
//!
//! Implements the initialization logic for a new Git repository, similar to `git init`.
//! Creates the necessary directory structure and initial files inside `.git`,
//! and sets up the default branch reference.

use crate::utils::*;

use std::{fs, path::Path, process};


/// Initializes a new Git repository in the current working directory.
///
/// # Arguments
/// * `initial_branch` - Optional name of the initial branch (defaults to "master").
///
/// # Behavior
/// 1. Deletes any existing `.git` directory if present.
/// 2. Recreates the `.git` directory and all required subdirectories.
/// 3. Sets up the initial `HEAD` reference pointing to the specified branch.
/// 4. Creates an empty index file and branch reference.
///
/// # Exits
/// * If writing the HEAD reference fails.
pub fn init(initial_branch: Option<String>) {
    let git_directory: String = utils::pwd() + "/.git";//"/.mygit";

    delete_original_directory(&git_directory);

    use storage::create_nonexist_directory;
    use storage::create_nonexist_file;

    create_nonexist_directory(&git_directory);
    // create_nonexist_directory(&format!("{}/branches", git_directory));
    // create_nonexist_directory(&format!("{}/logs", git_directory));
    create_nonexist_directory(&format!("{}/objects", git_directory));
    // create_nonexist_directory(&format!("{}/objects/info", git_directory));
    // create_nonexist_directory(&format!("{}/objects/pack", git_directory));
    create_nonexist_directory(&format!("{}/refs", git_directory));
    create_nonexist_directory(&format!("{}/refs/heads", git_directory));
    // create_nonexist_directory(&format!("{}/refs/tags", git_directory));
    // create_nonexist_file(&format!("{}/COMMIT_EDITMSG", git_directory));
    create_nonexist_file(&format!("{}/HEAD", git_directory));
    create_nonexist_file(&format!("{}/index", git_directory));

    let branch_name: String = match &initial_branch {
        Some(name) => name.clone(),
        None => "master".to_string()
    };

    if let Err(e) = storage::write_file( 
        &format!("{}/HEAD", git_directory), 
        &format!("ref: refs/heads/{}", branch_name).as_bytes()
    ) {
        eprintln!("Cannot write to {}/HEAD: {}", git_directory, e);
        process::exit(1);
    }
    create_nonexist_file(&format!("{}/refs/heads/{}", git_directory, branch_name));
}


/// Removes the existing `.git` directory if it exists.
///
/// # Arguments
/// * `git_directory` - Path to the `.git` directory to delete.
///
/// # Notes
/// * If deletion fails, only prints an error but does not exit.
fn delete_original_directory(git_directory: &str) {
    let path = Path::new(&git_directory);
    if path.exists() {
        if let Err(e) = fs::remove_dir_all(path) {
            eprintln!("Error deleting original repository: {}", e);
        } else {
            eprintln!("Reinitialized existing Git repository at: {}", git_directory);
        }
    }
}