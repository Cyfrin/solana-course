use anchor_lang::prelude::*;

#[error_code]
pub enum Error {
    #[msg("Lock amount must be > 0")]
    InvalidAmount,
    #[msg("Lock expiration must be in the future")]
    InvalidExpiration,
    #[msg("Lock has not expired yet")]
    LockNotExpired,
    #[msg("Destination mismatch")]
    DestinationMismatch,
}
