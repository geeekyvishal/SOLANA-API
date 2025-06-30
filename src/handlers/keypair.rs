use axum::Json;
use rand::rngs::OsRng;
use solana_sdk::signature::{Keypair, Signer};
use crate::{error::AppError, types::{ApiResponse, KeypairResponse}};

/// Generate a new Solana keypair
pub async fn generate_keypair() -> Result<Json<ApiResponse<KeypairResponse>>, AppError> {
    let keypair = Keypair::new();
    let pubkey = keypair.pubkey().to_string();
    let secret = bs58::encode(keypair.to_bytes()).into_string();

    let response = KeypairResponse { pubkey, secret };
    Ok(Json(ApiResponse::success(response)))
}