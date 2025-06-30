use serde::{Deserialize, Serialize};

/// Standard API response wrapper
#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
        }
    }
}

// Keypair types
#[derive(Serialize)]
pub struct KeypairResponse {
    pub pubkey: String,
    pub secret: String,
}

// Token types
#[derive(Deserialize)]
pub struct TokenCreateRequest {
    #[serde(rename = "mintAuthority")]
    pub mint_authority: String,
    pub mint: String,
    pub decimals: u8,
}

#[derive(Serialize)]
pub struct TokenCreateResponse {
    pub program_id: String,
    pub accounts: Vec<AccountMetaResponse>,
    pub instruction_data: String,
}

#[derive(Deserialize)]
pub struct TokenMintRequest {
    pub mint: String,
    pub destination: String,
    pub authority: String,
    pub amount: u64,
}

#[derive(Serialize)]
pub struct TokenMintResponse {
    pub program_id: String,
    pub accounts: Vec<AccountMetaResponse>,
    pub instruction_data: String,
}

// Message signing types
#[derive(Deserialize)]
pub struct MessageSignRequest {
    pub message: String,
    pub secret: String,
}

#[derive(Serialize)]
pub struct MessageSignResponse {
    pub signature: String,
    pub public_key: String,
    pub message: String,
}

#[derive(Deserialize)]
pub struct MessageVerifyRequest {
    pub message: String,
    pub signature: String,
    pub pubkey: String,
}

#[derive(Serialize)]
pub struct MessageVerifyResponse {
    pub valid: bool,
    pub message: String,
    pub pubkey: String,
}

// Send SOL types
#[derive(Deserialize)]
pub struct SendSolRequest {
    pub from: String,
    pub to: String,
    pub lamports: u64,
}

#[derive(Serialize)]
pub struct SendSolResponse {
    pub program_id: String,
    pub accounts: Vec<String>,
    pub instruction_data: String,
}

// Send token types
#[derive(Deserialize)]
pub struct SendTokenRequest {
    pub destination: String,
    pub mint: String,
    pub owner: String,
    pub amount: u64,
}

#[derive(Serialize)]
pub struct SendTokenResponse {
    pub program_id: String,
    pub accounts: Vec<SendTokenAccountMeta>,
    pub instruction_data: String,
}

#[derive(Serialize)]
pub struct SendTokenAccountMeta {
    pub pubkey: String,
    #[serde(rename = "isSigner")]
    pub is_signer: bool,
}

// Shared account metadata
#[derive(Serialize)]
pub struct AccountMetaResponse {
    pub pubkey: String,
    pub is_signer: bool,
    pub is_writable: bool,
}