use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

mod counter;

#[derive(BorshDeserialize, BorshSerialize)]
pub enum Cmd {
    Init,
    Inc,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let ix = Cmd::try_from_slice(instruction_data)?;
    match ix {
        Cmd::Init => {
            init(program_id, accounts)?;
        }
        Cmd::Inc => {
            inc(program_id, accounts)?;
        }
    }

    Ok(())
}

pub fn init(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> Result<(), ProgramError> {
    let account_iter = &mut accounts.iter();

    let payer = next_account_info(account_iter)?;
    let counter_account = next_account_info(account_iter)?;
    let counter_program = next_account_info(account_iter)?;
    let system_program = next_account_info(account_iter)?;

    let cmd = counter::Cmd::Init;

    let ix = Instruction::new_with_borsh(
        *counter_program.key,
        &cmd,
        vec![
            AccountMeta {
                pubkey: *payer.key,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: *counter_account.key,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: *system_program.key,
                is_signer: false,
                is_writable: true,
            },
        ],
    );

    invoke(
        &ix,
        &[
            payer.clone(),
            counter_account.clone(),
            system_program.clone(),
            counter_program.clone(),
        ],
    )?;

    Ok(())
}

pub fn inc(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> Result<(), ProgramError> {
    let account_iter = &mut accounts.iter();

    let counter_account = next_account_info(account_iter)?;
    let counter_program = next_account_info(account_iter)?;

    let cmd = counter::Cmd::Init;
    let ix = Instruction::new_with_borsh(
        *counter_program.key,
        &cmd,
        vec![AccountMeta {
            pubkey: *counter_account.key,
            is_signer: true,
            is_writable: true,
        }],
    );

    invoke(&ix, &[counter_account.clone()])?;

    Ok(())
}
