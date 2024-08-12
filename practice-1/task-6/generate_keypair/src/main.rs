use solana_sdk::signer::{keypair::Keypair, Signer};

fn main() {
    let pair = Keypair::new();
    println!("Public key: {}\n", pair.pubkey());
    println!("Private key: {:?}", pair.to_bytes());
}
