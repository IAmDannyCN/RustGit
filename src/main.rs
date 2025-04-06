use clap::{Parser, Subcommand};

mod commands;

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
    Init,

    /// Add file(s) to the index (staging area)
    Add {
        /// List of files to add
        #[arg(short, long)]
        files: Vec<String>,
    },

    /// Remove file(s) from the index
    Rm {
        /// List of files to remove
        #[arg(short, long)]
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
        Commands::Init => init(),
        Commands::Add { files } => add(files),
        Commands::Rm { files } => remove(files),
        Commands::Commit { message } => commit(message),
        Commands::Branch { name, delete } => branch(name, delete),
        Commands::Checkout { branch } => checkout(branch),
        Commands::Merge { branch } => merge(branch),
    }
}
