use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
    extract::rejection::JsonRejection,
};
use thiserror::Error;
use crate::types::ApiResponse;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("{0}")]
    BadRequest(String),
    #[error("Internal server error")]
    Internal,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, msg) = match &self {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
        };

        let body = serde_json::to_string(&ApiResponse::<()> {
            success: false,
            data: None,
            error: Some(msg),
        })
        .unwrap();

        (status, [("content-type", "application/json")], body).into_response()
    }
}

impl From<JsonRejection> for AppError {
    fn from(rej: JsonRejection) -> Self {
        AppError::BadRequest(format!("Invalid JSON: {}", rej))
    }
}