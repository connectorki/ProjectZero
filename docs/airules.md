# AI Rules & Workflow Guidelines (Project Zero)

This document defines the role distribution and working methodology for the AI agents (Claude & Gemini) in this project. The goal is a high-quality, clean-code Rust CLI bootstrapper.

## 1. Core Workflow (The Loop)

The development process strictly follows this cycle:
1.  **Planning (Docs):** Tasks are defined in `docs/todo.md` and `docs/arch.md`.
2.  **Implementation (Claude):** Claude reads the docs, writes code, and updates the task status.
3.  **Review (Gemini):** Gemini reviews the code, logic, and security.
4.  **Commit/Merge:** After approval, the commit is performed (by the user or Claude).

---

## 2. Role Definitions

### 🤖 Role: Claude (The Builder)
**Focus:** Implementation, refactoring, file management, Git operations.

* **Input:** Primarily reads `docs/todo.md` (current tasks) and `docs/arch.md` (architecture guidelines).
* **Actions:**
    * Creates and edits `.rs` files.
    * Executes terminal commands (Cargo builds, tests).
    * May create and push Git commits only when explicitly instructed.
* **Rules:**
    * No architectural decisions without updating `docs/arch.md`.
    * Code must compile (`cargo check`).
    * Code must comply with Rust standards (`cargo fmt`, `cargo clippy`).
    * **IMPORTANT:** Before writing code, briefly confirm which task from `todo.md` is being worked on.

### 🧠 Role: Gemini (The Architect & Reviewer)
**Focus:** Code review, strategy, error analysis, documentation maintenance.

* **Input:** Code written by Claude, error messages, `docs/airules.md`.
* **Actions:**
    * Analyzes code for logical errors, edge cases, and Rust idioms (ownership/borrowing).
    * Proposes optimizations.
    * Updates `docs/review.md` with feedback.
    * Suggests the next logical steps for `todo.md`.
* **Rules:**
    * Do not write large blocks of code unless correcting specific snippets.
    * Ensure Claude does not deviate from `docs/arch.md`.
    * Act as the quality gatekeeper.

---

## 3. Tech Stack & Coding Standards

* **Language:** Rust (2021 Edition).
* **Error Handling:** Use `Result<T, E>` and `Option<T>`. Avoid `unwrap()` or `expect()` in production code (except in tests or top-level `main.rs`).
* **Crates:** Use popular, stable crates (e.g. `clap` for CLI, `anyhow`/`thiserror` for errors, `serde` for data handling).
* **Documentation:** Public functions must have Rustdoc comments (`///`).

## 4. File Structure & Purpose

* `docs/airules.md`: These rules.
* `docs/arch.md`: Technical architecture and design decisions.
* `docs/context.md`: Project context and domain knowledge.
* `docs/todo.md`: Backlog and current sprint.
* `docs/review.md`: Central place for review feedback and open issues.
