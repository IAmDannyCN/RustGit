//! A version control system implementing a subset features of Git, written in Rust.
//!
//! Supported commands include:
//! - `init`: Initialize a new repository
//! - `add`: Stage files into the index
//! - `rm`: Remove files from the index or working directory
//! - `commit`: Record staged changes with a message
//! - `branch`: Create, delete, or list branches
//! - `checkout`: Switch branch or commit
//! - `merge`: Merge two branches
//! - `status`: Show working tree status
//! - `log`: View commit history
//! 
//! Support additional features include:
//! - Detailed, beaufitied output (add `-v` or `--verbose`)
//! - Operating multiple files or folders in one command
//! - `git log`, `git status` and `git branch`
//! - Out-of-folder operation, can assign a working directory by `-p` or `--path`
//! - DETACH HEAD Mode
//! - Operation avaliability check (check for uncommited changes/stages before checkout or merge), can be turned on by `-f false` or `--force false`
//! 
//! Behaviors different to Git:
//! - Use self-designed structure and base64 encoding for object storage (binary-INcompatible with Git)
//! - `-f` or `--force` is ON BY DEFAULT
//! - `-v` or `--verbose` is OFF BY DEFAULT

use clap::{Args, Parser, Subcommand};

mod commands;
mod utils;
mod core;

use commands::{
    add::add,
    branch::branch,
    checkout::checkout,
    commit::commit_index,
    init::init,
    merge::merge,
    rm::remove,
    status::status,
    log::log,
};


#[derive(Parser)]
#[command(name = "git")]
#[command(version = "0.1.0")]
#[command(about = "A version control system written in Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Args)]
struct CommonArgs {
    /// Working directory path
    #[arg(short = 'p', long, default_value = ".")]
    path: String,

    /// Print verbose information
    #[arg(short = 'v', long, default_value = "false")]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new Git repository
    Init {
        /// Use the specified name for the initial branch in the newly created repository.
        #[arg(short = 'b', long)]
        initial_branch: Option<String>,

        #[clap(flatten)]
        common: CommonArgs,
    },

    /// Add file(s) to the index (staging area)
    Add {
        /// List of files to add
        files: Vec<String>,

        #[clap(flatten)]
        common: CommonArgs,
    },

    /// Remove file(s) from the index
    Rm {
        /// Allow recursive removal when a leading directory name is given.
        #[arg(short = 'r', long, default_value = "false")]
        recursive: bool,

        /// Only remove cached files (in staging area).
        #[arg(long, default_value = "false")]
        cached: bool,

        /// List of files to remove
        files: Vec<String>,

        #[clap(flatten)]
        common: CommonArgs,
    },

    /// Commit staged changes
    Commit {
        /// Commit message
        #[arg(short, long)]
        message: String,

        #[clap(flatten)]
        common: CommonArgs,
    },

    /// Create, list, or delete branches
    Branch {
        /// Branch name (if omitted, list branches)
        name: Option<Vec<String>>,

        /// Delete the specified branch
        #[arg(short = 'd', long, default_value = "false")]
        delete: bool,

        #[clap(flatten)]
        common: CommonArgs,
    },

    /// Switch to another branch
    Checkout {
        /// Target branch name
        target: String,

        /// Force checkout even there are uncommited changes.
        #[arg(short = 'f', long, default_value = "true")]
        force: bool,

        /// Create a new branch and checkout
        #[arg(short = 'b', long, default_value = "false")]
        branch: bool,

        #[clap(flatten)]
        common: CommonArgs,
    },

    /// Merge the specified branch into the current one
    Merge {
        /// Branch to merge from
        branch: String,

        /// Force merge even there are uncommited changes.
        #[arg(short = 'f', long, default_value = "true")]
        force: bool,

        #[clap(flatten)]
        common: CommonArgs,
    },

    /// Displays paths that have differences between the index file and the current HEAD commit
    Status {
        #[clap(flatten)]
        common: CommonArgs,
    },

    /// Displays the log of commits
    Log {
        #[clap(flatten)]
        common: CommonArgs,
    }
}

fn main() {
    
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { initial_branch, common } => {
            utils::utils::set_pwd(&common.path);
            init(initial_branch);
        }
        Commands::Add { files, common } => {
            utils::utils::set_pwd(&common.path);
            add(files, common.verbose);
        }
        Commands::Rm { files, recursive, cached, common } => {
            utils::utils::set_pwd(&common.path);
            remove(files, recursive, cached, common.verbose);
        }
        Commands::Commit { message, common } => {
            utils::utils::set_pwd(&common.path);
            commit_index(message, common.verbose);
        }
        Commands::Branch { name, delete, common } => {
            utils::utils::set_pwd(&common.path);
            branch(name, delete, common.verbose);
        }
        Commands::Checkout { target, force, branch, common } => {
            utils::utils::set_pwd(&common.path);
            checkout(target, force, branch, common.verbose);
        }
        Commands::Merge { branch, force, common } => {
            utils::utils::set_pwd(&common.path);
            merge(branch, force);
        }
        Commands::Status { common } => {
            utils::utils::set_pwd(&common.path);
            status();
        }
        Commands::Log { common } => {
            utils::utils::set_pwd(&common.path);
            log();
        }
    }
}
