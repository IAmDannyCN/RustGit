use std::io::{self, Write};
use std::process::Command;

mod test;//执行的测试样例
use test::test;


fn main() -> io::Result<()> {
    test(); // 执行测试样例
    Ok(())
}

// 运行并打印命令行指令
pub fn excute(s: &str) {
    println!("command: {}", s);
    match run(s) {
        Ok(output) => {
            std::io::stdout().write(&output).unwrap();
        },
        Err(e) => eprintln!("Error: {}", e),
    }
}

// 执行命令行指令并返回结果
pub fn run(command: &str) -> io::Result<Vec<u8>> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()?;

    if output.status.success() {
        Ok(output.stdout)
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            String::from_utf8_lossy(&output.stderr).to_string(),
        ))
    }
}

/// 执行命令行指令并使用指定的检查器检查输出
pub fn expect<F>(command: &str, checker: F) -> io::Result<bool>
where
    F: Fn(&str) -> bool,
{
    let output_bytes = run(command)?;
    let output_str = String::from_utf8_lossy(&output_bytes);
    Ok(checker(&output_str))
}

