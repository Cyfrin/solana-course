use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::Token,
    token_interface::{
        close_account, CloseAccount, Mint, TokenAccount, TokenInterface,
    },
};

use super::lib;
use crate::error;
use crate::state;

#[derive(Accounts)]
pub struct Cancel<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    pub mint_sell: InterfaceAccount<'info, Mint>,
    pub mint_buy: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        seeds = [
            state::Auction::SEED_PREFIX,
            payer.key().as_ref(),
            mint_sell.key().as_ref(),
            mint_buy.key().as_ref()
        ],
        bump,
        close = payer,
    )]
    pub auction: Account<'info, state::Auction>,

    #[account(
        mut,
        associated_token::mint = mint_sell,
        associated_token::authority = auction,
    )]
    pub auction_sell_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_sell,
        associated_token::authority = payer,
    )]
    pub seller_sell_ata: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn cancel(ctx: Context<Cancel>) -> Result<()> {
    // Send sell token to seller
    let seeds: &[&[u8]] = &[
        state::Auction::SEED_PREFIX,
        &ctx.accounts.payer.key().to_bytes(),
        &ctx.accounts.mint_sell.key().to_bytes(),
        &ctx.accounts.mint_buy.key().to_bytes(),
        &[ctx.bumps.auction],
    ];

    lib::transfer_from_pda(
        &ctx.accounts.token_program,
        &ctx.accounts.auction_sell_ata,
        &ctx.accounts.seller_sell_ata,
        &ctx.accounts.auction,
        ctx.accounts.auction_sell_ata.amount,
        seeds,
    )?;

    // Close auction_sell_ata
    close_account(CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        CloseAccount {
            account: ctx.accounts.auction_sell_ata.to_account_info(),
            destination: ctx.accounts.payer.to_account_info(),
            authority: ctx.accounts.auction.to_account_info(),
        },
        &[seeds],
    ))?;

    // Close auction_sell_ata
    Ok(())
}
