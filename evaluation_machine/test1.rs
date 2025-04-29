use super::*;
const GIT: &str = "../test/git";
const PATH: &str = "./test_area/";

pub fn test() -> Result<(), Box<dyn std::error::Error>> {
    let mut command = format!("{} init -p {}", GIT, PATH);
    excute(&command);
    
    command = format!("tree -a {}.mygit", PATH);
    excute(&command);
    
    command = format!("ls -A {}", PATH);
    if !expect(&command, |output| output.contains(".mygit"))? {
        println!("directory does not contain '.mygit'");
        return Err("directory error".into());
    }
    println!("directory contains '.mygit'");
    
    command = format!("ls -A {}.mygit", PATH);
    if !expect(&command, |output| {
        output.contains("HEAD") && output.contains("refs") && output.contains("objects")
    })? {
        println!("directory does not contain 'HEAD', 'refs', and 'objects'");
        return Err("directory error".into());
    }
    println!("directory contains 'HEAD', 'refs', and 'objects'");
    
    Ok(())
}