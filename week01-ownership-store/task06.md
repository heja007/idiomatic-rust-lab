# Rust Backend — Task 06: HTTP API for textkit

## Goal

Implement an HTTP REST API service that exposes stats, uniq, and grep
for text data through a JSON interface.

This task focuses on:

- backend architecture
- HTTP layer and error handling
- serialization / deserialization
- HTTP API testing
- separation of core logic and transport layer

———

## Technologies

Recommended stack:

- axum
- tokio
- serde / serde_json
- thiserror
- tower / tower-http

actix-web is acceptable, but axum is preferred.

———

## API

### POST /v1/stats

#### Request

{
"text": "some\ntext\nhere\n"
}

#### Response

{
"lines": 3,
"words": 3,
"chars": 15,
"bytes": 15
}

#### Rules

- text is required
- max text size — 1 MB
- bytes = text.as_bytes().len()
- chars = text.chars().count()

———

### POST /v1/uniq

#### Request

{
"text": "a\na\nb\nb\nb\nc\n",
"all": false
}

#### Response

{
"text": "a\nb\nc\n",
"removed": 3
}

#### Rules

- Behavior is аналогous to Unix uniq
- Consecutive duplicates are collapsed
- all=true — the chosen semantics must be:
  - explicitly documented in README
  - covered by tests

———

### POST /v1/grep

#### Request

{
"text": "foo\nbar\nfood\n",
"pattern": "foo",
"line_number": true
}

#### Response

{
"matches": [
{ "line": 1, "text": "foo" },
{ "line": 3, "text": "food" }
],
"count": 2
}

#### Rules

- Substring search (not regex)
- pattern is required and non-empty
- line_number=true — line numbers are 1-based

———

## Errors

Unified error format:

{
"error": {
"code": "VALIDATION_ERROR",
"message": "pattern must not be empty"
}
}

### HTTP statuses

- 400 — validation errors
- 413 — text size limit exceeded
- 500 — internal server error

———

## Architecture

Required layer separation:

src/
├── main.rs
├── http/
│ ├── mod.rs
│ ├── handlers.rs
│ ├── types.rs
│ └── error.rs
└── core/
├── mod.rs
├── stats.rs
├── uniq.rs
└── grep.rs

### Requirements

- HTTP layer must not contain business logic
- Core layer must not depend on axum
- All errors are mapped to HTTP responses centrally

———

## Tests

Minimum 8 HTTP tests (integration tests):

1. /v1/stats — correct Unicode counting (chars vs bytes)
2. /v1/stats — empty text
3. /v1/grep — empty pattern → 400
4. /v1/grep — line_number=true
5. /v1/uniq — basic collapsing
6. /v1/uniq — all=true
7. Exceed 1MB limit → 413
8. All endpoints return application/json

———

## Acceptance Criteria

Task is complete if:

- cargo test — ✅
- cargo clippy -- -D warnings — ✅
- cargo fmt — no changes
- README includes:
  - server run instructions
  - curl examples
  - uniq all=true description
  - error format

———

## Optional Extensions

- GET /healthz
- correlation-id middleware
- request tracing (tower-http)
- concurrency limiting
- gzip support

———

## Result

An HTTP service that is:

- stable
- testable
- architecturally separated
- ready to evolve
