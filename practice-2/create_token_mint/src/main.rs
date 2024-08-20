use anyhow::{Ok, Result};
use dotenvy::dotenv;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    program_pack::Pack,
    pubkey::Pubkey,
    signer::{keypair::Keypair, Signer},
    transaction::Transaction,
};
use spl_token::state::Mint;
use std::env;

async fn create_mint(
    rpc_client: RpcClient,
    payer: Keypair,
    mint_authority: Pubkey,
    decimals: u8,
) -> Result<Pubkey> {
    let mint_account = Keypair::new();
    let minimum_balance_for_rent_exemption = rpc_client
        .get_minimum_balance_for_rent_exemption(spl_token::state::Mint::LEN)
        .await?;
    let create_account_instruction = solana_sdk::system_instruction::create_account(
        &payer.pubkey(),
        &mint_account.pubkey(),
        minimum_balance_for_rent_exemption,
        Mint::LEN as u64,
        &spl_token::ID,
    );
    let initialize_mint_instruction = spl_token::instruction::initialize_mint2(
        &spl_token::ID,
        &mint_account.pubkey(),
        &mint_authority,
        None,
        decimals,
    )?;
    let recent_blockhash = rpc_client.get_latest_blockhash().await?;
    let transaction = Transaction::new_signed_with_payer(
        &[create_account_instruction, initialize_mint_instruction],
        Some(&payer.pubkey()),
        &[&mint_account, &payer],
        recent_blockhash,
    );
    rpc_client
        .send_and_confirm_transaction_with_spinner(&transaction)
        .await?;
    Ok(mint_account.pubkey())
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let private_key_str = env::var("PRIVATE_KEY")?;
    let private_key_bytes: Vec<u8> = serde_json::from_str(&private_key_str)?;
    let user_keypair = Keypair::from_bytes(&private_key_bytes)?;
    let user_pubkey = user_keypair.pubkey();

    println!("Our public key is: {}", user_pubkey);

    let rpc_client = RpcClient::new("https://api.devnet.solana.com".to_string());

    let token_mint = create_mint(rpc_client, user_keypair, user_pubkey, 2).await?;
    println!("Token mint: {}", token_mint);

    Ok(())
}
