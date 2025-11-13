use anchor_client::solana_sdk::signature::Signer;
use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig,
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair},
        system_program,
    },
    Client, Cluster,
};
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

use piggy;

#[test]
fn test() {
    let program_id = piggy::ID;
    let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
    let payer = read_keypair_file(&anchor_wallet).unwrap();

    let client = Client::new_with_options(
        Cluster::Localnet,
        &payer,
        CommitmentConfig::confirmed(),
    );
    let program = client.program(program_id).unwrap();

    let dst = Keypair::new();

    let (pda, bump) = Pubkey::find_program_address(
        &[
            piggy::state::Lock::SEED_PREFIX,
            payer.pubkey().as_ref(),
            dst.pubkey().as_ref(),
        ],
        &program_id,
    );

    // Lock
    let amt = 1e9 as u64;
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let dt = 1;
    let exp = now + dt;

    program
        .request()
        .accounts(piggy::accounts::Lock {
            payer: payer.pubkey(),
            dst: dst.pubkey(),
            lock: pda,
            system_program: system_program::ID,
        })
        .signer(&payer)
        .signer(&dst)
        .args(piggy::instruction::Lock { amt, exp })
        .send()
        .unwrap();

    let lock: piggy::state::Lock = program.account(pda).unwrap();
    assert_eq!(lock.dst, dst.pubkey(), "lock.dst");
    assert_eq!(lock.exp, exp, "lock.exp");

    assert!(
        program.rpc().get_balance(&pda).unwrap() >= amt,
        "PDA balance"
    );

    // Unlock - before lock expiry
    let res = program
        .request()
        .accounts(piggy::accounts::Unlock {
            payer: payer.pubkey(),
            dst: dst.pubkey(),
            lock: pda,
            system_program: system_program::ID,
        })
        .signer(&payer)
        .signer(&dst)
        .args(piggy::instruction::Unlock {})
        .send();
    assert!(res.is_err());

    // Unlock
    std::thread::sleep(std::time::Duration::from_secs(dt));

    program
        .request()
        .accounts(piggy::accounts::Unlock {
            payer: payer.pubkey(),
            dst: dst.pubkey(),
            lock: pda,
            system_program: system_program::ID,
        })
        .signer(&payer)
        .signer(&dst)
        .args(piggy::instruction::Unlock {})
        .send()
        .unwrap();

    assert!(
        program.account::<piggy::state::Lock>(pda).is_err(),
        "PDA not closed"
    );
    assert!(
        program.rpc().get_balance(&dst.pubkey()).unwrap() >= amt,
        "dst balance"
    );
}
