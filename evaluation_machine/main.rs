use std::io::{self, Write};
use std::process::Command;

mod test2s;
use test2s::test;

fn main() -> io::Result<()> {
    let _ = test();
    Ok(())
}

// run a command and print the output
pub fn excute(s: &str) {
    println!("command: {}", s);
    match run(s) {
        Ok(output) => {
            std::io::stdout().write(&output).unwrap();
        },
        Err(e) => eprintln!("Error: {}", e),
    }
}

// excute a command and return the output
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

// excute a command and check the output with a closure
pub fn expect<F>(command: &str, checker: F) -> io::Result<bool>
where
    F: Fn(&str) -> bool,
{
    let output_bytes = run(command)?;
    let output_str = String::from_utf8_lossy(&output_bytes);
    Ok(checker(&output_str))
}

