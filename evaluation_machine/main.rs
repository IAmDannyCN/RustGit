use std::fs::File;
use std::io::{self, Read};
use std::process::Command;

mod test;//执行的测试样例\
use test::excute;


fn main() -> io::Result<()> {
    // 打开当前目录下的test.txt 文件
    let mut file = File::open("./test.txt")?;

    // 创建一个字符串来存储文件内容
    let mut contents = String::new();

    // 读取文件内容到字符串
    file.read_to_string(&mut contents)?;

    // 打印文件内容
    excute(&contents);
    Ok(())
}

pub fn run(command: &str) -> io::Result<Vec<u8>> {
    // 将命令字符串分割成命令和参数
    let parts: Vec<&str> = command.split_whitespace().collect();

    // 获取命令和参数
    let (cmd, args) = parts.split_first().unwrap();

    // 执行命令
    let output = Command::new(cmd)
        .args(args)
        .output()?;

    // 检查命令是否成功执行
    if output.status.success() {
        Ok(output.stdout)
    }
    else{
        Err(io::Error::new(
            io::ErrorKind::Other,
            String::from_utf8_lossy(&output.stderr).to_string(),
        ))
    }
}


/*

void run(string s);
void get(string s);

bool expect(string cmd, string s);
bool expect(string cmd, function<bool(string s)> checker);

*/