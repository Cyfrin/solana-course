use borsh::BorshDeserialize;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use super::lib;
use crate::constants;
use crate::state::Pool;

pub fn swap(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    fee: u16,
    a_for_b: bool,
    amount_in: u64,
    min_amount_out: u64,
    pool_bump: u8,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let payer = next_account_info(accounts_iter)?;
    let pool = next_account_info(accounts_iter)?;
    let mint_a = next_account_info(accounts_iter)?;
    let mint_b = next_account_info(accounts_iter)?;
    let pool_a = next_account_info(accounts_iter)?;
    let pool_b = next_account_info(accounts_iter)?;
    let payer_a = next_account_info(accounts_iter)?;
    let payer_b = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;

    // Verify payer is signer
    assert!(payer.is_signer, "payer not signer");

    // Verify provided pool PDA matches the one calculated by lib::get_pool_pda
    let expected_pool =
        lib::get_pool_pda(program_id, mint_a.key, mint_b.key, fee, pool_bump)?;
    assert!(*pool.key == expected_pool, "Invalid pool PDA");

    // Get Pool state
    let pool_state = {
        let pool_data = pool.data.borrow();
        Pool::try_from_slice(&pool_data)?
    };

    // Verify Pool state mint_a = mint_a from accounts_iter
    assert!(pool_state.mint_a == *mint_a.key, "Invalid mint_a");
    // Verify Pool state mint_b = mint_b from accounts_iter
    assert!(pool_state.mint_b == *mint_b.key, "Invalid mint_b");

    // Calculate amount out with fee
    let mut amount_out = amount_in;
    let amount_out_fee = amount_out
        .checked_mul(fee as u64)
        .ok_or(ProgramError::ArithmeticOverflow)?
        .checked_div(constants::MAX_POOL_FEE as u64)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    amount_out = amount_out
        .checked_sub(amount_out_fee)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    // Check amount out is >= minimum amount specified by payer
    assert!(amount_out >= min_amount_out, "amount out < min");

    // Determine swap direction
    let (mint_in, mint_out, pool_in, pool_out, payer_in, payer_out) = if a_for_b
    {
        (mint_a, mint_b, pool_a, pool_b, payer_a, payer_b)
    } else {
        (mint_b, mint_a, pool_b, pool_a, payer_b, payer_a)
    };

    // Transfer token from payer to pool
    lib::transfer(token_program, payer_in, pool_in, payer, amount_in)?;

    // Transfer token from pool to payer
    let seeds = &[
        constants::POOL_AUTH,
        mint_a.key.as_ref(),
        mint_b.key.as_ref(),
        &fee.to_le_bytes(),
        &[pool_bump],
    ];

    lib::transfer_from_pool(
        token_program,
        pool_out,
        payer_out,
        pool,
        amount_out,
        seeds,
    )?;

    Ok(())
}
