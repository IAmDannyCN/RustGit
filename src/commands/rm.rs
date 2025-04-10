pub fn remove(files: Vec<String>, recursive: bool, force: bool, cached: bool) {
    println!("rm called!");
    println!("recursive = {}, force = {}, cached = {}", recursive, force, cached);
    for file in files {
        println!("  removing: {}", file);
    }
}
