use borsh::BorshSerialize;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    program_error::ProgramError,
    pubkey::Pubkey,
};
use solana_program::{
    program::invoke_signed,
    system_instruction,
    sysvar::{Sysvar, rent::Rent},
};
use solana_program_pack::Pack;
use spl_token_interface;

use super::lib;
use crate::constants;
use crate::state::Pool;

pub fn init_pool(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    fee: u16,
    pool_bump: u8,
    mint_pool_bump: u8,
) -> Result<(), ProgramError> {
    let accounts_iter = &mut accounts.iter();

    let payer = next_account_info(accounts_iter)?;
    let pool = next_account_info(accounts_iter)?;
    let mint_a = next_account_info(accounts_iter)?;
    let mint_b = next_account_info(accounts_iter)?;
    let pool_a = next_account_info(accounts_iter)?;
    let pool_b = next_account_info(accounts_iter)?;
    let mint_pool = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    let ata_program = next_account_info(accounts_iter)?;
    let sys_program = next_account_info(accounts_iter)?;
    let rent_sysvar = next_account_info(accounts_iter)?;

    // Verify payer is signer
    assert!(payer.is_signer, "payer not signer");

    // Check token decimals are equal
    assert!(
        lib::get_decimals(mint_a) == lib::get_decimals(mint_b),
        "decimals mismatch"
    );

    // Verify pool, pool_a, pool_b and mint_pool accounts are not initialized
    assert!(pool.lamports() == 0, "pool already initialized");
    assert!(pool_a.lamports() == 0, "pool_a already initialized");
    assert!(pool_b.lamports() == 0, "pool_b already initialized");
    assert!(mint_pool.lamports() == 0, "mint_pool already initialized");

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

    // Create pool PDA
    let rent = Rent::get()?;

    invoke_signed(
        &system_instruction::create_account(
            payer.key,
            pool.key,
            rent.minimum_balance(Pool::SPACE as usize),
            Pool::SPACE,
            program_id,
        ),
        &[payer.clone(), pool.clone(), sys_program.clone()],
        &[&[
            constants::POOL_AUTH,
            mint_a.key.as_ref(),
            mint_b.key.as_ref(),
            fee.to_le_bytes().as_ref(),
            &[pool_bump],
        ]],
    )?;

    // Create pool_a associated token account
    lib::create_ata(
        payer,
        mint_a,
        pool,
        pool_a,
        token_program,
        sys_program,
        ata_program,
        rent_sysvar,
    )?;

    // Create pool_b associated token account
    lib::create_ata(
        payer,
        mint_b,
        pool,
        pool_b,
        token_program,
        sys_program,
        ata_program,
        rent_sysvar,
    )?;

    // Create mint_pool PDA
    invoke_signed(
        &system_instruction::create_account(
            payer.key,
            mint_pool.key,
            rent.minimum_balance(spl_token_interface::state::Mint::LEN),
            spl_token_interface::state::Mint::LEN as u64,
            token_program.key,
        ),
        &[payer.clone(), mint_pool.clone(), sys_program.clone()],
        &[&[
            constants::POOL_MINT,
            mint_a.key.as_ref(),
            mint_b.key.as_ref(),
            fee.to_le_bytes().as_ref(),
            &[mint_pool_bump],
        ]],
    )?;

    // Initialize mint_pool
    lib::init_mint(token_program, mint_pool, pool, rent_sysvar)?;

    // Initialize pool state
    let mut data = pool.data.borrow_mut();
    let pool_state = Pool {
        mint_a: *mint_a.key,
        mint_b: *mint_b.key,
    };
    pool_state.serialize(&mut &mut data[..])?;

    Ok(())
}
