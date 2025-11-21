use solana_address::Address;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::{Pubkey, PubkeyError};
use solana_program::{account_info::AccountInfo, program_error::ProgramError};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program::{invoke, invoke_signed},
};
use solana_program_pack::Pack;
use spl_associated_token_account_interface as spl_ata;
use spl_token_interface;

use crate::state::Auction;

pub fn get_pda(
    program_id: &Pubkey,
    seller: &Pubkey,
    mint_sell: &Pubkey,
    mint_buy: &Pubkey,
    bump: u8,
) -> Result<Pubkey, PubkeyError> {
    Pubkey::create_program_address(
        &[
            Auction::SEED_PREFIX,
            seller.as_ref(),
            mint_sell.as_ref(),
            mint_buy.as_ref(),
            &[bump],
        ],
        program_id,
    )
}

pub fn create_ata<'a>(
    payer: &AccountInfo<'a>,
    mint: &AccountInfo<'a>,
    owner: &AccountInfo<'a>,
    ata: &AccountInfo<'a>,
    token_program: &AccountInfo<'a>,
    sys_program: &AccountInfo<'a>,
    ata_program: &AccountInfo<'a>,
    rent_sysvar: &AccountInfo<'a>,
) -> Result<(), ProgramError> {
    // Added return type
    let spl_ix = spl_ata::instruction::create_associated_token_account(
        &Address::from(payer.key.to_bytes()),
        &Address::from(owner.key.to_bytes()),
        &Address::from(mint.key.to_bytes()),
        &Address::from(token_program.key.to_bytes()),
    );

    let ix = Instruction {
        program_id: Pubkey::from(spl_ix.program_id.to_bytes()),
        accounts: spl_ix
            .accounts
            .iter()
            .map(|acc| AccountMeta {
                pubkey: Pubkey::from(acc.pubkey.to_bytes()),
                is_signer: acc.is_signer,
                is_writable: acc.is_writable,
            })
            .collect(),
        data: spl_ix.data,
    };

    invoke(
        &ix,
        &[
            payer.clone(),         // Funding account
            ata.clone(),           // ATA to create
            owner.clone(),         // Wallet owner
            mint.clone(),          // Mint
            sys_program.clone(),   // System program
            token_program.clone(), // Token program
            ata_program.clone(),
            rent_sysvar.clone(),
        ],
    )?;

    Ok(())
}

pub fn close_ata<'a>(
    token_program: &AccountInfo<'a>,
    ata: &AccountInfo<'a>,
    dst: &AccountInfo<'a>,
    // ATA owner
    owner: &AccountInfo<'a>,
    signer_seeds: &[&[u8]],
) -> ProgramResult {
    let spl_ix = spl_token_interface::instruction::close_account(
        &Address::from(token_program.key.to_bytes()),
        &Address::from(ata.key.to_bytes()),
        &Address::from(dst.key.to_bytes()),
        &Address::from(owner.key.to_bytes()),
        // Signer pubkeys
        &[],
    )
    .map_err(|_| ProgramError::InvalidInstructionData)?;

    let ix = Instruction {
        program_id: Pubkey::from(spl_ix.program_id.to_bytes()),
        accounts: spl_ix
            .accounts
            .iter()
            .map(|acc| AccountMeta {
                pubkey: Pubkey::from(acc.pubkey.to_bytes()),
                is_signer: acc.is_signer,
                is_writable: acc.is_writable,
            })
            .collect(),
        data: spl_ix.data,
    };

    invoke_signed(
        &ix,
        &[
            ata.clone(),
            dst.clone(),
            owner.clone(),
            token_program.clone(),
        ],
        &[signer_seeds],
    )
}

pub fn get_ata(wallet: &Pubkey, mint: &Pubkey) -> Pubkey {
    let addr = spl_ata::address::get_associated_token_address(
        &Address::from(wallet.to_bytes()),
        &Address::from(mint.to_bytes()),
    );
    Pubkey::from(addr.to_bytes())
}

pub fn transfer<'a>(
    token_program: &AccountInfo<'a>,
    src: &AccountInfo<'a>,
    dst: &AccountInfo<'a>,
    // Transfer authority
    auth: &AccountInfo<'a>,
    amount: u64,
) -> ProgramResult {
    let spl_ix = spl_token_interface::instruction::transfer(
        &Address::from(token_program.key.to_bytes()),
        &Address::from(src.key.to_bytes()),
        &Address::from(dst.key.to_bytes()),
        &Address::from(auth.key.to_bytes()),
        // Signer pubkeys
        &[],
        amount,
    )
    .map_err(|_| ProgramError::InvalidInstructionData)?;

    let ix = Instruction {
        program_id: Pubkey::from(spl_ix.program_id.to_bytes()),
        accounts: spl_ix
            .accounts
            .iter()
            .map(|acc| AccountMeta {
                pubkey: Pubkey::from(acc.pubkey.to_bytes()),
                is_signer: acc.is_signer,
                is_writable: acc.is_writable,
            })
            .collect(),
        data: spl_ix.data,
    };

    invoke(
        &ix,
        &[
            src.clone(),
            dst.clone(),
            auth.clone(),
            token_program.clone(),
        ],
    )
}

pub fn transfer_from_pda<'a>(
    token_program: &AccountInfo<'a>,
    src: &AccountInfo<'a>,
    dst: &AccountInfo<'a>,
    // Transfer authority
    auth: &AccountInfo<'a>,
    amount: u64,
    signer_seeds: &[&[u8]],
) -> ProgramResult {
    let spl_ix = spl_token_interface::instruction::transfer(
        &Address::from(token_program.key.to_bytes()),
        &Address::from(src.key.to_bytes()),
        &Address::from(dst.key.to_bytes()),
        &Address::from(auth.key.to_bytes()),
        &[],
        amount,
    )
    .map_err(|_| ProgramError::InvalidInstructionData)?;

    let ix = Instruction {
        program_id: Pubkey::from(spl_ix.program_id.to_bytes()),
        accounts: spl_ix
            .accounts
            .iter()
            .map(|acc| AccountMeta {
                pubkey: Pubkey::from(acc.pubkey.to_bytes()),
                is_signer: acc.is_signer,
                is_writable: acc.is_writable,
            })
            .collect(),
        data: spl_ix.data,
    };

    invoke_signed(
        &ix,
        &[
            src.clone(),
            dst.clone(),
            auth.clone(),
            token_program.clone(),
        ],
        &[signer_seeds],
    )
}

pub fn get_token_balance<'a>(
    token_account: &AccountInfo<'a>,
) -> Result<u64, ProgramError> {
    let token_account_data = spl_token_interface::state::Account::unpack(
        &token_account.data.borrow(),
    )
    .map_err(|_| ProgramError::InvalidAccountData)?;

    Ok(token_account_data.amount)
}
