# Hello Solana (Native)

Create an empty folder and complete all tasks below
- Build
- Test locally with LiteSVM
- Deploy locally to `solana-test-validator` and test with Rust script
- Deploy to Devnet and run the Rust script again

# Create repo
Create empty repo then init it using

```shell
cargo init --lib
```

Copy paste content from course resources.

In /src/lib.rs :

```rust
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

```

In Cargo.toml :

```rust
cargo-features = ["edition2024"]

[package]
name = "hello"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib", "lib"]

[dependencies]
borsh = "1.5.7"
solana-program = "2.2.0"

[dev-dependencies]
litesvm = "0.7"
solana-sdk = "2.3"
solana-client = "2.3"
solana-transaction-status-client-types = "2.3"
```

# Build

Generates `.so` file under `target/deploy`
```shell
cargo build-sbf
```

# Test
```shell
cargo test -- --nocapture
```

# Test with script

Run local validator
```shell
solana config set -ul
solana-test-validator
```

Check program id
```shell
solana address -k ./target/deploy/hello-keypair.json
```

Deploy program
```shell
solana program deploy ./target/deploy/hello.so
```

Execute demo script
```shell
PROGRAM_ID=your program ID
RPC=http://localhost:8899
KEYPAIR=path to key pair

cargo run --example demo $KEYPAIR $RPC $PROGRAM_ID
```

# Deploy to Devnet

```shell
solana config set -ud

solana balance
# Airdrop if wallet balance is low
solana airdrop 1

cargo build-sbf

solana program deploy ./target/deploy/hello.so

PROGRAM_ID=your program ID
RPC=https://api.devnet.solana.com
KEYPAIR=path to key pair

cargo run --example demo $KEYPAIR $RPC $PROGRAM_ID
```

Check transaction signature at [Solana explorer](https://explorer.solana.com/?cluster=devnet)

Close program to reclaim SOL
```shell
solana program close $PROGRAM_ID
```
