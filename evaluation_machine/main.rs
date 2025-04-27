use std::fs::File;
use std::io::{self, Read};
use std::process::Command;

mod test;//执行的测试样例
use test::print;
use test::excute;


fn main() -> io::Result<()> {
    // 打开当前目录下的test.txt 文件
    let mut file = File::open("./test.txt")?;

    // 创建一个字符串来存储文件内容
    let mut contents = String::new();

    // 读取文件内容到字符串
    file.read_to_string(&mut contents)?;

    // 打印文件内容
    print(&contents);
    excute(&contents);
    Ok(())
}

/// 运行Linux命令并返回带颜色的输出结果
pub fn run(command: &str) -> io::Result<String> {
    // 检查命令是否为空
    if command.trim().is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput, 
            "\x1b[31mError: Empty command\x1b[0m".to_string()
        ));
    }

    // 打印带颜色的命令
    // println!("\x1b[36m$\x1b[0m \x1b[1m{}\x1b[0m", command); // 青色$符号，加粗命令

    // 将命令字符串分割成命令和参数
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "\x1b[31mError: Empty command\x1b[0m".to_string(), // 红色错误信息
        ));
    }

    // 获取命令和参数
    let (cmd, args) = parts.split_first().unwrap();

    // 执行命令
    let output = Command::new(cmd).args(args).output()?;

    // 检查命令是否成功执行
    if output.status.success() {
        // 将输出转换为字符串并添加颜色
        let stdout = String::from_utf8(output.stdout)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("\x1b[31m{}\x1b[0m", e)))?;
        Ok(stdout) // 绿色输出
    } else {
        // 如果命令执行失败，返回红色错误信息
        let stderr = String::from_utf8(output.stderr)
            .unwrap_or_else(|_| "\x1b[31mFailed to decode stderr\x1b[0m".to_string());
        Err(io::Error::new(io::ErrorKind::Other, format!("\x1b[31m{}\x1b[0m", stderr)))
    }
}


/*

void run(string s);
void get(string s);

bool expect(string cmd, string s);
bool expect(string cmd, function<bool(string s)> checker);

*/