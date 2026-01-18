mod args;
mod git_ops;
mod scaffold;

use anyhow::Result;
use clap::Parser;

use crate::args::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { name, git, docs } => {
            println!("[1/4] Creating project structure for '{}'...", name);
            scaffold::create_project(&name)?;
            println!("      Created directories: src/, docs/, tests/");
            println!("      Created files: README.md, src/main.rs");

            if docs {
                println!("[2/4] Generating documentation starter files...");
                scaffold::create_documentation(std::path::Path::new(&name), &name)?;
                println!("      Created: docs/ARCHITECTURE.md, docs/TODO.md, docs/CONTRIBUTING.md");
            } else {
                println!("[2/4] Skipping documentation files (use --docs to generate)");
            }

            if git {
                println!("[3/4] Initializing Git repository...");
                git_ops::init_repository(std::path::Path::new(&name))?;
                println!("      Git repository initialized successfully");
            } else {
                println!("[3/4] Skipping Git initialization (use --git to enable)");
            }

            println!("[4/4] Finalizing...");
            println!();
            println!("Success! Project '{}' has been created.", name);
            println!();
            println!("Next steps:");
            println!("  cd {}", name);
            if !git {
                println!("  git init              # Initialize version control");
            }
            println!("  cargo init            # Set up as a Rust project");
            println!("  cargo run             # Run your project");
        }
    }

    Ok(())
}
