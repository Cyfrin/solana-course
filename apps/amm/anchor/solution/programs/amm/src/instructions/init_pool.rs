use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::constants;
use crate::error;
use crate::state::Pool;

#[derive(Accounts)]
#[instruction(fee: u16)]
pub struct InitPool<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = 8 + Pool::INIT_SPACE,
        seeds = [
            constants::POOL_AUTH_SEED_PREFIX,
            mint_a.key().as_ref(),
            mint_b.key().as_ref(),
            fee.to_le_bytes().as_ref(),
        ],
        bump,
    )]
    pub pool: Account<'info, Pool>,

    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = payer,
        associated_token::mint = mint_a,
        associated_token::authority = pool,
    )]
    pub pool_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = payer,
        associated_token::mint = mint_b,
        associated_token::authority = pool,
    )]
    pub pool_b: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = payer,
        seeds = [
            constants::POOL_MINT_SEED_PREFIX,
            mint_a.key().as_ref(),
            mint_b.key().as_ref(),
            fee.to_le_bytes().as_ref(),
        ],
        bump,
        mint::decimals = 6,
        mint::authority = pool,
    )]
    pub mint_pool: InterfaceAccount<'info, Mint>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn init_pool(ctx: Context<InitPool>, fee: u16) -> Result<()> {
    require!(fee <= constants::MAX_POOL_FEE, error::Error::InvalidFee);
    require!(
        ctx.accounts.mint_a.decimals == ctx.accounts.mint_b.decimals,
        error::Error::DecimalsMismatch
    );

    let pool = &mut ctx.accounts.pool;
    pool.mint_a = ctx.accounts.mint_a.key();
    pool.mint_b = ctx.accounts.mint_b.key();

    Ok(())
}
