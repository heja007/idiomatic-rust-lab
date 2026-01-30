use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct StatsRequest {
    pub text: String,
}

#[derive(Serialize)]
pub struct StatsResponse {
    pub lines: usize,
    pub words: usize,
    pub chars: usize,
    pub bytes: usize,
}

#[derive(Serialize)]
pub struct ErrorBody {
    pub error: ErrorInfo,
}

#[derive(Serialize)]
pub struct ErrorInfo {
    pub code: &'static str,
    pub message: String,
}

#[derive(Deserialize)]
pub struct UniqRequest {
    pub text: String,
    pub all: Option<bool>,
}

#[derive(Serialize)]
pub struct UniqResponse {
    pub text: String,
    pub removed: usize,
}

#[derive(Deserialize)]
pub struct GrepRequest {
    pub text: String,
    pub pattern: String,
    pub line_number: Option<bool>,
}

#[derive(Serialize)]
pub struct GrepMatch {
    pub line: usize,
    pub text: String,
}

#[derive(Serialize)]
pub struct GrepResponse {
    pub matches: Vec<GrepMatch>,
    pub count: usize,
}
