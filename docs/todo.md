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
- [ ] Implement `--docs` flag to generate documentation folder
- [ ] Add tests for scaffolding logic
- [ ] Improve error messages and user feedback

## Phase 5: CI/CD & Release
- [ ] Set up GitHub Actions workflow for Rust
- [ ] Prepare first binary release
