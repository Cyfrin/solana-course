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
    if !seller.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    // Check that auction_pda matches expected PDA
    if *auction_pda.key
        != get_pda(program_id, seller.key, mint_sell.key, mint_buy.key, bump)?
    {
        return Err(ProgramError::InvalidSeeds);
    }
    // Check that auction_sell_ata matches calculated account
    if *auction_sell_ata.key != get_ata(auction_pda.key, mint_sell.key) {
        return Err(ProgramError::InvalidArgument);
    }
    // Check that buyer_sell_ata matches calculated account
    if *seller_sell_ata.key != get_ata(seller.key, mint_sell.key) {
        return Err(ProgramError::InvalidArgument);
    }

    // Get sell amount locked in auction_sell_ata
    let sell_amt = get_token_balance(auction_sell_ata)?;

    // Send sell token to seller
    let seeds = &[
        Auction::SEED_PREFIX,
        seller.key.as_ref(),
        mint_sell.key.as_ref(),
        mint_buy.key.as_ref(),
        &[bump],
    ];

    transfer_from_pda(
        token_program,
        auction_sell_ata,
        seller_sell_ata,
        auction_pda,
        sell_amt,
        seeds,
    )?;

    // Close auction_sell_ata
    close_ata(token_program, auction_sell_ata, seller, auction_pda, seeds)?;

    // Close auction_pda
    // Get PDA balance and transfer lamports directly
    let pda_lamports = auction_pda.lamports();

    **auction_pda.try_borrow_mut_lamports()? = 0;
    **seller.try_borrow_mut_lamports()? = seller
        .lamports()
        .checked_add(pda_lamports)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    // Clear out data
    auction_pda.resize(0)?;

    // Assign the account to the System Program
    auction_pda.assign(sys_program.key);

    Ok(())
}
