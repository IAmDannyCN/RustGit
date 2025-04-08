use std::fs;
use std::io;

pub fn read_file(file_name: &str) -> Result<String, io::Error> {
    fs::read_to_string(file_name)
}

pub fn write_file(file_name: &str, contents: &str) -> io::Result<()> {
    fs::write(file_name, contents)
}