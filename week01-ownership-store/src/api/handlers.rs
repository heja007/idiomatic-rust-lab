use crate::api::state::AppState;
use crate::error::ApiError;
use crate::model::KvPair;
use crate::storage::file;
use axum::Json;
use axum::extract::{Path, State};
use serde_json::Value;
use std::collections::HashMap;

pub async fn put_key(
    State(state): State<AppState>,
    Path(key): Path<String>,
    Json(value): Json<Value>,
) -> Result<Json<KvPair>, ApiError> {
    let mut guard = state.store.write().await;
    guard.insert(key.clone(), value.clone());
    file::save_to_file(&state.data_file, &guard).await?;

    Ok(Json(KvPair { key, value }))
}

pub async fn get_key(
    State(state): State<AppState>,
    Path(key): Path<String>,
) -> Result<Json<KvPair>, ApiError> {
    let guard = state.store.read().await;
    let value = guard.get(&key).ok_or(ApiError::NotFound)?;

    Ok(Json(KvPair {
        key,
        value: value.clone(),
    }))
}

pub async fn get_all(
    State(state): State<AppState>,
) -> Result<Json<HashMap<String, Value>>, ApiError> {
    let guard = state.store.read().await;
    Ok(Json(guard.clone()))
}

pub async fn delete_key(
    State(state): State<AppState>,
    Path(key): Path<String>,
) -> Result<Json<KvPair>, ApiError> {
    let mut guard = state.store.write().await;
    let value = guard.remove(&key).ok_or(ApiError::NotFound)?;
    file::save_to_file(&state.data_file, &guard).await?;

    Ok(Json(KvPair { key, value }))
}
