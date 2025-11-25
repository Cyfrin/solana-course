use borsh::BorshSerialize;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{Sysvar, clock::Clock, rent::Rent},
};

use super::lib::{create_ata, get_ata, get_pda, transfer};
use crate::state::Auction;

pub fn init(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    start_price: u64,
    end_price: u64,
    start_time: u64,
    end_time: u64,
    sell_amt: u64,
    // Auction PDA bump
    bump: u8,
) -> Result<(), ProgramError> {
    let account_iter = &mut accounts.iter();

    let seller = next_account_info(account_iter)?;
    let mint_sell = next_account_info(account_iter)?;
    let mint_buy = next_account_info(account_iter)?;
    let auction_pda = next_account_info(account_iter)?;
    let auction_sell_ata = next_account_info(account_iter)?;
    let seller_sell_ata = next_account_info(account_iter)?;
    let token_program = next_account_info(account_iter)?;
    let ata_program = next_account_info(account_iter)?;
    let sys_program = next_account_info(account_iter)?;
    let rent_sysvar = next_account_info(account_iter)?;

    // Check seller signed
    if !seller.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    // Check that auction_pda matches expected PDA
    if *auction_pda.key
        != get_pda(program_id, seller.key, mint_sell.key, mint_buy.key, bump)?
    {
        return Err(ProgramError::InvalidSeeds);
    }
    // Check auction_sell_ata
    if *auction_sell_ata.key != get_ata(auction_pda.key, mint_sell.key) {
        return Err(ProgramError::InvalidArgument);
    }
    // Check seller_sell_ata
    if *seller_sell_ata.key != get_ata(seller.key, mint_sell.key) {
        return Err(ProgramError::InvalidArgument);
    }
    // Check sell token != buy token
    if *mint_sell.key == *mint_buy.key {
        return Err(ProgramError::InvalidArgument);
    }
    // Check start_price >= end_price
    if start_price < end_price {
        return Err(ProgramError::InvalidArgument);
    }
    // Check now <= start_time < end_time
    let clock = Clock::get()?;
    let now: u64 = clock.unix_timestamp.try_into().unwrap();
    if (start_time < now) || (end_time <= start_time) {
        return Err(ProgramError::InvalidArgument);
    }
    // Check sell_amt > 0
    if sell_amt == 0 {
        return Err(ProgramError::InvalidArgument);
    }

    // 32 + 32 + 8 + 8 + 8 + 8 = 96
    let space = 96;
    let rent = Rent::get()?.minimum_balance(space);

    // Create PDA account
    invoke_signed(
        &system_instruction::create_account(
            seller.key,
            auction_pda.key,
            rent,
            space as u64,
            program_id,
        ),
        &[seller.clone(), auction_pda.clone(), sys_program.clone()],
        &[&[
            Auction::SEED_PREFIX,
            seller.key.as_ref(),
            mint_sell.key.as_ref(),
            mint_buy.key.as_ref(),
            &[bump],
        ]],
    )?;

    // Create auction_sell_ata
    create_ata(
        seller,
        mint_sell,
        auction_pda,
        auction_sell_ata,
        token_program,
        sys_program,
        ata_program,
        rent_sysvar,
    )?;

    // Send sell token to auction_sell_ata
    transfer(
        token_program,
        seller_sell_ata,
        auction_sell_ata,
        seller,
        sell_amt,
    )?;

    // Store Auction state
    let mut data = auction_pda.data.borrow_mut();
    let auction = Auction {
        mint_sell: *mint_sell.key,
        mint_buy: *mint_buy.key,
        start_price,
        end_price,
        start_time,
        end_time,
    };
    auction.serialize(&mut &mut data[..])?;

    Ok(())
}
