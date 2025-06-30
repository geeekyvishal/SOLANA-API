use axum::Json;
use serde_json::json;
use ed25519_dalek::Keypair;
use rand::rngs::OsRng;
use bs58;

pub async fn generate_keypair() -> Json<serde_json::Value> {
    let mut csprng = OsRng {};
    let keypair: Keypair = Keypair::generate(&mut csprng);

    let pubkey = bs58::encode(keypair.public.as_bytes()).into_string();
    let secret = bs58::encode(keypair.to_bytes()).into_string();

    Json(json!({
        "success": true,
        "data": {
            "pubkey": pubkey,
            "secret": secret
        }
    }))
}