use axum::{extract::Json as AxumJson, Json};
use base64::Engine;
use solana_sdk::signature::Signer;
use crate::{
    error::AppError,
    types::{
        ApiResponse, MessageSignRequest, MessageSignResponse, MessageVerifyRequest,
        MessageVerifyResponse,
    },
    utils::{parse_pubkey, parse_secret_key, validate_not_empty},
};

/// Sign a message with a private key
pub async fn sign_message(
    AxumJson(req): AxumJson<MessageSignRequest>,
) -> Result<Json<ApiResponse<MessageSignResponse>>, AppError> {
    // Validate inputs
    validate_not_empty(&req.message, "message")?;
    validate_not_empty(&req.secret, "secret")?;

    let keypair = parse_secret_key(&req.secret)?;
    let signature = keypair.sign_message(req.message.as_bytes());
    let signature_b64 = base64::engine::general_purpose::STANDARD.encode(signature.as_ref());
    let public_key = keypair.pubkey().to_string();

    let response = MessageSignResponse {
        signature: signature_b64,
        public_key,
        message: req.message,
    };

    Ok(Json(ApiResponse::success(response)))
}

/// Verify a message signature
pub async fn verify_message(
    AxumJson(req): AxumJson<MessageVerifyRequest>,
) -> Result<Json<ApiResponse<MessageVerifyResponse>>, AppError> {
    // Validate inputs
    validate_not_empty(&req.message, "message")?;
    validate_not_empty(&req.signature, "signature")?;
    validate_not_empty(&req.pubkey, "pubkey")?;

    let pubkey = parse_pubkey(&req.pubkey, "pubkey")?;

    let signature_bytes = base64::engine::general_purpose::STANDARD
        .decode(&req.signature)
        .map_err(|_| AppError::BadRequest("Invalid base64 for signature".to_string()))?;

    if signature_bytes.len() != 64 {
        return Err(AppError::BadRequest(format!(
            "Invalid signature length: expected 64 bytes, got {}",
            signature_bytes.len()
        )));
    }

    let signature = solana_sdk::signature::Signature::try_from(signature_bytes.as_slice())
        .map_err(|_| AppError::BadRequest("Invalid signature bytes".to_string()))?;

    let valid = signature.verify(pubkey.as_ref(), req.message.as_bytes());

    let response = MessageVerifyResponse {
        valid,
        message: req.message,
        pubkey: req.pubkey,
    };

    Ok(Json(ApiResponse::success(response)))
}