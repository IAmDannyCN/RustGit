use super::Command;
use crate::{excute, expect, run};

const GIT: &str = "../test/git";
const PATH: &str = "./test_area/";

pub struct Test1sCommand;

impl Command for Test1sCommand {
    fn execute(&self) -> super::CommandResult {
        let mut command = format!("{} init -p {}", GIT, PATH);
        excute(&command);
        command = format!("{} checkout -b main -p {}", GIT, PATH);
        excute(&command);
    
        command = format!("touch {}test.txt", PATH);
        let _ = run(&command);
        command = format!("echo \"main分支修改内容\" > {}test.txt", PATH);
        let _ = run(&command);

        command = format!("{} add {}. -p {}", GIT, PATH, PATH);
        excute(&command);
        command = format!("{} commit -m \"main\" -p {}", GIT, PATH);
        excute(&command);

        command = format!("{} branch temp1 -p {}", GIT, PATH);
        excute(&command);
        command = format!("{} branch temp2 -p {}", GIT, PATH);
        excute(&command);

        command = format!("{} checkout temp1 -p {}", GIT, PATH);
        excute(&command);
        command = format!("echo \"temp1分支修改内容\" > {}test.txt", PATH);
        let _ = run(&command);
        command = format!("{} add {}. -p {}", GIT, PATH, PATH);
        excute(&command);
        command = format!("{} commit -m \"temp1\" -p {}", GIT, PATH);
        excute(&command);

        command = format!("{} checkout main -p {}", GIT, PATH);
        excute(&command);

        command = format!("{} checkout temp2 -p {}", GIT, PATH);
        excute(&command);
        command = format!("echo \"temp2分支修改内容\" > {}test.txt", PATH);
        let _ = run(&command);
        command = format!("{} add {}. -p {}", GIT, PATH, PATH);
        excute(&command);
        command = format!("{} commit -m \"temp2\" -p {}", GIT, PATH);
        excute(&command);
        // command = format!("{} merge temp1 -p {} 2>&1", GIT, PATH);
        // excute(&command);

        command = format!("tree -a {}.mygit", PATH);
        excute(&command);
        command = format!("{} merge temp1 -p {} 2>&1 | grep -E \"Merge conflict in.*: 1\"", GIT, PATH);
        excute(&command);
        if expect(&command, |output| {output.is_empty()})? {
            println!("Conflict does not detected correctly");
            return Err("merge conflict error".into());
        }
        println!("Conflict detected correctly");
        Ok(())
    }
}