use borsh::BorshDeserialize;
use litesvm::LiteSVM;
use solana_program::sysvar::clock::Clock;
use solana_sdk::{signature::Signer, transaction::Transaction};

use amm::state::Pool;

mod helper;
use helper::{
    Test, create_add_liquidity_ix, create_init_pool_ix,
    create_remove_liquidity_ix, create_swap_ix, get_token_balance, setup,
};

#[test]
fn test_init_pool() {
    let mut svm = LiteSVM::new();
    let Test {
        program_id,
        payer,
        users,
        mint_a,
        mint_b,
        atas_a,
        atas_b,
        fee,
        pool_pda,
        pool_bump,
        mint_pool_pda,
        mint_pool_bump,
        pool_a,
        pool_b,
        ..
    } = setup(&mut svm);

    let ix = create_init_pool_ix(
        program_id,
        payer.pubkey(),
        fee,
        mint_a,
        mint_b,
        pool_pda,
        pool_bump,
        mint_pool_pda,
        mint_pool_bump,
        pool_a,
        pool_b,
    );

    let res = svm.send_transaction(Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        svm.latest_blockhash(),
    ));
    assert!(res.is_ok());

    let data = svm.get_account(&pool_pda).unwrap().data;
    let pool = Pool::try_from_slice(&data).unwrap();

    assert_eq!(pool.mint_a, mint_a);
    assert_eq!(pool.mint_b, mint_b);
    assert!(svm.get_balance(&pool_pda).unwrap() > 0);
    assert!(svm.get_balance(&pool_a).unwrap() > 0);
    assert!(svm.get_balance(&pool_b).unwrap() > 0);
    assert!(svm.get_balance(&mint_pool_pda).unwrap() > 0);
}

#[test]
fn test_add_liquidity() {
    let mut svm = LiteSVM::new();
    let Test {
        program_id,
        payer,
        users,
        mint_a,
        mint_b,
        atas_a,
        atas_b,
        fee,
        pool_pda,
        pool_bump,
        mint_pool_pda,
        mint_pool_bump,
        pool_a,
        pool_b,
        atas_liq,
    } = setup(&mut svm);

    // Init pool
    let ix = create_init_pool_ix(
        program_id,
        payer.pubkey(),
        fee,
        mint_a,
        mint_b,
        pool_pda,
        pool_bump,
        mint_pool_pda,
        mint_pool_bump,
        pool_a,
        pool_b,
    );

    let res = svm.send_transaction(Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        svm.latest_blockhash(),
    ));
    assert!(res.is_ok());

    // Add liquidity
    let amt_a = (10.0 * 1e6) as u64;
    let amt_b = (10.0 * 1e6) as u64;

    let ix = create_add_liquidity_ix(
        program_id,
        users[0].pubkey(),
        fee,
        mint_a,
        mint_b,
        pool_pda,
        pool_bump,
        mint_pool_pda,
        mint_pool_bump,
        pool_a,
        pool_b,
        amt_a,
        amt_b,
        atas_a[0],
        atas_b[0],
        atas_liq[0],
    );

    let res = svm.send_transaction(Transaction::new_signed_with_payer(
        &[ix],
        Some(&users[0].pubkey()),
        &[&users[0]],
        svm.latest_blockhash(),
    ));
    assert!(res.is_ok());

    assert_eq!(get_token_balance(&svm, &pool_a), amt_a);
    assert_eq!(get_token_balance(&svm, &pool_b), amt_b);
    assert_eq!(get_token_balance(&svm, &atas_liq[0]), amt_a + amt_b);
}

