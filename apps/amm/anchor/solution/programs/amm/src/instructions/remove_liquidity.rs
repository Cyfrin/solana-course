use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use super::lib;
use crate::constants;
use crate::error;
use crate::state::Pool;

#[derive(Accounts)]
#[instruction(fee: u16)]
pub struct RemoveLiquidity<'info> {
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
        mut,
        associated_token::mint = mint_pool,
        associated_token::authority = payer,
    )]
    pub payer_liquidity: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn remove_liquidity(
    ctx: Context<RemoveLiquidity>,
    fee: u16,
    shares: u64,
    min_amount_a: u64,
    min_amount_b: u64,
) -> Result<()> {
    /*
    shares / supply = (amount_a + amount_b) / (pool_a + pool_b)
    amount_a = shares / supply * pool_a_amount
    amount_b = shares / supply * pool_b_amount
    */
    let amount_a = shares
        .checked_mul(ctx.accounts.pool_a.amount)
        .unwrap()
        .checked_div(ctx.accounts.mint_pool.supply)
        .unwrap();
    let amount_b = shares
        .checked_mul(ctx.accounts.pool_b.amount)
        .unwrap()
        .checked_div(ctx.accounts.mint_pool.supply)
        .unwrap();

    require!(amount_a >= min_amount_a, error::Error::MinAmountOut);
    require!(amount_b >= min_amount_b, error::Error::MinAmountOut);

    // NOTE: No withdraw fee
    // payer can call add_liquidity + remove_liquidity to swap tokens without paying swap fee

    lib::burn(
        &ctx.accounts.token_program,
        &ctx.accounts.mint_pool,
        &ctx.accounts.payer_liquidity,
        &ctx.accounts.payer,
        shares,
    )?;

    let pool_bump = ctx.bumps.pool;
    let seeds = &[
        constants::POOL_AUTH_SEED_PREFIX,
        &ctx.accounts.mint_a.key().to_bytes(),
        &ctx.accounts.mint_b.key().to_bytes(),
        &fee.to_le_bytes(),
        &[pool_bump],
    ];

    if amount_a > 0 {
        lib::transfer_from_pool(
            &ctx.accounts.token_program,
            &ctx.accounts.pool_a,
            &ctx.accounts.payer_a,
            &ctx.accounts.pool,
            amount_a,
            seeds,
        )?;
    }

    if amount_b > 0 {
        lib::transfer_from_pool(
            &ctx.accounts.token_program,
            &ctx.accounts.pool_b,
            &ctx.accounts.payer_b,
            &ctx.accounts.pool,
            amount_b,
            seeds,
        )?;
    }

    Ok(())
}
