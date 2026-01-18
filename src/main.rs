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
            // TODO: Implement scaffolding logic
        }
    }

    Ok(())
}
