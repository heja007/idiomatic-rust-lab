# Rust Backend Task 07  
## Async HTTP Key-Value Service with Persistence

### 🎯 Goal
Build an asynchronous HTTP service in Rust that provides a key-value API,
supports concurrent access, persists data to disk, and handles errors
correctly.

---

## 📦 Recommended Stack
- tokio
- axum **or** actix-web
- serde / serde_json
- thiserror
- tracing

---

## 🧩 Task Description

Implement an HTTP API for a key-value storage service.

### API Endpoints

```
POST   /kv/:key     -> create or update a value
GET    /kv/:key     -> get value by key
DELETE /kv/:key     -> delete a value
GET    /kv          -> get all key-value pairs
```

### Data Format
- Values: `serde_json::Value`
- All responses must be JSON
- Errors must be returned in a structured JSON format

Example success response:
```json
{
  "key": "username",
  "value": "alice"
}
```

Example error response:
```json
{
  "error": "not_found",
  "message": "key does not exist"
}
```

---

## 🧠 Requirements

### 1. Asynchronous Design
- All I/O operations must be async
- Shared state must be stored in `Arc<RwLock<...>>` or `Arc<Mutex<...>>`

---

### 2. Storage

#### In-memory
- Use `HashMap<String, Value>`

#### Persistence
- Data must be stored in `data.json`
- On service startup:
  - if the file exists → load data from it
  - if not → start with an empty store
- On every mutation (create / update / delete):
  - save the state to disk
  - file writes must be **atomic**
    (write to temp file → rename)

---

### 3. Error Handling

Create a custom error type, for example:

```rust
enum ApiError {
    NotFound,
    InvalidJson,
    Io,
}
```

Map errors to HTTP status codes:
- `NotFound` → `404`
- `InvalidJson` → `400`
- `Io` → `500`

Errors must be serializable to JSON.

---

## 🧪 Testing (Required)

### Unit Tests
- storage operations (insert / get / delete)
- loading and saving data from/to file

### Integration / API Tests
- HTTP requests to all endpoints
- use:
  - `tower::ServiceExt` (axum)
  - or `actix_web::test`

#### Required Test Cases
- request for a non-existing key
- service restart → data is preserved
- updating an existing value

---

## ⭐ Optional Levels

### Level 2
- graceful shutdown (`ctrl+c`)
- logging with `tracing`

### Level 3
- `/health` endpoint
- optimistic locking (ETag / If-Match)

---

## 📁 Suggested Project Structure

```
src/
 ├─ main.rs
 ├─ api/
 │   ├─ mod.rs
 │   └─ handlers.rs
 ├─ storage/
 │   ├─ mod.rs
 │   └─ file.rs
 ├─ error.rs
 └─ model.rs
```

---

## 📚 Topics to Review Before Starting
- tokio: async / await, sync primitives
- axum / actix-web fundamentals
- serde_json::Value
- Arc, Mutex, RwLock
- Error handling in Rust

---

## ✅ Completion Criteria
- project compiles without warnings
- all tests pass
- data persists between restarts
- code is clean and well-structured

Good luck 🚀
