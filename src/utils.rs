use std::fs;
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
            process::exit(0)
        },
    };
    current_directory.push_str("/.git");
    current_directory
}

pub fn create_directory(directory_name: &str) {
    if let Err(e) = fs::create_dir_all(directory_name) {
        eprintln!("Failed to create directory '{}': {}", directory_name, e);
        process::exit(1);
    }
}

pub fn create_file(file_name: &str) {
    if let Err(e) = fs::File::create(file_name) {
        eprintln!("Failed to create file '{}': {}", file_name, e);
        process::exit(1);
    }
}

pub fn create_nonexist_directory(directory_name: &str) {
    match fs::exists(&directory_name) {
        Ok(res) => if res == true {
            println!("Directory {} already exists", directory_name);
            process::exit(0);
        }
        Err(e) => {
            println!("Error checking the exsitence of directory {}: {}", directory_name, e);
            process::exit(0);
        }
    }

    create_directory(&directory_name);
}

pub fn create_nonexist_file(file_name: &str) {
    match fs::exists(&file_name) {
        Ok(res) => if res == true {
            println!("File {} already exists", file_name);
            process::exit(0);
        }
        Err(e) => {
            println!("Error checking the exsitence of file {}: {}", file_name, e);
            process::exit(0);
        }
    }

    create_file(&file_name);
}

