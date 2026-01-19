# rust-command-lines-tools

[![License: Unlicense](https://img.shields.io/badge/license-Unlicense-blue.svg)](LICENSE)
[![Dependabot Status](https://img.shields.io/badge/dependabot-enabled-brightgreen.svg)](./.github/dependabot.yml)
[![CI](https://img.shields.io/badge/CI-GitHub%20Actions-blue.svg)](./.github/workflows/rust-ci.yml)

This repository is a collection of Rust exercises (each exercise is a separate Cargo crate). It is organized as a workspace to make building, testing, and linting across all exercises convenient.

Goals
- Small, isolated exercises that are easy to run and test.
- Shared developer tooling (single `target/` cache, centralized CI).
- Clear examples of best practices for small Rust projects.

Repository layout
- `rust-command-lines-tools/` — workspace root
  - `exercises/` — one directory per exercise (each is a Cargo crate)
  - `.github/workflows/rust-ci.yml` — CI workflow for linting & testing
  - `.github/dependabot.yml` — Dependabot configuration
  - `LICENSE` — repo license

Key files
- Workspace manifest: `Cargo.toml` (at the repo root when configured)
- CI workflow: `.github/workflows/rust-ci.yml`
- Dependabot config: `.github/dependabot.yml`

Quick start (local)
- To build the entire workspace:
```bash
cargo build --workspace
```

- To run a single exercise (from workspace root):
```bash
cargo run -p <package-name>
# Example:
# cargo run -p 01_hello_world
```

- To test the entire workspace:
```bash
cargo test --workspace
```

- To check formatting and lint locally:
```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
```

Creating a new exercise
1. Create a new binary crate inside `exercises/`:
```bash
cargo new --bin exercises/03_my_exercise
```

2. Implement logic in `src/lib.rs` (recommended) and use a thin `src/main.rs` to call into it. Example structure:
```markdown
exercises/03_my_exercise/
├─ Cargo.toml
└─ src/
   ├─ lib.rs    # core, testable logic
   └─ main.rs   # small runner calling into the library
```

Minimal `main.rs` example
```rust
fn main() {
    println!("Hello, rust-fundamentals!");
}
```
