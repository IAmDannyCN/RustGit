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
        #[arg(short = 'r', long)]
        recursive: bool,

        /// Force remove the original file in working area.
        #[arg(short = 'f', long)]
        force: bool,

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
        #[arg(short, long)]
        delete: bool,

        #[clap(flatten)]
        common: CommonArgs,
    },

    /// Switch to another branch
    Checkout {
        /// Target branch name
        #[arg(short, long)]
        branch: String,

        #[clap(flatten)]
        common: CommonArgs,
    },

    /// Merge the specified branch into the current one
    Merge {
        /// Branch to merge from
        #[arg(short, long)]
        branch: String,

        #[clap(flatten)]
        common: CommonArgs,
    },
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
            add(files);
        }
        Commands::Rm { files, recursive, force, common } => {
            utils::utils::set_pwd(&common.path);
            remove(files, recursive, force);
        }
        Commands::Commit { message, common } => {
            utils::utils::set_pwd(&common.path);
            commit_index(message);
        }
        Commands::Branch { name, delete, common } => {
            utils::utils::set_pwd(&common.path);
            branch(name, delete);
        }
        Commands::Checkout { branch, common } => {
            utils::utils::set_pwd(&common.path);
            checkout(branch);
        }
        Commands::Merge { branch, common } => {
            utils::utils::set_pwd(&common.path);
            merge(branch);
        }
    }
}
