use super::Command;
use crate::{excute, expect, run};

const GIT: &str = "../test/git";
const PATH: &str = "./test_area/";

pub struct Test3sCommand;

impl Command for Test3sCommand {
    fn execute(&self) -> super::CommandResult {
        let mut command = format!("{} init -p {}", GIT, PATH);
        excute(&command);
        command = format!("{} checkout -b main -p {}", GIT, PATH);
        excute(&command);
        command = format!("touch {}main.txt", PATH);
        let _ = run(&command);
        command = format!("echo \"main分支创建\" > {}main.txt", PATH);
        let _ = run(&command);

        command = format!("{} add {}. -p {}", GIT, PATH, PATH);
        excute(&command);
        command = format!("{} commit -m \"update main\" -p {}", GIT, PATH);
        excute(&command);

        command = format!("{} branch temp -p {}", GIT, PATH);
        excute(&command);
        command = format!("{} checkout temp -p {}", GIT, PATH);
        excute(&command);
        command = format!("touch {}temp.txt", PATH);
        let _ = run(&command);
        command = format!("echo \"test分支创建\" > {}temp.txt", PATH);
        let _ = run(&command);

        command = format!("{} add {}. -p {}", GIT, PATH, PATH);
        excute(&command);
        command = format!("{} commit -m \"update temp\" -p {}", GIT, PATH);
        excute(&command);

        command = format!("{} checkout main -p {}", GIT, PATH);
        excute(&command);
        command = format!("{} merge temp -p {}", GIT, PATH);
        excute(&command);
        command = format!("{} branch -d temp -p {}", GIT, PATH);
        excute(&command);

        command = format!("tree -a {}.mygit", PATH);
        excute(&command);
        command = format!("ls -A {}", PATH);
        if !expect(&command, |output| {
            output.contains("main.txt") && output.contains("temp.txt")
        })? {
            println!("directory does not contain 'main.txt' or 'temp.txt'");
            return Err("directory error".into());
        }
        println!("directory does contain 'main.txt' or 'temp.txt'");
        command = format!("ls -A {}.mygit/refs/heads", PATH);
        if !expect(&command, |output| {output.contains("main")})? {
            println!("refs directory does not contain 'main'");
            return Err("branch error".into());
        }
        println!("refs directory contains 'main'");
        command = format!("ls -A {}.mygit/refs/heads", PATH);
        if expect(&command, |output| {output.contains("temp")})? {
            println!("refs directory does not contain 'temp'");
            return Err("branch error".into());
        }
        println!("refs directory contains 'temp'");
        Ok(())
    }
}