use std::process;

use crate::utils::*;

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

pub fn get_tag(tag_name: &str) -> String {
    let ref_path = utils::get_git_directory() + "/refs/tags/" + tag_name;
    match storage::read_file(&ref_path) {
        Ok(content) => {
            std::str::from_utf8(&content).expect("Not valid UTF-8").to_string()
        },
        Err(e) => {
            eprintln!("Error when reading tag file {} : {}", ref_path, e);
            process::exit(1)
        }
    }
}

pub fn get_remote(remote_name: &str, ref_name: &str) -> String {
    let ref_path = utils::get_git_directory() + &format!("/refs/remotes/{}/{}", remote_name, ref_name);
    match storage::read_file(&ref_path) {
        Ok(content) => {
            std::str::from_utf8(&content).expect("Not valid UTF-8").to_string()
        },
        Err(e) => {
            eprintln!("Error when reading remote head file {} : {}", ref_path, e);
            process::exit(1)
        }
    }
}

pub fn store_head(ref_name: &str, hash: &str) {
    let ref_path = utils::get_git_directory() + "/refs/heads/" + ref_name;
    if let Err(e) = storage::write_file(&ref_path, hash.as_bytes()) {
        eprintln!("Error when writing head file {} : {}", ref_path, e);
        process::exit(1)
    }
}

pub fn store_tag(ref_name: &str, hash: &str) {
    let ref_path = utils::get_git_directory() + "/refs/tags/" + ref_name;
    if let Err(e) = storage::write_file(&ref_path, hash.as_bytes()) {
        eprintln!("Error when writing tag file {} : {}", ref_path, e);
        process::exit(1)
    }
}

pub fn store_remote(remote_name: &str, ref_name: &str, hash: &str) {
    let ref_path = utils::get_git_directory() + &format!("/refs/remotes/{}/{}", remote_name, ref_name);
    if let Err(e) = storage::write_file(&ref_path, hash.as_bytes()) {
        eprintln!("Error when writing remote head file {} : {}", ref_path, e);
        process::exit(1)
    }
}