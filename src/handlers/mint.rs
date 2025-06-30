// src/handlers/mint.rs
use axum::Json;
use serde::Deserialize;
use serde_json::json;
use solana_program::{instruction::AccountMeta, pubkey::Pubkey};
use spl_token::instruction::mint_to;
use std::str::FromStr;
use base64;

#[derive(Deserialize)]
pub struct MintTokenRequest {
    mint: String,
    destination: String,
    authority: String,
    amount: u64,
}

pub async fn mint_token(Json(payload): Json<MintTokenRequest>) -> Json<serde_json::Value> {
    let mint = Pubkey::from_str(&payload.mint);
    let destination = Pubkey::from_str(&payload.destination);
    let authority = Pubkey::from_str(&payload.authority);

    if mint.is_err() || destination.is_err() || authority.is_err() {
        return Json(json!({ "success": false, "error": "Invalid pubkey input" }));
    }

    let ix = match mint_to(
        &spl_token::ID,
        &mint.unwrap(),
        &destination.unwrap(),
        &authority.unwrap(),
        &[],
        payload.amount,
    ) {
        Ok(ix) => ix,
        Err(e) => return Json(json!({ "success": false, "error": format!("Failed to build instruction: {e}") })),
    };

    let accounts: Vec<_> = ix.accounts.iter().map(|a| json!({
        "pubkey": a.pubkey.to_string(),
        "is_signer": a.is_signer,
        "is_writable": a.is_writable
    })).collect();

    let encoded_data = base64::encode(ix.data);

    Json(json!({
        "success": true,
        "data": {
            "program_id": ix.program_id.to_string(),
            "accounts": accounts,
            "instruction_data": encoded_data
        }
    }))
}