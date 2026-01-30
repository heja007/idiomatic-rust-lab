# Week01 Ownership Store — Task06 HTTP API

This crate exposes a small HTTP JSON API for text processing using the `textkit` core library.

## Run

```bash
cargo run -p week01-ownership-store
```

Server listens on `http://127.0.0.1:3000`.

## Endpoints

### POST `/v1/stats`
Counts lines, words, chars, bytes.

```bash
curl -s -X POST http://127.0.0.1:3000/v1/stats \
  -H 'Content-Type: application/json' \
  -d '{"text":"some\ntext\n"}'
```

### POST `/v1/uniq`
Collapses duplicate lines.

```bash
curl -s -X POST http://127.0.0.1:3000/v1/uniq \
  -H 'Content-Type: application/json' \
  -d '{"text":"a\na\nb\nb\n","all":false}'
```

`all=true` semantics: keep the first occurrence of each unique line (non-consecutive duplicates are removed as well).

### POST `/v1/grep`
Substring search, optional line numbers.

```bash
curl -s -X POST http://127.0.0.1:3000/v1/grep \
  -H 'Content-Type: application/json' \
  -d '{"text":"foo\nbar\nfood\n","pattern":"foo","line_number":true}'
```

## Errors

Errors use a unified JSON format:

```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "pattern must not be empty"
  }
}
```

### Status codes

- `400` — validation errors
- `413` — payload too large (text > 1MB)
- `500` — internal server error
