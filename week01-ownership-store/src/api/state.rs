use crate::storage::Store;

#[derive(Clone)]
pub struct AppState {
    pub store: Store,
    pub data_file: String,
}
