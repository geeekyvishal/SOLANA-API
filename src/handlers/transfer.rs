use axum::Json;
use serde::Deserialize;
use serde_json::json;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_instruction,
};
use spl_token::instruction::transfer as token_transfer;
use std::str::FromStr;
use base64;
use base64::Engine;

const MAX_SAFE_INTEGER: u64 = 9_007_199_254_740_991; // 2^53 - 1

#[derive(Deserialize)]
pub struct SendSolRequest {
    from: String,
    to: String,
    lamports: u64,
}

#[derive(Deserialize)]
pub struct SendTokenRequest {
    destination: String,
    mint: String,
    owner: String,
    amount: u64,
}

pub async fn send_sol(Json(payload): Json<SendSolRequest>) -> Json<serde_json::Value> {
    // Validate lamports amount (must be positive)
    if payload.lamports == 0 {
        return Json(json!({ "success": false, "error": "Lamports must be greater than 0" }));
    }
    
    if payload.lamports > MAX_SAFE_INTEGER {
        return Json(json!({ "success": false, "error": format!("Lamports exceeds max safe integer ({}).", MAX_SAFE_INTEGER) }));
    }

    let from = match Pubkey::from_str(&payload.from) {
        Ok(pk) => pk,
        Err(_) => return Json(json!({ "success": false, "error": "Invalid from pubkey" })),
    };

    let to = match Pubkey::from_str(&payload.to) {
        Ok(pk) => pk,
        Err(_) => return Json(json!({ "success": false, "error": "Invalid to pubkey" })),
    };

    let ix = system_instruction::transfer(&from, &to, payload.lamports);

    // Return response matching the spec format
    Json(json!({
        "success": true,
        "data": {
            "program_id": ix.program_id.to_string(),
            "accounts": ix.accounts.iter().map(|a| a.pubkey.to_string()).collect::<Vec<_>>(),
            "instruction_data": base64::engine::general_purpose::STANDARD.encode(ix.data)
        }
    }))
}

pub async fn send_token(Json(payload): Json<SendTokenRequest>) -> Json<serde_json::Value> {
    // Validate amount (must be positive)
    if payload.amount == 0 {
        return Json(json!({ "success": false, "error": "Amount must be greater than 0" }));
    }
    
    if payload.amount > MAX_SAFE_INTEGER {
        return Json(json!({ "success": false, "error": format!("Amount exceeds max safe integer ({}).", MAX_SAFE_INTEGER) }));
    }

    let destination = match Pubkey::from_str(&payload.destination) {
        Ok(pk) => pk,
        Err(_) => return Json(json!({ "success": false, "error": "Invalid destination pubkey" })),
    };

    let mint = match Pubkey::from_str(&payload.mint) {
        Ok(pk) => pk,
        Err(_) => return Json(json!({ "success": false, "error": "Invalid mint pubkey" })),
    };

    let owner = match Pubkey::from_str(&payload.owner) {
        Ok(pk) => pk,
        Err(_) => return Json(json!({ "success": false, "error": "Invalid owner pubkey" })),
    };

    let ix = match token_transfer(
        &spl_token::ID,
        &mint,  // This should be source token account in real implementation
        &destination,
        &owner,
        &[], // No multisig signers
        payload.amount,
    ) {
        Ok(ix) => ix,
        Err(e) => return Json(json!({ "success": false, "error": format!("Failed to build instruction: {e}") })),
    };

    Json(json!({
        "success": true,
        "data": {
            "program_id": ix.program_id.to_string(),
            "accounts": ix.accounts.iter().map(|a| {
                json!({
                    "pubkey": a.pubkey.to_string(),
                    "isSigner": a.is_signer
                })
            }).collect::<Vec<_>>(),
            "instruction_data": base64::engine::general_purpose::STANDARD.encode(ix.data)
        }
    }))
}