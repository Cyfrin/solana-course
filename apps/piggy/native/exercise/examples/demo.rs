use solana_client::rpc_client::RpcClient;
use solana_program::system_program;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer, read_keypair_file},
    system_instruction,
    transaction::Transaction,
};
use std::path::PathBuf;
use std::str::FromStr;

use borsh::BorshDeserialize;
use piggy::Cmd;
use piggy::state::Lock;

fn create_lock_ix(
    program_id: Pubkey,
    payer: Pubkey,
    dst: Pubkey,
    pda: Pubkey,
    amt: u64,
    exp: u64,
    bump: u8,
) -> Instruction {
    let cmd = Cmd::Lock { amt, exp, bump };

    Instruction::new_with_borsh(
        program_id,
        &cmd,
        vec![
            AccountMeta {
                pubkey: payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: dst,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pda,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: system_program::ID,
                is_signer: false,
                is_writable: true,
            },
        ],
    )
}

fn create_unlock_ix(
    program_id: Pubkey,
    payer: Pubkey,
    pda: Pubkey,
    dst: Pubkey,
    bump: u8,
) -> Instruction {
    let cmd = Cmd::Unlock { bump };

    Instruction::new_with_borsh(
        program_id,
        &cmd,
        vec![
            AccountMeta {
                pubkey: payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: dst,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pda,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: system_program::ID,
                is_signer: false,
                is_writable: true,
            },
        ],
    )
}

fn create_account(
    client: &RpcClient,
    payer: &Keypair,
    dst: &Keypair,
    program_id: Pubkey,
    space: usize,
) {
    let lamports = client
        .get_minimum_balance_for_rent_exemption(space)
        .unwrap();

    let ix = system_instruction::create_account(
        &payer.pubkey(),
        &dst.pubkey(),
        lamports,
        space as u64,
        &program_id,
    );

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));

    let blockhash = client.get_latest_blockhash().unwrap();
    tx.sign(&[&payer, &dst], blockhash);

    let res = client.send_and_confirm_transaction(&tx);
    res.unwrap();
}

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

    let dst = Keypair::new();
    println!("dst: {:?}", dst.pubkey());
    // Need to create account to receive SOL
    create_account(&client, &payer, &dst, system_program::ID, 0);

    let (pda, bump) = Pubkey::find_program_address(
        &[b"lock", payer.pubkey().as_ref(), dst.pubkey().as_ref()],
        &program_id,
    );
    println!("PDA: {:?}", pda);

    // Lock - invalid bump
    let now = client.get_block_time(client.get_slot().unwrap()).unwrap() as u64;
    let amt = 1e9 as u64;
    let exp = now + 3;

    let ix = create_lock_ix(
        program_id,
        payer.pubkey(),
        dst.pubkey(),
        pda,
        amt,
        exp,
        if bump == 0 { bump + 1 } else { 0 },
    );

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer, &dst], client.get_latest_blockhash().unwrap());

    let res = client.send_and_confirm_transaction(&tx);
    assert!(res.is_err());

    // Lock - amt = 0
    let now = client.get_block_time(client.get_slot().unwrap()).unwrap() as u64;
    let amt = 0;
    let exp = now + 3;

    let ix = create_lock_ix(
        program_id,
        payer.pubkey(),
        dst.pubkey(),
        pda,
        amt,
        exp,
        bump,
    );

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer, &dst], client.get_latest_blockhash().unwrap());

    let res = client.send_and_confirm_transaction(&tx);
    assert!(res.is_err());

    // Lock - exp <= now
    let now = client.get_block_time(client.get_slot().unwrap()).unwrap() as u64;
    let amt = 1e9 as u64;
    let exp = now;

    let ix = create_lock_ix(
        program_id,
        payer.pubkey(),
        dst.pubkey(),
        pda,
        amt,
        exp,
        bump,
    );

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer, &dst], client.get_latest_blockhash().unwrap());

    let res = client.send_and_confirm_transaction(&tx);
    assert!(res.is_err());

    // Lock
    let now = client.get_block_time(client.get_slot().unwrap()).unwrap() as u64;
    let amt = 1e9 as u64;
    let dt = 1;
    let exp = now + dt;

    let ix = create_lock_ix(
        program_id,
        payer.pubkey(),
        dst.pubkey(),
        pda,
        amt,
        exp,
        bump,
    );

    let mut tx =
        Transaction::new_with_payer(&[ix.clone()], Some(&payer.pubkey()));
    tx.sign(&[&payer, &dst], client.get_latest_blockhash().unwrap());

    let res = client.send_and_confirm_transaction(&tx);

    if res.is_err() {
        println!("Lock tx: {:#?}", res);
    }
    assert!(res.is_ok());

    let data = client
        .get_account_data(&pda)
        .expect("Failed to fetch account data");

    let lock = Lock::try_from_slice(&data).expect("Failed to deserialize");

    println!("lock.dst: {:?}", lock.dst);
    println!("lock.exp: {:?}", lock.exp);
    println!("locked amt: {:?}", client.get_balance(&pda));

    assert_eq!(lock.dst, dst.pubkey());
    assert_eq!(lock.exp, exp);
    assert!(client.get_balance(&pda).unwrap() >= amt);

    // Cannot re-lock
    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer, &dst], client.get_latest_blockhash().unwrap());

    let res = client.send_and_confirm_transaction(&tx);
    assert!(res.is_err());

    // Unlock before expiry
    let ix =
        create_unlock_ix(program_id, payer.pubkey(), pda, dst.pubkey(), bump);
    tx.sign(&[&payer], client.get_latest_blockhash().unwrap());

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer], client.get_latest_blockhash().unwrap());

    let res = client.send_and_confirm_transaction(&tx);
    assert!(res.is_err());

    println!("Waiting for lock expiry {:?} seconds...", dt + 1);
    std::thread::sleep(std::time::Duration::from_secs(dt + 1));

    // Unlock invalid bump
    let ix = create_unlock_ix(
        program_id,
        payer.pubkey(),
        pda,
        dst.pubkey(),
        if bump == 0 { bump + 1 } else { 0 },
    );

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer], client.get_latest_blockhash().unwrap());

    let res = client.send_and_confirm_transaction(&tx);
    assert!(res.is_err());

    // Unlock invalid dst
    let ix = create_unlock_ix(
        program_id,
        payer.pubkey(),
        pda,
        payer.pubkey(),
        if bump == 0 { bump + 1 } else { 0 },
    );

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer], client.get_latest_blockhash().unwrap());

    let res = client.send_and_confirm_transaction(&tx);
    assert!(res.is_err());

    // Unlock
    let ix =
        create_unlock_ix(program_id, payer.pubkey(), pda, dst.pubkey(), bump);

    let mut tx =
        Transaction::new_with_payer(&[ix.clone()], Some(&payer.pubkey()));
    tx.sign(&[&payer], client.get_latest_blockhash().unwrap());

    let res = client.send_and_confirm_transaction(&tx);

    if res.is_err() {
        println!("Unlock tx: {:#?}", res);
    }
    assert!(res.is_ok());

    // Err when account doesn't exist
    assert!(client.get_account_data(&pda).is_err());
    assert_eq!(client.get_balance(&pda).unwrap_or(0), 0);

    assert!(client.get_balance(&dst.pubkey()).unwrap() >= amt);

    // Cannot unlock again
    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer], client.get_latest_blockhash().unwrap());

    let res = client.send_and_confirm_transaction(&tx);
    assert!(res.is_err());

    println!("All tests passed");
}
