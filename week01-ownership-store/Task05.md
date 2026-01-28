# Task05 — File-backed Store (Persistence)

## Goal
Implement persistence for the in-memory `Store`, so data is saved to disk and correctly restored on application startup.

---

## Tasks

### 1. `persistence` module
Create a `persistence` module with a public API:

```rust
pub fn load_from_file(path: impl AsRef<Path>) -> Result<Store, StoreError>;
pub fn save_to_file(store: &Store, path: impl AsRef<Path>) -> Result<(), StoreError>;
```

Storage format: **JSON** (`serde`, `serde_json`).

---

### 2. Atomic save
`save_to_file` must be atomic:
- write to a temporary file (`.tmp`) in the same directory
- `rename` to the final file
- on error, the original file must not be corrupted
Note: `rename` is atomic only within the same filesystem.

---

### 3. Format versioning

```json
{
  "version": 1,
  "records": {
    "key1": { "id": 1, "payload": [1, 2, 3] },
    "key2": { "id": 2, "payload": [4, 5] }
  }
}
```

Behavior:
- if the file is missing → return an empty `Store`
- if the version is unknown → `StoreError::UnsupportedVersion(u32)`

---

### 4. Errors
Extend `StoreError`:
- I/O errors
- serialization errors (`serde_json`)
- unsupported version
- (optional) corrupted format

Errors should support `?` (`From` / `thiserror`).

---

### 5. Tests (required)

Minimum:

1. Loading a missing file → empty `Store`
2. Save → Load roundtrip
3. After saving, no `.tmp` file remains
4. Unsupported version → error
5. (optional) broken JSON

Use `tempfile`.

---

## Constraints
- `Store` remains in-memory
- Persistence is a separate module
- No `panic!` in prod code
- Readable and tidy code

---

## Definition of Done
- `load_from_file` / `save_to_file` implemented
- Atomic save
- `version = 1` supported
- All tests pass
- `cargo test` is green
