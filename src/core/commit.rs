use crate::utils::{hash, serialize};
use super::object::*;

pub struct CommitData {
    pub message: String,
    pub user: String,
    pub time: String,
    pub tree_hash: String,
}

pub struct Commit {
    pub hash: Option<String>,
    pub data: Option<CommitData>,
}

pub trait CommitTrait {
    fn read_commit(&mut self);
    fn write_commit(&mut self);
}

impl CommitTrait for Commit {

    /// `read_commit`: read file and update `self.data` based on `self.hash`
    fn read_commit(&mut self) {

        assert!(self.hash.is_none() == false);
        assert!(self.data.is_none() == true);

        let hash = self.hash.as_ref().unwrap();
        
        let raw_content = read_object_file(hash);
        let full_content = serialize::deserialize(&raw_content);
        assert!(full_content.len() >= 4);
        assert!(full_content[..4] == "CMIT"[..]);

        let parts: Vec<&str> = full_content[4..].split('\0').collect();

        let data: CommitData = CommitData {  
            message:    parts[0].to_string(),
            user:       parts[1].to_string(),
            time:       parts[2].to_string(),
            tree_hash:  parts[3].to_string(),
        };

        self.data = Some(data);

    }

    /// `write_commit`: calculate `self.hash` and write file based on `self.data`
    fn write_commit(&mut self) {

        assert!(self.hash.is_none() == true);
        assert!(self.data.is_none() == false);

        let commit_data = self.data.as_ref().unwrap();
        let mut data: String = Default::default();

        data.push_str(&format!("{}\0{}\0{}\0{}", 
            commit_data.message, commit_data.user, commit_data.time, commit_data.tree_hash));

        let full_content = "CMIT".to_string() + &data;
        let hash = hash::sha1(&full_content);
        let raw_content = serialize::serialize(&full_content);

        write_object_file(&hash, &raw_content);
        self.hash = Some(hash);
    }
}