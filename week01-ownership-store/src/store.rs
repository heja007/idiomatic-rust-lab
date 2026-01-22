use crate::{error::StoreError, model::Record};
use std::collections::HashMap;

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
