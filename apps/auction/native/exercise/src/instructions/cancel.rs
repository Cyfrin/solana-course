use solana_program::{
    account_info::{AccountInfo, next_account_info},
    program_error::ProgramError,
    pubkey::Pubkey,
};

use super::lib::{
    close_ata, get_ata, get_pda, get_token_balance, transfer_from_pda,
};
use crate::state::Auction;

pub fn cancel(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    // Auction PDA bump
    bump: u8,
) -> Result<(), ProgramError> {
    let account_iter = &mut accounts.iter();

    let seller = next_account_info(account_iter)?;
    let mint_sell = next_account_info(account_iter)?;
    let mint_buy = next_account_info(account_iter)?;
    let auction_pda = next_account_info(account_iter)?;
    let auction_sell_ata = next_account_info(account_iter)?;
    let seller_sell_ata = next_account_info(account_iter)?;
    let token_program = next_account_info(account_iter)?;
    let sys_program = next_account_info(account_iter)?;

    // Check seller signed
    // Check that auction_pda matches expected PDA
    // Check that auction_sell_ata matches calculated account
    // Check that buyer_sell_ata matches calculated account

    // Get sell amount locked in auction_sell_ata

    // Send sell token to seller
    let seeds = &[
        Auction::SEED_PREFIX,
        seller.key.as_ref(),
        mint_sell.key.as_ref(),
        mint_buy.key.as_ref(),
        &[bump],
    ];

    // Close auction_sell_ata

    // Close auction_pda

    Ok(())
}
