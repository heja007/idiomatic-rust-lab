Task 04 — extend the API without clone, touch lifetimes and iterators

Add methods (signature 1-to-1):

```rust
impl Store {
    pub fn contains(&self, key: &str) -> bool;

    pub fn payload(&self, key: &str) -> Option<&[u8]>;

    pub fn iter(&self) -> impl Iterator<Item = (&str, &Record)>;

    pub fn rename_key(&mut self, old: &str, new: String) -> Result<(), StoreError>;
}
```

Implementation requirements
• contains: use self.map.contains_key(key)
• payload: self.get(key).map(|r| r.payload.as_slice())
• iter: no allocations, return iterator over (&str, &Record):
  • hint: self.map.iter().map(|(k, v)| (k.as_str(), v))
• rename_key:
  • if old is missing → KeyNotFound
  • if new already exists → KeyAlreadyExists
  • no clone of payload
  • most importantly: do not lose the record if new already exists

Tests (minimum 4)
1. contains_true_false
2. payload_returns_slice
3. iter_returns_borrowed_items
4. rename_key_moves_record_no_clone + error cases

“Design review” moment (required)

rename_key — this is your first small lesson about exception safety / transactional semantics.
If you do remove(old)?; insert(new, rec) and insert fails — you must put rec back under old.
In Rust this is done explicitly, and that’s exactly the feeling I want you to get.

DoD
• cargo fmt
• cargo clippy -- -D warnings
• cargo test
• commit: week01: store api extensions
