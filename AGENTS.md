# Repository Guidelines

## Project Structure & Module Organization
- Root is a Rust workspace (`Cargo.toml`) with two members: `week01-text-stats/` and `week01-ownership-store/`.
- Each weekly directory is a focused exercise with its own `src/` and task notes (`TASK*.md`).
- `week01-text-stats/` also contains nested crates in `text_stats/` (library) and `textkit/` (CLI + lib). Integration tests live in `week01-text-stats/textkit/tests/`.
- `target/` directories are build artifacts and should not be edited.

## Build, Test, and Development Commands
- `cargo build` (from repo root): build the workspace.
- `cargo test` (from repo root): run all workspace tests.
- `cargo test -p week01-ownership-store`: run tests for that crate only.
- `cargo test` (from `week01-text-stats/textkit/`): run the CLI crate tests, including `tests/cli.rs`.
- `cargo run -p week01-text-stats` or `cargo run` from a crate directory to run examples/binaries if present.

## Coding Style & Naming Conventions
- Follow standard Rust formatting (run `cargo fmt` if you change formatting). No custom rustfmt config is present.
- Keep module names and filenames lowercase with underscores (e.g., `src/error.rs`, `src/store.rs`).
- Prefer clear, small modules; group domain types in `model.rs` and error types in `error.rs`/`errors.rs` where used.

## Testing Guidelines
- Use Rust’s built-in test framework (`#[test]`) and integration tests in `tests/`.
- Name tests after behavior (e.g., `parses_empty_input`), and keep input fixtures inline unless they are reused.
- Run `cargo test` in the relevant crate directory before submitting changes.

## Commit & Pull Request Guidelines
- Commit messages in this repo are short and descriptive (e.g., `week01: ownership store`, `second task`).
- Use a simple format: optional scope + concise summary (`week01: <summary>`).
- PRs (or change requests) should include: a short description, tests run, and a link to the relevant `TASK*.md` when applicable.

## Notes for Agents
- Keep edits focused to the relevant week’s directory.
- Avoid modifying `target/` and generated files.
