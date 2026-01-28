use crate::error::StoreError;
use crate::model::Record;
use crate::store::Store;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::{fs, io};

const VERSION: u32 = 1;

#[derive(Deserialize)]
struct PersistedStoreOwned {
    version: u32,
    records: HashMap<String, PersistedRecordOwned>,
}

#[derive(Deserialize)]
struct PersistedRecordOwned {
    id: u64,
    payload: Vec<u8>,
}

#[derive(Serialize)]
struct PersistedStore<'a> {
    version: u32,
    records: HashMap<&'a str, PersistedRecord<'a>>,
}

#[derive(Serialize)]
struct PersistedRecord<'a> {
    id: u64,
    payload: &'a [u8],
}

pub fn load_from_file(path: impl AsRef<Path>) -> Result<Store, StoreError> {
    let path = path.as_ref();

    let content = match fs::read_to_string(path) {
        Ok(data) => data,
        Err(err) if err.kind() == io::ErrorKind::NotFound => {
            return Ok(Store::new());
        }
        Err(err) => return Err(StoreError::from(err)),
    };

    let persisted: PersistedStoreOwned = serde_json::from_str(&content)?;

    if persisted.version != VERSION {
        return Err(StoreError::UnsupportedVersion(persisted.version));
    }

    let mut store = Store::new();

    for (key, record) in persisted.records {
        store.insert(
            key,
            Record {
                id: record.id,
                payload: record.payload,
            },
        )?;
    }

    Ok(store)
}

pub fn save_to_file(store: &Store, path: impl AsRef<Path>) -> Result<(), StoreError> {
    let path = path.as_ref();
    let tmp_path = path.with_extension("tmp");

    let records: HashMap<&str, PersistedRecord<'_>> = store
        .iter()
        .map(|(k, v)| {
            (
                k,
                PersistedRecord {
                    id: v.id,
                    payload: v.payload.as_slice(),
                },
            )
        })
        .collect();

    let persisted = PersistedStore {
        version: VERSION,
        records,
    };

    let json = serde_json::to_string(&persisted)?;

    // write to tmp first
    fs::write(&tmp_path, json)?;

    // atomically replace
    fs::rename(&tmp_path, path)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn load_from_file_reads_fixture() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("fixtures/store_v1.json");

        let store = load_from_file(path).expect("load_from_file should succeed");

        let rec = store.get("key1").expect("key1 should exist");
        assert_eq!(rec.id, 1);
        assert_eq!(rec.payload, vec![1, 2, 3]);
    }

    #[test]
    fn load_from_file_missing_returns_empty_store() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("fixtures/does_not_exist.json");

        let store = load_from_file(path).expect("missing file should yield empty store");

        assert_eq!(store.iter().count(), 0);
    }

    #[test]
    fn save_to_file_roundtrip() {
        let dir = tempdir().expect("tempdir should be created");
        let path = dir.path().join("store.json");

        let mut store = Store::new();
        store
            .insert(
                "alpha".to_string(),
                Record {
                    id: 1,
                    payload: vec![1, 2, 3],
                },
            )
            .unwrap();

        save_to_file(&store, &path).expect("save_to_file should succeed");

        let loaded = load_from_file(&path).expect("load_from_file should succeed");
        let rec = loaded.get("alpha").expect("alpha should exist");
        assert_eq!(rec.id, 1);
        assert_eq!(rec.payload, vec![1, 2, 3]);
    }

    #[test]
    fn save_to_file_removes_tmp() {
        let dir = tempdir().expect("tempdir should be created");
        let path = dir.path().join("store.json");
        let tmp_path = path.with_extension("tmp");

        let store = Store::new();
        save_to_file(&store, &path).expect("save_to_file should succeed");

        assert!(!tmp_path.exists(), "tmp file should not remain");
    }

    #[test]
    fn load_from_file_unsupported_version() {
        let dir = tempdir().expect("tempdir should be created");
        let path = dir.path().join("store.json");

        let bad = r#"{"version": 999, "records": {}}"#;
        fs::write(&path, bad).expect("write should succeed");

        match load_from_file(&path) {
            Err(err) => assert!(matches!(err, StoreError::UnsupportedVersion(999))),
            Ok(_) => panic!("expected UnsupportedVersion error"),
        }
    }

    #[test]
    fn load_from_file_invalid_json() {
        let dir = tempdir().expect("tempdir should be created");
        let path = dir.path().join("store.json");

        fs::write(&path, "not-json").expect("write should succeed");

        match load_from_file(&path) {
            Err(err) => assert!(matches!(err, StoreError::Json(_))),
            Ok(_) => panic!("expected Json error"),
        }
    }
}
