use anyhow::{Ok, Result};
use dotenvy::dotenv;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::system_transaction;
use std::env;
use std::str::FromStr;

const RPC_CLIENT_URL: &str = "https://api.devnet.solana.com";

fn main() -> Result<()> {
    dotenv()?;

    let private_key_str = env::var("PRIVATE_KEY")?;
    let private_key_bytes: Vec<u8> = serde_json::from_str(&private_key_str)?;
    let sender_keypair = Keypair::from_bytes(&private_key_bytes)?;

    let recipient_from_env = env::var("RECIPIENT")?;
    let recipient = Pubkey::from_str(&recipient_from_env)?;

    let rpc_client = RpcClient::new(RPC_CLIENT_URL);

    // 0.01 SOL
    let amount_lamports: u64 = 10_000_000;

    let send_sol_instruction = &system_transaction::transfer(
        &sender_keypair,
        &recipient,
        amount_lamports,
        rpc_client.get_latest_blockhash()?,
    );
    let signature = rpc_client.send_and_confirm_transaction(send_sol_instruction);

    println!("Transaction confirmed, signature: {:?}!", signature);

    Ok(())
}
