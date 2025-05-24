//! Module: utils
//!
//! Provides utility functions for path manipulation, environment handling,
//! time formatting, and working directory management used throughout the Git implementation.

use std::path::{Path, PathBuf};
use std::{env, process};
use std::sync::OnceLock;

use chrono::Local;

static PWD: OnceLock<String> = OnceLock::new();


/// Sets the current working directory (PWD) to the absolute path of the given directory.
///
/// # Arguments
/// * `path` - The directory path to set as the working directory.
///
/// # Exits
/// * If canonicalization fails or if PWD was already set.
pub fn set_pwd(path: &str) {
    let abs_path = match Path::new(path).canonicalize() {
        Ok(p) => p.to_string_lossy().into_owned(),
        Err(e) => {
            eprintln!("Error when canonicalizing path {}: {}", path, e);
            process::exit(1)
        }
    };

    if PWD.set(abs_path).is_err() {
        eprintln!("PWD has already been set.");
        process::exit(1);
    }
}


/// Returns the current working directory (PWD).
///
/// # Exits
/// * If PWD has not been set yet.
pub fn pwd() -> String {
    match PWD.get() {
        Some(res) => res.clone(),
        None => {
            eprintln!("PWD not set");
            process::exit(1);
        }
    }
}


/// Locates and returns the absolute path to the `.git` directory by searching from the current directory upward.
///
/// # Exits
/// * If no `.git` directory is found in the current or any parent directory.
pub fn get_git_directory() -> String {
    let mut path = PathBuf::from(pwd());

    loop {
        let git_path = path.join(".git");//path.join(".mygit");
        if git_path.exists() {
            return git_path.to_string_lossy().into_owned();
        }
        if !path.pop() {
            eprintln!("Not a git repository (or any of the parent directories)");
            process::exit(1)
        }
    }
}


/// Checks whether one path is a subpath of another.
///
/// # Arguments
/// * `parent` - Potential parent path.
/// * `child` - Potential child path.
///
/// # Returns
/// * `bool` - True if `child` is under `parent`.
pub fn is_subpath(parent: &str, child: &str) -> bool {
    let parent = Path::new(parent).components().collect::<Vec<_>>();
    let child = Path::new(child).components().collect::<Vec<_>>();

    if parent.len() > child.len() {
        return false;
    }

    parent.iter().zip(child.iter()).all(|(a, b)| a == b)
}


/// Returns the relative path from the `parent` to the `child`.
///
/// # Arguments
/// * `parent` - Base path.
/// * `child` - Full path to derive the relative part from.
///
/// # Panics
/// * If `child` is not under `parent`.
pub fn relative_path(parent: &str, child: &str) -> String {
    let parent = Path::new(parent);
    let child = Path::new(child);

    match child.strip_prefix(parent) {
        Ok(rel) => rel.to_string_lossy().into_owned(),
        Err(_) => {
            panic!("child is not under parent");
        }
    }
}


/// Splits a path into the first component and the rest.
///
/// # Arguments
/// * `path` - A string representing a file path.
///
/// # Returns
/// * `(String, String)` - First component and the remaining path.
pub fn split_path_by_first(path: &str) -> (String, String) {
    match path.find('/') {
        Some(pos) => {
            let cur_name = path[..pos].to_string();
            let after_name = path[pos + 1..].to_string();
            (cur_name, after_name)
        }
        None => (path.to_string(), "".to_string()),
    }
}


/// Splits a path into everything except the last component and the last component.
///
/// # Arguments
/// * `path` - A string representing a file path.
///
/// # Returns
/// * `(String, String)` - Parent path and last component.
pub fn split_path_by_last(path: &str) -> (String, String) {
    match path.rfind('/') {
        Some(pos) => {
            let before = path[..pos].to_string();
            let last = path[pos + 1..].to_string();
            (before, last)
        }
        None => ("".to_string(), path.to_string()),
    }
}


/// Returns the current local timestamp formatted as a string.
///
/// Format: `YYYYMMDDHHMMSSmmm` (millisecond precision)
///
/// # Returns
/// * `String` - Formatted timestamp.
pub fn get_time_string() -> String {
    Local::now().format("%Y%m%d%H%M%S%3f").to_string()
}


/// Returns the current user's username from environment variables.
///
/// Tries `USER` first, then `USERNAME`.
///
/// # Returns
/// * `String` - Username or "unknown" if none is found.
pub fn get_username() -> String {
    match env::var("USER")
        .or_else(|_| env::var("USERNAME"))
    {
        Ok(user) => user,
        Err(_) => "unknown".to_string(),
    }
}


/// Returns the relative path from `parent_path` to `path`.
///
/// # Arguments
/// * `parent_path` - Base directory.
/// * `path` - Full path to convert to relative.
///
/// # Exits
/// * If `path` is not under `parent_path`.
pub fn get_relative_path(parent_path: &str, path: &str) -> String {
    let parent = Path::new(parent_path);
    let full = Path::new(path);

    match full.strip_prefix(parent) {
        Ok(rel_path) => rel_path.to_string_lossy().to_string(),
        Err(_) => {
            eprintln!("utils::get_relative_path: not a subpath.");
            process::exit(1);
        }
    }
}