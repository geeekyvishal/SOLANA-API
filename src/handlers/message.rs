use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::json;
use base64::{engine::general_purpose::STANDARD, Engine};
use bs58;
use ed25519_dalek::{Keypair, PublicKey, Signature, Signer, Verifier};

#[derive(Deserialize)]
pub struct SignMessageRequest {
    message: String,
    secret: String,
}

#[derive(Deserialize)]
pub struct VerifyMessageRequest {
    message: String,
    signature: String,
    pubkey: String,
}

pub async fn sign_message(Json(payload): Json<SignMessageRequest>) -> Json<serde_json::Value> {
    let secret_bytes = match bs58::decode(&payload.secret).into_vec() {
        Ok(bytes) => bytes,
        Err(_) => return Json(json!({ "success": false, "error": "Invalid secret key" })),
    };

    let keypair = match Keypair::from_bytes(&secret_bytes) {
        Ok(kp) => kp,
        Err(_) => return Json(json!({ "success": false, "error": "Failed to parse secret key" })),
    };

    let signature = keypair.sign(payload.message.as_bytes());
    Json(json!({
        "success": true,
        "data": {
            "signature": STANDARD.encode(signature.to_bytes()),
            // "public_key": keypair.public.to_string(),
            "public_key": bs58::encode(keypair.public).into_string(),
            "message": payload.message
        }
    }))
}

pub async fn verify_message(Json(payload): Json<VerifyMessageRequest>) -> Json<serde_json::Value> {
    let pubkey_bytes = match bs58::decode(&payload.pubkey).into_vec() {
        Ok(bytes) => bytes,
        Err(_) => return Json(json!({ "success": false, "error": "Invalid public key" })),
    };

    let sig_bytes = match STANDARD.decode(&payload.signature) {
        Ok(bytes) => bytes,
        Err(_) => return Json(json!({ "success": false, "error": "Invalid signature base64" })),
    };

    let public_key = match PublicKey::from_bytes(&pubkey_bytes) {
        Ok(pk) => pk,
        Err(_) => return Json(json!({ "success": false, "error": "Invalid public key bytes" })),
    };

    let signature = match Signature::from_bytes(&sig_bytes) {
        Ok(sig) => sig,
        Err(_) => return Json(json!({ "success": false, "error": "Invalid signature bytes" })),
    };

    let is_valid = public_key.verify(payload.message.as_bytes(), &signature).is_ok();

    Json(json!({
        "success": true,
        "data": {
            "valid": is_valid,
            "message": payload.message,
            "pubkey": payload.pubkey
        }
    }))
}