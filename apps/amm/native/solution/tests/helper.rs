use litesvm::LiteSVM;
use litesvm_token::{
    CreateAssociatedTokenAccount, CreateMint, MintTo, get_spl_account,
    spl_token::state::Account as TokenAccount,
};
use solana_address::Address;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use spl_associated_token_account_interface::address::get_associated_token_address;

use amm::Cmd;

pub fn create_mint(svm: &mut LiteSVM, payer: &Keypair) -> Pubkey {
    CreateMint::new(svm, payer)
        .authority(&payer.pubkey())
        .decimals(1e6 as u8)
        .send()
        .unwrap()
}

pub fn get_ata(mint: &Pubkey, owner: &Pubkey) -> Pubkey {
    let ata_addr = get_associated_token_address(
        &Address::from(owner.to_bytes()),
        &Address::from(mint.to_bytes()),
    );
    Pubkey::from(ata_addr.to_bytes())
}

pub fn create_ata(
    svm: &mut LiteSVM,
    payer: &Keypair,
    owner: &Pubkey,
    mint: &Pubkey,
) -> Pubkey {
    CreateAssociatedTokenAccount::new(svm, payer, mint)
        .owner(owner)
        .send()
        .unwrap()
}

pub fn mint_to(
    svm: &mut LiteSVM,
    payer: &Keypair,
    mint: &Pubkey,
    dst: &Pubkey,
    amt: u64,
) {
    MintTo::new(svm, payer, mint, dst, amt)
        .owner(payer)
        .send()
        .unwrap();
}

pub fn get_token_balance(svm: &LiteSVM, account: &Pubkey) -> u64 {
    let token_account: TokenAccount = get_spl_account(svm, account).unwrap();
    token_account.amount
}

pub fn create_init_pool_ix(
    program_id: Pubkey,
    payer: Pubkey,
    fee: u16,
    mint_a: Pubkey,
    mint_b: Pubkey,
    pool: Pubkey,
    pool_bump: u8,
    mint_pool: Pubkey,
    mint_pool_bump: u8,
    pool_a: Pubkey,
    pool_b: Pubkey,
) -> Instruction {
    let cmd = Cmd::InitPool {
        fee,
        pool_bump,
        mint_pool_bump,
    };

    Instruction::new_with_borsh(
        program_id,
        &cmd,
        vec![
            AccountMeta {
                pubkey: payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: mint_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: mint_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pool_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pool_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: mint_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: Pubkey::from(spl_token_interface::ID.to_bytes()),
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: Pubkey::from(
                    spl_associated_token_account_interface::program::ID
                        .to_bytes(),
                ),
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: solana_sdk::system_program::id(),
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: solana_sdk::sysvar::rent::id(),
                is_signer: false,
                is_writable: true,
            },
        ],
    )
}

pub fn create_add_liquidity_ix(
    program_id: Pubkey,
    payer: Pubkey,
    fee: u16,
    mint_a: Pubkey,
    mint_b: Pubkey,
    pool: Pubkey,
    pool_bump: u8,
    mint_pool: Pubkey,
    mint_pool_bump: u8,
    pool_a: Pubkey,
    pool_b: Pubkey,
    amount_a: u64,
    amount_b: u64,
    payer_a: Pubkey,
    payer_b: Pubkey,
    payer_liq: Pubkey,
) -> Instruction {
    let cmd = Cmd::AddLiquidity {
        fee,
        pool_bump,
        mint_pool_bump,
        amount_a,
        amount_b,
    };

    Instruction::new_with_borsh(
        program_id,
        &cmd,
        vec![
            AccountMeta {
                pubkey: payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: mint_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: mint_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pool_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pool_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: mint_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: payer_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: payer_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: payer_liq,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: Pubkey::from(spl_token_interface::ID.to_bytes()),
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: Pubkey::from(
                    spl_associated_token_account_interface::program::ID
                        .to_bytes(),
                ),
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: solana_sdk::system_program::id(),
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: solana_sdk::sysvar::rent::id(),
                is_signer: false,
                is_writable: true,
            },
        ],
    )
}

