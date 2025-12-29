Week 1 · Task 1

Rust core: ownership, errors, iterators, testing

Goal
By the end of this task you should:

- write idiomatic Rust, not “Go/C++ in Rust syntax”
- work confidently with:
  - ownership / borrowing
  - Result / Option
  - iterators instead of for
  - unit tests
- feel why the compiler is right, not fight it

Task: text_stats
You implement a mini text-analysis library.

No I/O, no CLI, no async — only core Rust.

Project structure
cargo new text_stats --lib

src/
├── lib.rs
├── stats.rs
└── errors.rs

Functional requirements

1. Main function
   In stats.rs implement:

pub fn analyze(text: &str) -> Result<TextStats, StatsError>

2. Result structure

pub struct TextStats {
pub lines: usize,
pub words: usize,
pub chars: usize,
pub non_empty_lines: usize,
pub top_word: Option<String>,
}

3. Counting rules

- lines — number of lines (\n)
- words — words separated by whitespace
- chars — number of chars (NOT bytes)
- non_empty_lines — lines containing at least one non-whitespace character
- top_word:
  - case-insensitive
  - ignore punctuation .,! ?;:
  - if no words → None
  - on ties — any

Errors
In errors.rs:

#[derive(Debug, PartialEq)]
pub enum StatsError {
EmptyInput,
}

analyze must return Err(EmptyInput) if:

- text.is_empty()

Tests (required)

- minimum 5 tests
- must cover:
  - empty input
  - Unicode ("Привет мир")
  - multiple lines
  - punctuation
  - repeated words

Restrictions (important)

Forbidden:

- clone() without justification
- unwrap(), expect()
- mutable global state
- for, if it can be expressed via iterators

Allowed:

- HashMap
- iter(), map, filter, fold
- split_whitespace

Review criteria

1. ownership boundaries (&str vs String)
2. iterator mindset
3. correct error handling
4. readability and maintainability
5. meaningful tests

Checklist

- project created
- analyze implemented
- StatsError implemented
- at least 5 tests
- no unwrap/expect
- no unnecessary clone

Submission

- stats.rs
- errors.rs
- tests
