use crate::{error::StoreError, model::Record};
use std::collections::HashMap;

// codex resume 019bfbda-3bac-7782-af07-d580ec241043
pub struct Store {
    map: HashMap<String, Record>,
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}

impl Store {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, value: Record) -> Result<(), StoreError> {
        if self.map.contains_key(&key) {
            return Err(StoreError::KeyAlreadyExists);
        }

        self.map.insert(key, value);
        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<&Record> {
        self.map.get(key)
    }

    pub fn remove(&mut self, key: &str) -> Result<Record, StoreError> {
        match self.map.remove(key) {
            Some(record) => Ok(record),
            None => Err(StoreError::KeyNotFound),
        }
    }

    pub fn contains(&self, key: &str) -> bool {
        self.map.contains_key(key)
    }

    pub fn payload(&self, key: &str) -> Option<&[u8]> {
        self.get(key).map(|r| r.payload.as_slice())
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &Record)> {
        self.map.iter().map(|(k, v)| (k.as_str(), v))
    }

    pub fn rename_key(&mut self, old: &str, new: String) -> Result<(), StoreError> {
        if !self.map.contains_key(old) {
            return Err(StoreError::KeyNotFound);
        }

        if self.map.contains_key(&new) {
            return Err(StoreError::KeyAlreadyExists);
        }

        let record = self.map.remove(old).expect("checked exists");
        self.map.insert(new, record);

        Ok(())
    }
}

#[test]
fn insert_new_key_ok() {
    let mut s = Store::new();

    let res = s.insert(
        String::from("Once upon a time..."),
        Record {
            id: 1,
            payload: Vec::new(),
        },
    );

    assert!(res.is_ok());
}

#[test]
fn insert_duplicate_key_err() {
    let mut s = Store::new();

    let res = s.insert(
        String::from("Once upon a time..."),
        Record {
            id: 1,
            payload: Vec::new(),
        },
    );

    assert!(res.is_ok());

    let res2 = s.insert(
        String::from("Once upon a time..."),
        Record {
            id: 1,
            payload: Vec::new(),
        },
    );

    assert!(matches!(res2, Err(StoreError::KeyAlreadyExists)));
}

#[test]
fn get_returns_borrowed_ref() {
    let mut s = Store::new();

    s.insert(
        "k1".to_string(),
        Record {
            id: 42,
            payload: vec![1, 2, 3],
        },
    )
    .unwrap();

    let got = s.get("k1").unwrap();

    assert_eq!(got.id, 42);
    assert_eq!(got.payload.len(), 3);

    assert!(s.get("k1").is_some());
}

#[test]
fn remove_transfers_ownership() {
    let mut s = Store::new();

    s.insert(
        "k1".to_string(),
        Record {
            id: 7,
            payload: vec![9, 9],
        },
    )
    .unwrap();

    let removed = s.remove("k1").unwrap();
    assert_eq!(removed.id, 7);

    assert!(s.get("k1").is_none());

    let err = s.remove("missing");
    assert!(matches!(err, Err(StoreError::KeyNotFound)));
}

#[test]
fn contains_true_false() {
    let mut s = Store::new();

    s.insert(
        String::from("key1"),
        Record {
            id: 1,
            payload: Vec::new(),
        },
    )
    .unwrap();

    assert_eq!(s.contains("key1"), true);
    assert_eq!(s.contains("key2"), false);
}

#[test]
fn payload_returns_slice() {
    let mut s = Store::new();

    s.insert(
        String::from("k1"),
        Record {
            id: 1,
            payload: vec![1, 2, 3],
        },
    )
    .unwrap();

    let payload = s.payload("k1").unwrap();
    assert_eq!(payload, &[1, 2, 3]);
}

#[test]
fn iter_returns_borrowed_items() {
    let mut s = Store::new();

    s.insert(
        "a".to_string(),
        Record {
            id: 1,
            payload: vec![1],
        },
    )
    .unwrap();

    s.insert(
        "b".to_string(),
        Record {
            id: 2,
            payload: vec![2, 2],
        },
    )
    .unwrap();

    let mut items: Vec<(&str, u64)> = s.iter().map(|(k, v)| (k, v.id)).collect();
    items.sort_by_key(|(k, _)| *k);

    assert_eq!(items, vec![("a", 1), ("b", 2)]);
}

#[test]
fn rename_key_moves_record_no_clone() {
    let mut s = Store::new();

    s.insert(
        "old".to_string(),
        Record {
            id: 10,
            payload: vec![1, 2, 3],
        },
    )
    .unwrap();

    s.rename_key("old", "new".to_string()).unwrap();

    assert!(s.get("old").is_none());
    let rec = s.get("new").unwrap();
    assert_eq!(rec.id, 10);
    assert_eq!(rec.payload, vec![1, 2, 3]);

    // KeyNotFound
    let err = s.rename_key("missing", "x".to_string());
    assert!(matches!(err, Err(StoreError::KeyNotFound)));

    // KeyAlreadyExists (когда new уже существует)
    s.insert(
        "existing".to_string(),
        Record {
            id: 99,
            payload: vec![9],
        },
    )
    .unwrap();

    let err2 = s.rename_key("new", "existing".to_string());
    assert!(matches!(err2, Err(StoreError::KeyAlreadyExists)));

    // убедимся, что запись не потерялась
    assert!(s.get("new").is_some());
}
