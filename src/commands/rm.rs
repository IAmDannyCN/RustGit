use std::collections::HashSet;
use std::path::Path;
use std::process;

use crate::{core::*, utils::*};
use crate::core::index::IndexEntry;


pub fn remove(files: Vec<String>, recursive: bool, force: bool) {
    let mut index = index::read_index();
    let repo_path = utils::pwd();

    let mut rm_entries: HashSet<IndexEntry> = Default::default();

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

        index::register_files(&file_path, &utils::relative_path(&repo_path, &file_path), &mut rm_entries, &recursive);
    }

    let mut remove_log: HashSet<String> = Default::default();

    for entry in rm_entries {
        let path = entry.path.clone();
        if let Some(_) = index.get(&path) {
            remove_log.insert(path);
        }
    }

    for path in &remove_log {
        index.remove(path);
        if force {
            storage::remove_file(path);
        }
    }

    index::write_index(&index);

    print!("remove files: ");
    for file in remove_log {
        print!("{} ", file);
    } println!();

}
