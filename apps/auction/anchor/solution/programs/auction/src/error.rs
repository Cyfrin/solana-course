use anchor_lang::prelude::*;

#[error_code]
pub enum Error {
    #[msg("Sell mint = buy mint")]
    InvalidMints,
    #[msg("Invalid prices")]
    InvalidPrices,
    #[msg("Invalid start time")]
    InvalidStartTime,
    #[msg("Invalid end time")]
    InvalidEndTime,
    #[msg("Sell amount must be > 0")]
    InvalidSellAmount,
    #[msg("Auction not started")]
    AuctionNotStarted,
    #[msg("Auction ended")]
    AuctionEnded,
    #[msg("Invalid current price")]
    InvalidCurrentPrice,
    #[msg("Max price")]
    MaxPrice,
}
