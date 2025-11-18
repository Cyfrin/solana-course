use anchor_client::solana_sdk::signature::Signer;
use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig,
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair},
        system_instruction, system_program,
        transaction::Transaction,
    },
    Client, Cluster, Program,
};
use anchor_spl::associated_token::{
    self, get_associated_token_address, spl_associated_token_account,
};
use anchor_spl::token::spl_token::solana_program::program_pack::Pack;
use anchor_spl::token::{self, spl_token, Mint, Token};
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

use super::token_helper;
use auction;

// TODO:
// test init
// test buy
// test cancel

#[test]
fn test() {
    let program_id = auction::ID;
    let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
    let payer = read_keypair_file(&anchor_wallet).unwrap();

    // Seller and buyer
    let seller = payer;
    let buyer = Keypair::new();

    let client = Client::new_with_options(
        Cluster::Localnet,
        &seller,
        CommitmentConfig::confirmed(),
    );
    let program = client.program(program_id).unwrap();

    let rpc = program.rpc();

    // Airdrop
    rpc.request_airdrop(&seller.pubkey(), 100 * (1e9 as u64))
        .unwrap();
    rpc.request_airdrop(&buyer.pubkey(), 100 * (1e9 as u64))
        .unwrap();

    // Mint sell and buy tokens
    let token_program = client.program(token::ID).unwrap();
    let mint_sell = Keypair::new();
    let mint_buy = Keypair::new();

    token_helper::create_mint(&token_program, &seller, &mint_sell, 6);
    token_helper::create_mint(&token_program, &seller, &mint_buy, 6);

    // Create associated token accounts
    let seller_sell_ata = token_helper::create_ata(
        &token_program,
        &seller,
        &mint_sell.pubkey(),
        &seller.pubkey(),
    )
    .unwrap();

    let buyer_sell_ata = token_helper::create_ata(
        &token_program,
        &seller,
        &mint_sell.pubkey(),
        &buyer.pubkey(),
    )
    .unwrap();

    let seller_buy_ata = token_helper::create_ata(
        &token_program,
        &seller,
        &mint_buy.pubkey(),
        &seller.pubkey(),
    )
    .unwrap();

    let buyer_buy_ata = token_helper::create_ata(
        &token_program,
        &seller,
        &mint_buy.pubkey(),
        &buyer.pubkey(),
    )
    .unwrap();

    // Mint tokens
    token_helper::mint_to(
        &token_program,
        &seller,
        &mint_sell.pubkey(),
        &seller_sell_ata,
        100 * (1e6 as u64),
    )
    .unwrap();
    token_helper::mint_to(
        &token_program,
        &seller,
        &mint_buy.pubkey(),
        &buyer_buy_ata,
        200 * (1e6 as u64),
    )
    .unwrap();

    // Calculate Auction PDA
    let (pda, bump) = Pubkey::find_program_address(
        &[
            auction::state::Auction::SEED_PREFIX,
            &seller.pubkey().as_ref(),
            mint_sell.pubkey().as_ref(),
            mint_buy.pubkey().as_ref(),
        ],
        &program_id,
    );

    // Init
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let start_price = (2.0 * 1e6) as u64;
    let end_price = (1.1 * 1e6) as u64;
    let start_time = now + 1;
    let end_time = start_time + 10;
    let sell_amt = 100 * (1e6 as u64);
    let auction_sell_ata =
        get_associated_token_address(&pda, &mint_sell.pubkey());

    program
        .request()
        .accounts(auction::accounts::Init {
            payer: seller.pubkey(),
            mint_sell: mint_sell.pubkey(),
            mint_buy: mint_buy.pubkey(),
            auction: pda,
            auction_sell_ata,
            seller_sell_ata,
            seller_buy_ata,
            token_program: token::ID,
            associated_token_program: spl_associated_token_account::ID,
            system_program: system_program::ID,
        })
        .signer(&seller)
        .args(auction::instruction::Init {
            start_price,
            end_price,
            start_time,
            end_time,
            sell_amt,
        })
        .send()
        .unwrap();

    let auction: auction::state::Auction = program.account(pda).unwrap();
    assert_eq!(auction.mint_sell, mint_sell.pubkey(), "auction.mint_sell");
    assert_eq!(auction.mint_buy, mint_buy.pubkey(), "auction.mint_buy");
    assert_eq!(auction.start_time, start_time, "auction.start_time");
    assert_eq!(auction.end_time, end_time, "auction.end_time");
    assert_eq!(auction.end_time, end_time, "auction.end_time");

    assert_eq!(
        token_helper::get_balance(&token_program, &seller_sell_ata).unwrap(),
        0,
        "Seller sell ATA balance"
    );
    assert_eq!(
        token_helper::get_balance(&token_program, &auction_sell_ata).unwrap(),
        sell_amt,
        "Auction sell ATA balance"
    );

    // Buy
    let wait_time = start_time - now + 1;
    println!("Waiting {:?} seconds for auction to start", wait_time);
    std::thread::sleep(std::time::Duration::from_secs(wait_time));

    program
        .request()
        .accounts(auction::accounts::Buy {
            buyer: buyer.pubkey(),
            seller: seller.pubkey(),
            mint_sell: mint_sell.pubkey(),
            mint_buy: mint_buy.pubkey(),
            auction: pda,
            auction_sell_ata,
            buyer_buy_ata,
            buyer_sell_ata,
            seller_buy_ata,
            token_program: token::ID,
            associated_token_program: spl_associated_token_account::ID,
            system_program: system_program::ID,
        })
        .signer(&buyer)
        .args(auction::instruction::Buy {
            max_price: start_price - 1,
        })
        .send()
        .unwrap();

    assert!(
        program.account::<auction::state::Auction>(pda).is_err(),
        "Auction not closed"
    );

    assert!(
        token_helper::get_balance(&token_program, &seller_buy_ata).unwrap() > 0,
        "Seller buy ATA balance"
    );
    assert_eq!(
        token_helper::get_balance(&token_program, &buyer_sell_ata).unwrap(),
        sell_amt,
        "Buyer sell ATA balance"
    );
    assert_eq!(
        token_helper::get_balance(&token_program, &auction_sell_ata)
            .unwrap_or(0),
        0,
        "Auction sell ATA balance"
    );
}
