# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands
- Build: `cargo build`
- Run: `cargo run -- [args]`
- Run with release optimizations: `cargo run --release -- [args]`
- Test: `cargo test`
- Lint: `cargo clippy`

## Code Style Guidelines
- Use Rust 2024 edition conventions
- Follow standard Rust naming conventions:
  - snake_case for variables, functions, and files
  - CamelCase for types and structs
  - SCREAMING_SNAKE_CASE for constants
- Properly handle errors using Result<T, E> and the ? operator
- Use descriptive variable names
- Use consistent indentation (4 spaces)
- Organize imports alphabetically and group by standard library, external crates, and internal modules
- Prefer functional approaches where appropriate
- Document public functions and modules with proper comments