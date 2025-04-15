use std::{io, path::PathBuf, process};

use crate::{core::*, utils::*};

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

pub fn try_get_head(head_name: &str) -> Result<String, io::Error> {
    let ref_path = utils::get_git_directory() + "/refs/heads/" + head_name;
    match storage::read_file(&ref_path) {
        Ok(content) => {
            Ok(std::str::from_utf8(&content).expect("Not valid UTF-8").to_string())
        },
        Err(e) => Err(e)
    }
}

// pub fn get_tag(tag_name: &str) -> String {
//     let ref_path = utils::get_git_directory() + "/refs/tags/" + tag_name;
//     match storage::read_file(&ref_path) {
//         Ok(content) => {
//             std::str::from_utf8(&content).expect("Not valid UTF-8").to_string()
//         },
//         Err(e) => {
//             eprintln!("Error when reading tag file {} : {}", ref_path, e);
//             process::exit(1)
//         }
//     }
// }

// pub fn get_remote(remote_name: &str, ref_name: &str) -> String {
//     let ref_path = utils::get_git_directory() + &format!("/refs/remotes/{}/{}", remote_name, ref_name);
//     match storage::read_file(&ref_path) {
//         Ok(content) => {
//             std::str::from_utf8(&content).expect("Not valid UTF-8").to_string()
//         },
//         Err(e) => {
//             eprintln!("Error when reading remote head file {} : {}", ref_path, e);
//             process::exit(1)
//         }
//     }
// }

pub fn store_head(ref_name: &str, hash: &str) {
    let ref_path = utils::get_git_directory() + "/refs/heads/" + ref_name;
    if let Err(e) = storage::write_file(&ref_path, hash.as_bytes()) {
        eprintln!("Error when writing head file {} : {}", ref_path, e);
        process::exit(1)
    }
}

// pub fn store_tag(ref_name: &str, hash: &str) {
//     let ref_path = utils::get_git_directory() + "/refs/tags/" + ref_name;
//     if let Err(e) = storage::write_file(&ref_path, hash.as_bytes()) {
//         eprintln!("Error when writing tag file {} : {}", ref_path, e);
//         process::exit(1)
//     }
// }

// pub fn store_remote(remote_name: &str, ref_name: &str, hash: &str) {
//     let ref_path = utils::get_git_directory() + &format!("/refs/remotes/{}/{}", remote_name, ref_name);
//     if let Err(e) = storage::write_file(&ref_path, hash.as_bytes()) {
//         eprintln!("Error when writing remote head file {} : {}", ref_path, e);
//         process::exit(1)
//     }
// }

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

// pub fn get_current_commit() -> String {
//     let head_path = utils::get_git_directory() + "/HEAD";
//     match storage::read_file(&head_path) {
//         Ok(content) => {
//             let content = std::str::from_utf8(&content).expect("Not valid UTF-8").to_string();
//             if content.starts_with("ref: refs/heads/") {
//                 return get_head(&content[16..].to_string());
//             }
//             // checks if content is a CMIT_hash
//             match object::get_object_type(&content) {
//                 object::ObjectType::Commit => {
//                     return content;
//                 }
//                 _ => {
//                     eprintln!(".git/HEAD: {} does not refer to a commit.", content);
//                     process::exit(1);
//                 }
//             }
//         },
//         Err(e) => {
//             eprintln!("Error when reading HEAD {} : {}", head_path, e);
//             process::exit(1);
//         }
//     }
// }

pub fn store_current_branch_ref(ref_name: &str) {
    let head_path = utils::get_git_directory() + "/HEAD";
    if let Err(e) = storage::write_text_file(&head_path, &format!("ref: refs/heads/{}", ref_name)) {
        eprintln!("Error writing to HEAD file {} : {}", head_path, e);
        process::exit(1);
    }
}

pub fn store_current_branch_commit(commit_hash: &str) {
    let head_path = utils::get_git_directory() + "/HEAD";
    if let Err(e) = storage::write_text_file(&head_path, commit_hash) {
        eprintln!("Error writing to HEAD file {} : {}", head_path, e);
        process::exit(1);
    }
}

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

pub fn create_head(head_name: &str, content: &str) {
    let ref_path = utils::get_git_directory() + "/refs/heads/" + head_name;
    storage::create_nonexist_file(&ref_path);
    if let Err(e) = storage::write_text_file(&ref_path, content) {
        eprintln!("Error writing to {} : {}", ref_path, e);
        process::exit(1);
    }
}

pub fn is_prev_branch(prev_branch: &str, post_branch: &str) -> bool {
    let prev_commit = get_head(prev_branch);
    let post_commit = get_head(post_branch);
    commit::is_prev_commit(&prev_commit, &post_commit)
}