use super::*;
const GIT: &str = "../test/git";
const PATH: &str = "./test_area/";

pub fn test() -> Result<(), Box<dyn std::error::Error>> {
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

    command = format!("{} branch test -p {}", GIT, PATH);
    excute(&command);
    command = format!("{} checkout test -p {}", GIT, PATH);
    excute(&command);

    command = format!("echo \"test分支修改内容\" > {}test.txt", PATH);
    let _ = run(&command);
    command = format!("{} add {}. -p {}", GIT, PATH, PATH);
    excute(&command);
    command = format!("{} commit -m \"test\" -p {}", GIT, PATH);
    excute(&command);

    command = format!("{} checkout main -p {}", GIT, PATH);
    excute(&command);
    command = format!("touch {}additional.txt", PATH);
    let _ = run(&command);
    command = format!("echo \"additional content in main\" > {}additional.txt", PATH);
    let _ = run(&command);
    command = format!("{} add {}. -p {}", GIT, PATH, PATH);
    excute(&command);
    command = format!("{} commit -m \"additional commit in main\" -p {}", GIT, PATH);
    excute(&command);
    command = format!("{} merge test -p {} 2>&1", GIT, PATH);
    excute(&command);

    command = format!("tree -a {}.mygit", PATH);
    excute(&command);
    // command = format!("{}main | grep \"测试分支合并\"", PATH);
    // if expect(&command, |output| {output.is_empty()})? {
    //     println!("git merge failed, main.rs output is wrong");
    //     return Err("merge error".into());
    // }
    // println!("git merge succeeded and main.rs output is correct");
    Ok(())
}