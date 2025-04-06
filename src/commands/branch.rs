pub fn branch(name: Option<String>, delete: bool) {
    println!("branch called!");
    match (name, delete) {
        (Some(branch), true) => println!("  deleting branch: {}", branch),
        (Some(branch), false) => println!("  creating or switching to branch: {}", branch),
        (None, _) => println!("  listing branches"),
    }
}
