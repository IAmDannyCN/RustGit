use run;
use std::io::Write;

fn print_with_style(output: Vec<u8>) {
    std::io::stdout().write(&output).unwrap();
}

pub fn excute(s: &str) {
    println!("command: {}", s);
    match run(s) {
        Ok(output) => print_with_style(output),
        Err(e) => eprintln!("Error: {}", e),
    }
}