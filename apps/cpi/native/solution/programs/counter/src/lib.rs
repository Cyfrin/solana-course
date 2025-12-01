use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

#[derive(BorshDeserialize, BorshSerialize)]
pub enum Cmd {
    Init,
    Inc,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Counter {
    pub count: u64,
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
            inc(accounts)?;
        }
    }
    Ok(())
}

pub fn init(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> Result<(), ProgramError> {
    let account_iter = &mut accounts.iter();

    // Payer and counter_account must sign
    let payer = next_account_info(account_iter)?;
    let counter_account = next_account_info(account_iter)?;
    let system_program = next_account_info(account_iter)?;

    let space: usize = 8;
    let rent = (Rent::get()?).minimum_balance(space);

    // Create a counter account owned by the counter program
    invoke(
        &system_instruction::create_account(
            payer.key,
            counter_account.key,
            rent,
            space as u64,
            program_id,
        ),
        &[
            payer.clone(),
            counter_account.clone(),
            system_program.clone(),
        ],
    )?;

    // Initialize counter state
    let mut data = counter_account.data.borrow_mut();
    let mut counter = Counter::try_from_slice(&data)?;
    counter.count = 0;
    counter.serialize(&mut &mut data[..])?;

    Ok(())
}

pub fn inc(accounts: &[AccountInfo]) -> Result<(), ProgramError> {
    let account_iter = &mut accounts.iter();
    let counter_account = next_account_info(account_iter)?;

    let mut data = counter_account.data.borrow_mut();
    let mut counter = Counter::try_from_slice(&data)?;
    counter.count += 1;
    counter.serialize(&mut &mut data[..])?;

    Ok(())
}