pub fn create_remove_liquidity_ix(
    program_id: Pubkey,
    payer: Pubkey,
    fee: u16,
    mint_a: Pubkey,
    mint_b: Pubkey,
    pool: Pubkey,
    pool_bump: u8,
    mint_pool: Pubkey,
    mint_pool_bump: u8,
    pool_a: Pubkey,
    pool_b: Pubkey,
    shares: u64,
    min_amount_a: u64,
    min_amount_b: u64,
    payer_a: Pubkey,
    payer_b: Pubkey,
    payer_liq: Pubkey,
) -> Instruction {
    let cmd = Cmd::RemoveLiquidity {
        fee,
        pool_bump,
        mint_pool_bump,
        shares,
        min_amount_a,
        min_amount_b,
    };

    Instruction::new_with_borsh(
        program_id,
        &cmd,
        vec![
            AccountMeta {
                pubkey: payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: mint_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: mint_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pool_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pool_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: mint_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: payer_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: payer_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: payer_liq,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: Pubkey::from(spl_token_interface::ID.to_bytes()),
                is_signer: false,
                is_writable: true,
            },
        ],
    )
}

pub fn create_swap_ix(
    program_id: Pubkey,
    payer: Pubkey,
    fee: u16,
    mint_a: Pubkey,
    mint_b: Pubkey,
    pool: Pubkey,
    pool_bump: u8,
    pool_a: Pubkey,
    pool_b: Pubkey,
    payer_a: Pubkey,
    payer_b: Pubkey,
    a_for_b: bool,
    amount_in: u64,
    min_amount_out: u64,
) -> Instruction {
    let cmd = Cmd::Swap {
        fee,
        pool_bump,
        a_for_b,
        amount_in,
        min_amount_out,
    };

    Instruction::new_with_borsh(
        program_id,
        &cmd,
        vec![
            AccountMeta {
                pubkey: payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: mint_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: mint_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pool_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pool_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: payer_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: payer_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: Pubkey::from(spl_token_interface::ID.to_bytes()),
                is_signer: false,
                is_writable: true,
            },
        ],
    )
}

#[derive(Debug)]
pub struct Test {
    pub program_id: Pubkey,
    pub payer: Keypair,
    pub users: Vec<Keypair>,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub atas_a: Vec<Pubkey>,
    pub atas_b: Vec<Pubkey>,
    pub fee: u16,
    pub pool_pda: Pubkey,
    pub pool_bump: u8,
    pub mint_pool_pda: Pubkey,
    pub mint_pool_bump: u8,
    pub pool_a: Pubkey,
    pub pool_b: Pubkey,
    pub atas_liq: Vec<Pubkey>,
}

pub fn setup(svm: &mut LiteSVM) -> Test {
    let payer = Keypair::new();

    let program_keypair = Keypair::new();
    let program_id = program_keypair.pubkey();
    svm.add_program_from_file(program_id, "target/deploy/amm.so")
        .unwrap();

    let mut users = Vec::new();
    users.push(Keypair::new());
    users.push(Keypair::new());

    // Airdrop
    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();
    for user in users.iter() {
        svm.airdrop(&user.pubkey(), 1_000_000_000).unwrap();
    }

    // Mints
    let mint_a = create_mint(svm, &payer);
    let mint_b = create_mint(svm, &payer);

    // Pool PDA
    let fee: u16 = 500;
    let (pool_pda, pool_bump) = Pubkey::find_program_address(
        &[
            amm::constants::POOL_AUTH,
            mint_a.as_ref(),
            mint_b.as_ref(),
            fee.to_le_bytes().as_ref(),
        ],
        &program_id,
    );

    let (mint_pool_pda, mint_pool_bump) = Pubkey::find_program_address(
        &[
            amm::constants::POOL_MINT,
            mint_a.as_ref(),
            mint_b.as_ref(),
            fee.to_le_bytes().as_ref(),
        ],
        &program_id,
    );

    // ATA
    let mut atas_a = Vec::new();
    let mut atas_b = Vec::new();
    for user in users.iter() {
        let ata_a = create_ata(svm, &payer, &user.pubkey(), &mint_a);
        atas_a.push(ata_a);

        let ata_b = create_ata(svm, &payer, &user.pubkey(), &mint_b);
        atas_b.push(ata_b);

        mint_to(svm, &payer, &mint_a, &ata_a, 1e9 as u64);
        mint_to(svm, &payer, &mint_b, &ata_b, 1e9 as u64);
    }

    let pool_a = get_ata(&mint_a, &pool_pda);
    let pool_b = get_ata(&mint_b, &pool_pda);

    let mut atas_liq = Vec::new();
    for user in users.iter() {
        atas_liq.push(get_ata(&mint_pool_pda, &user.pubkey()));
    }

    Test {
        program_id,
        payer,
        users,
        mint_a,
        mint_b,
        atas_a,
        atas_b,
        fee,
        pool_pda,
        pool_bump,
        mint_pool_pda,
        mint_pool_bump,
        pool_a,
        pool_b,
        atas_liq,
    }
}
