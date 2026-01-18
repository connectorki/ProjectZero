mod args;
mod git_ops;
mod scaffold;

use anyhow::Result;
use clap::Parser;

use crate::args::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { name } => {
            println!("Initializing project: {}", name);
            scaffold::create_project(&name)?;
            println!("Project '{}' created successfully!", name);
        }
    }

    Ok(())
}
