use std::io::{self, Write};
use std::process::Command;

mod commands;
use commands::{Command as CmdTrait, CommandResult};

const GIT: &str = "../test/git";
const PATH: &str = "./test_area/";

fn main() -> CommandResult {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: {} <command>", args[0]);
        println!("Available commands: test1, test2, test3, test4, test5, test1s, test2s, test3s");
        return Ok(());
    }

    let command = format!("rm -rf {}{{*,.mygit}} 2>/dev/null", PATH);
    // excute(&command);
    let _ = run(&command);
    
    match args[1].as_str() {
        "test1" => commands::test1::Test1Command.execute()?,
        "test2" => commands::test2::Test2Command.execute()?,
        "test3" => commands::test3::Test3Command.execute()?,
        "test4" => commands::test4::Test4Command.execute()?,
        "test5" => commands::test5::Test5Command.execute()?,
        "test1s" => commands::test1s::Test1sCommand.execute()?,
        "test2s" => commands::test2s::Test2sCommand.execute()?,
        "test3s" => commands::test3s::Test3sCommand.execute()?,
        "testLJH1" => commands::testLJH1::TestLJH1Command.execute()?,
        _ => {
            println!("Unknown command: {}", args[1]);
            println!("Available commands: test1, test2, test3, test4, test5, test1s, test2s, test3s, testLJH1");
        }
    }

    Ok(())
}

// run a command and print the output
pub fn excute(s: &str) {
    println!("command: {}", s);
    match run(s) {
        Ok(output) => {
            std::io::stdout().write_all(&output).unwrap();
        },
        Err(e) => eprintln!("Error: {}", e),
    }
}

// execute a command and return the output
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

// execute a command and check the output with a closure
pub fn expect<F>(command: &str, checker: F) -> io::Result<bool>
where
    F: Fn(&str) -> bool,
{
    let output_bytes = run(command)?;
    let output_str = String::from_utf8_lossy(&output_bytes);
    Ok(checker(&output_str))
}