mod args;
mod git_ops;
mod scaffold;

use anyhow::Result;
use clap::Parser;

use crate::args::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { name, git } => {
            println!("Initializing project: {}", name);
            scaffold::create_project(&name)?;

            if git {
                println!("Initializing Git repository...");
                git_ops::init_repository(std::path::Path::new(&name))?;
                println!("Git repository initialized.");
            }

            println!("Project '{}' created successfully!", name);
        }
    }

    Ok(())
}
