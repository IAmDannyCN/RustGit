use std::path::{Path, PathBuf};
use std::process;
use std::sync::OnceLock;

static PWD: OnceLock<String> = OnceLock::new();

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

pub fn pwd() -> String {
    match PWD.get() {
        Some(res) => res.clone(),
        None => {
            eprintln!("PWD not set");
            process::exit(1);
        }
    }
}

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

pub fn is_subpath(parent: &str, child: &str) -> bool {
    let parent = match Path::new(parent).canonicalize() {
        Ok(p) => p,
        Err(_) => return false,
    };

    let child = match Path::new(child).canonicalize() {
        Ok(p) => p,
        Err(_) => return false,
    };

    child.starts_with(&parent)
}

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