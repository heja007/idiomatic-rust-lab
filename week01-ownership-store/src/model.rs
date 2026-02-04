use serde::{Deserialize, Serialize};
use serde_json::Value;

#[allow(dead_code)]
pub struct Record {
    pub id: u64,
    pub payload: Vec<u8>,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct KvPair {
    pub key: String,
    pub value: Value,
}
