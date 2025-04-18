use std::collections::HashMap;
use std::{collections::HashSet, process};

use crate::{core::*, utils::*};
use crate::core::index::IndexEntry;
use crate::core::commit::{Commit, CommitTrait};

pub fn status() {
    let index = index::read_index();
    let mut entries: HashSet<IndexEntry> = Default::default();
    for kv in index {
        entries.insert(kv.1);
    }

    let current_branch: String;

    let mut add_log: HashSet<String> = Default::default();
    let mut remove_log: HashSet<String> = Default::default();
    let mut modify_log: HashSet<String> = Default::default();

    match &reference::get_current_branch() {
        None => {
            println!("You are in 'detached HEAD' state.");
            println!("Staging area now contains:");
            for entry in entries {
                println!("    {}", entry.path);
            }
            process::exit(0);
        }
        Some(branch_name) => {
            current_branch = branch_name.to_string();
        }
    }

    let repo_path = utils::pwd();

    let commit_hash: String = reference::get_head(&current_branch);
    let mut commit = Commit { hash: Some(commit_hash), data: None };
    commit.read_commit();

    let commit_root_hash = commit.data.unwrap().tree_hash;

    let mut commit_entries: HashMap<String, IndexEntry> = Default::default();

    storage::restore_index_by_tree(&commit_root_hash, &utils::pwd(), &mut commit_entries);

    // println!("commit_entries");
    // for commit_entry in &commit_entries {
    //     println!("{} -> {} : {}", commit_entry.0, commit_entry.1.path, commit_entry.1.hash);
    // }
    // println!("(index) entries");
    // for entry in &entries {
    //     println!("{} : {}", entry.hash, entry.path);
    // }

    // entry.path and commit_entry.path are all RELATIVE PATHS
    for entry in &entries {
        let path = &entry.path;
        let abs_path = format!("{}/{}", repo_path, path);
        match commit_entries.get(&abs_path) {
            Some(commit_entry) => {
                if commit_entry.hash != entry.hash {
                    modify_log.insert(path.clone());
                }
            }
            None => {
                add_log.insert(path.clone());
            }
        }
    }

    let mut index_entry_paths: HashSet<String> = Default::default();
    for entry in &entries {
        index_entry_paths.insert(entry.path.clone());
    }
    for commit_entry_kv in &commit_entries {
        if let None = index_entry_paths.get(&commit_entry_kv.1.path) {
            remove_log.insert(utils::get_relative_path(&repo_path, commit_entry_kv.0));
        }
    }
    
    print!("Add files   : ");
    for file in add_log {
        print!("{} ", file);
    } println!();

    print!("Remove files: ");
    for file in remove_log {
        print!("{} ", file);
    } println!();

    print!("Modify files: ");
    for file in modify_log {
        print!("{} ", file);
    } println!();

}