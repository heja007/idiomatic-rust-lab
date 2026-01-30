use crate::http::errors::ApiError;
use crate::http::types::{
    GrepMatch, GrepRequest, GrepResponse, StatsRequest, StatsResponse, UniqRequest, UniqResponse,
};
use axum::Json;
use textkit::grep::grep_lines;
use textkit::stats::analyze;
use textkit::uniq::uniq_lines;

const MAX_TEXT_BYTES: usize = 1_048_576;

pub async fn stats(Json(payload): Json<StatsRequest>) -> Result<Json<StatsResponse>, ApiError> {
    let bytes = payload.text.len();
    if bytes > MAX_TEXT_BYTES {
        return Err(ApiError::TooLarge("text exceeds 1MB limit".to_string()));
    }

    let stats = match analyze(&payload.text) {
        Ok(v) => v,
        Err(e) => return Err(ApiError::Internal(e.to_string())),
    };

    Ok(Json(StatsResponse {
        lines: stats.lines,
        words: stats.words,
        chars: stats.chars,
        bytes: payload.text.len(),
    }))
}

pub async fn uniq(Json(payload): Json<UniqRequest>) -> Result<Json<UniqResponse>, ApiError> {
    if payload.text.len() > MAX_TEXT_BYTES {
        return Err(ApiError::TooLarge("text exceeds 1MB limit".to_string()));
    }

    let all = payload.all.unwrap_or(false);

    let lines = match uniq_lines(&payload.text, all) {
        Ok(v) => v,
        Err(e) => return Err(ApiError::Internal(e.to_string())),
    };

    let out = lines.join("\n");
    let removed = payload.text.lines().count().saturating_sub(lines.len());

    Ok(Json(UniqResponse {
        text: if out.is_empty() {
            String::new()
        } else {
            format!("{out}\n")
        },
        removed,
    }))
}

pub async fn grep(Json(payload): Json<GrepRequest>) -> Result<Json<GrepResponse>, ApiError> {
    if payload.text.len() > MAX_TEXT_BYTES {
        return Err(ApiError::TooLarge("text exceeds 1MB limit".to_string()));
    }

    if payload.pattern.trim().is_empty() {
        return Err(ApiError::Validation(
            "pattern must not be empty".to_string(),
        ));
    }

    let line_numbers = payload.line_number.unwrap_or(false);
    let lines = grep_lines(&payload.text, &payload.pattern, false, line_numbers)
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    let matches = if line_numbers {
        lines
            .into_iter()
            .filter_map(|line| {
                let mut parts = line.splitn(2, ':');
                let n = parts.next()?.parse::<usize>().ok()?;
                let text = parts.next().unwrap_or("").to_string();
                Some(GrepMatch { line: n, text })
            })
            .collect::<Vec<_>>()
    } else {
        lines
            .into_iter()
            .map(|text| GrepMatch { line: 0, text })
            .collect::<Vec<_>>()
    };

    let count = matches.len();

    Ok(Json(GrepResponse { matches, count }))
}
