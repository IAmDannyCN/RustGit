use std::collections::HashSet;
use std::process;
use std::{fs, path::Path};

use crate::core::blob::BlobTrait;
use crate::{core::*, utils::*};
use crate::core::index::IndexEntry;

fn register_files(path: &str, rel_path: &str, index: &mut HashSet<IndexEntry>) {

    if &path == &utils::get_git_directory() {
        return ;
    }

    let path = Path::new(path);

    if let Ok(metadata) = fs::symlink_metadata(path) {
        if metadata.file_type().is_symlink() {
            return;
        } else if metadata.is_file() {
            index.insert(
                IndexEntry {
                path: rel_path.to_string(),
                hash: String::default(),
                }
            );
        } else if metadata.is_dir() {
            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries.flatten() {
                    let file_name = entry.file_name();
                    let file_name_str = file_name.to_string_lossy();
                    let new_abs_path = entry.path();
                    let new_rel_path = if rel_path.is_empty() {
                        file_name_str.to_string()
                    } else {
                        format!("{}/{}", rel_path, file_name_str)
                    };
                    register_files(new_abs_path.to_str().unwrap(), &new_rel_path, index);
                }
            }
        }
    }
}


pub fn add(files: Vec<String>) {
    
    let mut index = index::read_index();
    let repo_path = utils::pwd();

    let mut new_entries: HashSet<IndexEntry> = Default::default();
    let mut base_entries: HashSet<String> = Default::default();
    
    for file in files {
        let file_path = match Path::new(&file).canonicalize() {
            Ok(p) => p.to_string_lossy().into_owned(),
            Err(e) => {
                eprintln!("Error when canonicalizing path {}: {}", file, e);
                process::exit(1)
            }
        };
        if !utils::is_subpath(&repo_path, &file_path) {
            eprintln!("File {} does not belong to repository {}.", file_path, repo_path);
            process::exit(1);
        }

        register_files(&file_path, &utils::relative_path(&repo_path, &file_path), &mut new_entries);
        base_entries.insert(file_path);
    }

    // After registration, ALL entry.hash ARE EMPTY in new_entries !!!

    let mut add_log: HashSet<String> = Default::default();
    let mut remove_log: HashSet<String> = Default::default();
    let mut modify_log: HashSet<String> = Default::default();

    for entry in &new_entries  {

        // entry.hash is empty !!!

        let file_path = format!("{}/{}", utils::pwd(), entry.path);
        let mut blob = blob::get_blob_from_file(&file_path);
        
        let path = entry.path.clone();
        let hash = blob.hash.clone().unwrap();

        match index.get(&path) {
            None => {
                // A new entry
                add_log.insert(path.clone());

                let new_entry = IndexEntry { path: path.clone(), hash: hash.clone() };

                index.insert(path, new_entry);
                blob.write_blob();
                
                continue;
            }
            Some(old_entry) => {
                // An old entry
                if old_entry.hash == hash {
                    continue;
                } else {
                    modify_log.insert(path.clone());
                    
                    let new_entry = IndexEntry { path: path.clone(), hash: hash.clone() };

                    index.remove(&path);
                    index.insert(path, new_entry);
                    blob.write_blob();
                    
                    continue;
                }
            }
        }
    }

    let mut new_entry_paths: HashSet<String> = Default::default();
    for entry in &new_entries {
        new_entry_paths.insert(entry.path.clone());
    }

    index.retain(|_, old_entry| {
        if !new_entry_paths.contains(&old_entry.path) {
            for base_entry in &base_entries {
                let old_entry_full = format!("{}/{}", utils::pwd(), &old_entry.path);
                if utils::is_subpath(&base_entry, &old_entry_full) {
                    remove_log.insert(old_entry.path.clone());
                    return false;
                }
            }
            true
        } else {
            true
        }
    });

    index::write_index(&index);

    print!("add files: ");
    for file in add_log {
        print!("{} ", file);
    } println!();

    print!("remove files: ");
    for file in remove_log {
        print!("{} ", file);
    } println!();

    print!("modify files: ");
    for file in modify_log {
        print!("{} ", file);
    } println!();
    
}
