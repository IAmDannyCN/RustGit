use std::{collections::{HashMap, HashSet}, path::Path, process, str};

use crate::{commands::commit::commit_merge, core::{blob::{Blob, BlobTrait}, commit::{Commit, CommitTrait}, index::IndexEntry, tree::*, *}, utils::*};

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

fn register_blob_by_commit(commit_hash: &str, blob_table: &mut HashMap<String, TreeEntry>) {
    let mut commit = Commit { hash: Some(commit_hash.to_owned()), data: None };
    commit.read_commit();
    let root_hash = commit.data.unwrap().tree_hash;

    let mut success: bool = true;
    register_blob(&root_hash, &utils::pwd(), blob_table, &mut success);
    if !success {
        eprintln!("Failed to merge. Nothing changed.");
        process::exit(1);
    }
}

// fn register_blob_by_branch(branch_name: &str, blob_table: &mut HashMap<String, TreeEntry>) {
//     let commit_hash = reference::get_head(&branch_name);
//     register_blob_by_commit(&commit_hash, blob_table);
// }

fn analyse_merge_conflict(path: &str, entry1: &TreeEntry, entry2: &TreeEntry) {
    let mut blob1 = Blob { hash: Some(entry1.hash.clone()), data: None };
    let mut blob2 = Blob { hash: Some(entry2.hash.clone()), data: None };
    blob1.read_blob();
    blob2.read_blob();
    let data1: Vec<u8> = blob1.data.unwrap();
    let data2: Vec<u8> = blob2.data.unwrap();

    let Ok(text1) = str::from_utf8(&data1) else {
        eprintln!("Merge conflicit in {}", path);
        return;
    };
    let Ok(text2) = str::from_utf8(&data2) else {
        eprintln!("Merge conflicit in {}", path);
        return;
    };

    let lines1: Vec<_> = text1.lines().collect();
    let lines2: Vec<_> = text2.lines().collect();
    let max_len = lines1.len().max(lines2.len());

    let mut conflict_ranges = vec![];
    let mut in_conflict = false;
    let mut start = 0;

    for i in 0..max_len {
        let line1 = lines1.get(i).unwrap_or(&"");
        let line2 = lines2.get(i).unwrap_or(&"");

        if line1 != line2 {
            if !in_conflict {
                in_conflict = true;
                start = i + 1; // 1-based
            }
        } else if in_conflict {
            in_conflict = false;
            conflict_ranges.push((start, i));
        }
    }

    if in_conflict {
        conflict_ranges.push((start, max_len));
    }

    for (start, end) in conflict_ranges {
        if start == end {
            eprintln!("Merge conflicit in {}: {}", path, start);
        } else {
            eprintln!("Merge conflicit in {}: [{}, {}]", path, start, end);
        }
    }
}

