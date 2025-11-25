use solana_address::Address;
use solana_client::rpc_client::RpcClient;
use solana_program_pack::Pack;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer, read_keypair_file},
    system_instruction,
    transaction::Transaction,
};
use std::path::PathBuf;
use std::str::FromStr;
use std::thread;
use std::time::Duration;

use auction::Cmd;
use auction::state::Auction;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let keypair_path: PathBuf = [&args[1]].iter().collect();
    let payer =
        read_keypair_file(keypair_path).expect("Cannot read keypair file");

    // Connect to local cluster
    let rpc_url = String::from(&args[2]);
    let client =
        RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    let program_id = Pubkey::from_str(&args[3]).expect("Invalid program ID");

    let seller = Keypair::new();
    let buyer = Keypair::new();

    airdrop(&client, &payer.pubkey(), 1e9 as u64);
    airdrop(&client, &seller.pubkey(), 1e9 as u64);
    airdrop(&client, &buyer.pubkey(), 1e9 as u64);

    let mint_sell = create_mint(&client, &payer, &payer.pubkey(), 6);
    let mint_buy = create_mint(&client, &payer, &payer.pubkey(), 6);

    // Create token accounts for users
    let seller_sell_ata =
        create_ata(&client, &payer, &seller.pubkey(), &mint_sell);
    let seller_buy_ata =
        create_ata(&client, &payer, &seller.pubkey(), &mint_buy);
    let buyer_sell_ata =
        create_ata(&client, &payer, &buyer.pubkey(), &mint_sell);
    let buyer_buy_ata = create_ata(&client, &payer, &buyer.pubkey(), &mint_buy);

    // Mint tokens to users
    mint_to(&client, &payer, &mint_sell, &seller_sell_ata, 1e9 as u64);
    mint_to(&client, &payer, &mint_buy, &buyer_buy_ata, 1e9 as u64);

    // Derive PDAs
    let (auction_pda, bump) = Pubkey::find_program_address(
        &[
            Auction::SEED_PREFIX,
            seller.pubkey().as_ref(),
            mint_sell.as_ref(),
            mint_buy.as_ref(),
        ],
        &program_id,
    );

    let auction_sell_ata = get_ata(&auction_pda, &mint_sell);

    // Init
    println!("Init");

    let now = client.get_block_time(client.get_slot().unwrap()).unwrap() as u64;
    let start_price: u64 = (2.0 * 1e6) as u64;
    let end_price: u64 = (1.5 * 1e6) as u64;
    let start_time: u64 = now + 1;
    let end_time: u64 = start_time + 10;
    let sell_amt: u64 = 1e8 as u64;

    let ix = create_init_ix(
        program_id,
        start_price,
        end_price,
        start_time,
        end_time,
        sell_amt,
        bump,
        seller.pubkey(),
        mint_sell,
        mint_buy,
        auction_pda,
        auction_sell_ata,
        seller_sell_ata,
    );

    let mut tx = Transaction::new_with_payer(&[ix], Some(&seller.pubkey()));
    let blockhash = client.get_latest_blockhash().unwrap();
    tx.sign(&[&seller], blockhash);

    let res = client.send_and_confirm_transaction(&tx);
    res.unwrap();

    // TODO: check state
    /*
    let data = client
        .get_account_data(&pda)
        .expect("Failed to fetch account data");

    let lock = Lock::try_from_slice(&data).expect("Failed to deserialize");

    println!("lock.dst: {:?}", lock.dst);
    println!("lock.exp: {:?}", lock.exp);
    println!("locked amt: {:?}", client.get_balance(&pda));
    */
    let auction_sell_bal = get_token_balance(&client, &auction_sell_ata);
    println!("Auction sell ATA balance: {:?}", auction_sell_bal);

    assert_eq!(auction_sell_bal, sell_amt, "Auction sell ATA balance");

    // Buy
    println!("Buy");
    thread::sleep(Duration::from_millis(1500));

    let cmd = Cmd::Buy {
        max_price: start_price - 1,
        bump,
    };

    let ix = Instruction::new_with_borsh(
        program_id,
        &cmd,
        vec![
            AccountMeta {
                pubkey: buyer.pubkey(),
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: seller.pubkey(),
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
    );

    let mut tx = Transaction::new_with_payer(&[ix], Some(&buyer.pubkey()));
    let blockhash = client.get_latest_blockhash().unwrap();
    tx.sign(&[&buyer], blockhash);

    let res = client.send_and_confirm_transaction(&tx);
    res.unwrap();

    let buyer_buy_ata_bal = get_token_balance(&client, &buyer_buy_ata);
    let buyer_sell_ata_bal = get_token_balance(&client, &buyer_sell_ata);
    println!("Buyer buy ATA balance: {:?}", buyer_buy_ata_bal);
    println!("Buyer sell ATA balance: {:?}", buyer_sell_ata_bal);

    let seller_buy_ata_bal = get_token_balance(&client, &seller_buy_ata);
    let seller_sell_ata_bal = get_token_balance(&client, &seller_sell_ata);
    println!("Seller buy ATA balance: {:?}", seller_buy_ata_bal);
    println!("Seller sell ATA balance: {:?}", seller_sell_ata_bal);

    assert_eq!(buyer_sell_ata_bal, sell_amt, "Buyer sell ATA balance");
    assert!(seller_buy_ata_bal > 0, "Seller buy ATA balance");

    // Create auction
    println!("Init");
    let now = client.get_block_time(client.get_slot().unwrap()).unwrap() as u64;
    let start_price: u64 = (2.0 * 1e6) as u64;
    let end_price: u64 = (1.5 * 1e6) as u64;
    let start_time: u64 = now + 1;
    let end_time: u64 = start_time + 10;
    let sell_amt: u64 = 1e8 as u64;

    let ix = create_init_ix(
        program_id,
        start_price,
        end_price,
        start_time,
        end_time,
        sell_amt,
        bump,
        seller.pubkey(),
        mint_sell,
        mint_buy,
        auction_pda,
        auction_sell_ata,
        seller_sell_ata,
    );

    let mut tx = Transaction::new_with_payer(&[ix], Some(&seller.pubkey()));
    let blockhash = client.get_latest_blockhash().unwrap();
    tx.sign(&[&seller], blockhash);

    let res = client.send_and_confirm_transaction(&tx);
    res.unwrap();

    // Cancel
    println!("Cancel");
    let cmd = Cmd::Cancel { bump };

    let ix = Instruction::new_with_borsh(
        program_id,
        &cmd,
        vec![
            AccountMeta {
                pubkey: seller.pubkey(),
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
    );

    let seller_sell_ata_bal_before =
        get_token_balance(&client, &seller_sell_ata);

    let mut tx = Transaction::new_with_payer(&[ix], Some(&seller.pubkey()));
    let blockhash = client.get_latest_blockhash().unwrap();
    tx.sign(&[&seller], blockhash);

    let res = client.send_and_confirm_transaction(&tx);
    res.unwrap();

    let seller_sell_ata_bal = get_token_balance(&client, &seller_sell_ata);
    println!("Seller sell ATA balance: {:?}", seller_sell_ata_bal);
    assert_eq!(
        seller_sell_ata_bal,
        seller_sell_ata_bal_before + sell_amt,
        "Seller sell ATA balance"
    );
}

fn create_init_ix(
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

fn airdrop(client: &RpcClient, pubkey: &Pubkey, lamports: u64) {
    let bal = client.get_balance(pubkey).unwrap();
    if bal >= lamports {
        return;
    }

    let sig = client.request_airdrop(pubkey, lamports).unwrap();

    // Wait for airdrop confirmation
    while !client.confirm_transaction(&sig).unwrap_or(false) {
        thread::sleep(Duration::from_millis(500));
    }
}

fn create_mint(
    client: &RpcClient,
    payer: &Keypair,
    authority: &Pubkey,
    decimals: u8,
) -> Pubkey {
    let mint = Keypair::new();
    let rent = client
        .get_minimum_balance_for_rent_exemption(
            spl_token_interface::state::Mint::LEN,
        )
        .unwrap();

    let create_account_ix = system_instruction::create_account(
        &payer.pubkey(),
        &mint.pubkey(),
        rent,
        spl_token_interface::state::Mint::LEN as u64,
        &Pubkey::from(spl_token_interface::ID.to_bytes()),
    );

    let init_ix = spl_token_interface::instruction::initialize_mint(
        &Address::from(spl_token_interface::ID.to_bytes()),
        &Address::from(mint.pubkey().to_bytes()),
        &Address::from(authority.to_bytes()),
        None,
        decimals,
    )
    .unwrap();

    let init_ix = Instruction {
        program_id: Pubkey::from(init_ix.program_id.to_bytes()),
        accounts: init_ix
            .accounts
            .iter()
            .map(|acc| AccountMeta {
                pubkey: Pubkey::from(acc.pubkey.to_bytes()),
                is_signer: acc.is_signer,
                is_writable: acc.is_writable,
            })
            .collect(),
        data: init_ix.data,
    };

    let mut tx = Transaction::new_with_payer(
        &[create_account_ix, init_ix],
        Some(&payer.pubkey()),
    );
    let blockhash = client.get_latest_blockhash().unwrap();
    tx.sign(&[payer, &mint], blockhash);

    client.send_and_confirm_transaction(&tx).unwrap();

    mint.pubkey()
}

fn get_ata(owner: &Pubkey, mint: &Pubkey) -> Pubkey {
    let ata_addr = spl_associated_token_account_interface::address::get_associated_token_address(
        &Address::from(owner.to_bytes()),
        &Address::from(mint.to_bytes()),
    );
    Pubkey::from(ata_addr.to_bytes())
}

fn create_ata(
    client: &RpcClient,
    payer: &Keypair,
    owner: &Pubkey,
    mint: &Pubkey,
) -> Pubkey {
    let ata = get_ata(owner, mint);

    if client.get_account(&ata).is_ok() {
        return ata;
    }

    let create_ata_ix =
        spl_associated_token_account_interface::instruction::create_associated_token_account(
            &Address::from(payer.pubkey().to_bytes()),
            &Address::from(owner.to_bytes()),
            &Address::from(mint.to_bytes()),
            &Address::from(spl_token_interface::ID.to_bytes()),
        );

    let create_ata_ix = Instruction {
        program_id: Pubkey::from(create_ata_ix.program_id.to_bytes()),
        accounts: create_ata_ix
            .accounts
            .iter()
            .map(|acc| AccountMeta {
                pubkey: Pubkey::from(acc.pubkey.to_bytes()),
                is_signer: acc.is_signer,
                is_writable: acc.is_writable,
            })
            .collect(),
        data: create_ata_ix.data,
    };

    let mut tx =
        Transaction::new_with_payer(&[create_ata_ix], Some(&payer.pubkey()));
    let blockhash = client.get_latest_blockhash().unwrap();
    tx.sign(&[payer], blockhash);

    client.send_and_confirm_transaction(&tx).unwrap();

    ata
}

fn mint_to(
    client: &RpcClient,
    payer: &Keypair,
    mint: &Pubkey,
    dest: &Pubkey,
    amount: u64,
) {
    let mint_ix = spl_token_interface::instruction::mint_to(
        &Address::from(spl_token_interface::ID.to_bytes()),
        &Address::from(mint.to_bytes()),
        &Address::from(dest.to_bytes()),
        &Address::from(payer.pubkey().to_bytes()),
        &[],
        amount,
    )
    .unwrap();

    let mint_ix = Instruction {
        program_id: Pubkey::from(mint_ix.program_id.to_bytes()),
        accounts: mint_ix
            .accounts
            .iter()
            .map(|acc| AccountMeta {
                pubkey: Pubkey::from(acc.pubkey.to_bytes()),
                is_signer: acc.is_signer,
                is_writable: acc.is_writable,
            })
            .collect(),
        data: mint_ix.data,
    };

    let mut tx = Transaction::new_with_payer(&[mint_ix], Some(&payer.pubkey()));
    let blockhash = client.get_latest_blockhash().unwrap();
    tx.sign(&[payer], blockhash);

    client.send_and_confirm_transaction(&tx).unwrap();
}

fn get_token_balance(client: &RpcClient, token_account: &Pubkey) -> u64 {
    let data = client.get_account_data(token_account).unwrap();
    let account = spl_token_interface::state::Account::unpack(&data).unwrap();
    account.amount
}
