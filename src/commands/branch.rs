//! Module: branch
//!
//! Implements branch management functionality, including:
//! - Creating new branches
//! - Deleting existing branches
//! - Listing all available branches
//!
//! Ensures safety by preventing deletion of the current branch or non-ancestor branches.

use std::process;

use crate::{core::reference, utils::{storage, utils}};


/// Manages Git branches: create, delete, or list branches based on input arguments.
///
/// # Arguments
/// * `name` - Optional vector containing names for creating or operating on branches.
/// * `delete` - If true, deletes the specified branches.
/// * `verbose` - If true, displays beautified output instead of minimal status.
///
/// # Behavior
/// 1. **Create mode**: Creates a new branch pointing to the current commit.
///    - Requires exactly one name in `name`.
///    - Fails if in detached HEAD state.
/// 2. **Delete mode**: Deletes the specified branches.
///    - Prevents deleting the current branch.
///    - Only allows deletion of branches that are direct ancestors of the current branch.
/// 3. **List mode**: Lists all branches with an asterisk next to the current branch.
///
/// # Exits
/// - If trying to delete current branch.
/// - If trying to delete non-ancestor branch.
/// - If trying to create branch while in detached HEAD state.
/// - If trying to create more than one branch at once.
pub fn branch(name: Option<Vec<String>>, delete: bool, verbose: bool) {
    match (name, delete) {
        (Some(branches), true) => {
            // Deleting branch(es)
            let current_branch: String;

            match &reference::get_current_branch() {
                None => {
                    eprintln!("You are in 'detached HEAD' state. Cannot delete branch.");
                    process::exit(1);
                }
                Some(branch_name) => {
                    current_branch = branch_name.to_string();
                }
            }
            
            let mut can_delete = true;

            for branch in &branches {
                if !reference::is_prev_branch(branch, &current_branch) {
                    eprintln!("branch {} is not direct ancestor of current branch {}.", branch, current_branch);
                    can_delete = false;
                } else if branch == &current_branch {
                    eprintln!("Cannot delete current branch {}.", current_branch);
                    can_delete = false;
                }
            }

            if !can_delete {
                eprintln!("Did not remove any branch.");
                process::exit(1);
            }

            for branch in &branches {
                storage::remove_file(&format!("{}/refs/heads/{}", utils::get_git_directory(), branch));
            }

            if verbose {
                for branch in &branches {
                    eprintln!("Removed branch `{}`.", branch);
                }
            }

        }
        (Some(branch), false) => {
            // Creating branch
            if branch.len() != 1 {
                eprintln!("Can only create 1 branch.");
                process::exit(1);
            }
            let name = branch.get(0).unwrap();

            let current_branch: String;

            match &reference::get_current_branch() {
                None => {
                    eprintln!("You are in 'detached HEAD' state. Cannot create branch.");
                    process::exit(1);
                }
                Some(branch_name) => {
                    current_branch = branch_name.to_string();
                }
            }

            reference::create_head(name, &reference::get_head(&current_branch));
            
            if verbose {
                eprintln!("Created branch {}.", name);
            }
        }
        (None, _) => {
            // Listing branches
            let heads = reference::get_all_heads();
            let current_branch = reference::get_current_branch();
            for head in heads {
                let is_current_branch = match &current_branch {
                    None => false,
                    Some(branch_name) => branch_name == &head,
                };
                eprintln!(" {} {}\x1b[0m", if is_current_branch { "\x1b[32m*" } else { " " }, head);
            }
        }
    }
}
