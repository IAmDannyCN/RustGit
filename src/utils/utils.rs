use std::path::{Path, PathBuf};
use std::{env, process};
use std::sync::OnceLock;

use chrono::Local;

static PWD: OnceLock<String> = OnceLock::new();

/// Sets the PWD environment variable to the absolute path of the given directory.
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

/// Returns the current working directory (PWD) as a String.
pub fn pwd() -> String {
    match PWD.get() {
        Some(res) => res.clone(),
        None => {
            eprintln!("PWD not set");
            process::exit(1);
        }
    }
}

/// Returns the absolute path of the current working directory.
pub fn get_git_directory() -> String {
    let mut path = PathBuf::from(pwd());

    loop {
        let git_path = path.join(".mygit");
        if git_path.exists() {
            return git_path.to_string_lossy().into_owned();
        }
        if !path.pop() {
            eprintln!("Not a git repository (or any of the parent directories)");
            process::exit(1)
        }
    }
}

// pub fn is_subpath(parent: &str, child: &str) -> bool {
//     let parent = match Path::new(parent).canonicalize() {
//         Ok(p) => p,
//         Err(_) => return false,
//     };

//     let child = match Path::new(child).canonicalize() {
//         Ok(p) => p,
//         Err(_) => return false,
//     };
//     child.starts_with(&parent)
// }

/// Checks if the child path is a subpath of the parent path.
pub fn is_subpath(parent: &str, child: &str) -> bool {
    let parent = Path::new(parent).components().collect::<Vec<_>>();
    let child = Path::new(child).components().collect::<Vec<_>>();

    if parent.len() > child.len() {
        return false;
    }

    parent.iter().zip(child.iter()).all(|(a, b)| a == b)
}

/// Returns the relative path from the parent to the child path.
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

// pub fn get_dir_name(path: &str) -> String {
//     Path::new(path)
//         .parent()
//         .map(|p| p.to_string_lossy().to_string())
//         .unwrap_or_else(|| "".to_string())
// }

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

pub fn get_time_string() -> String {
    Local::now().format("%Y%m%d%H%M%S%3f").to_string()
}

pub fn get_username() -> String {
    match env::var("USER")
        .or_else(|_| env::var("USERNAME"))
    {
        Ok(user) => user,
        Err(_) => "unknown".to_string(),
    }
}

/// Returns the relative path from the parent path to the full path.
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