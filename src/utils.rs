use solana_sdk::{pubkey::Pubkey, signature::Keypair};
use crate::error::AppError;
use ed25519_dalek::Keypair as DalekKeypair;

/// Parse a base58 encoded public key string
pub fn parse_pubkey(pubkey_str: &str, field_name: &str) -> Result<Pubkey, AppError> {
    if pubkey_str.trim().is_empty() {
        return Err(AppError::BadRequest(format!(
            "Missing required field: {}",
            field_name
        )));
    }

    let bytes = bs58::decode(pubkey_str)
        .into_vec()
        .map_err(|_| AppError::BadRequest(format!("Invalid base58 for {}", field_name)))?;

    if bytes.len() != 32 {
        return Err(AppError::BadRequest(format!(
            "Invalid {} pubkey length: expected 32 bytes, got {}",
            field_name,
            bytes.len()
        )));
    }

    Pubkey::try_from(bytes.as_slice())
        .map_err(|_| AppError::BadRequest(format!("Invalid {} pubkey", field_name)))
}

/// Parse a base58 encoded secret key string into a Keypair
pub fn parse_secret_key(secret_str: &str) -> Result<Keypair, AppError> {
    if secret_str.trim().is_empty() {
        return Err(AppError::BadRequest(
            "Missing required field: secret".to_string(),
        ));
    }

    let bytes = bs58::decode(secret_str)
        .into_vec()
        .map_err(|_| AppError::BadRequest("Invalid base58 for secret".to_string()))?;

    if bytes.len() != 64 && bytes.len() != 32 {
        return Err(AppError::BadRequest(format!(
            "Invalid secret key length: expected 32 or 64 bytes, got {}",
            bytes.len()
        )));
    }

    if bytes.len() == 64 {
        Keypair::from_bytes(&bytes)
            .map_err(|_| AppError::BadRequest("Invalid secret key bytes".to_string()))
    } else {
        let seed: [u8; 32] = bytes
            .try_into()
            .map_err(|_| AppError::BadRequest("Invalid secret key seed length".to_string()))?;
        // Use Keypair::from_bytes after expanding the seed to a 64-byte array using ed25519_dalek
        let dalek_keypair = ed25519_dalek::Keypair::from_bytes(&seed)
            .map_err(|_| AppError::BadRequest("Invalid secret key seed bytes".to_string()))?;
        Keypair::from_bytes(&dalek_keypair.to_bytes())
            .map_err(|_| AppError::BadRequest("Invalid secret key seed".to_string()))
    }
}

/// Validate that an amount is within safe bounds
pub fn validate_amount(amount: u64, field_name: &str) -> Result<(), AppError> {
    if amount == 0 {
        return Err(AppError::BadRequest(format!(
            "{} must be greater than 0",
            field_name
        )));
    }

    if amount > crate::config::MAX_SAFE_INTEGER {
        return Err(AppError::BadRequest(format!(
            "{} exceeds max safe integer ({}).",
            field_name,
            crate::config::MAX_SAFE_INTEGER
        )));
    }

    Ok(())
}

/// Validate that two public keys are different
pub fn validate_different_pubkeys(
    pubkey1: &Pubkey,
    pubkey2: &Pubkey,
    field1_name: &str,
    field2_name: &str,
) -> Result<(), AppError> {
    if pubkey1 == pubkey2 {
        return Err(AppError::BadRequest(format!(
            "{} and {} cannot be the same public key.",
            field1_name,
            field2_name
        )));
    }
    Ok(())
}

/// Validate that a string field is not empty
pub fn validate_not_empty(value: &str, field_name: &str) -> Result<(), AppError> {
    if value.trim().is_empty() {
        return Err(AppError::BadRequest(format!(
            "Missing required field: {}",
            field_name
        )));
    }
    Ok(())
}