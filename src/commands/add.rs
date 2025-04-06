pub fn add(files: Vec<String>) {
    println!("add called!");
    for file in files {
        println!("  adding: {}", file);
    }
}
