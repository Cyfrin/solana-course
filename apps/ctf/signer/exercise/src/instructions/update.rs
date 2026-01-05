use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    program_error::ProgramError,
};

use crate::state::Oracle;

pub fn update(
    accounts: &[AccountInfo],
    price: u64,
) -> Result<(), ProgramError> {
    let account_iter = &mut accounts.iter();
    let oracle_account = next_account_info(account_iter)?;
    let signer = next_account_info(account_iter)?;

    let mut data = oracle_account.data.borrow_mut();
    let mut oracle = Oracle::try_from_slice(&data)?;

    if oracle.owner != *signer.key {
        return Err(ProgramError::IllegalOwner);
    }

    oracle.price = price;
    oracle.serialize(&mut &mut data[..])?;

    Ok(())
}
