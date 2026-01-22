Week01 — Ownership Store (Task 03)

Context

Goal — practice ownership/borrowing and API discipline in Rust without hacks (clone, Rc/Arc/RefCell). Build a small in-memory store where function signatures “tell the truth”, and tests lock in the ownership semantics.

Goals
• Practice move / borrow / ownership transfer
• Learn to design APIs without unnecessary clone()
• Master Result/Option + custom errors via thiserror
• Build the habit of writing tests for behavior and contracts

Constraints

Forbidden:
• Rc, Arc, RefCell, any interior mutability
• unsafe
• clone() without explicit necessity (if it appears, leave a comment “why it can’t be otherwise”)
• global mutable structures

Allowed:
• standard library
• thiserror for errors

Required structure

Project: week01-ownership-store

Files:
• src/lib.rs
• src/model.rs
• src/store.rs
• src/error.rs

Domain model

Record — “heavy value” so cloning is expensive:

```rust
pub struct Record {
    pub id: u64,
    pub payload: Vec<u8>,
}
```

API to implement

```rust
pub struct Store { /* internal */ }

impl Store {
    pub fn new() -> Self;

    pub fn insert(&mut self, key: String, value: Record)
        -> Result<(), StoreError>;

    pub fn get(&self, key: &str)
        -> Option<&Record>;

    pub fn remove(&mut self, key: &str)
        -> Result<Record, StoreError>;
}
```

Errors
Create a custom error type:

```rust
pub enum StoreError {
    KeyAlreadyExists,
    KeyNotFound,
}
```

• use thiserror
• errors must be Debug + Display
• errors must implement std::error::Error (thiserror provides it)

Tests (required)

Minimum 4 tests (in store.rs or separate tests/):

1. insert_new_key_ok
   • inserting a new key succeeds
2. insert_duplicate_key_err
   • inserting the same key again returns StoreError::KeyAlreadyExists
3. get_returns_borrowed_ref
   • get returns Option<&Record>
   • in the test, ensure it’s a borrow (e.g., compare id without moving the value)
4. remove_transfers_ownership
   • remove returns Record by ownership
   • after remove, get for this key returns None
   • removing a missing key returns StoreError::KeyNotFound

Acceptance criteria
• cargo test passes
• no Rc/Arc/RefCell, no unsafe
• signatures exactly as in the task
• idiomatic logic (minimal mut, correct borrows)
• clone() absent or justified with a comment

Check commands
• cargo fmt
• cargo clippy -- -D warnings
• cargo test

Definition of Done
• model, error, store implemented
• tests cover contracts
• clippy without warnings
• one commit with message: week01: ownership store
