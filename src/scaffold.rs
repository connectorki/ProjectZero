//! Project scaffolding logic.
//!
//! This module contains functions for creating project directories and files.

use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

/// Creates a new project directory structure at the specified path.
///
/// # Arguments
///
/// * `project_name` - The name of the project to create.
///
/// # Errors
///
/// Returns an error if the directory already exists or cannot be created.
pub fn create_project(project_name: &str) -> Result<()> {
    let project_path = Path::new(project_name);

    if project_path.exists() {
        anyhow::bail!("Directory '{}' already exists", project_name);
    }

    // Create main project directory
    fs::create_dir(project_path)
        .with_context(|| format!("Failed to create project directory '{}'", project_name))?;

    // Create standard subdirectories
    let subdirs = ["src", "docs", "tests"];
    for subdir in subdirs {
        let subdir_path = project_path.join(subdir);
        fs::create_dir(&subdir_path)
            .with_context(|| format!("Failed to create directory '{}'", subdir_path.display()))?;
    }

    // Create initial files
    create_readme(project_path, project_name)?;
    create_main_rs(project_path)?;

    Ok(())
}

/// Creates a README.md file for the project.
fn create_readme(project_path: &Path, project_name: &str) -> Result<()> {
    let readme_content = format!(
        "# {}\n\nA new project created with Project Zero.\n\n## Getting Started\n\nTODO: Add getting started instructions.\n",
        project_name
    );
    create_file(&project_path.join("README.md"), &readme_content)
}

/// Creates a basic main.rs file in the src directory.
fn create_main_rs(project_path: &Path) -> Result<()> {
    let main_content = "fn main() {\n    println!(\"Hello, world!\");\n}\n";
    create_file(&project_path.join("src").join("main.rs"), main_content)
}

/// Creates a file at the specified path with the given content.
///
/// # Arguments
///
/// * `path` - The path where the file should be created.
/// * `content` - The content to write to the file.
///
/// # Errors
///
/// Returns an error if the file cannot be created or written to.
pub fn create_file(path: &Path, content: &str) -> Result<()> {
    fs::write(path, content)
        .with_context(|| format!("Failed to write file '{}'", path.display()))?;
    Ok(())
}
