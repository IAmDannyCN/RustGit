use std::fs;
use std::io;
use std::process;

pub fn read_file(file_name: &str) -> Result<String, io::Error> {
    fs::read_to_string(file_name)
}

pub fn write_file(file_name: &str, contents: &str) -> io::Result<()> {
    fs::write(file_name, contents)
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
            eprintln!("Directory {} already exists", directory_name);
            process::exit(1);
        }
        Err(e) => {
            eprintln!("Error checking the exsitence of directory {}: {}", directory_name, e);
            process::exit(1);
        }
    }

    create_directory(&directory_name);
}

pub fn create_nonexist_file(file_name: &str) {
    match fs::exists(&file_name) {
        Ok(res) => if res == true {
            eprintln!("File {} already exists", file_name);
            process::exit(1);
        }
        Err(e) => {
            eprintln!("Error checking the exsitence of file {}: {}", file_name, e);
            process::exit(1);
        }
    }

    create_file(&file_name);
}