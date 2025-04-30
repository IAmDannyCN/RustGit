use super::Command;
use crate::{excute, expect, run};

const GIT: &str = "../test/git";
const PATH: &str = "./test_area/";

pub struct Test2sCommand;

impl Command for Test2sCommand {
    fn execute(&self) -> super::CommandResult {
        let mut command = format!("{} init -p {}", GIT, PATH);
        excute(&command);
        command = format!("dd if=/dev/zero of={}large_file.bin bs=1M count=10", PATH);
        let _ = run(&command);
        command = format!("ls -A {}", PATH);
        if !expect(&command, |output| {output.contains("large_file.bin")})? {
            println!("directory does not contain 'large_file.bin'");
            return Err("directory error".into());
        }
        println!("directory contains 'large_file.bin'");
    
        command = format!("{} add {}large_file.bin -p {}", GIT, PATH, PATH);
        excute(&command);
        command = format!("{} commit -m \"Add large file\" -p {}", GIT, PATH);
        excute(&command);
    
        command = format!("tree -a {}.mygit", PATH);
        excute(&command);
        // command = format!("ls -A {}.mygit/objects", PATH);
        // if expect(&command, |output| {output.is_empty()})? {
        //     println!("objects directory is empty.");
        //     return Err("objects error".into());
        // }
        // println!("objects directory is not empty.");
        // command = format!("ls -A {}.mygit/refs/heads", PATH);
        // if !expect(&command, |output| {output.contains("master")})? {
        //     println!("refs directory does not contain 'master'");
        //     return Err("objects error".into());
        // }
        // println!("refs directory contains 'master'");
        Ok(())
    }
}