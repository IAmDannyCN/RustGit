use crate::utils::*;

use std::process;

pub fn init() {
    let git_directory: String = utils::get_git_directory();

    use storage::create_nonexist_directory;
    use storage::create_nonexist_file;

    create_nonexist_directory(&git_directory);
    create_nonexist_directory(&format!("{}/branches", git_directory));
    create_nonexist_directory(&format!("{}/logs", git_directory));
    create_nonexist_directory(&format!("{}/objects", git_directory));
    create_nonexist_directory(&format!("{}/objects/info", git_directory));
    create_nonexist_directory(&format!("{}/objects/pack", git_directory));
    create_nonexist_directory(&format!("{}/refs", git_directory));
    create_nonexist_directory(&format!("{}/refs/heads", git_directory));
    create_nonexist_directory(&format!("{}/refs/tags", git_directory));
    create_nonexist_file(&format!("{}/COMMIT_EDITMSG", git_directory));
    create_nonexist_file(&format!("{}/HEAD", git_directory));
    create_nonexist_file(&format!("{}/index", git_directory));

    if let Err(e) = storage::write_file( &format!("{}/HEAD", git_directory), 
                                        "ref: refs/heads/master") {
        println!("Cannot write to {}/HEAD: {}", git_directory, e);
        process::exit(1);
    }
}
