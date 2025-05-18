use std::process;

use crate::utils::*;

pub enum ObjectType {
    Blob,
    Tree,
    Commit,
}

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