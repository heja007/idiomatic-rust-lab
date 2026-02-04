use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;
use tokio::fs;

use crate::error::ApiError;
pub async fn load_from_file(path: impl AsRef<Path>) -> Result<HashMap<String, Value>, ApiError> {
    let file = path.as_ref();

    let content = match fs::read_to_string(file).await {
        Ok(content) => content,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(HashMap::new()),
        Err(err) => return Err(ApiError::Io(err)),
    };

    let map =
        serde_json::from_str::<HashMap<String, Value>>(&content).map_err(ApiError::InvalidJson)?;

    Ok(map)
}

pub async fn save_to_file(
    path: impl AsRef<Path>,
    map: &HashMap<String, Value>,
) -> Result<(), ApiError> {
    let file = path.as_ref();
    let tmp_file = file.with_extension("tmp");

    let json = serde_json::to_string_pretty(&map).map_err(ApiError::InvalidJson)?;
    fs::write(&tmp_file, json).await.map_err(ApiError::Io)?;
    fs::rename(&tmp_file, path).await.map_err(ApiError::Io)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tempfile::tempdir;

    #[tokio::test]
    async fn load_missing_file_returns_empty() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("data.json");

        let map = load_from_file(&path).await.unwrap();
        assert!(map.is_empty());
    }

    #[tokio::test]
    async fn save_then_load_roundtrip() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("data.json");

        let mut map = HashMap::new();
        map.insert("k1".to_string(), json!("v1"));

        save_to_file(&path, &map).await.unwrap();
        let loaded = load_from_file(&path).await.unwrap();

        assert_eq!(loaded.get("k1"), Some(&json!("v1")));
    }
}
