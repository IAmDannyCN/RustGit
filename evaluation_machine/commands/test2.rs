use super::Command;
use crate::{excute, expect, run};

const GIT: &str = "../test/git";
const PATH: &str = "./test_area/";

pub struct Test2Command;

impl Command for Test2Command {
    fn execute(&self) -> super::CommandResult {
        let mut command = format!("{} init -p {}", GIT, PATH);
        excute(&command);
        command = format!("touch {}test.txt", PATH);
        let _ = run(&command);
        command = format!("echo \"Hello, Rust!\" > {}test.txt", PATH);
        let _ = run(&command);
        command = format!("ls -A {}", PATH);
        if !expect(&command, |output| {output.contains("test.txt")})? {
            println!("directory does not contain 'test.txt'");
            return Err("directory error".into());
        }
        println!("directory contains 'test.txt'");
        
        command = format!("{} add {}test.txt -p {}", GIT, PATH, PATH);
        excute(&command);
        command = format!("{} commit -m \"Initial commit\" -p {}", GIT, PATH);
        excute(&command);
        
        command = format!("tree -a {}.mygit", PATH);
        excute(&command);
        command = format!("ls -A {}.mygit/objects", PATH);
        if expect(&command, |output| {output.is_empty()})? {
            println!("objects directory is empty.");
            return Err("objects error".into());
        }
        println!("objects directory is not empty.");
        command = format!("ls -A {}.mygit/refs/heads", PATH);
        if !expect(&command, |output| {output.contains("master")})? {
            println!("refs directory does not contain 'master'");
            return Err("objects error".into());
        }
        println!("refs directory contains 'master'");
        Ok(())
    }
}