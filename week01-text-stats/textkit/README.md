# textkit

A small set of Unix‑style text utilities implemented in Rust.  
The project is structured as a library with pure functions and a thin CLI wrapper.

## Build

```bash
cargo build
```

## Usage

### stats

Counts lines, words, bytes and Unicode scalar values (chars).

```bash
cargo run -- stats <path>
```

Example output:

```text
lines: 10
words: 42
chars: 240
bytes: 256
```

### uniq

Collapses **consecutive duplicate lines**.

```bash
cargo run -- uniq <path>
```

Flag `--all` makes uniqueness across the whole file, preserving first occurrences:

```bash
cargo run -- uniq --all <path>
```

### grep

Prints lines that contain a substring `pattern`.

```bash
cargo run -- grep <pattern> <path>
```

Flags:

- `-n`, `--line-number` — prefix lines with line numbers
- `-i`, `--ignore-case` — ASCII case‑insensitive search

Example:

```bash
cargo run -- grep -n -i error <path>
```

## Tests

```bash
cargo test
```

## Pre‑submit checks

```bash
cargo fmt
cargo clippy -- -D warnings
cargo test
```
