//! Module: object
//!
//! This module provides basic operations for Git-like objects, including
//! locating object file paths, reading and writing object files, and
//! determining object types.
//!
//! Supported object types: Blob, Tree, Commit.

use std::process;

use crate::utils::*;

/// Enum representing the type of a Git object.
pub enum ObjectType {
    Blob,
    Tree,
    Commit,
}


/// Returns the full file path of a Git object in the `.git/objects/` directory.
///
/// # Arguments
///
/// * `object_name` - A SHA-1 hash string identifying the object.
///
/// # Panics / Exits
///
/// This function prints an error and exits the process if the given hash length
/// is not equal to the expected hash length.
///
/// # Returns
///
/// A `String` representing the full path to the object file.
pub fn get_object_path(object_name: &str) -> String {
    if (&object_name).len() != hash::HASH_LENGTH {
        eprintln!("{} is not a valid object.", object_name);
        process::exit(1);
    }
    let git_directory = utils::get_git_directory();
    let file_path = git_directory.clone() + 
                            "/objects/" + &object_name[..hash::FOLDER_LENGTH] +
                            "/" + &object_name[hash::FOLDER_LENGTH..];
    file_path
}


/// Reads the content of a Git object file.
///
/// # Arguments
///
/// * `object_name` - A SHA-1 hash string identifying the object.
///
/// # Panics / Exits
///
/// This function prints an error and exits the process if the file cannot be read.
///
/// # Returns
///
/// A `String` containing the raw content of the object file.
pub fn read_object_file(object_name: &str) -> String {

    let file_path = get_object_path(&object_name);

    match storage::read_text_file(&file_path) {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Failed to read object '{}': {}", object_name, e);
            process::exit(1)
        }
    }
}


/// Writes content to a Git object file if it does not already exist.
///
/// # Arguments
///
/// * `object_name` - A SHA-1 hash string identifying the object.
/// * `content` - The content to write into the object file.
///
/// # Panics / Exits
///
/// This function prints an error and exits the process if writing the file fails.
pub fn write_object_file(object_name: &str, content: &str) {

    let file_path = get_object_path(&object_name);
    
    if std::path::Path::new(&file_path).exists() {
        return ;
    }
    
    match storage::write_text_file(&file_path, content) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Failed to write object '{}': {}", object_name, e);
            process::exit(1)
        }
    }
}


/// Determines the type of the Git object (Blob, Tree, or Commit).
///
/// # Arguments
///
/// * `object_name` - A SHA-1 hash string identifying the object.
///
/// # Panics / Exits
///
/// This function prints an error and exits the process if:
/// - The object is malformed
/// - The content cannot be decoded
/// - The header is unrecognized
///
/// # Returns
///
/// An `ObjectType` enum indicating the object’s type.
pub fn get_object_type(object_name: &str) -> ObjectType {

    let raw_content = read_object_file(object_name);
    let vecu8_content = serialize::deserialize(&raw_content);
    let full_content = std::str::from_utf8(&vecu8_content).expect("Invalid UTF-8");

    if full_content.len() < 4 {
        eprintln!("Broken object: {}", object_name);
        process::exit(1);
    }

    let header = &full_content[..4];

    match header {
        "BLOB" => ObjectType::Blob,
        "TREE" => ObjectType::Tree,
        "CMIT" => ObjectType::Commit,
        _ => {
            eprintln!("Broken object: {}", object_name);
            process::exit(1);
        }
    }
}