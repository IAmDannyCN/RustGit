use std::collections::{HashMap, HashSet};
use std::process;

use crate::core::index::IndexEntry;
use crate::{core::*, utils::*};
use crate::core::tree::{Tree, TreeEntry, TreeEntryType, TreeTrait};
use crate::core::commit::{Commit, CommitData, CommitTrait};

fn create_tree_for_path(path: &str, father_path: &str, trees: &mut HashMap<String, Tree>) {
    if path == "" {
        return ;
    }
    let (cur_name, after_path) = utils::split_path_by_first(path);
    let cur_path = match father_path {
        "" => cur_name.clone(),
        _ => format!("{}/{}", father_path, cur_name)
    };
    // e.g. A/B/C/D/E
    //      path        : C/D/E (Not NULL)
    //      father_path : A/B   (Can NULL)
    //      cur_name    : C     (Not NULL)
    //      after_path  : D/E   (Can NULL)
    //      cur_path    : A/B/C (Not NULL)

    match trees.get(&cur_path) {
        Some(_) => {
            create_tree_for_path(&after_path, &cur_path, trees);
        }
        None => {
            trees.insert(cur_path.clone(), Tree { hash: None, data: Some(Default::default())});
            if let Some(parent_tree) = trees.get_mut(father_path) {
                if let Some(vec) = parent_tree.data.as_mut() {
                    vec.push(TreeEntry {
                        entry_type: TreeEntryType::Tree,
                        name: cur_name,
                        hash: Default::default(),
                    });
                } else {
                    parent_tree.data = Some(vec![TreeEntry {
                        entry_type: TreeEntryType::Tree,
                        name: cur_name,
                        hash: Default::default(),
                    }]);
                }                
            }
            create_tree_for_path(&after_path, &cur_path, trees);
        }
    }
}

fn add_entry_to_tree(
    entry: &IndexEntry,
    trees: &mut HashMap<String, Tree>,
    tree_table: Option<&HashMap<String, TreeEntry>>
) {
    let file_path = entry.path.clone();
    let (dir_path, file_name) = utils::split_path_by_last(&file_path);
    // e.g. A/B/C/D/E/1.txt
    //      file_path:  A/B/C/D/E/1.txt
    //      dir_path:   A/B/C/D/E
    //      file_name:  1.txt

    create_tree_for_path(&dir_path, "", trees);

    if let Some(folder_tree) = trees.get_mut(&dir_path) {
        let blob_path = format!("{}/{}", utils::pwd(), file_path);
        let entry_type = match tree_table {
            None => {
                blob::get_blob_type(&blob_path)
            }
            Some(tree_table) => {
                tree_table.get(&blob_path).unwrap().entry_type.clone()
            }
        };
        if let Some(vec) = folder_tree.data.as_mut() {
            vec.push(TreeEntry {
                entry_type,
                name: file_name,
                hash: entry.hash.clone(),
            });
        } else {
            folder_tree.data = Some(vec![TreeEntry {
                entry_type,
                name: file_name,
                hash: entry.hash.clone(),
            }]);
        }
    } else {
        panic!("commit::add_entry_to_tree: Cannot get dir_path.");
    }
}

fn hash_then_write_tree(path: &str, trees: &mut HashMap<String, Tree>) -> String {

    // collect all sub_trees to be hashed and written

    let tree = trees.get_mut(path)
        .unwrap_or_else(|| panic!("commit::hash_then_write_tree: Cannot find path {}.", path));
    let data = tree.data.as_mut().unwrap();
    let mut sub_trees = Vec::new();

    for entry in data.iter_mut() {
        if let TreeEntryType::Tree = entry.entry_type {
            let next_path = match path {
                "" => entry.name.clone(),
                _ => format!("{}/{}", path, entry.name),
            };
            sub_trees.push(next_path);
        }
    }

    // do the hashing on the sub_paths and record their hash result

    let mut hash_result: HashMap<String, String> = HashMap::new(); // next_path -> hash

    for next_path in sub_trees {
        hash_result.insert(next_path.clone(), hash_then_write_tree(&next_path, trees));
    }

    // write back hash result to current_tree.data

    let tree = trees.get_mut(path).unwrap();
    let data = tree.data.as_mut().unwrap();

    for entry in data.iter_mut() {
        if let TreeEntryType::Tree = entry.entry_type {
            let next_path = match path {
                "" => entry.name.clone(),
                _ => format!("{}/{}", path, entry.name),
            };
            entry.hash = hash_result.get(&next_path).unwrap().clone();
        }
    }

    // hash current_tree

    let tree = trees.get_mut(path).unwrap();
    tree.write_tree();

    tree.hash.clone().unwrap()
}

pub fn commit(  entries: &HashSet<IndexEntry>,
                message: String, time: String,
                user: String,
                parent_commits: Vec<String>) -> String {

    let mut trees: HashMap<String, Tree> = Default::default();
    trees.insert("".to_string(), Tree { hash: None, data: Some(Default::default())});

    for entry in entries {
        add_entry_to_tree(entry, &mut trees, None);
    }

    let tree_hash = hash_then_write_tree("", &mut trees);

    let commit_data = CommitData { message, user, time, tree_hash, parent_commits };
    let mut commit = Commit { hash: None, data: Some(commit_data) };

    commit.write_commit();

    commit.hash.unwrap()
}

pub fn commit_merge( 
    entries: &HashSet<IndexEntry>,
    message: String, time: String,
    user: String,
    parent_commits: Vec<String>,
    new_blob_table: HashMap<String, TreeEntry>
) -> String {
    let mut trees: HashMap<String, Tree> = Default::default();
    trees.insert("".to_string(), Tree { hash: None, data: Some(Default::default())});

    for entry in entries {
        add_entry_to_tree(entry, &mut trees, Some(&new_blob_table));
    }

    let tree_hash = hash_then_write_tree("", &mut trees);

    let commit_data = CommitData { message, user, time, tree_hash, parent_commits };
    let mut commit = Commit { hash: None, data: Some(commit_data) };

    commit.write_commit();

    commit.hash.unwrap()
}

pub fn commit_index(message: String) {
    
    let index = index::read_index();
    let mut entries: HashSet<IndexEntry> = Default::default();
    for kv in index {
        entries.insert(kv.1);
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

    let parent_commits: Vec<String> = [ reference::get_head(&current_branch) ].to_vec();

    let new_head_hash = commit(&entries, message, utils::get_time_string(), utils::get_username(), parent_commits);

    reference::store_head(&current_branch, &new_head_hash);

    storage::clear_index();

    println!("Committed changes {} to head {}.", new_head_hash, current_branch);
}
