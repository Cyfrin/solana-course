use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg,
    pubkey::Pubkey,
};

// Defines where execution of the program begins
entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("Hello Solana!");
    Ok(())
}

#[cfg(test)]
mod test {
    use litesvm::LiteSVM;
    use solana_sdk::{
        instruction::Instruction,
        message::Message,
        signature::{Keypair, Signer},
        transaction::Transaction,
    };

    #[test]
    fn test_hello() {
        let mut svm = LiteSVM::new();

        let payer = Keypair::new();

        svm.airdrop(&payer.pubkey(), 1e9 as u64).unwrap();

        let program_keypair = Keypair::new();
        let program_id = program_keypair.pubkey();
        svm.add_program_from_file(program_id, "target/deploy/hello.so")
            .unwrap();

        let instruction = Instruction {
            program_id,
            accounts: vec![],
            data: vec![],
        };

        let message = Message::new(&[instruction], Some(&payer.pubkey()));
        let tx = Transaction::new(&[&payer], message, svm.latest_blockhash());

        let res = svm.send_transaction(tx);
        assert!(res.is_ok(), "Transaction should succeed");
        let logs = res.unwrap().logs;
        println!("Logs: {logs:#?}");
    }
}