pub fn merge(merge_branch: String, force: bool) {

    if !force && commit::check_has_uncommitted() {
        eprintln!("Detected uncommited files. Cannot merge.");
        process::exit(1);
    }

    let current_branch: String;

    match &reference::get_current_branch() {
        None => {
            eprintln!("You are in 'detached HEAD' state. Cannot merge.");
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

    let current_commit = reference::get_head(&current_branch);
    let merge_commit = reference::get_head(&merge_branch);
    let base_commit = commit::get_merge_base(&current_commit, &merge_commit);

    let mut current_blob_table: HashMap<String, TreeEntry> = Default::default();
    let mut merge_blob_table: HashMap<String, TreeEntry> = Default::default();
    let mut base_blob_table: HashMap<String, TreeEntry> = Default::default();
    register_blob_by_commit(&current_commit, &mut current_blob_table);
    register_blob_by_commit(&merge_commit, &mut merge_blob_table);
    register_blob_by_commit(&base_commit, &mut base_blob_table);

    // println!("[base]");
    // for blob in &base_blob_table {
    //     println!("path: {}, hash: {}", blob.0, blob.1.hash);
    // }
    // println!("[current]");
    // for blob in &current_blob_table {
    //     println!("path: {}, hash: {}", blob.0, blob.1.hash);
    // }
    // println!("[merge]");
    // for blob in &merge_blob_table {
    //     println!("path: {}, hash: {}", blob.0, blob.1.hash);
    // }

    let (
        current_add_log,
        current_remove_log,
        current_modify_log
    ) = commit::diff_commit_to_commit(&base_blob_table, &current_blob_table);
    let (
        merge_add_log,
        merge_remove_log,
        merge_modify_log
    ) = commit::diff_commit_to_commit(&base_blob_table, &merge_blob_table);
    
    // println!("<current>");
    // for log in &current_add_log {
    //     println!("add {}, hash = {}", log.0, log.1.hash.clone());
    // }
    // for log in &current_remove_log {
    //     println!("remove {}, hash = {}", log.0, log.1.hash.clone());
    // }
    // for log in &current_modify_log {
    //     println!("modify {}, hash = {}", log.0, log.1.hash.clone());
    // }
    // println!("<merge>");
    // for log in &merge_add_log {
    //     println!("add {}, hash = {}", log.0, log.1.hash.clone());
    // }
    // for log in &merge_remove_log {
    //     println!("remove {}, hash = {}", log.0, log.1.hash.clone());
    // }
    // for log in &merge_modify_log {
    //     println!("modify {}, hash = {}", log.0, log.1.hash.clone());
    // }

    // construct new blob_table based on: 1. base_blob_table 2. logs
    let mut new_blob_table: HashMap<String, TreeEntry> = base_blob_table.clone();
    let mut conflicts: HashSet<(String, TreeEntry, TreeEntry)> = Default::default();
        // (String, TreeEntry, TreeEntry): (path, entry1, entry2)
    let mut has_operation_conflict: bool = false;
    
    // add
    for (current_path, current_entry) in &current_add_log {
        if let Some(merge_entry) = merge_add_log.get(current_path) {
            if merge_entry.hash != current_entry.hash || merge_entry.entry_type != current_entry.entry_type {
                conflicts.insert((
                    current_path.to_owned(),
                    current_entry.to_owned(),
                    merge_entry.to_owned()
                ));
            }
        }
        new_blob_table.insert(current_path.to_owned(), current_entry.to_owned());
    }
    for (merge_path, merge_entry) in &merge_add_log {
        if let Some(current_entry) = current_add_log.get(merge_path) {
            if merge_entry.hash != current_entry.hash || merge_entry.entry_type != current_entry.entry_type {
                conflicts.insert((
                    merge_path.to_owned(),
                    current_entry.to_owned(),
                    merge_entry.to_owned()
                ));
            }
        }
        new_blob_table.insert(merge_path.to_owned(), merge_entry.to_owned());
    }
    // remove
    for (current_path, _) in &current_remove_log {
        if merge_modify_log.contains_key(current_path) {
            eprintln!("Detected operation conflict: {}", current_path);
            eprintln!("    Removed in: {}", current_commit);
            eprintln!("    Modified in: {}", merge_commit);
            has_operation_conflict = true;
            continue;
        }
        if new_blob_table.contains_key(current_path) {
            new_blob_table.remove(current_path);
        }
    }
    for (merge_path, _) in &merge_remove_log {
        if current_modify_log.contains_key(merge_path) {
            eprintln!("Detected operation conflict: {}", merge_path);
            eprintln!("    Removed in: {}", merge_commit);
            eprintln!("    Modified in: {}", current_commit);
            has_operation_conflict = true;
            continue;
        }
        if new_blob_table.contains_key(merge_path) {
            new_blob_table.remove(merge_path);
        }
    }
    // modify
    for (current_path, current_entry) in &current_modify_log {
        if merge_remove_log.contains_key(current_path) {
            continue;
        }
        if let Some(merge_entry) = merge_modify_log.get(current_path) {
            if merge_entry.hash != current_entry.hash || merge_entry.entry_type != current_entry.entry_type {
                conflicts.insert((
                    current_path.to_owned(),
                    current_entry.to_owned(),
                    merge_entry.to_owned()
                ));
            }
        }
        new_blob_table.insert(current_path.to_owned(), current_entry.to_owned());
    }
    for (merge_path, merge_entry) in &merge_modify_log {
        if current_remove_log.contains_key(merge_path) {
            continue;
        }
        if let Some(current_entry) = current_modify_log.get(merge_path) {
            if merge_entry.hash != current_entry.hash || merge_entry.entry_type != current_entry.entry_type {
                conflicts.insert((
                    merge_path.to_owned(),
                    current_entry.to_owned(),
                    merge_entry.to_owned()
                ));
            }
        }
        new_blob_table.insert(merge_path.to_owned(), merge_entry.to_owned());
    }

    // println!("== new_blob_table ==");
    // for blob in &new_blob_table {
    //     println!("path: {}, hash: {}", blob.0, blob.1.hash.clone());
    // }
    // println!("== conflicts ==");
    // for conflict in &conflicts {
    //     println!("path: {}", conflict.0);
    // }

    if conflicts.len() != 0 {
        for (path, entry1, entry2) in &conflicts {
            analyse_merge_conflict(path, entry1, entry2);
        }
        process::exit(1);
    }
    if has_operation_conflict {
        process::exit(1);
    }

    // Can merge now

    // blob_table now stores every entry.

    // generate new commit
    let mut index_entries: HashSet<IndexEntry> = Default::default();
    
    for (blob_path, blob_entry) in &new_blob_table {
        let repo_path = utils::pwd();
        assert!(blob_path.starts_with(&repo_path));
        let rel_path = Path::new(&blob_path)
            .strip_prefix(&repo_path)
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        index_entries.insert(IndexEntry { path: rel_path, hash: blob_entry.hash.to_owned() });
    }

    let parent_commits = [
        reference::get_head(&current_branch),
        reference::get_head(&merge_branch)
    ];

    let new_head_hash = commit_merge(
        &index_entries,
        format!("Merge branch {} and {}.", current_branch, merge_branch),
        utils::get_time_string(),
        utils::get_username(),
        parent_commits.to_vec(),
        new_blob_table
    );

    reference::store_head(&current_branch, &new_head_hash);

    storage::clear_index();

    // restore working area
    storage::clear_working_area();
    storage::restore_working_area(&new_head_hash);

    println!("Merged branches {} and {}.", current_branch, merge_branch);
    
    /*
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

    println!("Merged branches {} and {}.", current_branch, merge_branch);
    */
}


/*
Merge Test:

rm 1.txt
echo "1" > 1.txt

./git init

./git add .
./git commit -m "1"
./git branch A
./git branch B
./git checkout A

echo -e "1\nA\nA\n2\nA" > 1.txt
./git add .
./git commit -m "A"
./git branch Abak

./git checkout B
echo -e "1\nB\nB\n2\nB" > 1.txt
./git add .
./git commit -m "B"
./git branch Bbak

./git merge A


*/