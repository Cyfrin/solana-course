# solana-course

# Course intro

- Solana CLI 3.0
- Solana SDK and program 2.2
- Anchor 0.31

- [ ] [Course intro](./notes/course_intro.md)
    - Prerequisites
        - Rust
        - Blockchain
    - Learn
        - AI (starter -> fix code)
            - Anchor -> Native
            - test
        - crates.rs -> docs.rs
- [ ] [Setup](./notes/install.md)
    - Install
    - CLI basics
    - Wallet
    - Exercises
        - native, anchor, exercise, solution
            - README is exercise
        - Debug
            - `msg`
        - Anchor tests in Rust are unreliable

# Core concepts
- Solana vs Ethereum
- Accounts
    - data
    - lamports
    - owner
    - Keypair must prove existence of keypair before sending transaction to create account, receive SOL, etc
    - Rent
        - Data must be empty if SOL = 0
- Programs
    - program id - how is it derived?
    - Private key needed for upgrade
    - Pub key needed for user interaction
    - BPF loader
    - System program
    - Token program
    - [Limitations](https://solana.com/docs/programs/limitations)
- Instructions
- Transactions
- PDA (program derived address)
    - no private key
- CPI (cross program invocation)
- IDL

# Hello
- [ ] [Native](./apps/hello/native)
    - Borsh
    - `entrypoint`
    - `msg`
    - Build, test, deploy
    - `cargo build-sbf`
    - Test
        - Script
            - `solana-test-calidator`
            - `.so`
    - Deploy
        - [Solana explorer](https://explorer.solana.com/)
    - Exercises
- [ ] [Anchor](./apps/hello/anchor)
    - `anchor init hello --test-template rust`
    - https://www.anchor-lang.com/docs/basics/program-structure
        - `declare_id` -> Anchor.toml
        - `#program`
        - `Accounts`

# Oracle
- [ ] [Native](./apps/oracle/native)
    - State - Borsh
    - Program owns oracle account
    - Program owns oracle account
    - Oracle data space
    - Order of account is important
    - `owner` must sign
- [ ] [Anchor](./apps/oracle/anchor)
    - `anchor keys sync`
    - `InitSpace`
    - `Signer`
    - `mut`
    - `init`
    - `constraint`
    - Discriminator

# Piggy bank - PDA
- [ ] [Native](./apps/piggy-bank/native)
    - CPI - invoke
    - Calculate PDA before PDA is initialized
    - Create PDA
    - Transfer SOL
    - Manually send SOL
    - Account data must be empty when account has 0 SOL
- [ ] [Anchor](./apps/piggy-bank/anchor)
    - PDA bump, init and close

# Dutch auction
- [ ] [SPL Token](./notes/spl.png)
- [ ] [Token CLI](./notes/token.md)
    - Create mint
        - Mint account state
        - Mint authority
    - Create token account
    - Mint tokens
- [ ] [Anchor](./apps/auction/anchor)
- [ ] [Native](./apps/auction/native)

# AMM
- [ ] [Anchor](./apps/amm/anchor)
- [ ] [Native](./apps/amm/native)

# CPI and IDL
- [ ] [Anchor](./apps/cpi/anchor)
    - compile counter
    - `Factory counter; Signer<info>`
    // In project root, copy IDL files to idls
    //  cp target/idl/*.json ./idls/
    - `Anchor.toml`
    - tests/Cargo.toml (`no-entrypoint`)
- [ ] [Native](./apps/cpi/native)

# Wormhole

# Resources

- [Solana docs](https://solana.com/docs)
- [Solana program](https://www.solana-program.com/)
- [GitHub - Solana program](https://github.com/solana-program)
- [GitHub - Anchor](https://github.com/solana-foundation/anchor)
- [Anchor doc](https://www.anchor-lang.com/docs)
- [GitHub - solana-developers/program-examples](https://github.com/solana-developers/program-examples)
- [GitHub - litesvm](https://github.com/LiteSVM/litesvm)
- [docs.rs - litesvm](https://docs.rs/litesvm/latest/litesvm/)
- [Solana explorer](https://explorer.solana.com/)
- [crates.io](https://crates.io/)
- [Solana playground](https://beta.solpg.io/)
