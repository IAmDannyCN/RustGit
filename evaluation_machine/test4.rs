use super::*;
const GIT: &str = "../test/git";
const PATH: &str = "./test_area/";

pub fn test() -> Result<(), Box<dyn std::error::Error>> {
    let mut command = format!("{} init -p {}", GIT, PATH);
    excute(&command);
    command = format!("{} checkout -b main -p {}", GIT, PATH);
    excute(&command);
    
    command = format!("touch {}main.rs", PATH);
    let _ = run(&command);
    command = format!("echo \'//main.rs
use std::fs::File;
use std::io::{{self, Read}};
fn main() -> io::Result<()> {{
    // 打开当前目录下的 test.txt 文件
    let mut file = File::open(\"{}test.txt\")?;
    // 创建一个字符串来存储文件内容
    let mut contents = String::new();
    // 读取文件内容到字符串
    file.read_to_string(&mut contents)?;
    // 打印文件内容
    println!(\"{{}}\", contents);
    Ok(())
}}\' > {}main.rs", PATH, PATH);
    let _ = run(&command);

    command = format!("{} add {}. -p {}", GIT, PATH, PATH);
    excute(&command);
    command = format!("{} commit -m \"update main.rs\" -p {}", GIT, PATH);
    excute(&command);

    command = format!("{} branch test -p {}", GIT, PATH);
    excute(&command);
    command = format!("{} checkout test -p {}", GIT, PATH);
    excute(&command);

    command = format!("touch {}test.txt", PATH);
    let _ = run(&command);
    command = format!("echo \"测试分支合并\" > {}test.txt", PATH);
    let _ = run(&command);
    command = format!("{} add {}. -p {}", GIT, PATH, PATH);
    excute(&command);
    command = format!("{} commit -m \"update test.txt\" -p {}", GIT, PATH);
    excute(&command);

    command = format!("{} checkout main -p {}", GIT, PATH);
    excute(&command);
    command = format!("{} merge test -p {}", GIT, PATH);
    excute(&command);

    command = format!("tree -a {}.mygit", PATH);
    excute(&command);
    command = format!("rustc {}main.rs -o {}main", PATH, PATH);
    excute(&command);
    command = format!("{}main | grep \"测试分支合并\"", PATH);
    if expect(&command, |output| {output.is_empty()})? {
        println!("git merge failed, main.rs output is wrong");
        return Err("merge error".into());
    }
    println!("git merge succeeded and main.rs output is correct");
    Ok(())
}