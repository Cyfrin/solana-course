use borsh::BorshDeserialize;
use litesvm::LiteSVM;
use solana_program::{pubkey::Pubkey, system_program, sysvar::clock::Clock};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    signature::{Keypair, Signer},
    transaction::Transaction,
};

use piggy::{Cmd, state::Lock};

#[test]
fn test() {
    let mut svm = LiteSVM::new();

    let payer = Keypair::new();
    let dst = Keypair::new();

    let program_keypair = Keypair::new();
    let program_id = program_keypair.pubkey();
    svm.add_program_from_file(program_id, "target/deploy/piggy.so")
        .unwrap();

    svm.airdrop(&payer.pubkey(), 10_000_000_000).unwrap();

    let (pda, bump) = Pubkey::find_program_address(
        &[b"lock", payer.pubkey().as_ref(), dst.pubkey().as_ref()],
        &program_id,
    );

    // Lock
    let now = svm.get_sysvar::<Clock>().unix_timestamp;
    let amt = 1e9 as u64;
    let exp = (now + 100) as u64;
    let ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta {
                pubkey: payer.pubkey(),
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: dst.pubkey(),
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
        data: borsh::to_vec(&Cmd::Lock { amt, exp, bump }).unwrap(),
    };

    svm.send_transaction(Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer, &dst],
        svm.latest_blockhash(),
    ))
    .unwrap();

    let data = svm.get_account(&pda).unwrap().data;
    let lock = Lock::try_from_slice(&data).unwrap();
    assert_eq!(lock.dst, dst.pubkey());
    assert_eq!(lock.exp, exp);
    assert!(svm.get_balance(&pda).unwrap() >= amt);

    // Unlock
    let ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta {
                pubkey: payer.pubkey(),
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: dst.pubkey(),
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
        data: borsh::to_vec(&Cmd::Unlock { bump }).unwrap(),
    };

    let mut clock = svm.get_sysvar::<Clock>();
    clock.unix_timestamp = (exp + 1) as i64;
    svm.set_sysvar(&clock);

    svm.send_transaction(Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        svm.latest_blockhash(),
    ))
    .unwrap();

    let acc = svm.get_account(&pda);
    assert_eq!(acc, None);
    assert_eq!(svm.get_balance(&pda), None);
    assert!(svm.get_balance(&dst.pubkey()).unwrap() >= amt);
}
