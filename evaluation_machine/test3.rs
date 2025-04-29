use super::*;
const GIT: &str = "../test/git";
const PATH: &str = "./test_area/";

pub fn test() -> Result<(), Box<dyn std::error::Error>> {
    let mut command = format!("{} init -p {}", GIT, PATH);
    excute(&command);
    command = format!("touch {}test.txt", PATH);
    let _ = run(&command);
    command = format!("echo \"Hello, Rust!\" > {}test.txt", PATH);
    let _ = run(&command);

    command = format!("{} add {}test.txt -p {}", GIT, PATH, PATH);
    excute(&command);
    command = format!("{} commit -m \"Initial commit\" -p {}", GIT, PATH);
    excute(&command);

    command = format!("{} branch test -p {}", GIT, PATH);
    excute(&command);
    command = format!("{} checkout test -p {}", GIT, PATH);
    excute(&command);

    command = format!("tree -a {}.mygit", PATH);
    excute(&command);
    command = format!("ls -A {}.mygit/refs/heads", PATH);
    if !expect(&command, |output| {output.contains("test")})? {
        println!("refs directory does not contain 'test'");
        return Err("branch error".into());
    }
    println!("refs directory contains 'test'");
    command = format!("grep \"ref: refs/heads/test\" {}.mygit/HEAD", PATH);
    if expect(&command, |output| {output.is_empty()})? {
        println!("HEAD does not contain 'test'");
        return Err("branch error".into());
    }
    println!("HEAD contains 'test'");
    Ok(())
}