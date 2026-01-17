# Project Context & Domain Knowledge

## 1. Vision & Motivation
"Project Zero" exists to eliminate the mental resistance involved in starting new Rust projects.  
The goal is not to build the most powerful tool in the world, but a tool that perfectly reflects **our** specific workflow:

* Strict separation of source code (`src/`), documentation (`docs/`), and scripts (`scripts/`).
* Documentation-first approach (therefore automatic creation of `docs/` files).

## 2. User Persona (The Developer)
The user (us) has:

* A **developer mindset**: understands systems but does not want to waste time on boilerplate.
* A strong focus on **Rust**: actively learning the language and valuing idiomatic, safe code (ownership, error handling).
* A clear standard: "clean architecture" from day one.
* A you have to work in english, but speak german with user

## 3. Domain Language (Glossary)
Terms we use consistently throughout the codebase:

* **Scaffold**: The physical folder and file structure written to disk.
* **Blueprint**: The internal definition/template of what a project should look like (before it is written).
* **Project Root**: The base directory of the new project.

## 4. Design Philosophy
* **Opinionated**: We do not ask the user about every detail. We make sensible default decisions (e.g., Git is always included, docs are always included).
* **Minimal**: No unnecessary frills. Only what is required for a clean start.
* **Fail-fast**: If a directory already exists or write permissions are missing -> immediate abort with a clear error message. No half measures.
