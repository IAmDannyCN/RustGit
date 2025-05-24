//! Module: checkout
//!
//! Implements the checkout functionality for switching between branches or commits.
//! Supports both branch checkout and detached HEAD state, with optional force flag to override uncommitted changes.

use std::{collections::HashMap, process};

use super::branch;
use crate::{core::{commit::{Commit, CommitTrait}, index::IndexEntry, *}, utils::*};


/// Switches to the specified branch or commit.
///
/// # Arguments
/// * `target` - Name of the branch or SHA-1 hash of the commit to switch to.
/// * `force` - If true, allows checkout even if there are uncommitted changes.
/// * `branch` - If true, creates a new branch with the given target name.
/// * `verbose` - If true, displays beautified output instead of minimal status.
///
/// # Behavior
/// 1. Optionally creates a new branch if the `branch` flag is set.
/// 2. Checks whether the target is a valid branch or commit.
/// 3. Clears the working directory and restores it from the target commit.
/// 4. Updates `.git/HEAD` to reflect the current branch or commit.
///
/// # Exits
/// * If there are uncommitted changes and `force` is false.
/// * If the target is neither a valid branch nor a valid commit.
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


/// Restores the working area and index from the specified commit.
///
/// # Arguments
/// * `target_commit_hash` - SHA-1 hash of the commit to restore.
/// * `force` - If true, ignores uncommitted changes in the working area.
///
/// # Exits
/// * If there are uncommitted changes and `force` is false.
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
