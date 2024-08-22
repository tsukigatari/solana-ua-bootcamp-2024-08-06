use std::{env, str::FromStr};

use anyhow::Result;
use dotenvy::dotenv;
use mpl_token_metadata::{instructions::CreateMetadataAccountV3Builder, types::DataV2};
use solana_client::rpc_client::RpcClient;
use solana_program::{
    instruction::Instruction as ProgramInstruction, pubkey::Pubkey as ProgramPubkey,
};
use solana_sdk::{
    instruction::Instruction as SdkInstruction, pubkey::Pubkey, signature::Keypair, signer::Signer,
    transaction::Transaction,
};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let private_key_str = env::var("PRIVATE_KEY")?;
    let private_key_bytes: Vec<u8> = serde_json::from_str(&private_key_str)?;
    let user_keypair = Keypair::from_bytes(&private_key_bytes)?;

    let user_pubkey = ProgramPubkey::from_str(&user_keypair.pubkey().to_string())?;
    println!("Our public key is: {}", user_keypair.pubkey());

    let token_mint_account_str = env::var("TOKEN_MINT_ACCOUNT")?;
    let token_mint_account = ProgramPubkey::from_str(&token_mint_account_str)?;

    //E
    //mismatched types solana_sdk::pubkey::Pubkey and solana_program::pubkey::Pubkey have similar names, but are actually distinct types
    //perhaps two different versions of crate solana_program are being used?
    let token_metadata_program_id =
        ProgramPubkey::from_str("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s")?;
    let token_metadata_program_id_2 =
        Pubkey::from_str("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s")?;

    let rpc_client = RpcClient::new("https://api.devnet.solana.com");

    let metadata_data = DataV2 {
        name: "RainCoin".to_string(),
        symbol: "Rain".to_string(),
        // this url is simply an example
        uri: "https://gateway.pinata.cloud/ipfs/1234".to_string(),
        seller_fee_basis_points: 0,
        creators: None,
        collection: None,
        uses: None,
    };

    let (metadata_pda, _bump) = ProgramPubkey::find_program_address(
        &[
            b"metadata",
            token_metadata_program_id.as_ref(),
            token_mint_account.as_ref(),
        ],
        &token_metadata_program_id,
    );

    let create_metadata_instruction: ProgramInstruction = CreateMetadataAccountV3Builder::new()
        .metadata(metadata_pda)
        .mint(token_mint_account)
        .mint_authority(user_pubkey)
        .payer(user_pubkey)
        .update_authority(user_pubkey, true)
        .data(metadata_data)
        .is_mutable(true)
        .instruction();

    let create_metadata_instruction_sdk = SdkInstruction {
        program_id: token_metadata_program_id_2,
        accounts: create_metadata_instruction
            .accounts
            .iter()
            .map(|account_meta| solana_sdk::instruction::AccountMeta {
                pubkey: Pubkey::new_from_array(account_meta.pubkey.to_bytes()),
                is_signer: account_meta.is_signer,
                is_writable: account_meta.is_writable,
            })
            .collect(),
        data: create_metadata_instruction.data,
    };

    let recent_blockhash = rpc_client.get_latest_blockhash()?;
    let transaction = Transaction::new_signed_with_payer(
        &[create_metadata_instruction_sdk],
        Some(&user_keypair.pubkey()),
        &[&user_keypair],
        recent_blockhash,
    );

    let signature = rpc_client.send_and_confirm_transaction_with_spinner(&transaction)?;

    println!("Signature: {}", signature);

    Ok(())
}
