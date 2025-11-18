use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use super::lib;
use crate::error;
use crate::state;

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    pub mint_sell: InterfaceAccount<'info, Mint>,
    pub mint_buy: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = payer,
        space = 8 + state::Auction::INIT_SPACE,
        seeds = [
            state::Auction::SEED_PREFIX,
            payer.key().as_ref(),
            mint_sell.key().as_ref(),
            mint_buy.key().as_ref()
        ],
        bump,
    )]
    pub auction: Account<'info, state::Auction>,

    // Associated token account to lock seller's sell token
    #[account(
        init,
        payer = payer,
        associated_token::mint = mint_sell,
        associated_token::authority = auction,
    )]
    pub auction_sell_ata: InterfaceAccount<'info, TokenAccount>,

    // Associated token account where the seller holds the sell token
    #[account(
        mut,
        associated_token::mint = mint_sell,
        associated_token::authority = payer,
    )]
    pub seller_sell_ata: InterfaceAccount<'info, TokenAccount>,

    // Associated token account where seller receives buy token
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint_buy,
        associated_token::authority = payer,
    )]
    pub seller_buy_ata: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn init(
    ctx: Context<Init>,
    start_price: u64,
    end_price: u64,
    start_time: u64,
    end_time: u64,
    sell_amt: u64,
) -> Result<()> {
    let clock = Clock::get()?;
    let now = u64::try_from(clock.unix_timestamp).unwrap();

    // Check sell token != buy token
    require!(
        ctx.accounts.mint_sell.key() != ctx.accounts.mint_buy.key(),
        error::Error::InvalidMints
    );

    // Check start_price >= end_price
    require!(start_price >= end_price, error::Error::InvalidPrices);

    // Check now <= start_time < end_time
    require!(now <= start_time, error::Error::InvalidStartTime);
    require!(start_time < end_time, error::Error::InvalidEndTime);

    // Check sell_amt > 0
    require!(sell_amt > 0, error::Error::InvalidSellAmount);

    // Send sell token to auction_sell_ata
    lib::transfer(
        &ctx.accounts.token_program,
        &ctx.accounts.seller_sell_ata,
        &ctx.accounts.auction_sell_ata,
        &ctx.accounts.payer,
        sell_amt,
    )?;

    // Store Auction state
    let auction = &mut ctx.accounts.auction;
    auction.mint_sell = ctx.accounts.mint_sell.key();
    auction.mint_buy = ctx.accounts.mint_buy.key();
    auction.start_price = start_price;
    auction.end_price = end_price;
    auction.start_time = start_time;
    auction.end_time = end_time;

    Ok(())
}
