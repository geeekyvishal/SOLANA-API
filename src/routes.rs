use axum::{
    routing::{get, post},
    Router,
};

use crate::handlers::{
    keypair::generate_keypair,
    token::{create_token, mint_token},
    message::{sign_message, verify_message},
    transfer::{send_sol, send_token},
    account::create_account,
};

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(|| async { "Solana HTTP Server is running!" }))
        .route("/keypair", post(generate_keypair))
        .route("/token/create", post(create_token))
        .route("/token/mint", post(mint_token))
        .route("/message/sign", post(sign_message))
        .route("/message/verify", post(verify_message))
        .route("/send/sol", post(send_sol))
        .route("/send/token", post(send_token))
        .route("/account/create", post(create_account))
}