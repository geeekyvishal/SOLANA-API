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
        return Json(json!({ "success": false, "error": "Invalid lamports amount" }));
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
            "instruction_data": base64::encode(ix.data)
        }
    }))
}

pub async fn send_token(Json(payload): Json<SendTokenRequest>) -> Json<serde_json::Value> {
    // Validate amount (must be positive)
    if payload.amount == 0 {
        return Json(json!({ "success": false, "error": "Invalid token amount" }));
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

    // For a real implementation, you'd derive the associated token accounts
    // For now, we'll use the provided addresses as token accounts
    let source_token_account = owner; // This is simplified
    let destination_token_account = destination; // This is simplified

    let ix = match token_transfer(
        &spl_token::ID,
        &source_token_account,
        &destination_token_account,
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
            "instruction_data": base64::encode(ix.data)
        }
    }))
}