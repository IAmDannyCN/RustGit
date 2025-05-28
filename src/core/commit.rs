//! Module: commit
//!
//! Provides structures and logic for reading, writing, and comparing commit objects,
//! including traversal, merge base identification, and detecting uncommitted changes.

use std::{collections::{HashMap, HashSet, VecDeque}, i32::MAX, process};

use crate::{commands::*, utils::*};
use super::{index::IndexEntry, object::*, reference, tree::TreeEntry};

/// Struct holding all metadata associated with a commit.
#[derive(Default)]
pub struct CommitData {
    pub message: String,
    pub user: String,
    pub time: String,
    pub tree_hash: String,
    pub parent_commits: Vec<String>,
}

/// Represents a Git commit object, which stores file content.
///
/// - `hash`: Optional SHA-1 hash of the commit content.
/// - `data`: Optional metadata of the commit.
pub struct Commit {
    pub hash: Option<String>,
    pub data: Option<CommitData>,
}

pub trait CommitTrait {
    fn read_commit(&mut self);
    fn write_commit(&mut self);
    fn calculate_hash(&mut self);
}

impl CommitTrait for Commit {

    /// Reads the commit object from storage and populates `self.data`.
    ///
    /// Requires that `self.hash` is set and `self.data` is empty.
    fn read_commit(&mut self) {

        assert!(self.hash.is_none() == false);
        assert!(self.data.is_none() == true);

        let hash = self.hash.as_ref().unwrap();

        if hash == "" {
            self.data = Some(Default::default());
            return ;
        }
        
        let raw_content = read_object_file(hash);
        let vecu8_content = serialize::deserialize(&raw_content);
        let full_content = std::str::from_utf8(&vecu8_content).expect("Invalid UTF-8");

        assert!(full_content.len() >= 4);
        assert!(full_content.starts_with("CMIT"));

        let parts: Vec<&str> = full_content[4..].split('\0').collect();

        let data: CommitData = CommitData {  
            message:    parts[0].to_string(),
            user:       parts[1].to_string(),
            time:       parts[2].to_string(),
            tree_hash:  parts[3].to_string(),
            parent_commits: parts[4].split('&').map(|s| s.to_string()).collect(),
        };

        self.data = Some(data);

    }


    /// Serializes and writes the commit to storage.
    ///
    /// Calculates the hash if not already present. Prepends "CMIT" as a type header.
    fn write_commit(&mut self) {

        assert!(self.data.is_none() == false);

        if self.hash.is_none() {
            self.calculate_hash();
        }

        let commit_data = self.data.as_ref().unwrap();
        let data: String = format!("{}\0{}\0{}\0{}\0{}", 
            commit_data.message,
            commit_data.user,
            commit_data.time,
            commit_data.tree_hash,
            commit_data.parent_commits.join("&")
        );

        let full_content = "CMIT".to_string() + &data;
        let raw_content = serialize::serialize(&full_content.as_bytes());

        write_object_file(self.hash.as_ref().unwrap(), &raw_content);
    }

    
    /// Computes the SHA-1 hash for the commit's content.
    fn calculate_hash(&mut self) {
        assert!(self.data.is_none() == false);
        let commit_data = self.data.as_ref().unwrap();
        let mut data: String = Default::default();

        data.push_str(&format!("{}\0{}\0{}\0{}", 
            commit_data.message, commit_data.user, commit_data.time, commit_data.tree_hash));
        
        self.hash = Some(hash::sha1(&data.as_bytes()));
    }
}


/// Recursively determines whether `prev_commit_hash` is an ancestor of `post_commit_hash`.
///
/// # Arguments
/// * `prev_commit_hash` - The potential ancestor commit hash.
/// * `post_commit_hash` - The commit hash to start searching from.
/// * `searched_commits` - A mutable set used to track visited commits and prevent cycles.
///
/// # Returns
/// * `true` if `prev_commit_hash` is an ancestor (or equal to) `post_commit_hash`, or if `prev_commit_hash` is empty.
/// * `false` otherwise.
fn is_prev_commit_search(
    prev_commit_hash: &str,
    post_commit_hash: &str,
    searched_commits: &mut HashSet<String>
) -> bool {
    if prev_commit_hash == post_commit_hash {
        return true;
    } else if prev_commit_hash == "" {
        return true;
    } else if post_commit_hash == "" {
        return false;
    }

    if let Some(_) = searched_commits.get(post_commit_hash) {
        return false;
    }
    searched_commits.insert(post_commit_hash.to_string());
    
    // prev != post, and both != ""
    let mut post_commit = Commit { hash: Some(post_commit_hash.to_string()), data: None };
    post_commit.read_commit();

    for parent_commit_hash in post_commit.data.unwrap().parent_commits {
        if is_prev_commit_search(prev_commit_hash, &parent_commit_hash, searched_commits) {
            return true;
        }
    }

    false
}


/// Determines whether `prev_commit_hash` is an ancestor of `post_commit_hash`.
///
/// This is a public interface that wraps the internal recursive function with a fresh `searched_commits` set.
///
/// # Arguments
/// * `prev_commit_hash` - The potential ancestor commit hash.
/// * `post_commit_hash` - The descendant commit hash to check against.
///
/// # Returns
/// * `true` if `prev_commit_hash` is an ancestor or the same commit.
/// * `false` otherwise.
pub fn is_prev_commit(prev_commit_hash: &str, post_commit_hash: &str) -> bool {
    let mut searched_commits = HashSet::new();
    is_prev_commit_search(prev_commit_hash, post_commit_hash, &mut searched_commits)
}


