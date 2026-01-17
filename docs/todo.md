# Todo List - Project Zero

## Phase 1: Foundation (Setup & CLI)
- [ ] Initialize Cargo project (`cargo init`)
- [ ] Add dependencies to `Cargo.toml` (`clap`, `anyhow`)
- [ ] Implement basic CLI structure in `src/args.rs`
- [ ] Set up error handling in `src/main.rs`

## Phase 2: Core Logic (Scaffolding)
- [ ] Implement folder creation logic in `src/scaffold.rs`
- [ ] Implement file creation logic (README, etc.)
- [ ] Add support for custom project names

## Phase 3: Git Integration
- [ ] Implement `git init` wrapper in `src/git_ops.rs`
- [ ] Add optional `--git` flag to the CLI

## Phase 4: Documentation & Polish
- [ ] Implement `--docs` flag to generate documentation folder
- [ ] Add tests for scaffolding logic
- [ ] Improve error messages and user feedback

## Phase 5: CI/CD & Release
- [ ] Set up GitHub Actions workflow for Rust
- [ ] Prepare first binary release
