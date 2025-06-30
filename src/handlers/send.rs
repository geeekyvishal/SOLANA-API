use axum::{extract::Json as AxumJson, Json};
use base64::Engine;
use spl_token::instruction as token_instruction;
use solana_program::pubkey::Pubkey;
use crate::{
    error::AppError,
    types::{
        ApiResponse, SendSolRequest, SendSolResponse, SendTokenRequest, SendTokenResponse,
        SendTokenAccountMeta,
    },
    utils::{parse_pubkey, validate_amount, validate_different_pubkeys},
};

/// Create a SOL transfer instruction
pub async fn send_sol(
    AxumJson(req): AxumJson<SendSolRequest>,
) -> Result<Json<ApiResponse<SendSolResponse>>, AppError> {
    let from = parse_pubkey(&req.from, "from")?;
    let to = parse_pubkey(&req.to, "to")?;

    // Validate inputs
    validate_different_pubkeys(&from, &to, "From", "to")?;
    validate_amount(req.lamports, "Lamports")?;

    let ix = solana_sdk::system_instruction::transfer(&from, &to, req.lamports);

    let accounts = ix
        .accounts
        .iter()
        .map(|meta| meta.pubkey.to_string())
        .collect();

    let instruction_data = base64::engine::general_purpose::STANDARD.encode(&ix.data);

    let response = SendSolResponse {
        program_id: ix.program_id.to_string(),
        accounts,
        instruction_data,
    };

    Ok(Json(ApiResponse::success(response)))
}

/// Create a token transfer instruction
pub async fn send_token(
    AxumJson(req): AxumJson<SendTokenRequest>,
) -> Result<Json<ApiResponse<SendTokenResponse>>, AppError> {
    // Parse using solana_program::pubkey::Pubkey first
    let destination = req.destination.parse::<Pubkey>()
        .map_err(|_| AppError::BadRequest("Invalid destination pubkey".to_string()))?;
    let mint = req.mint.parse::<Pubkey>()
        .map_err(|_| AppError::BadRequest("Invalid mint pubkey".to_string()))?;
    let owner = req.owner.parse::<Pubkey>()
        .map_err(|_| AppError::BadRequest("Invalid owner pubkey".to_string()))?;

    // Convert to solana_sdk::pubkey::Pubkey for the instruction
    let destination_sdk = solana_sdk::pubkey::Pubkey::from(destination.to_bytes());
    let mint_sdk = solana_sdk::pubkey::Pubkey::from(mint.to_bytes());
    let owner_sdk = solana_sdk::pubkey::Pubkey::from(owner.to_bytes());

    // Validate inputs using SDK pubkeys
    validate_different_pubkeys(&destination_sdk, &owner_sdk, "Destination", "owner")?;
    validate_amount(req.amount, "Amount")?;

    let ix = token_instruction::transfer(
        &spl_token::id(),
        &mint_sdk,
        &destination_sdk,
        &owner_sdk,
        &[],
        req.amount,
    )
    .map_err(|e| AppError::BadRequest(format!("Failed to create instruction: {}", e)))?;

    let accounts = ix
        .accounts
        .iter()
        .map(|meta| SendTokenAccountMeta {
            pubkey: meta.pubkey.to_string(),
            is_signer: meta.is_signer,
        })
        .collect();

    let instruction_data = base64::engine::general_purpose::STANDARD.encode(&ix.data);

    let response = SendTokenResponse {
        program_id: ix.program_id.to_string(),
        accounts,
        instruction_data,
    };

    Ok(Json(ApiResponse::success(response)))
}