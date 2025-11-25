use borsh::BorshDeserialize;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{Sysvar, clock::Clock},
};

use super::lib::{
    close_ata, get_ata, get_pda, get_token_balance, transfer, transfer_from_pda,
};
use crate::state::Auction;

pub fn buy(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    max_price: u64,
    // Auction PDA bump
    bump: u8,
) -> Result<(), ProgramError> {
    let account_iter = &mut accounts.iter();

    let buyer = next_account_info(account_iter)?;
    let seller = next_account_info(account_iter)?;
    let mint_sell = next_account_info(account_iter)?;
    let mint_buy = next_account_info(account_iter)?;
    let auction_pda = next_account_info(account_iter)?;
    let auction_sell_ata = next_account_info(account_iter)?;
    let buyer_sell_ata = next_account_info(account_iter)?;
    let buyer_buy_ata = next_account_info(account_iter)?;
    let seller_buy_ata = next_account_info(account_iter)?;
    let token_program = next_account_info(account_iter)?;
    let sys_program = next_account_info(account_iter)?;

    // Check buyer signed
    if !buyer.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    // Check that auction_pda matches expected PDA
    if *auction_pda.key
        != get_pda(program_id, seller.key, mint_sell.key, mint_buy.key, bump)?
    {
        return Err(ProgramError::InvalidSeeds);
    }
    // Check that auction_sell_ata matches calculated account
    if *auction_sell_ata.key != get_ata(auction_pda.key, mint_sell.key) {
        return Err(ProgramError::InvalidArgument);
    }
    // Check that buyer_sell_ata matches calculated account
    if *buyer_sell_ata.key != get_ata(buyer.key, mint_sell.key) {
        return Err(ProgramError::InvalidArgument);
    }
    // Check that buyer_buy_ata matches calculated account
    if *buyer_buy_ata.key != get_ata(buyer.key, mint_buy.key) {
        return Err(ProgramError::InvalidArgument);
    }
    // Check that seller_buy_ata matches calculated account
    if *seller_buy_ata.key != get_ata(seller.key, mint_buy.key) {
        return Err(ProgramError::InvalidArgument);
    }

    let clock = Clock::get()?;
    let now: u64 = clock.unix_timestamp.try_into().unwrap();

    let auction = {
        let data = auction_pda.data.borrow();
        Auction::try_from_slice(&data)?
    }; // Drop borrow here

    // Check auction has started
    assert!(auction.start_time <= now, "auction not started");
    // Check auction has not ended
    assert!(now < auction.end_time, "auction ended");

    // Calculate price
    let price_decrease = (auction.start_price - auction.end_price)
        * (now - auction.start_time)
        / (auction.end_time - auction.start_time);

    let price = auction.start_price - price_decrease;

    // Check current price is greater than or equal to end_price
    assert!(price >= auction.end_price, "price < min");

    // Check current price is less than or equal to max_price
    assert!(price <= max_price, "price > max");

    // Calculate amount of buy token to send to seller
    let sell_amt = get_token_balance(auction_sell_ata)?;
    let buy_amt = sell_amt * price / (1e6 as u64);

    // Send buy token to seller
    transfer(token_program, buyer_buy_ata, seller_buy_ata, buyer, buy_amt)?;

    // Send sell token to buyer
    let seeds = &[
        Auction::SEED_PREFIX,
        seller.key.as_ref(),
        mint_sell.key.as_ref(),
        mint_buy.key.as_ref(),
        &[bump],
    ];

    transfer_from_pda(
        token_program,
        auction_sell_ata,
        buyer_sell_ata,
        auction_pda,
        sell_amt,
        seeds,
    )?;

    // Close auction_sell_ata
    close_ata(token_program, auction_sell_ata, seller, auction_pda, seeds)?;

    // Close auction_pda
    // Get PDA balance and transfer lamports directly
    let pda_lamports = auction_pda.lamports();

    **auction_pda.try_borrow_mut_lamports()? = 0;
    **seller.try_borrow_mut_lamports()? = seller
        .lamports()
        .checked_add(pda_lamports)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    // Clear out data
    auction_pda.resize(0)?;

    // Assign the account to the System Program
    auction_pda.assign(sys_program.key);

    Ok(())
}
