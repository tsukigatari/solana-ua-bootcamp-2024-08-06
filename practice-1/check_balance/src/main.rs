use solana_client::rpc_client::RpcClient;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::pubkey;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let rpc_client = RpcClient::new("https://api.devnet.solana.com");

    let public_key = pubkey!("DGnvPrBSWhEyXW7hssFweZ5zi8ViqSQW9XqLpQKZgH8F");

    let balance = rpc_client.get_balance(&public_key)?;
    println!("Balance: {} SOL", balance / LAMPORTS_PER_SOL);

    Ok(())
}
