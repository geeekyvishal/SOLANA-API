use axum::{extract::Json as AxumJson, Json};
use base64::Engine;
use spl_token::instruction as token_instruction;
use solana_program::pubkey::Pubkey;
use crate::{
    error::AppError,
    types::{
        ApiResponse, TokenCreateRequest, TokenCreateResponse, TokenMintRequest,
        TokenMintResponse, AccountMetaResponse,
    },
    utils::{validate_amount, validate_different_pubkeys},
};

/// Create a new token mint
pub async fn create_token(
    AxumJson(req): AxumJson<TokenCreateRequest>,
) -> Result<Json<ApiResponse<TokenCreateResponse>>, AppError> {
    let mint_authority = req
        .mint_authority
        .parse::<Pubkey>()
        .map_err(|_| AppError::BadRequest("Invalid mintAuthority pubkey".to_string()))?;
    let mint = req.mint.parse::<Pubkey>()
        .map_err(|_| AppError::BadRequest("Invalid mint pubkey".to_string()))?;

    // Convert to solana_sdk::pubkey::Pubkey for the instruction
    let mint_sdk = solana_sdk::pubkey::Pubkey::from(mint.to_bytes());
    let mint_authority_sdk = solana_sdk::pubkey::Pubkey::from(mint_authority.to_bytes());

    let ix = token_instruction::initialize_mint(
        &spl_token::id(),
        &mint_sdk,
        &mint_authority_sdk,
        None,
        req.decimals,
    )
    .map_err(|e| AppError::BadRequest(format!("Failed to create instruction: {}", e)))?;

    let accounts = ix
        .accounts
        .iter()
        .map(|meta| AccountMetaResponse {
            pubkey: meta.pubkey.to_string(),
            is_signer: meta.is_signer,
            is_writable: meta.is_writable,
        })
        .collect();

    let instruction_data = base64::engine::general_purpose::STANDARD.encode(&ix.data);

    let response = TokenCreateResponse {
        program_id: ix.program_id.to_string(),
        accounts,
        instruction_data,
    };

    Ok(Json(ApiResponse::success(response)))
}


/// Mint tokens to a destination account
pub async fn mint_token(
    AxumJson(req): AxumJson<TokenMintRequest>,
) -> Result<Json<ApiResponse<TokenMintResponse>>, AppError> {
    let mint = req.mint.parse::<Pubkey>()
        .map_err(|_| AppError::BadRequest("Invalid mint pubkey".to_string()))?;
    let destination = req.destination.parse::<Pubkey>()
        .map_err(|_| AppError::BadRequest("Invalid destination pubkey".to_string()))?;
    let authority = req.authority.parse::<Pubkey>()
        .map_err(|_| AppError::BadRequest("Invalid authority pubkey".to_string()))?;

    // Convert to solana_sdk::pubkey::Pubkey for validation and instruction
    let mint_sdk = solana_sdk::pubkey::Pubkey::from(mint.to_bytes());
    let dest_sdk = solana_sdk::pubkey::Pubkey::from(destination.to_bytes());
    let auth_sdk = solana_sdk::pubkey::Pubkey::from(authority.to_bytes());
    
    validate_different_pubkeys(&dest_sdk, &auth_sdk, "Destination", "authority")?;
    validate_amount(req.amount, "Amount")?;

    let ix = token_instruction::mint_to(
        &spl_token::id(),
        &mint_sdk,
        &dest_sdk,
        &auth_sdk,
        &[],
        req.amount,
    )
    .map_err(|e| AppError::BadRequest(format!("Failed to create instruction: {}", e)))?;

    let accounts = ix
        .accounts
        .iter()
        .map(|meta| AccountMetaResponse {
            pubkey: meta.pubkey.to_string(),
            is_signer: meta.is_signer,
            is_writable: meta.is_writable,
        })
        .collect();

    let instruction_data = base64::engine::general_purpose::STANDARD.encode(&ix.data);

    let response = TokenMintResponse {
        program_id: ix.program_id.to_string(),
        accounts,
        instruction_data,
    };

    Ok(Json(ApiResponse::success(response)))
}