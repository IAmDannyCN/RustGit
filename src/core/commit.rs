use crate::{commands::commit, utils::{hash, serialize}};
use super::object::*;

pub struct CommitData {
    pub message: String,
    pub user: String,
    pub time: String,
    pub tree_hash: String,
    pub parent_commits: Vec<String>,
}

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

    /// `read_commit`: read file and update `self.data` based on `self.hash`
    fn read_commit(&mut self) {

        assert!(self.hash.is_none() == false);
        assert!(self.data.is_none() == true);

        let hash = self.hash.as_ref().unwrap();
        
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

    /// `write_commit`: calculate `self.hash` and write file based on `self.data`
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

    /// calculate the hash for `commit.data`
    fn calculate_hash(&mut self) {
        assert!(self.data.is_none() == false);
        let commit_data = self.data.as_ref().unwrap();
        let mut data: String = Default::default();

        data.push_str(&format!("{}\0{}\0{}\0{}", 
            commit_data.message, commit_data.user, commit_data.time, commit_data.tree_hash));
        
        self.hash = Some(hash::sha1(&data.as_bytes()));
    }
}