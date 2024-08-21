use std::{env, str::FromStr};

use anyhow::Result;
use dotenvy::dotenv;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer, transaction::Transaction};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let private_key_str = env::var("PRIVATE_KEY")?;
    let private_key_bytes: Vec<u8> = serde_json::from_str(&private_key_str)?;
    let signer_keypair = Keypair::from_bytes(&private_key_bytes)?;
    println!("Our public key is: {}", signer_keypair.pubkey());

    let token_mint_account_str = env::var("TOKEN_MINT_ACCOUNT")?;
    let token_mint_account = Pubkey::from_str(&token_mint_account_str)?;

    let rpc_client = RpcClient::new("https://api.devnet.solana.com");

    // 10
    let amount = 10 * 100;

    let atc = spl_associated_token_account::get_associated_token_address(
        &signer_keypair.pubkey(),
        &token_mint_account,
    );

    let mint_to_instruction = spl_token::instruction::mint_to(
        &spl_token::ID,
        &token_mint_account,
        &atc,
        &signer_keypair.pubkey(),
        &[&signer_keypair.pubkey()],
        amount,
    )?;

    let recent_blockhash = rpc_client.get_latest_blockhash()?;
    let transaction = Transaction::new_signed_with_payer(
        &[mint_to_instruction],
        Some(&signer_keypair.pubkey()),
        &[&signer_keypair],
        recent_blockhash,
    );

    let signature = rpc_client.send_and_confirm_transaction_with_spinner(&transaction);
    println!("Success! Mint Token Transaction: {:?}", signature);

    Ok(())
}
