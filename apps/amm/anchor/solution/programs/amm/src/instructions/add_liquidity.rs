use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token,
    token::{MintTo, Token, Transfer},
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use super::lib;
use crate::constants;
use crate::state::Pool;

#[derive(Accounts)]
#[instruction(fee: u16)]
pub struct AddLiquidity<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        seeds = [
            constants::POOL_AUTH_SEED_PREFIX,
            mint_a.key().as_ref(),
            mint_b.key().as_ref(),
            fee.to_le_bytes().as_ref(),
        ],
        bump,
        has_one = mint_a,
        has_one = mint_b,
    )]
    pub pool: Account<'info, Pool>,

    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = pool,
    )]
    pub pool_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = pool,
    )]
    pub pool_b: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [
            constants::POOL_MINT_SEED_PREFIX,
            mint_a.key().as_ref(),
            mint_b.key().as_ref(),
            fee.to_le_bytes().as_ref(),
        ],
        bump,
    )]
    pub mint_pool: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = payer,
    )]
    pub payer_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = payer,
    )]
    pub payer_b: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint_pool,
        associated_token::authority = payer,
    )]
    pub payer_liquidity: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn add_liquidity(
    ctx: Context<AddLiquidity>,
    fee: u16,
    amount_a: u64,
    amount_b: u64,
) -> Result<()> {
    /*
    shares = user_liquidity / pool_liquidity * supply
    */
    let user_liquidity = amount_a.checked_add(amount_b).unwrap();
    let pool_liquidity = ctx
        .accounts
        .pool_a
        .amount
        .checked_add(ctx.accounts.pool_b.amount)
        .unwrap();
    let supply = ctx.accounts.mint_pool.supply;

    let mut shares = 0;
    if pool_liquidity > 0 {
        shares = user_liquidity.checked_mul(supply).unwrap() / pool_liquidity;
    } else {
        shares = user_liquidity;
    }

    if amount_a > 0 {
        lib::transfer(
            &ctx.accounts.token_program,
            &ctx.accounts.payer_a,
            &ctx.accounts.pool_a,
            &ctx.accounts.payer,
            amount_a,
        )?;
    }

    if amount_b > 0 {
        lib::transfer(
            &ctx.accounts.token_program,
            &ctx.accounts.payer_b,
            &ctx.accounts.pool_b,
            &ctx.accounts.payer,
            amount_b,
        )?;
    }

    if shares > 0 {
        let pool_bump = ctx.bumps.pool;
        let seeds = &[
            constants::POOL_AUTH_SEED_PREFIX,
            &ctx.accounts.mint_a.key().to_bytes(),
            &ctx.accounts.mint_b.key().to_bytes(),
            &fee.to_le_bytes(),
            &[pool_bump],
        ];

        lib::mint(
            &ctx.accounts.token_program,
            &ctx.accounts.mint_pool,
            &ctx.accounts.payer_liquidity,
            &ctx.accounts.pool,
            shares,
            seeds,
        )?;
    }

    Ok(())
}
