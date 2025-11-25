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
    // Check that auction_pda matches expected PDA
    // Check that auction_sell_ata matches calculated account
    // Check that buyer_sell_ata matches calculated account
    // Check that buyer_buy_ata matches calculated account
    // Check that seller_buy_ata matches calculated account

    let clock = Clock::get()?;
    let now: u64 = clock.unix_timestamp.try_into().unwrap();

    // Check auction has started
    // Check auction has not ended

    // Calculate price
    // Check current price is greater than or equal to end_price
    // Check current price is less than or equal to max_price

    // Calculate amount of buy token to send to seller

    // Send buy token to seller

    // Send sell token to buyer
    let seeds = &[
        Auction::SEED_PREFIX,
        seller.key.as_ref(),
        mint_sell.key.as_ref(),
        mint_buy.key.as_ref(),
        &[bump],
    ];

    // Close auction_sell_ata

    // Close auction_pda

    Ok(())
}
