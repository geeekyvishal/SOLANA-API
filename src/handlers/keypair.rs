use axum::{Json};
use serde_json::json;
use solana_sdk::signature::{Keypair, Signer};
use bs58;

pub async fn generate_keypair() -> Json<serde_json::Value> {
    let keypair = Keypair::new();
    let pubkey = keypair.pubkey().to_string();
    let secret = bs58::encode(keypair.to_bytes()).into_string();

    Json(json!({
        "success": true,
        "data": {
            "pubkey": pubkey,
            "secret": secret
        }
    }))
}