use super::*;
const GIT: &str = "../test/git";
const PATH: &str = "./test_area/";

pub fn test() -> Result<(), Box<dyn std::error::Error>> {
    let mut command = format!("{} init -p {}", GIT, PATH);
    excute(&command);
    command = format!("{} checkout -b main -p {}", GIT, PATH);
    excute(&command);
    command = format!("touch {}delete.txt", PATH);
    let _ = run(&command);
    command = format!("echo \"delete me\" > {}delete.txt", PATH);
    let _ = run(&command);

    
    command = format!("{} add {}. -p {}", GIT, PATH, PATH);
    excute(&command);
    // command = format!("{} status -p {} 2>&1", GIT, PATH);
    // excute(&command);
    command = format!("{} commit -m \"add file\" -p {}", GIT, PATH);
    excute(&command);

    command = format!("{} branch temp -p {}", GIT, PATH);
    excute(&command);
    command = format!("{} checkout temp -p {}", GIT, PATH);
    excute(&command);

    
    command = format!("{} rm {}delete.txt -p {} 2>&1", GIT, PATH, PATH);
    excute(&command);
    command = format!("{} commit -m \"delete file\" -p {}", GIT, PATH);
    excute(&command);

    command = format!("{} checkout main -p {}", GIT, PATH);
    excute(&command);
    command = format!("{} merge temp -p {}", GIT, PATH);
    excute(&command);

    command = format!("tree -a {}.mygit", PATH);
    excute(&command);
    command = format!("ls -A {}", PATH);
    if expect(&command, |output| {output.contains("delete.txt")})? {
        println!("directory does contain 'delete'");
        return Err("branch error".into());
    }
    println!("directory does not contain 'delete'");
    Ok(())
}