use clap::{Parser, Subcommand};

mod commands;
mod utils;
mod core;

use commands::{
    add::add,
    branch::branch,
    checkout::checkout,
    commit::commit,
    init::init,
    merge::merge,
    rm::remove,
};


#[derive(Parser)]
#[command(name = "git")]
#[command(version = "0.1.0")]
#[command(about = "A version control system written in Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new Git repository
    Init {
        /// Allow recursive removal when a leading directory name is given.
        #[arg(short = 'b', long)]
        initial_branch: Option<String>,
    },

    /// Add file(s) to the index (staging area)
    Add {
        /// List of files to add
        files: Vec<String>,
    },

    /// Remove file(s) from the index
    Rm {
        /// Allow recursive removal when a leading directory name is given.
        #[arg(short = 'r', long)]
        recursive: bool,

        /// Override the up-to-date check.
        #[arg(short = 'f', long)]
        force: bool,

        /// Use this option to unstage and remove paths only from the index. Working tree files, whether modified or not, will be left alone.
        #[arg(long)]
        cached: bool,

        /// List of files to remove
        files: Vec<String>,
    },

    /// Commit staged changes
    Commit {
        /// Commit message
        #[arg(short, long)]
        message: String,
    },

    /// Create, list, or delete branches
    Branch {
        /// Branch name (if omitted, list branches)
        name: Option<String>,

        /// Delete the specified branch
        #[arg(short, long)]
        delete: bool,
    },

    /// Switch to another branch
    Checkout {
        /// Target branch name
        #[arg(short, long)]
        branch: String,
    },

    /// Merge the specified branch into the current one
    Merge {
        /// Branch to merge from
        #[arg(short, long)]
        branch: String,
    },
}

fn main() {
    
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { initial_branch} => init(initial_branch),
        Commands::Add { files } => add(files),
        Commands::Rm { files, recursive, force, cached } =>
            remove(files, recursive, force, cached),
        Commands::Commit { message } => commit(message),
        Commands::Branch { name, delete } => branch(name, delete),
        Commands::Checkout { branch } => checkout(branch),
        Commands::Merge { branch } => merge(branch),
    }
}
