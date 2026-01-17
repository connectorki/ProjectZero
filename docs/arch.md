# Architecture & Tech Stack

## Tech Stack
* **Language:** Rust (latest stable)
* **CLI Framework:** `clap` (with the `derive` feature for type-safe parsing)
* **Error Handling:** `anyhow` (for application-level error handling)
* **Filesystem:** `std::fs`, `std::path`

## Core Modules (Separation of Concerns)

1. **`main.rs`**:
   * Entry point.
   * Initializes CLI parsing.
   * Invokes the high-level logic.
   * Handles top-level errors (prints them in a clean, user-friendly way).

2. **`args.rs` (or `cli.rs`)**:
   * Defines the structure of the command-line arguments (`struct Cli`).
   * Contains subcommands (e.g. `Init`).

3. **`scaffold.rs`**:
   * The actual logic ("business logic").
   * Functions for creating directories and files.

4. **`git_ops.rs`**:
   * Wrapper around Git commands (via `std::process::Command`), used when the `--git` flag is set.

## Data Flow
`User Input` -> `Clap Parser` -> `Config Struct` -> `Scaffold Executor` -> `Disk Writes`
