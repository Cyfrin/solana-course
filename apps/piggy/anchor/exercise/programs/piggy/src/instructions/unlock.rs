use anchor_lang::prelude::*;

use crate::error;
use crate::state;

#[derive(Accounts)]
pub struct Unlock<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub dst: Signer<'info>,

    #[account(
        mut,
        seeds = [state::Lock::SEED_PREFIX, payer.key().as_ref(), dst.key().as_ref()],
        // Calculated off-chain, verified on-chain by this program
        bump,
        // automatically close this account
        // and send its remaining lamports to payer
        close = payer,
        constraint = lock.dst == dst.key() @ error::Error::DestinationMismatch
    )]
    pub lock: Account<'info, state::Lock>,

    pub system_program: Program<'info, System>,
}

pub fn unlock(ctx: Context<Unlock>) -> Result<()> {
    let clock = Clock::get()?;

    // Check expiration

    // Transfer all lamports to dst

    Ok(())
}
