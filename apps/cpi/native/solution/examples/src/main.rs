use solana_address::Address;
use solana_client::rpc_client::RpcClient;
use solana_program_pack::Pack;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use std::path::PathBuf;
use std::str::FromStr;
use std::thread;
use std::time::Duration;

use borsh::BorshDeserialize;
use counter::Counter;
use factory::Cmd;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let keypair_path: PathBuf = [&args[1]].iter().collect();
    let payer =
        read_keypair_file(keypair_path).expect("Cannot read keypair file");

    // Connect to local cluster
    let rpc_url = String::from(&args[2]);
    let client =
        RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    let factory_program_id =
        Pubkey::from_str(&args[3]).expect("Invalid factory program ID");
    let counter_program_id =
        Pubkey::from_str(&args[3]).expect("Invalid counter program ID");

    airdrop(&client, &payer.pubkey(), 1e9 as u64);

    // Init
    println!("--- Init  ---");

    let counter_account = Keypair::new();

    let cmd = factory::Cmd::Init;

    let ix = Instruction::new_with_borsh(
        factory_program_id,
        &cmd,
        vec![
            AccountMeta {
                pubkey: payer.pubkey(),
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: counter_account.pubkey(),
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: counter_program_id,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: solana_sdk::system_program::id(),
                is_signer: false,
                is_writable: true,
            },
        ],
    );

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    let blockhash = client.get_latest_blockhash().unwrap();
    tx.sign(&[&payer], blockhash);

    let res = client.send_and_confirm_transaction(&tx);
    res.unwrap();

    // Inc
    println!("--- Inc  ---");

    let counter_account = Keypair::new();

    let cmd = factory::Cmd::Inc;

    let ix = Instruction::new_with_borsh(
        factory_program_id,
        &cmd,
        vec![
            AccountMeta {
                pubkey: counter_account.pubkey(),
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: counter_program_id,
                is_signer: false,
                is_writable: true,
            },
        ],
    );

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    let blockhash = client.get_latest_blockhash().unwrap();
    tx.sign(&[&payer], blockhash);

    let res = client.send_and_confirm_transaction(&tx);
    res.unwrap();

    // TODO: read state
    let data = client
        .get_account_data(&counter_account.pubkey())
        .expect("Failed to fetch account data");

    let counter_data =
        Counter::try_from_slice(&data).expect("Failed to deserialize");

    println!("Counter: {}", counter_data.count);
}

fn airdrop(client: &RpcClient, pubkey: &Pubkey, lamports: u64) {
    let bal = client.get_balance(pubkey).unwrap();
    if bal >= lamports {
        return;
    }

    let sig = client.request_airdrop(pubkey, lamports).unwrap();

    // Wait for airdrop confirmation
    while !client.confirm_transaction(&sig).unwrap_or(false) {
        thread::sleep(Duration::from_millis(500));
    }
}
