use borsh::BorshSerialize;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{Sysvar, clock::Clock, rent::Rent},
};

use super::lib::get_pda;
use crate::state::Lock;

pub fn lock(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amt: u64,
    exp: u64,
    bump: u8,
) -> Result<(), ProgramError> {
    let account_iter = &mut accounts.iter();
    let payer = next_account_info(account_iter)?;
    let dst = next_account_info(account_iter)?;
    let pda = next_account_info(account_iter)?;
    let sys_program = next_account_info(account_iter)?;

    // Check dst signed, verifies dst exists and approved to later receive SOL

    // Check that the given account key matches expected PDA

    // Check amt > 0

    // Verify expiration is in the future
    let clock = Clock::get()?;
    let now: u64 = clock.unix_timestamp.try_into().unwrap();

    // 32 + 8
    let space = 40;
    let rent = Rent::get()?.minimum_balance(space);

    // Create PDA account

    // Transfer SOL from payer to PDA

    // Create and save lock state into PDA data

    msg!("Lock created: amt={}, exp={}", amt, exp);

    Ok(())
}
