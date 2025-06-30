use axum::Json;
use crate::{error::AppError, types::ApiResponse};

/// Health check endpoint
pub async fn health_check() -> Result<Json<ApiResponse<&'static str>>, AppError> {
    Ok(Json(ApiResponse::success("Solana HTTP server is running!")))
}