use anchor_lang::prelude::*;
use anchor_spl::{
    token,
    token::{Burn, MintTo, Transfer},
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::state::Pool;

pub fn transfer<'info>(
    token_program: &Interface<'info, TokenInterface>,
    src: &InterfaceAccount<'info, TokenAccount>,
    dst: &InterfaceAccount<'info, TokenAccount>,
    auth: &Signer<'info>,
    amount: u64,
) -> Result<()> {
    token::transfer(
        CpiContext::new(
            token_program.to_account_info(),
            Transfer {
                from: src.to_account_info(),
                to: dst.to_account_info(),
                authority: auth.to_account_info(),
            },
        ),
        amount,
    )
}

pub fn transfer_from_pool<'info>(
    token_program: &Interface<'info, TokenInterface>,
    pool: &InterfaceAccount<'info, TokenAccount>,
    dst: &InterfaceAccount<'info, TokenAccount>,
    auth: &Account<'info, Pool>,
    amount: u64,
    seeds: &[&[u8]],
) -> Result<()> {
    token::transfer(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            Transfer {
                from: pool.to_account_info(),
                to: dst.to_account_info(),
                authority: auth.to_account_info(),
            },
            &[&seeds[..]],
        ),
        amount,
    )
}

pub fn mint<'info>(
    token_program: &Interface<'info, TokenInterface>,
    mint: &InterfaceAccount<'info, Mint>,
    dst: &InterfaceAccount<'info, TokenAccount>,
    auth: &Account<'info, Pool>,
    amount: u64,
    seeds: &[&[u8]],
) -> Result<()> {
    token::mint_to(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            MintTo {
                mint: mint.to_account_info(),
                to: dst.to_account_info(),
                authority: auth.to_account_info(),
            },
            &[&seeds[..]],
        ),
        amount,
    )
}

pub fn burn<'info>(
    token_program: &Interface<'info, TokenInterface>,
    mint: &InterfaceAccount<'info, Mint>,
    src: &InterfaceAccount<'info, TokenAccount>,
    auth: &Signer<'info>,
    amount: u64,
) -> Result<()> {
    token::burn(
        CpiContext::new(
            token_program.to_account_info(),
            Burn {
                mint: mint.to_account_info(),
                from: src.to_account_info(),
                authority: auth.to_account_info(),
            },
        ),
        amount,
    )
}
