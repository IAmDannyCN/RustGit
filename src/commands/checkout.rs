use std::{collections::HashMap, process};

use super::branch;
use crate::{core::{commit::{Commit, CommitTrait}, index::IndexEntry, *}, utils::*};

fn checkout_to_commit(target_commit_hash: &str, force: bool) {
    if !force && commit::check_has_uncommitted() {
        eprintln!("Detected uncommited files. Cannot checkout.");
        eprintln!("Use `git checkout --force/-f` to force checkout.");
        process::exit(1);
    }
    storage::clear_working_area();
    if target_commit_hash != "" {
        storage::restore_working_area(target_commit_hash);
    }

    let mut commit = Commit {
        hash: Some(target_commit_hash.to_string()),
        data: None
    };
    commit.read_commit();

    let mut index_entries: HashMap<String, IndexEntry> = Default::default();
    storage::restore_index_by_tree(&commit.data.unwrap().tree_hash, &utils::pwd(), &mut index_entries);
    index::write_index(&index_entries);
}

pub fn checkout(target: String, force: bool, branch: bool, verbose: bool) {

    if branch {
        branch::branch(Some([target.clone()].to_vec()), false, verbose);
    }

    match reference::try_get_head(&target) {
        Ok(head_hash) => {
            // target is a head, e.g. target == master
            checkout_to_commit(&head_hash, force);
            reference::store_current_branch_ref(&target);
            if verbose {
                eprintln!("Now on head {}.", target);
            }
        }
        Err(_) => {
            // target is not a head
            match object::get_object_type(&target) {
                object::ObjectType::Commit => {
                    // target is a commit, e.g. target == s65df41d6sf...
                    checkout_to_commit(&target, force);
                    reference::store_current_branch_commit(&target);
                    if verbose {
                        eprintln!("Now in 'detached HEAD' state on {}.", target);
                    }
                }
                _ => {
                    eprintln!("Unrecognized checkout target {}.", target);
                }
            }
        }
    }
}
