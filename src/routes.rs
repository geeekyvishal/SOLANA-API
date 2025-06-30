use axum::{routing::{get, post}, Router};
use crate::handlers;

/// Health check routes
pub fn health_routes() -> Router {
    Router::new().route("/health", get(handlers::health::health_check))
}

/// Keypair management routes
pub fn keypair_routes() -> Router {
    Router::new().route("/keypair", post(handlers::keypair::generate_keypair))
}

/// Token-related routes
pub fn token_routes() -> Router {
    Router::new()
        .route("/token/create", post(handlers::token::create_token))
        .route("/token/mint", post(handlers::token::mint_token))
}

/// Message signing routes
pub fn message_routes() -> Router {
    Router::new()
        .route("/message/sign", post(handlers::message::sign_message))
        .route("/message/verify", post(handlers::message::verify_message))
}

/// Send transaction routes
pub fn send_routes() -> Router {
    Router::new()
        .route("/send/sol", post(handlers::send::send_sol))
        .route("/send/token", post(handlers::send::send_token))
}