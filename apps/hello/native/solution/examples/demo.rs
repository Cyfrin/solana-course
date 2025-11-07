use solana_client::{rpc_client::RpcClient, rpc_config::RpcTransactionConfig};
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::Instruction,
    pubkey::Pubkey,
    signature::{Keypair, Signer, read_keypair_file},
    transaction::Transaction,
};
use solana_transaction_status_client_types::{
    UiTransactionEncoding, option_serializer::OptionSerializer,
};
use std::path::PathBuf;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let keypair_path: PathBuf = [&args[1]].iter().collect();
    let payer =
        read_keypair_file(keypair_path).expect("Cannot read keypair file");

    // Connect to local cluster
    let rpc_url = String::from(&args[2]);
    let client =
        RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    let program_id = Pubkey::from_str(&args[3]).expect("Invalid program ID");

    println!("Wallet: {}", payer.pubkey());

    // Wallet balance
    let lamports = client.get_balance(&payer.pubkey()).unwrap();
    let sol =
        lamports as f64 / solana_sdk::native_token::LAMPORTS_PER_SOL as f64;
    println!("Balance: {} SOL ({} lamports)", sol, lamports);

    // Request airdrop of 1 SOL for transaction fees
    if sol < 1.0 {
        println!("Requesting airdrop...");
        let airdrop_signature = client
            .request_airdrop(&payer.pubkey(), 1_000_000_000)
            .expect("Failed to request airdrop");

        // Wait for airdrop confirmation
        while !client
            .confirm_transaction(&airdrop_signature)
            .unwrap_or(false)
        {
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
        println!("Airdrop confirmed");
    }

    // Create the instruction
    let ix = Instruction::new_with_borsh(
        program_id,
        &(),    // Empty instruction data
        vec![], // No accounts needed
    );

    // Sign and send transaction
    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer], client.get_latest_blockhash().unwrap());

    let sig = client.send_and_confirm_transaction(&tx).unwrap();

    println!("Transaction signature: {}", sig);

    // Fetch transaction details with logs
    let tx_info = client
        .get_transaction_with_config(
            &sig,
            RpcTransactionConfig {
                encoding: Some(UiTransactionEncoding::Json),
                commitment: Some(CommitmentConfig::confirmed()),
                max_supported_transaction_version: Some(0),
            },
        )
        .expect("Failed to get transaction info");

    if let Some(meta) = tx_info.transaction.meta {
        if let OptionSerializer::Some(logs) = meta.log_messages {
            println!("--- Transaction Logs ---");
            for (i, log) in logs.iter().enumerate() {
                println!("{i}: {}", log);
            }
        } else {
            println!("No logs");
        }
    } else {
        println!("Transaction metadata not found");
    }
}