/// Checks if there are any uncommitted changes in either the staging area or working directory.
///
/// This function checks two aspects:
/// 1. If the index (staging area) contains any entries.
/// 2. If there are changes in the working directory compared to the last commit.
///
/// # Returns
/// * `true` if there are any uncommitted changes.
/// * `false` if the working directory is clean.
pub fn check_has_uncommitted() -> bool {

    // Check the staging area
    // let index = index::read_index();
    // if index.len() > 0 {
    //     return true;
    // }

    // Check the working area
    let (index, _, _, _) = add::add_core(&[utils::pwd()].to_vec());
    let mut entries: HashSet<IndexEntry> = Default::default();
    for kv in &index {
        entries.insert(kv.1.clone());
    }

    let (add_log,
        remove_log,
        modify_log) =
        status::diff_index_entries_to_commit(&entries, &reference::get_current_commit());
    if add_log.len() != 0 || remove_log.len() != 0 || modify_log.len() != 0 {
        true
    } else {
        false
    }
}


/// Computes the shortest distance (in number of commits) from the given commit to each of its reachable ancestors.
///
/// This is a BFS traversal over the commit graph.
///
/// # Arguments
/// * `commit_hash` - The starting commit hash.
///
/// # Returns
/// * A map of ancestor commit hash → distance (integer depth from `commit_hash`).
fn get_parent_commit_dis(commit_hash: &str) -> HashMap<String, i32> {
    let mut dis: HashMap<String, i32> = Default::default();

    // run BFS
    let mut visited: HashSet<String> = Default::default();
    let mut queue: VecDeque<(String, i32)> = Default::default();
    visited.insert(commit_hash.to_owned());
    queue.push_back((commit_hash.to_owned(), 0));

    while let Some((cur_hash, cur_dis)) = queue.pop_front() {

        dis.insert(cur_hash.clone(), cur_dis.clone());

        let mut commit = Commit {
            hash: Some(cur_hash.clone()),
            data: None,
        };
        commit.read_commit();

        let commit_data = commit.data.unwrap();
        for parent in commit_data.parent_commits {
            if !visited.contains(&parent) {
                visited.insert(parent.clone());
                queue.push_back((parent, cur_dis + 1));
            }
        }
    }

    dis
}


/// Merges two ancestor distance maps and returns a map of common ancestors with summed distances.
///
/// # Arguments
/// * `dis1` - Distance map from first commit.
/// * `dis2` - Distance map from second commit.
///
/// # Returns
/// * A map containing common ancestors with the sum of their distances from both commits.
fn merge_parent_commit_dis(dis1: HashMap<String, i32>, dis2: HashMap<String, i32>) -> HashMap<String, i32> {
    let mut dis: HashMap<String, i32> = Default::default();
    for kv in &dis2 {
        if !dis1.contains_key(kv.0) {
            continue;
        }
        dis.insert(kv.0.to_owned(), kv.1 + dis1.get(kv.0).unwrap());
    }
    dis
}


/// Finds the lowest common ancestor (merge base) of two commits by minimizing the sum of distances.
///
/// # Arguments
/// * `c1` - First commit hash.
/// * `c2` - Second commit hash.
///
/// # Returns
/// * The hash of the merge base commit.
///
/// # Panics
/// * If no common ancestor is found.
/// * If multiple common ancestors have the same minimal distance (ambiguous base).
pub fn get_merge_base(c1: &str, c2: &str) -> String {
    
    let dis1 = get_parent_commit_dis(c1);
    let dis2 = get_parent_commit_dis(c2);
    let dis = merge_parent_commit_dis(dis1, dis2);
    
    let mut min_dis: i32 = MAX;
    let mut min_cnt: i32 = 0;
    let mut base_commit: Option<String> = None;

    for kv in dis {
        if kv.1 < min_dis {
            base_commit = Some(kv.0);
            min_dis = kv.1;
            min_cnt = 1;
        } else if kv.1 == min_dis {
            min_cnt += 1;
        }
    }

    if min_cnt == 0 {
        panic!("core::commit::get_merge_base: cannot find any parent.");
    } else if min_cnt > 1 {
        eprintln!("core::commit::get_merge_base: found multiple base candidates.");
        process::exit(1);
    }

    base_commit.unwrap()
}


/// Compares two sets of blobs (file snapshots) and returns added, removed, and modified files.
///
/// # Arguments
/// * `base_blob_table` - Blob table of the base commit.
/// * `new_blob_table` - Blob table of the new commit.
///
/// # Returns
/// * Tuple of:
///     - `added`: files present only in `new_blob_table`
///     - `removed`: files present only in `base_blob_table`
///     - `modified`: files present in both but with different hash or type
pub fn diff_commit_to_commit(
    base_blob_table: &HashMap<String, TreeEntry>,
    new_blob_table: &HashMap<String, TreeEntry>
) -> (
    HashMap<String, TreeEntry>, HashMap<String, TreeEntry>, HashMap<String, TreeEntry>
) {
    // (String, String): (path, hash)
    let mut add_log: HashMap<String, TreeEntry> = Default::default();
    let mut remove_log: HashMap<String, TreeEntry> = Default::default();
    let mut modify_log: HashMap<String, TreeEntry> = Default::default();

    for (path, entry) in base_blob_table {
        if !new_blob_table.contains_key(path) {
            remove_log.insert(path.to_owned(), entry.to_owned());
        } else {
            let new_entry = new_blob_table.get(path).unwrap();
            if entry.hash != new_entry.hash || entry.entry_type != new_entry.entry_type {
                modify_log.insert(path.to_owned(), new_entry.to_owned());
            }
        } 
    }

    for (path, entry) in new_blob_table {
        if !base_blob_table.contains_key(path) {
            add_log.insert(path.to_owned(), entry.to_owned());
        }
    }

    (add_log, remove_log, modify_log)
}