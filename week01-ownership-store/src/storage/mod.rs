use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub type Store = Arc<RwLock<HashMap<String, Value>>>;

pub fn new_store() -> Store {
    Arc::new(RwLock::new(HashMap::new()))
}

pub mod file;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn store_insert_get_delete() {
        let store = new_store();

        // insert
        {
            let mut guard = store.write().await;
            guard.insert("k1".to_string(), json!("v1"));
        }

        // get
        {
            let guard = store.read().await;
            assert_eq!(guard.get("k1"), Some(&json!("v1")));
        }

        // delete
        {
            let mut guard = store.write().await;
            let removed = guard.remove("k1");
            assert_eq!(removed, Some(json!("v1")));
        }

        // get after delete
        {
            let guard = store.read().await;
            assert!(guard.get("k1").is_none());
        }
    }
}
