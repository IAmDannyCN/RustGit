use std::process;

use crate::utils::*;
use super::{blob::*, tree::*, commit::*};

pub enum ObjectType {
    Blob(Blob),
    Tree(Tree),
    Commit(Commit),
}

pub fn get_object_path(object_name: &str) -> String {
    assert!((&object_name).len() == hash::HASH_LENGTH);
    let git_directory = utils::get_git_directory();
    let file_path = git_directory.clone() + 
                            "/objects/" + &object_name[..hash::FOLDER_LENGTH] +
                            "/" + &object_name[hash::FOLDER_LENGTH..];
    file_path
}

// pub fn get_object_folder(object_name: &str) -> String {
//     assert!((&object_name).len() == hash::HASH_LENGTH);
//     let git_directory = utils::get_git_directory();
//     let folder_path = git_directory.clone() + 
//                             "/" + &object_name[..hash::FOLDER_LENGTH];
//     folder_path
// }

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

pub fn write_object_file(object_name: &str, content: &str) {

    let file_path = get_object_path(&object_name);
    // let folder_path = get_object_folder(&object_name);
    
    match storage::write_text_file(&file_path, content) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Failed to write object '{}': {}", object_name, e);
            process::exit(1)
        }
    }
}