use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        close_account, CloseAccount, Mint, TokenAccount, TokenInterface,
    },
};

use super::lib;
use crate::error;
use crate::state;

#[derive(Accounts)]
pub struct Buy<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    /// CHECK: Receiver of PDA rents
    #[account(mut)]
    pub seller: UncheckedAccount<'info>,

    pub mint_sell: InterfaceAccount<'info, Mint>,
    pub mint_buy: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        seeds = [
            state::Auction::SEED_PREFIX,
            seller.key().as_ref(),
            mint_sell.key().as_ref(),
            mint_buy.key().as_ref()
        ],
        bump,
        close = seller,
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
        associated_token::mint = mint_buy,
        associated_token::authority = buyer,
    )]
    pub buyer_buy_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_sell,
        associated_token::authority = buyer,
    )]
    pub buyer_sell_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_buy,
        associated_token::authority = seller,
    )]
    pub seller_buy_ata: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn buy(ctx: Context<Buy>, max_price: u64) -> Result<()> {
    let clock = Clock::get()?;
    let now = u64::try_from(clock.unix_timestamp).unwrap();

    let auction = &mut ctx.accounts.auction;

    // Check auction has started
    require!(auction.start_time <= now, error::Error::AuctionNotStarted);
    // Check auction has not ended
    require!(now < auction.end_time, error::Error::AuctionEnded);

    // Calculate price
    let price_decrease = (auction.start_price - auction.end_price)
        * (now - auction.start_time)
        / (auction.end_time - auction.start_time);

    let price = auction.start_price - price_decrease;

    // Check current price is greater than or equal to end_price
    require!(
        price >= auction.end_price,
        error::Error::InvalidCurrentPrice
    );

    // Check current price is less than or equal to max_price
    require!(price <= max_price, error::Error::MaxPrice);

    // Calculate amount of buy token to send to seller
    let sell_amt = ctx.accounts.auction_sell_ata.amount;
    let buy_amt = sell_amt * price / (1e6 as u64);

    // Send buy token to seller
    lib::transfer(
        &ctx.accounts.token_program,
        &ctx.accounts.buyer_buy_ata,
        &ctx.accounts.seller_buy_ata,
        &ctx.accounts.buyer,
        buy_amt,
    )?;

    // Send sell token to buyer
    let seeds: &[&[u8]] = &[
        state::Auction::SEED_PREFIX,
        &ctx.accounts.seller.key().to_bytes(),
        &ctx.accounts.mint_sell.key().to_bytes(),
        &ctx.accounts.mint_buy.key().to_bytes(),
        &[ctx.bumps.auction],
    ];

    lib::transfer_from_pda(
        &ctx.accounts.token_program,
        &ctx.accounts.auction_sell_ata,
        &ctx.accounts.buyer_sell_ata,
        &ctx.accounts.auction,
        sell_amt,
        seeds,
    )?;

    // Close auction_sell_ata
    close_account(CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        CloseAccount {
            account: ctx.accounts.auction_sell_ata.to_account_info(),
            destination: ctx.accounts.seller.to_account_info(),
            authority: ctx.accounts.auction.to_account_info(),
        },
        &[seeds],
    ))?;

    Ok(())
}
