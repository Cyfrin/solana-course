use borsh::BorshDeserialize;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use solana_program_pack::Pack;
use spl_token_interface;

use super::lib;
use crate::constants;
use crate::state::Pool;

pub fn add_liquidity(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    fee: u16,
    amount_a: u64,
    amount_b: u64,
    pool_bump: u8,
    mint_pool_bump: u8,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let payer = next_account_info(accounts_iter)?;
    let pool = next_account_info(accounts_iter)?;
    let mint_a = next_account_info(accounts_iter)?;
    let mint_b = next_account_info(accounts_iter)?;
    let pool_a = next_account_info(accounts_iter)?;
    let pool_b = next_account_info(accounts_iter)?;
    let mint_pool = next_account_info(accounts_iter)?;
    let payer_a = next_account_info(accounts_iter)?;
    let payer_b = next_account_info(accounts_iter)?;
    let payer_liq = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    let ata_program = next_account_info(accounts_iter)?;
    let sys_program = next_account_info(accounts_iter)?;
    let rent_sysvar = next_account_info(accounts_iter)?;

    // Verify payer is signer
    assert!(payer.is_signer, "payer not signer");

    // Verify provided pool PDA matches the one calculated by lib::get_pool_pda
    let expected_pool =
        lib::get_pool_pda(program_id, mint_a.key, mint_b.key, fee, pool_bump)?;
    assert!(*pool.key == expected_pool, "Invalid pool PDA");

    // Verify provided mint_pool PDA matches the one calculated by lib::get_mint_pool_pda
    let expected_mint_pool = lib::get_mint_pool_pda(
        program_id,
        mint_a.key,
        mint_b.key,
        fee,
        mint_pool_bump,
    )?;
    assert!(
        *mint_pool.key == expected_mint_pool,
        "Invalid mint_pool PDA"
    );

    // Get Pool state
    let pool_state = {
        let pool_data = pool.data.borrow();
        Pool::try_from_slice(&pool_data)?
    };

    // Verify Pool state mint_a = mint_a from accounts_iter
    assert!(pool_state.mint_a == *mint_a.key, "Invalid mint_a");
    // Verify Pool state mint_b = mint_b from accounts_iter
    assert!(pool_state.mint_b == *mint_b.key, "Invalid mint_b");

    // Get pool_a and pool_b amounts
    let pool_a_account = {
        let pool_a_data = pool_a.data.borrow();
        spl_token_interface::state::Account::unpack(&pool_a_data).unwrap()
    };
    let pool_a_amount = pool_a_account.amount;

    let pool_b_account = {
        let pool_b_data = pool_b.data.borrow();
        spl_token_interface::state::Account::unpack(&pool_b_data).unwrap()
    };
    let pool_b_amount = pool_b_account.amount;

    // Get mint_pool supply
    let mint_pool_account = {
        let mint_pool_data = mint_pool.data.borrow();
        spl_token_interface::state::Mint::unpack(&mint_pool_data).unwrap()
    };
    let supply = mint_pool_account.supply;

    // Calculate shares to mint
    let user_liquidity = amount_a
        .checked_add(amount_b)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    let pool_liquidity = pool_a_amount
        .checked_add(pool_b_amount)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    let shares = if pool_liquidity > 0 {
        user_liquidity
            .checked_mul(supply)
            .ok_or(ProgramError::ArithmeticOverflow)?
            .checked_div(pool_liquidity)
            .ok_or(ProgramError::ArithmeticOverflow)?
    } else {
        user_liquidity
    };

    // Initialize payer_liq (associated token account for mint_pool owned by payer) if not initialized.
    if payer_liq.lamports() == 0 {
        lib::create_ata(
            payer,
            mint_pool,
            payer,
            payer_liq,
            token_program,
            sys_program,
            ata_program,
            rent_sysvar,
        )?;
    }

    // Transfer mint_a from payer to pool_a
    if amount_a > 0 {
        lib::transfer(token_program, payer_a, pool_a, payer, amount_a)?;
    }

    // Transfer mint_b from payer to pool_b
    if amount_b > 0 {
        lib::transfer(token_program, payer_b, pool_b, payer, amount_b)?;
    }

    // Mint LP tokens to payer
    if shares > 0 {
        let seeds = &[
            constants::POOL_AUTH,
            mint_a.key.as_ref(),
            mint_b.key.as_ref(),
            &fee.to_le_bytes(),
            &[pool_bump],
        ];

        lib::mint_to(token_program, mint_pool, payer_liq, pool, shares, seeds)?;
    }

    Ok(())
}
