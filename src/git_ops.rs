//! Git operations wrapper.
//!
//! This module provides wrapper functions for Git commands using `std::process::Command`.

use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

/// Initializes a new Git repository at the specified path.
///
/// # Arguments
///
/// * `path` - The path where the Git repository should be initialized.
///
/// # Errors
///
/// Returns an error if Git is not installed or the initialization fails.
pub fn init_repository(path: &Path) -> Result<()> {
    let output = Command::new("git")
        .arg("init")
        .current_dir(path)
        .output()
        .context("Failed to execute 'git init'. Is Git installed?")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Git init failed: {}", stderr.trim());
    }

    Ok(())
}
