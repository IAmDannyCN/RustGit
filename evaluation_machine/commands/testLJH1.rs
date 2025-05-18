use super::Command;
use crate::{excute, expect, run};

const GIT: &str = "../test/git";
const PATH: &str = "./test_area/";

pub struct TestLJH1Command;

impl Command for TestLJH1Command {
    fn execute(&self) -> super::CommandResult {
        let mut command = format!("{} init -p {}", GIT, PATH);
        excute(&command);
        command = format!("touch {}test.txt", PATH);
        let _ = run(&command);
        command = format!("echo \"你好Rust!\" > {}test.txt", PATH);
        let _ = run(&command);
        command = format!("ls -A {}", PATH);
        if !expect(&command, |output| {output.contains("test.txt")})? {
            println!("directory does not contain 'test.txt'");
            return Err("directory error".into());
        }
        println!("directory contains 'test.txt'");
        command = format!("mkdir {}testDir/", PATH);
        excute(&command);
        command = format!("touch {}/testDir/hello.txt", PATH);
        let _ = run(&command);
        command = format!("echo \"你好!\" > {}/testDir/hello.txt", PATH);
        let _ = run(&command);
        command = format!("mkdir {}test_dir/", PATH);
        excute(&command);

        command = format!("{} add {}test.txt -p {}", GIT, PATH, PATH);
        excute(&command);
        command = format!("{} commit -m \"初始提交\" -p {}", GIT, PATH);
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