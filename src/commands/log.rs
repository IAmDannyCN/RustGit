//! Module: log
//!
//! Implements the commit history viewer, displaying a formatted list of commits
//! starting from the current branch head and following parent links.

use std::collections::{HashSet, VecDeque};
use std::process;
use crate::core::*;
use crate::core::commit::{Commit, CommitTrait};
use crate::core::commit::CommitData;


/// Displays the commit history of the current branch in a human-readable format.
///
/// # Exits
/// * If the repository is in a detached HEAD state (not on any branch).
///
/// # Output Format
/// For each commit:
/// - Commit hash (shortened)
/// - Merge parents (if any)
/// - Author name
/// - Commit timestamp
/// - Commit message
///
/// Uses ANSI color codes for better visual clarity.
pub fn log() {
    // get the current branch name
    let current_branch = match reference::get_current_branch() {
        Some(branch) => branch,
        None => {
            eprintln!("\x1b[31mError:\x1b[0m Not on any branch (detached HEAD state)");
            process::exit(1);
        }
    };

    // get the commit history for the current branch
    let initial_commit_hash = reference::get_head(&current_branch);
    
    eprintln!("\x1b[1mCommit history for branch '\x1b[32m{}\x1b[0m\x1b[1m':\x1b[0m", current_branch);
    eprintln!("\x1b[90m--------------------------------\x1b[0m");

    // get the commit history for the current branch
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(initial_commit_hash.clone());
    visited.insert(initial_commit_hash.clone());

    while let Some(commit_hash) = queue.pop_front() {
        if commit_hash.is_empty() {
            continue;
        }

        let mut commit = Commit {
            hash: Some(commit_hash.clone()),
            data: None,
        };
        commit.read_commit();
        
        let commit_data = commit.data.unwrap();
        
        print_commit(&commit.hash.unwrap(), &commit_data);
        
        // handle merge commits
        for parent in commit_data.parent_commits {
            if !visited.contains(&parent) {
                visited.insert(parent.clone());
                queue.push_back(parent);
            }
        }
    }
}


/// Prints detailed information about a single commit to standard error.
///
/// # Arguments
/// * `hash` - SHA-1 hash of the commit.
/// * `commit_data` - Parsed data of the commit object.
///
/// Includes support for displaying merge commit parent hashes with colored formatting.
fn print_commit(hash: &str, commit_data: &CommitData) {
    eprintln!("\x1b[33mcommit {}\x1b[0m", &hash[..7]);
    
    // display the parent commits
    if commit_data.parent_commits.len() > 1 {
        eprint!("\x1b[35mMerge:\x1b[0m");
        for parent in &commit_data.parent_commits {
            eprint!(" \x1b[36m{}\x1b[0m", &parent[..7]);
        }
        eprintln!();
    }
    
    eprintln!("\x1b[34mAuthor:\x1b[0m {}", commit_data.user);
    eprintln!("\x1b[34mDate:  \x1b[0m {}", commit_data.time);
    eprintln!();
    for line in commit_data.message.lines() {
        eprintln!("    \x1b[1m{}\x1b[0m", line);
    }
    eprintln!();
    eprintln!("\x1b[90m--------------------------------\x1b[0m");
}