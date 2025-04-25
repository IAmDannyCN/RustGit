// use std::collections::HashMap;
// use std::{collections::HashSet, process};

use std::collections::{HashSet, VecDeque};
use std::process;
use crate::core::*;
use crate::core::commit::{Commit, CommitTrait};
use crate::core::commit::CommitData;

pub fn log() {
    // get the current branch name
    let current_branch = match reference::get_current_branch() {
        Some(branch) => branch,
        None => {
            eprintln!("Not on any branch (detached HEAD state)");
            process::exit(1);
        }
    };

    // get the commit history for the current branch
    let initial_commit_hash = reference::get_head(&current_branch);
    
    eprintln!("Commit history for branch '{}':", current_branch);
    eprintln!("--------------------------------");

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

fn print_commit(hash: &str, commit_data: &CommitData) {
    eprintln!("commit {}", hash);
    
    // display the parent commits
    if commit_data.parent_commits.len() > 1 {
        eprint!("Merge:");
        for parent in &commit_data.parent_commits {
            print!(" {}", &parent[..7]);
        }
        eprintln!();
    }
    
    eprintln!("Author: {}", commit_data.user);
    eprintln!("Date:   {}", commit_data.time);
    eprintln!();
    eprintln!("    {}", commit_data.message);
    eprintln!();
    eprintln!("--------------------------------");
}

// fn is_prev_commit_search(
//     prev_commit_hash: &str,
//     post_commit_hash: &str,
//     searched_commits: &mut HashSet<String>
// ) -> bool {
//     if prev_commit_hash == post_commit_hash {
//         return true;
//     } else if prev_commit_hash == "" {
//         return true;
//     } else if post_commit_hash == "" {
//         return false;
//     }

//     if let Some(_) = searched_commits.get(post_commit_hash) {
//         return false;
//     }
//     searched_commits.insert(post_commit_hash.to_string());
    
//     // prev != post, and both != ""
//     let mut post_commit = Commit { hash: Some(post_commit_hash.to_string()), data: None };
//     post_commit.read_commit();

//     for parent_commit_hash in post_commit.data.unwrap().parent_commits {
//         if is_prev_commit_search(prev_commit_hash, &parent_commit_hash, searched_commits) {
//             return true;
//         }
//     }

//     false
// }