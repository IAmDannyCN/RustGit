use super::*;
const GIT: &str = "../test/git";
const PATH: &str = "./test_area/";

pub fn test() {
    let command = format!("{} init -p {}", GIT, PATH);
    excute(&command);
    let command = format!("tree -a {}.mygit", PATH);
    excute(&command);
    let command = format!("tree -a {}.mygit", PATH);
    match expect(&command, |output| {
        output.contains(".mygit")
    }) {
        Ok(true) => println!("✅ Output contains only '.mygit'"),
        Ok(false) => println!("❌ Output does not contain only '.mygit'"),
        Err(e) => eprintln!("Error: {}", e),
    }
}