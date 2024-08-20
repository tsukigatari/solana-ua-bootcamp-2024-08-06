use std::{env, str::FromStr};

use anyhow::{Ok, Result};
use dotenvy::dotenv;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer, transaction::Transaction};

async fn get_or_create_associated_token_account(
    rpc_client: RpcClient,
    payer: Keypair,
    mint: Pubkey,
    owner: Pubkey,
) -> Result<Pubkey> {
    let associated_token =
        spl_associated_token_account::get_associated_token_address(&owner, &mint);
    let account_info = rpc_client.get_account(&associated_token);
    if account_info.is_err() {
        let create_associated_token_account_instruction =
            spl_associated_token_account::instruction::create_associated_token_account(
                &payer.pubkey(),
                &owner,
                &mint,
                &spl_token::ID,
            );
        let recent_blockhash = rpc_client.get_latest_blockhash()?;
        let transaction = Transaction::new_signed_with_payer(
            &[create_associated_token_account_instruction],
            Some(&payer.pubkey()),
            &[payer],
            recent_blockhash,
        );
        rpc_client.send_and_confirm_transaction_with_spinner(&transaction)?;
        Ok(associated_token)
    } else {
        Ok(associated_token)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let private_key_str = env::var("PRIVATE_KEY")?;
    let private_key_bytes: Vec<u8> = serde_json::from_str(&private_key_str)?;
    let payer_keypair = Keypair::from_bytes(&private_key_bytes)?;
    let payer_pubkey = payer_keypair.pubkey();
    println!("Our public key is: {}", payer_pubkey);

    let token_mint_account_str = env::var("TOKEN_MINT_ACCOUNT")?;
    let owner_str = env::var("OWNER")?;

    let token_mint_account = Pubkey::from_str(&token_mint_account_str)?;
    let owner = Pubkey::from_str(&owner_str)?;

    let rpc_client = RpcClient::new("https://api.devnet.solana.com");

    let token_account = get_or_create_associated_token_account(
        rpc_client,
        payer_keypair,
        token_mint_account,
        owner,
    )
    .await?;

    println!("Token Account: {}", token_account);
    Ok(())
}
