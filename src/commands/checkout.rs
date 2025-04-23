use std::process;

use crate::{core::*, utils::*};

fn checkout_to_commit(target_commit_hash: &str, force: bool) {
    if !force && commit::check_has_uncommitted() {
        eprintln!("Detected uncommited files. Cannot checkout.");
        process::exit(1);
    }
    storage::clear_working_area();
    if target_commit_hash != "" {
        storage::restore_working_area(target_commit_hash);
    }
}

pub fn checkout(target: String, force: bool) {
    match reference::try_get_head(&target) {
        Ok(head_hash) => {
            // target is a head, e.g. target == master
            checkout_to_commit(&head_hash, force);
            reference::store_current_branch_ref(&target);
        }
        Err(_) => {
            // target is not a head
            match object::get_object_type(&target) {
                object::ObjectType::Commit => {
                    // target is a commit, e.g. target== s65df41d6sf...
                    checkout_to_commit(&target, force);
                    reference::store_current_branch_commit(&target);
                }
                _ => {
                    eprintln!("Unrecognized checkout target {}", target);
                }
            }
        }
    }
}
