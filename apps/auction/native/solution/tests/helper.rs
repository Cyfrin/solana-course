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

use auction::{Cmd, state::Auction};

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

pub fn create_init_ix(
    program_id: Pubkey,
    start_price: u64,
    end_price: u64,
    start_time: u64,
    end_time: u64,
    sell_amt: u64,
    bump: u8,
    seller: Pubkey,
    mint_sell: Pubkey,
    mint_buy: Pubkey,
    auction_pda: Pubkey,
    auction_sell_ata: Pubkey,
    seller_sell_ata: Pubkey,
) -> Instruction {
    let cmd = Cmd::Init {
        start_price,
        end_price,
        start_time,
        end_time,
        sell_amt,
        bump,
    };

    Instruction::new_with_borsh(
        program_id,
        &cmd,
        vec![
            AccountMeta {
                pubkey: seller,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: mint_sell,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: mint_buy,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: auction_pda,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: auction_sell_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: seller_sell_ata,
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

pub fn create_buy_ix(
    program_id: Pubkey,
    max_price: u64,
    bump: u8,
    buyer: Pubkey,
    seller: Pubkey,
    mint_sell: Pubkey,
    mint_buy: Pubkey,
    auction_pda: Pubkey,
    auction_sell_ata: Pubkey,
    buyer_sell_ata: Pubkey,
    buyer_buy_ata: Pubkey,
    seller_buy_ata: Pubkey,
) -> Instruction {
    let cmd = Cmd::Buy { max_price, bump };

    Instruction::new_with_borsh(
        program_id,
        &cmd,
        vec![
            AccountMeta {
                pubkey: buyer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: seller,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: mint_sell,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: mint_buy,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: auction_pda,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: auction_sell_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: buyer_sell_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: buyer_buy_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: seller_buy_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: Pubkey::from(spl_token_interface::ID.to_bytes()),
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: solana_sdk::system_program::id(),
                is_signer: false,
                is_writable: true,
            },
        ],
    )
}

pub fn create_cancel_ix(
    program_id: Pubkey,
    bump: u8,
    seller: Pubkey,
    mint_sell: Pubkey,
    mint_buy: Pubkey,
    auction_pda: Pubkey,
    auction_sell_ata: Pubkey,
    seller_sell_ata: Pubkey,
) -> Instruction {
    let cmd = Cmd::Cancel { bump };

    Instruction::new_with_borsh(
        program_id,
        &cmd,
        vec![
            AccountMeta {
                pubkey: seller,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: mint_sell,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: mint_buy,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: auction_pda,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: auction_sell_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: seller_sell_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: Pubkey::from(spl_token_interface::ID.to_bytes()),
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: solana_sdk::system_program::id(),
                is_signer: false,
                is_writable: true,
            },
        ],
    )
}

pub struct Test {
    pub program_id: Pubkey,
    pub payer: Keypair,
    pub seller: Keypair,
    pub buyer: Keypair,
    pub mint_sell: Pubkey,
    pub mint_buy: Pubkey,
    pub seller_sell_ata: Pubkey,
    pub seller_buy_ata: Pubkey,
    pub buyer_sell_ata: Pubkey,
    pub buyer_buy_ata: Pubkey,
    pub auction_pda: Pubkey,
    pub auction_bump: u8,
    pub auction_sell_ata: Pubkey,
}

pub fn setup(svm: &mut LiteSVM) -> Test {
    let payer = Keypair::new();
    let seller = Keypair::new();
    let buyer = Keypair::new();
    let program_keypair = Keypair::new();
    let program_id = program_keypair.pubkey();
    svm.add_program_from_file(program_id, "target/deploy/auction.so")
        .unwrap();

    // Airdrop
    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();
    svm.airdrop(&seller.pubkey(), 1_000_000_000).unwrap();
    svm.airdrop(&buyer.pubkey(), 1_000_000_000).unwrap();

    // Mints
    let mint_sell = create_mint(svm, &payer);
    let mint_buy = create_mint(svm, &payer);

    // Auction PDA
    let (auction_pda, auction_bump) = Pubkey::find_program_address(
        &[
            Auction::SEED_PREFIX,
            seller.pubkey().as_ref(),
            mint_sell.as_ref(),
            mint_buy.as_ref(),
        ],
        &program_id,
    );

    // ATA
    let seller_sell_ata = create_ata(svm, &payer, &seller.pubkey(), &mint_sell);
    let seller_buy_ata = create_ata(svm, &payer, &seller.pubkey(), &mint_buy);
    let buyer_sell_ata = create_ata(svm, &payer, &buyer.pubkey(), &mint_sell);
    let buyer_buy_ata = create_ata(svm, &payer, &buyer.pubkey(), &mint_buy);
    let auction_sell_ata = get_ata(&mint_sell, &auction_pda);

    // Mint to
    mint_to(svm, &payer, &mint_sell, &seller_sell_ata, 1e9 as u64);
    mint_to(svm, &payer, &mint_buy, &buyer_buy_ata, 1e9 as u64);

    Test {
        program_id,
        payer,
        seller,
        buyer,
        mint_sell,
        mint_buy,
        seller_sell_ata,
        seller_buy_ata,
        buyer_sell_ata,
        buyer_buy_ata,
        auction_pda,
        auction_bump,
        auction_sell_ata,
    }
}
