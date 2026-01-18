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
        anyhow::bail!(
            "Cannot create project: directory '{}' already exists. \
            Please choose a different name or remove the existing directory.",
            project_name
        );
    }

    // Validate project name
    if project_name.is_empty() {
        anyhow::bail!("Project name cannot be empty. Please provide a valid project name.");
    }

    if project_name.contains(std::path::MAIN_SEPARATOR) {
        anyhow::bail!(
            "Project name '{}' contains path separators. \
            Please provide a simple directory name without slashes.",
            project_name
        );
    }

    // Create main project directory
    fs::create_dir(project_path).with_context(|| {
        format!(
            "Failed to create project directory '{}'. \
            Check that you have write permissions in the current directory.",
            project_name
        )
    })?;

    // Create standard subdirectories
    let subdirs = ["src", "docs", "tests"];
    for subdir in subdirs {
        let subdir_path = project_path.join(subdir);
        fs::create_dir(&subdir_path).with_context(|| {
            format!(
                "Failed to create '{}' directory. The project may be in an incomplete state.",
                subdir_path.display()
            )
        })?;
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

/// Creates documentation starter files in the project's docs folder.
///
/// # Arguments
///
/// * `project_path` - The path to the project root.
/// * `project_name` - The name of the project.
///
/// # Errors
///
/// Returns an error if the documentation files cannot be created.
pub fn create_documentation(project_path: &Path, project_name: &str) -> Result<()> {
    let docs_path = project_path.join("docs");

    // Create ARCHITECTURE.md
    let arch_content = format!(
        "# {} - Architecture\n\n## Overview\n\nTODO: Describe the high-level architecture.\n\n## Components\n\nTODO: List and describe major components.\n\n## Data Flow\n\nTODO: Explain how data flows through the system.\n",
        project_name
    );
    create_file(&docs_path.join("ARCHITECTURE.md"), &arch_content)?;

    // Create TODO.md
    let todo_content = format!(
        "# {} - Task List\n\n## In Progress\n\n- [ ] Initial project setup\n\n## Backlog\n\n- [ ] Add core functionality\n- [ ] Write tests\n- [ ] Add documentation\n\n## Done\n\n- [x] Project scaffolding\n",
        project_name
    );
    create_file(&docs_path.join("TODO.md"), &todo_content)?;

    // Create CONTRIBUTING.md
    let contrib_content = format!(
        "# Contributing to {}\n\nThank you for your interest in contributing!\n\n## How to Contribute\n\n1. Fork the repository\n2. Create a feature branch\n3. Make your changes\n4. Submit a pull request\n\n## Code Style\n\nTODO: Describe code style guidelines.\n\n## Questions?\n\nTODO: Add contact information.\n",
        project_name
    );
    create_file(&docs_path.join("CONTRIBUTING.md"), &contrib_content)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    #[serial]
    fn test_create_project_success() {
        let temp = tempdir().expect("Failed to create temp directory");
        let project_name = "test-project";
        let project_path = temp.path().join(project_name);

        // Change to temp directory for the test
        let original_dir = std::env::current_dir().expect("Failed to get current dir");
        std::env::set_current_dir(temp.path()).expect("Failed to change dir");

        let result = create_project(project_name);
        std::env::set_current_dir(original_dir).expect("Failed to restore dir");

        assert!(result.is_ok(), "create_project should succeed");
        assert!(project_path.exists(), "Project directory should exist");
        assert!(
            project_path.join("src").exists(),
            "src directory should exist"
        );
        assert!(
            project_path.join("docs").exists(),
            "docs directory should exist"
        );
        assert!(
            project_path.join("tests").exists(),
            "tests directory should exist"
        );
        assert!(
            project_path.join("README.md").exists(),
            "README.md should exist"
        );
        assert!(
            project_path.join("src").join("main.rs").exists(),
            "src/main.rs should exist"
        );
    }

    #[test]
    #[serial]
    fn test_create_project_already_exists() {
        let temp = tempdir().expect("Failed to create temp directory");
        let project_name = "existing-project";

        // Create the directory first
        fs::create_dir(temp.path().join(project_name)).expect("Failed to create test dir");

        let original_dir = std::env::current_dir().expect("Failed to get current dir");
        std::env::set_current_dir(temp.path()).expect("Failed to change dir");

        let result = create_project(project_name);
        std::env::set_current_dir(original_dir).expect("Failed to restore dir");

        assert!(
            result.is_err(),
            "create_project should fail for existing directory"
        );
        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("already exists"),
            "Error message should mention 'already exists'"
        );
    }

    #[test]
    fn test_create_file_success() {
        let temp = tempdir().expect("Failed to create temp directory");
        let file_path = temp.path().join("test.txt");
        let content = "Hello, World!";

        let result = create_file(&file_path, content);

        assert!(result.is_ok(), "create_file should succeed");
        assert!(file_path.exists(), "File should exist");
        let read_content = fs::read_to_string(&file_path).expect("Failed to read file");
        assert_eq!(read_content, content, "File content should match");
    }

    #[test]
    fn test_create_documentation() {
        let temp = tempdir().expect("Failed to create temp directory");
        let project_name = "doc-test-project";

        // Create the docs directory first (mimicking create_project behavior)
        let docs_path = temp.path().join("docs");
        fs::create_dir(&docs_path).expect("Failed to create docs dir");

        let result = create_documentation(temp.path(), project_name);

        assert!(result.is_ok(), "create_documentation should succeed");
        assert!(
            docs_path.join("ARCHITECTURE.md").exists(),
            "ARCHITECTURE.md should exist"
        );
        assert!(docs_path.join("TODO.md").exists(), "TODO.md should exist");
        assert!(
            docs_path.join("CONTRIBUTING.md").exists(),
            "CONTRIBUTING.md should exist"
        );

        // Verify content contains project name
        let arch_content =
            fs::read_to_string(docs_path.join("ARCHITECTURE.md")).expect("Failed to read file");
        assert!(
            arch_content.contains(project_name),
            "ARCHITECTURE.md should contain project name"
        );
    }

    #[test]
    #[serial]
    fn test_readme_contains_project_name() {
        let temp = tempdir().expect("Failed to create temp directory");
        let project_name = "my-awesome-project";
        let project_path = temp.path().join(project_name);

        let original_dir = std::env::current_dir().expect("Failed to get current dir");
        std::env::set_current_dir(temp.path()).expect("Failed to change dir");

        create_project(project_name).expect("create_project should succeed");
        std::env::set_current_dir(original_dir).expect("Failed to restore dir");

        let readme_content =
            fs::read_to_string(project_path.join("README.md")).expect("Failed to read README");
        assert!(
            readme_content.contains(project_name),
            "README should contain project name"
        );
    }
}
