pub fn print(s: &str) {
    println!("{}", s);
}

pub fn excute(s: &str) {
    let output = crate::run(s).unwrap();
    println!("{}", output);
}