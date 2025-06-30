use axum::{
    Router,
    routing::{get, post},
    Json,
};
use serde_json::{json, Value};

use crate::handlers::keypair::generate_keypair;
use crate::handlers::token::{create_token, mint_token};
use crate::handlers::account::create_account;
use crate::handlers::message::{sign_message, verify_message};
use crate::handlers::transfer::{send_sol, send_token};

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/keypair", post(generate_keypair))
        .route("/token/create", post(create_token))
        .route("/token/mint", post(mint_token))
        .route("/account/create", post(create_account))
        .route("/message/sign", post(sign_message))
        .route("/message/verify", post(verify_message))
        .route("/send/sol", post(send_sol))
        .route("/send/token", post(send_token))
}

async fn root() -> Json<Value> {
    Json(json!({
        "success": true,
        "message": "Welcome to the Solana Rust HTTP API"
    }))
}

async fn health() -> Json<Value> {
    Json(json!({
        "success": true,
        "status": "ok"
    }))
}