#[test]
fn test_remove_liquidity() {
    let mut svm = LiteSVM::new();
    let Test {
        program_id,
        payer,
        users,
        mint_a,
        mint_b,
        atas_a,
        atas_b,
        fee,
        pool_pda,
        pool_bump,
        mint_pool_pda,
        mint_pool_bump,
        pool_a,
        pool_b,
        atas_liq,
    } = setup(&mut svm);

    // Init pool
    let ix = create_init_pool_ix(
        program_id,
        payer.pubkey(),
        fee,
        mint_a,
        mint_b,
        pool_pda,
        pool_bump,
        mint_pool_pda,
        mint_pool_bump,
        pool_a,
        pool_b,
    );

    let res = svm.send_transaction(Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        svm.latest_blockhash(),
    ));
    assert!(res.is_ok());

    // Add liquidity
    let amt_a = (10.0 * 1e6) as u64;
    let amt_b = (10.0 * 1e6) as u64;

    let ix = create_add_liquidity_ix(
        program_id,
        users[0].pubkey(),
        fee,
        mint_a,
        mint_b,
        pool_pda,
        pool_bump,
        mint_pool_pda,
        mint_pool_bump,
        pool_a,
        pool_b,
        amt_a,
        amt_b,
        atas_a[0],
        atas_b[0],
        atas_liq[0],
    );

    let res = svm.send_transaction(Transaction::new_signed_with_payer(
        &[ix],
        Some(&users[0].pubkey()),
        &[&users[0]],
        svm.latest_blockhash(),
    ));
    assert!(res.is_ok());

    // Remove liquidity
    let shares = amt_a + amt_b;
    let min_amt_a = 1;
    let min_amt_b = 1;

    let user_a_bal_before = get_token_balance(&svm, &atas_a[0]);
    let user_b_bal_before = get_token_balance(&svm, &atas_b[0]);

    let ix = create_remove_liquidity_ix(
        program_id,
        users[0].pubkey(),
        fee,
        mint_a,
        mint_b,
        pool_pda,
        pool_bump,
        mint_pool_pda,
        mint_pool_bump,
        pool_a,
        pool_b,
        shares,
        min_amt_a,
        min_amt_b,
        atas_a[0],
        atas_b[0],
        atas_liq[0],
    );

    let res = svm.send_transaction(Transaction::new_signed_with_payer(
        &[ix],
        Some(&users[0].pubkey()),
        &[&users[0]],
        svm.latest_blockhash(),
    ));
    assert!(res.is_ok());

    let user_a_bal_after = get_token_balance(&svm, &atas_a[0]);
    let user_b_bal_after = get_token_balance(&svm, &atas_b[0]);

    assert!(user_a_bal_after >= user_a_bal_before);
    assert!(user_b_bal_after >= user_b_bal_before);

    assert_eq!(get_token_balance(&svm, &pool_a), 0);
    assert_eq!(get_token_balance(&svm, &pool_b), 0);
    assert_eq!(get_token_balance(&svm, &atas_liq[0]), 0);
}

#[test]
fn test_swap() {
    let mut svm = LiteSVM::new();
    let Test {
        program_id,
        payer,
        users,
        mint_a,
        mint_b,
        atas_a,
        atas_b,
        fee,
        pool_pda,
        pool_bump,
        mint_pool_pda,
        mint_pool_bump,
        pool_a,
        pool_b,
        atas_liq,
    } = setup(&mut svm);

    // Init pool
    let ix = create_init_pool_ix(
        program_id,
        payer.pubkey(),
        fee,
        mint_a,
        mint_b,
        pool_pda,
        pool_bump,
        mint_pool_pda,
        mint_pool_bump,
        pool_a,
        pool_b,
    );

    let res = svm.send_transaction(Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        svm.latest_blockhash(),
    ));
    assert!(res.is_ok());

    // Add liquidity
    let amt_a = (10.0 * 1e6) as u64;
    let amt_b = (10.0 * 1e6) as u64;

    let ix = create_add_liquidity_ix(
        program_id,
        users[0].pubkey(),
        fee,
        mint_a,
        mint_b,
        pool_pda,
        pool_bump,
        mint_pool_pda,
        mint_pool_bump,
        pool_a,
        pool_b,
        amt_a,
        amt_b,
        atas_a[0],
        atas_b[0],
        atas_liq[0],
    );

    let res = svm.send_transaction(Transaction::new_signed_with_payer(
        &[ix],
        Some(&users[0].pubkey()),
        &[&users[0]],
        svm.latest_blockhash(),
    ));
    assert!(res.is_ok());

    // Swap
    let a_for_b = true;
    let amt_in = 1e6 as u64;
    let min_amt_out = (0.95 * 1e6) as u64;

    let user_a_bal_before = get_token_balance(&svm, &atas_a[1]);
    let user_b_bal_before = get_token_balance(&svm, &atas_b[1]);
    let pool_a_bal_before = get_token_balance(&svm, &pool_a);
    let pool_b_bal_before = get_token_balance(&svm, &pool_b);

    let ix = create_swap_ix(
        program_id,
        users[1].pubkey(),
        fee,
        mint_a,
        mint_b,
        pool_pda,
        pool_bump,
        pool_a,
        pool_b,
        atas_a[1],
        atas_b[1],
        a_for_b,
        amt_in,
        min_amt_out,
    );

    let res = svm.send_transaction(Transaction::new_signed_with_payer(
        &[ix],
        Some(&users[1].pubkey()),
        &[&users[1]],
        svm.latest_blockhash(),
    ));
    assert!(res.is_ok());

    let user_a_bal_after = get_token_balance(&svm, &atas_a[1]);
    let user_b_bal_after = get_token_balance(&svm, &atas_b[1]);
    let pool_a_bal_after = get_token_balance(&svm, &pool_a);
    let pool_b_bal_after = get_token_balance(&svm, &pool_b);

    assert!(user_a_bal_after < user_a_bal_before);
    assert!(user_b_bal_after > user_b_bal_before);
    assert!(pool_a_bal_before < pool_a_bal_after);
    assert!(pool_b_bal_before > pool_b_bal_after);
}
