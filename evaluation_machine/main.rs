use std::fs::File;
use std::io::{self, Read};
use std::process::Command;



fn main() -> io::Result<()> {
    // 打开当前目录下的test.txt 文件
    let mut file = File::open("./test.txt")?;

    // 创建一个字符串来存储文件内容
    let mut contents = String::new();

    // 读取文件内容到字符串
    file.read_to_string(&mut contents)?;

    // 打印文件内容
    println!("{}", contents);

    Ok(())
}

/*

void run(string s);
void get(string s);

bool expect(string cmd, string s);
bool expect(string cmd, function<bool(string s)> checker);

*/