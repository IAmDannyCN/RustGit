use std::{collections::{HashMap, HashSet}, path::Path, process};

use crate::{core::{commit::{Commit, CommitTrait}, index::IndexEntry, tree::*, *}, utils::*};
use crate::commands::commit::commit;

fn register_blob(
    tree_hash: &str,
    tree_path: &str,
    blob_table: &mut HashMap<String, tree::TreeEntry>,
    success: &mut bool
) {
    let mut tree = Tree { hash: Some(tree_hash.to_owned()), data: None};
    tree.read_tree();

    let entries = tree.data.as_ref().unwrap();
    for entry in entries {
        let entry_path = format!("{}/{}", tree_path, entry.name);
        match entry.entry_type {
            TreeEntryType::Tree => {
                register_blob(&entry.hash, &entry_path, blob_table, success);
            }
            _ => {
                // This is a blob
                if let Some(stored_entry) = blob_table.get(&entry_path) {
                    if stored_entry != entry {
                        eprintln!("Conflict deteced on {}.", entry_path);
                        *success = false;
                    }
                } else {
                    blob_table.insert(entry_path, entry.clone());
                }
            }
        }
    }
}

fn register_blob_by_branch(branch_name: &str, blob_table: &mut HashMap<String, tree::TreeEntry>) {
    let commit_hash = reference::get_head(&branch_name);
    let mut commit = Commit { hash: Some(commit_hash), data: None };
    commit.read_commit();
    let root_hash = commit.data.unwrap().tree_hash;

    let mut success: bool = true;
    register_blob(&root_hash, &utils::pwd(), blob_table, &mut success);
    if !success {
        eprintln!("Failed to merge. Nothing changed.");
        process::exit(1);
    }
}

pub fn merge(merge_branch: String) {

    if commit::check_has_uncommitted() {
        eprintln!("Detected uncommited files. Cannot merge.");
        process::exit(1);
    }

    let current_branch: String;

    match &reference::get_current_branch() {
        None => {
            eprintln!("You are in 'detached HEAD' state. Cannot commit.");
            process::exit(1);
        }
        Some(branch_name) => {
            current_branch = branch_name.to_string();
        }
    }

    if reference::is_prev_branch(&merge_branch, &current_branch) {
        println!("Already up to date.");
        process::exit(0);
    } else if reference::is_prev_branch(&current_branch, &merge_branch) {
        reference::store_head(&current_branch, &reference::get_head(&merge_branch));
        println!("Merged branch {} to {} by Fast-Forward Policy.", current_branch, merge_branch);

        storage::clear_working_area();
        storage::restore_working_area(&reference::get_head(&current_branch));

        process::exit(0);
    }

    // Need to produce a new commit
    let mut blob_table: HashMap<String, tree::TreeEntry> = Default::default();

    register_blob_by_branch(&current_branch, &mut blob_table);
    register_blob_by_branch(&merge_branch, &mut blob_table);

    // blob_table now stores every entry.

    // restore working area
    storage::clear_working_area();
    storage::restore_working_area(&reference::get_head(&current_branch));
    storage::restore_working_area(&reference::get_head(&merge_branch));

    // generate new commit
    let mut index_entries: HashSet<IndexEntry> = Default::default();
    
    for (blob_path, blob_entry) in blob_table {
        let repo_path = utils::pwd();
        assert!(blob_path.starts_with(&repo_path));
        let rel_path = Path::new(&blob_path)
            .strip_prefix(&repo_path)
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        index_entries.insert(IndexEntry { path: rel_path, hash: blob_entry.hash });
    }

    let parent_commits = [
        reference::get_head(&current_branch),
        reference::get_head(&merge_branch)
    ];

    let new_head_hash = commit(
        &index_entries,
        format!("Merge branch {} and {}.", current_branch, merge_branch),
        utils::get_time_string(),
        utils::get_username(),
        parent_commits.to_vec()
    );

    reference::store_head(&current_branch, &new_head_hash);

    storage::clear_index();

    println!("Merged branches {} and {}", current_branch, merge_branch);
    
}
