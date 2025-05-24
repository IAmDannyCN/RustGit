//! Module: status
//!
//! Provides functionality to compare the index against the latest commit and display a human-readable status.
//! Shows which files have been added, removed, or modified since the last commit.

use std::collections::HashMap;
use std::collections::HashSet;

use crate::{core::*, utils::*};
use crate::core::index::IndexEntry;
use crate::core::commit::{Commit, CommitTrait};


/// Compares the given index entries with the contents of a specific commit and returns differences.
///
/// # Arguments
/// * `entries` - A set of index entries currently in the staging area.
/// * `commit_hash` - The hash of the commit to compare against.
///
/// # Returns
/// A tuple containing:
/// * Set of files added in the index.
/// * Set of files removed compared to the commit.
/// * Set of files modified (with both old and new hashes).
pub fn diff_index_entries_to_commit(entries: &HashSet<IndexEntry>, commit_hash: &str) ->
    (HashSet<IndexEntry>, HashSet<IndexEntry>, HashSet<(IndexEntry, IndexEntry)>)
{

    // let current_branch = reference::get_current_branch()?.to_string();

    // let commit_hash: String = reference::get_head(&current_branch);
    let mut commit = Commit { hash: Some(commit_hash.to_string()), data: None };
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

    let mut add_log: HashSet<IndexEntry> = Default::default();
    let mut remove_log: HashSet<IndexEntry> = Default::default();
    let mut modify_log: HashSet<(IndexEntry, IndexEntry)> = Default::default();

    let repo_path = utils::pwd();

    // entry.path and commit_entry.path are all RELATIVE PATHS
    for entry in entries {
        let path = &entry.path;
        let abs_path = format!("{}/{}", repo_path, path);
        match commit_entries.get(&abs_path) {
            Some(commit_entry) => {
                if commit_entry.hash != entry.hash {
                    modify_log.insert((commit_entry.clone(), entry.clone()));
                }
            }
            None => {
                add_log.insert(entry.clone());
            }
        }
    }

    let mut index_entry_paths: HashSet<String> = Default::default();
    for entry in entries {
        index_entry_paths.insert(entry.path.clone());
    }
    for commit_entry_kv in &commit_entries {
        let commit_entry = commit_entry_kv.1;
        if let None = index_entry_paths.get(&commit_entry.path) {
            let log = IndexEntry {
                path: utils::get_relative_path(&repo_path, commit_entry_kv.0),
                hash: commit_entry.hash.clone()
            };
            remove_log.insert(log);
        }
    }

    (add_log, remove_log, modify_log)
}


/// Displays the current status of the working directory and index compared to the last commit.
///
/// Shows:
/// - Files added to the index
/// - Files removed since the last commit
/// - Files modified (with old and new blob hashes)
///
/// Outputs the result to stderr with colored labels for better readability.
pub fn status() {
    let index = index::read_index();

    let mut entries: HashSet<IndexEntry> = Default::default();
    for kv in &index {
        entries.insert(kv.1.clone());
    }

    let commit_hash = reference::get_current_commit();

    let (add_log,
        remove_log,
        modify_log) = diff_index_entries_to_commit(&entries, &commit_hash);
    // else {
    //     println!("You are in 'detached HEAD' state.");
    //     println!("Staging area now contains:");
    //     for entry in entries {
    //         println!("    {}", entry.path);
    //     }
    //     process::exit(0);
    // };

    eprintln!(
        "Added {} file(s), Removed {} file(s), Modified {} file(s).", 
        add_log.len(), remove_log.len(), modify_log.len()
    );
    if !(add_log.len() == 0 && remove_log.len() == 0 && modify_log.len() == 0) {
        eprintln!();
        for entry in &add_log {
            eprintln!("    \x1b[32mAdd:\x1b[0m    {} ({})", entry.path, entry.hash);
        }
        for entry in &remove_log {
            eprintln!("    \x1b[31mRemove:\x1b[0m {} ({})", entry.path, entry.hash);
        }
        for entry in &modify_log {
            eprintln!("    \x1b[33mModify:\x1b[0m {} ({} -> {})", entry.0.path, entry.0.hash, entry.1.hash);
        }
        eprintln!();
    }

}