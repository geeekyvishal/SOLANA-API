use axum::Json;
use serde::Deserialize;
use serde_json::json;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};
use spl_token::instruction::transfer as token_transfer;
use std::str::FromStr;
use base64::{engine::general_purpose::STANDARD, Engine};

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
    let from = Pubkey::from_str(&payload.from);
    let to = Pubkey::from_str(&payload.to);

    if from.is_err() || to.is_err() {
        return Json(json!({ "success": false, "error": "Invalid pubkey(s)" }));
    }

    // let ix = solana_sdk::system_instruction::transfer(&from.unwrap(), &to.unwrap(), payload.lamports);
    use solana_program::system_instruction;

    let from = Pubkey::from_str(&payload.from).unwrap();
    let to = Pubkey::from_str(&payload.to).unwrap();
    let ix = system_instruction::transfer(&from, &to, payload.lamports);


    Json(json!({
        "success": true,
        "data": {
            "program_id": ix.program_id.to_string(),
            "accounts": ix.accounts.iter().map(|a| a.pubkey.to_string()).collect::<Vec<_>>(),
            "instruction_data": STANDARD.encode(ix.data)
        }
    }))
}

pub async fn send_token(Json(payload): Json<SendTokenRequest>) -> Json<serde_json::Value> {
    let destination = Pubkey::from_str(&payload.destination);
    let mint = Pubkey::from_str(&payload.mint);
    let owner = Pubkey::from_str(&payload.owner);

    if destination.is_err() || mint.is_err() || owner.is_err() {
        return Json(json!({ "success": false, "error": "Invalid pubkey(s)" }));
    }

    let destination = destination.unwrap();
    let owner = owner.unwrap();
    let mint = mint.unwrap();

    // This is a dummy simplification - for a real tx you’d lookup token accounts!
    let source_token_account = owner;
    let destination_token_account = destination;

    let ix = match token_transfer(
        &spl_token::ID,
        &source_token_account,
        &destination_token_account,
        &owner,
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
                    "isSigner": a.is_signer
                })
            }).collect::<Vec<_>>(),
            "instruction_data": STANDARD.encode(ix.data)
        }
    }))
}