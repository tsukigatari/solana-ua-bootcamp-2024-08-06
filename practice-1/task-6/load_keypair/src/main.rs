use dotenvy::dotenv;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;
use std::env;

fn main() {
    dotenv().expect(".env file not found");

    let private_key_str = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set");

    let private_key_bytes: Vec<u8> =
        serde_json::from_str(&private_key_str).expect("Failed to parse PRIVATE_KEY");

    print!("Private key bytes: {:?}\n", private_key_bytes);

    let keypair = Keypair::from_bytes(&private_key_bytes)
        .expect("Failed to create Keypair from private key bytes");

    let public_key = keypair.pubkey();

    println!("Public key: {}", public_key);
}
