use std::process;

use crate::{core::reference, utils::{storage, utils}};

pub fn branch(name: Option<Vec<String>>, delete: bool) {
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
            println!("Created branch {}.", name);
        }
        (None, _) => {
            // Listing branches
            let heads = reference::get_all_heads();
            let current_branch = reference::get_current_branch();
            for head in heads {
                let is_current_branch = match &current_branch {
                    None => true,
                    Some(branch_name) => branch_name == &head,
                };
                println!(" {} {}", if is_current_branch { "*" } else { " " }, head);
            }
        }
    }
}
