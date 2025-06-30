use std::str::FromStr;
use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::json;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};
use spl_token::instruction::initialize_mint;
use base64;

#[derive(Deserialize)]
pub struct CreateTokenRequest {
    mint_authority: String,
    mint: String,
    decimals: u8,
}

#[derive(Serialize)]
pub struct CreateTokenResponse {
    program_id: String,
    accounts: Vec<AccountMetaJson>,
    instruction_data: String,
}

#[derive(Serialize)]
pub struct AccountMetaJson {
    pubkey: String,
    is_signer: bool,
    is_writable: bool,
}

pub async fn create_token(Json(payload): Json<CreateTokenRequest>) -> Json<serde_json::Value> {
    // Parse input pubkeys
    let mint_authority = match Pubkey::from_str(&payload.mint_authority) {
        Ok(pk) => pk,
        Err(_) => {
            return Json(json!({ "success": false, "error": "Invalid mint_authority pubkey" }));
        }
    };

    let mint = match Pubkey::from_str(&payload.mint) {
        Ok(pk) => pk,
        Err(_) => {
            return Json(json!({ "success": false, "error": "Invalid mint pubkey" }));
        }
    };

    // Create initialize_mint instruction
    let ix = match initialize_mint(
        &spl_token::ID,
        &mint,
        &mint_authority,
        None,
        payload.decimals,
    ) {
        Ok(ix) => ix,
        Err(e) => {
            return Json(json!({ "success": false, "error": format!("Failed to build instruction: {e}") }));
        }
    };

    // Convert accounts to serializable format
    let accounts_json: Vec<AccountMetaJson> = ix
        .accounts
        .iter()
        .map(|acct| AccountMetaJson {
            pubkey: acct.pubkey.to_string(),
            is_signer: acct.is_signer,
            is_writable: acct.is_writable,
        })
        .collect();

    let encoded_data = base64::encode(ix.data);

    Json(json!({
        "success": true,
        "data": {
            "program_id": ix.program_id.to_string(),
            "accounts": accounts_json,
            "instruction_data": encoded_data
        }
    }))
}

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
        return Json(json!({ "success": false, "error": "Invalid pubkey(s)" }));
    }

    let ix = match spl_token::instruction::mint_to(
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

    Json(json!({
        "success": true,
        "data": {
            "program_id": ix.program_id.to_string(),
            "accounts": ix.accounts.iter().map(|a| {
                json!({
                    "pubkey": a.pubkey.to_string(),
                    "is_signer": a.is_signer,
                    "is_writable": a.is_writable
                })
            }).collect::<Vec<_>>(),
            "instruction_data": base64::encode(ix.data)
        }
    }))
}