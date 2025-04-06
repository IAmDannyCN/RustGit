pub fn remove(files: Vec<String>) {
    println!("rm called!");
    for file in files {
        println!("  removing: {}", file);
    }
}
