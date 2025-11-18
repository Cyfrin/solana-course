use anchor_client::{
    solana_sdk::{
        pubkey::Pubkey, signature::Keypair, signer::Signer, system_instruction,
        transaction::Transaction,
    },
    Program,
};
use anchor_spl::associated_token::{self, get_associated_token_address};
use anchor_spl::token::spl_token::solana_program::program_pack::Pack;
use anchor_spl::token::{self, spl_token};

pub fn create_mint(
    program: &Program<&Keypair>,
    payer: &Keypair,
    mint_keypair: &Keypair,
    decimals: u8,
) {
    let rpc = program.rpc();

    let rent = rpc
        .get_minimum_balance_for_rent_exemption(spl_token::state::Mint::LEN)
        .unwrap();

    let mint = mint_keypair.pubkey();

    let create_account_ix = system_instruction::create_account(
        &payer.pubkey(),
        &mint,
        rent,
        spl_token::state::Mint::LEN as u64,
        &token::ID,
    );

    let init_mint_ix = spl_token::instruction::initialize_mint(
        &token::ID,
        &mint,
        &payer.pubkey(),
        None,
        decimals,
    )
    .unwrap();

    let blockhash = rpc.get_latest_blockhash().unwrap();
    let tx = Transaction::new_signed_with_payer(
        &[create_account_ix, init_mint_ix],
        Some(&payer.pubkey()),
        &[payer, mint_keypair],
        blockhash,
    );

    rpc.send_and_confirm_transaction(&tx).unwrap();
}

pub fn create_ata(
    program: &Program<&Keypair>,
    payer: &Keypair,
    mint: &Pubkey,
    owner: &Pubkey,
) -> Result<Pubkey, Box<dyn std::error::Error>> {
    let rpc = program.rpc();

    let ata = get_associated_token_address(owner, mint);

    let ix =
        associated_token::spl_associated_token_account::instruction::create_associated_token_account(
            &payer.pubkey(),
            owner,
            mint,
            &token::ID,
        );

    let blockhash = rpc.get_latest_blockhash()?;
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[payer],
        blockhash,
    );

    rpc.send_and_confirm_transaction(&tx)?;

    Ok(ata)
}

pub fn mint_to(
    program: &Program<&Keypair>,
    // Mint authority
    auth: &Keypair,
    mint: &Pubkey,
    dst: &Pubkey,
    amt: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let rpc = program.rpc();

    let ix = spl_token::instruction::mint_to(
        &token::ID,
        mint,
        dst,
        &auth.pubkey(),
        &[],
        amt,
    )?;

    let blockhash = rpc.get_latest_blockhash()?;
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&auth.pubkey()),
        &[auth],
        blockhash,
    );

    rpc.send_and_confirm_transaction(&tx)?;

    Ok(())
}

pub fn get_balance(
    program: &Program<&Keypair>,
    token_account: &Pubkey,
) -> Result<u64, Box<dyn std::error::Error>> {
    let account_data = program.rpc().get_account_data(token_account)?;
    let token_account_info = spl_token::state::Account::unpack(&account_data)?;
    Ok(token_account_info.amount)
}
