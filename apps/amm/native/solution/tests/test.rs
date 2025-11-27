use borsh::BorshDeserialize;
use litesvm::LiteSVM;
use solana_program::sysvar::clock::Clock;
use solana_sdk::{signature::Signer, transaction::Transaction};

use amm::{Cmd, state::Pool};

mod helper;
use helper::{Test, create_init_pool_ix, get_token_balance, setup};

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

/*
#[test]
fn test_buy() {
    let mut svm = LiteSVM::new();
    let Test {
        program_id,
        payer,
        seller,
        buyer,
        mint_sell,
        mint_buy,
        seller_sell_ata,
        seller_buy_ata,
        buyer_sell_ata,
        buyer_buy_ata,
        auction_pda,
        auction_bump,
        auction_sell_ata,
    } = setup(&mut svm);

    // Init
    let now = svm.get_sysvar::<Clock>().unix_timestamp as u64;
    let start_time = now + 1;
    let end_time = start_time + 10;
    let start_price = (2.0 * 1e6) as u64;
    let end_price = (1.5 * 1e6) as u64;
    let sell_amt = 1e8 as u64;

    let ix = create_init_ix(
        program_id,
        start_price,
        end_price,
        start_time,
        end_time,
        sell_amt,
        auction_bump,
        seller.pubkey(),
        mint_sell,
        mint_buy,
        auction_pda,
        auction_sell_ata,
        seller_sell_ata,
    );

    let res = svm.send_transaction(Transaction::new_signed_with_payer(
        &[ix],
        Some(&seller.pubkey()),
        &[&seller],
        svm.latest_blockhash(),
    ));
    assert!(res.is_ok());

    // Buy
    let mut clock = svm.get_sysvar::<Clock>();
    clock.unix_timestamp = (start_time + 2) as i64;
    svm.set_sysvar(&clock);

    let max_price = start_price - 1;

    let ix = create_buy_ix(
        program_id,
        max_price,
        auction_bump,
        buyer.pubkey(),
        seller.pubkey(),
        mint_sell,
        mint_buy,
        auction_pda,
        auction_sell_ata,
        buyer_sell_ata,
        buyer_buy_ata,
        seller_buy_ata,
    );

    let res = svm.send_transaction(Transaction::new_signed_with_payer(
        &[ix],
        Some(&buyer.pubkey()),
        &[&buyer],
        svm.latest_blockhash(),
    ));
    assert!(res.is_ok());

    assert_eq!(svm.get_balance(&auction_pda).unwrap_or(0), 0);
    assert_eq!(svm.get_balance(&auction_sell_ata).unwrap_or(0), 0);
    assert_eq!(get_token_balance(&svm, &buyer_sell_ata), sell_amt);
    assert!(get_token_balance(&svm, &seller_buy_ata) > 0);
}

#[test]
fn test_cancel() {
    let mut svm = LiteSVM::new();
    let Test {
        program_id,
        payer,
        seller,
        buyer,
        mint_sell,
        mint_buy,
        seller_sell_ata,
        seller_buy_ata,
        buyer_sell_ata,
        buyer_buy_ata,
        auction_pda,
        auction_bump,
        auction_sell_ata,
    } = setup(&mut svm);

    // Init
    let now = svm.get_sysvar::<Clock>().unix_timestamp as u64;
    let start_time = now + 1;
    let end_time = start_time + 10;
    let start_price = (2.0 * 1e6) as u64;
    let end_price = (1.5 * 1e6) as u64;
    let sell_amt = 1e8 as u64;

    let ix = create_init_ix(
        program_id,
        start_price,
        end_price,
        start_time,
        end_time,
        sell_amt,
        auction_bump,
        seller.pubkey(),
        mint_sell,
        mint_buy,
        auction_pda,
        auction_sell_ata,
        seller_sell_ata,
    );

    let res = svm.send_transaction(Transaction::new_signed_with_payer(
        &[ix],
        Some(&seller.pubkey()),
        &[&seller],
        svm.latest_blockhash(),
    ));
    assert!(res.is_ok());

    // Cancel
    let ix = create_cancel_ix(
        program_id,
        auction_bump,
        seller.pubkey(),
        mint_sell,
        mint_buy,
        auction_pda,
        auction_sell_ata,
        seller_sell_ata,
    );

    let res = svm.send_transaction(Transaction::new_signed_with_payer(
        &[ix],
        Some(&seller.pubkey()),
        &[&seller],
        svm.latest_blockhash(),
    ));
    assert!(res.is_ok());

    assert_eq!(svm.get_balance(&auction_pda).unwrap_or(0), 0);
    assert_eq!(svm.get_balance(&auction_sell_ata).unwrap_or(0), 0);
    assert_eq!(get_token_balance(&svm, &seller_sell_ata), 1e9 as u64);


}

*/
