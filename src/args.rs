//! Command-line argument parsing for Project Zero.

use clap::{Parser, Subcommand};

/// A CLI bootstrapper for creating new projects.
#[derive(Parser, Debug)]
#[command(name = "project-zero")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Available subcommands.
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize a new project with the given name.
    Init {
        /// The name of the project to create.
        name: String,

        /// Initialize a Git repository in the project.
        #[arg(long, short)]
        git: bool,

        /// Generate documentation starter files in the docs folder.
        #[arg(long, short)]
        docs: bool,
    },
}
