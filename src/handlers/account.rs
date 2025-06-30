use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::json;
use solana_program::{instruction::AccountMeta, pubkey::Pubkey};
use spl_token::instruction::initialize_account;
use std::str::FromStr;
use base64;

use base64::engine::general_purpose::STANDARD;
use base64::Engine;


#[derive(Deserialize)]
pub struct CreateAccountRequest {
    wallet: String,
    token_account: String,
    mint: String,
}

pub async fn create_account(Json(payload): Json<CreateAccountRequest>) -> Json<serde_json::Value> {
    // Parse input pubkeys
    let wallet = match Pubkey::from_str(&payload.wallet) {
        Ok(pk) => pk,
        Err(_) => {
            return Json(json!({ "success": false, "error": "Invalid wallet pubkey" }));
        }
    };

    let token_account = match Pubkey::from_str(&payload.token_account) {
        Ok(pk) => pk,
        Err(_) => {
            return Json(json!({ "success": false, "error": "Invalid token_account pubkey" }));
        }
    };

    let mint = match Pubkey::from_str(&payload.mint) {
        Ok(pk) => pk,
        Err(_) => {
            return Json(json!({ "success": false, "error": "Invalid mint pubkey" }));
        }
    };

    // Build the instruction
    let ix = match initialize_account(&spl_token::ID, &token_account, &mint, &wallet) {
        Ok(ix) => ix,
        Err(e) => {
            return Json(json!({
                "success": false,
                "error": format!("Failed to build instruction: {}", e)
            }));
        }
    };

    // Format account metas
    let accounts: Vec<_> = ix.accounts.iter().map(|a| {
        json!({
            "pubkey": a.pubkey.to_string(),
            "is_signer": a.is_signer,
            "is_writable": a.is_writable
        })
    }).collect();

    // Encode instruction data
    let encoded_data = STANDARD.encode(ix.data);

    Json(json!({
        "success": true,
        "data": {
            "program_id": ix.program_id.to_string(),
            "accounts": accounts,
            "instruction_data": encoded_data
        }
    }))
}