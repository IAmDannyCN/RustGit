use std::process;
use std::env;

pub fn pwd() -> Result<String, std::io::Error> {
    match env::current_dir() {
        Ok(path) => Ok(path.to_string_lossy().into_owned()),
        Err(e) => Err(e),
    }
}

pub fn get_git_directory() -> String {
    let mut current_directory: String;
    match pwd() {
        Ok(path) => current_directory = path,
        Err(e) => {
            println!("Error when fetching git directory: {}", e);
            process::exit(1)
        },
    };
    current_directory.push_str("/.git");
    current_directory
}