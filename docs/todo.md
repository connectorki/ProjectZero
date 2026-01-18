# Todo List - Project Zero

## Phase 1: Foundation (Setup & CLI)
- [x] Initialize Cargo project (`cargo init`)
- [x] Add dependencies to `Cargo.toml` (`clap`, `anyhow`)
- [x] Implement basic CLI structure in `src/args.rs`
- [x] Set up error handling in `src/main.rs`
- [x] Create missing module files (`src/scaffold.rs`, `src/git_ops.rs`)

## Phase 2: Core Logic (Scaffolding)
- [x] Implement folder creation logic in `src/scaffold.rs`
- [x] Implement file creation logic (README, etc.)
- [x] Add support for custom project names

## Phase 3: Git Integration
- [x] Implement `git init` wrapper in `src/git_ops.rs`
- [x] Add optional `--git` flag to the CLI

## Phase 4: Documentation & Polish
- [x] Implement `--docs` flag to generate documentation folder
- [x] Add tests for scaffolding logic
- [x] Improve error messages and user feedback

## Phase 5: CI/CD & Release
- [ ] Set up GitHub Actions workflow (`.github/workflows/ci.yml`)
    - [ ] Run `cargo check`
    - [ ] Run `cargo test`
    - [ ] Run `cargo clippy`
    - [ ] Run `cargo fmt -- --check`
- [ ] Prepare first binary release (manual verification)