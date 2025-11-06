# solana-course

# Course intro

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

# Core concepts
- Solana vs Ethereum
- Accounts
    - data
    - lamports
    - owner
- Programs
    - program id - how is it derived?
    - Private key needed for upgrade
    - Pub key needed for user interaction
    - BPF loader
    - System program
    - Token program
- Instructions
- Transactions
- PDA (program derived address)
    - no private key
- CPI (cross program invocation)

# Hello
- Native
    - Borsh
    - `entrypoint`
    - `msg`
    - Build, test, deploy
    - `cargo build-sbf`
    - LiteSVM
    - Script
        - `solana-test-calidator`
        - `.so`
- Anchor
- TODO: deploy, run Solana script on Anchor programs?

# Oracle
- Native
- Anchor

# Piggy bank - PDA
- Native
- Anchor

# Dutch auction ? - Token
- CLI
- Native
- Anchor

# AMM
- Native
- Anchor

# Wormhole

# Resources

- [Solana docs](https://solana.com/docs)
    - [Installation](https://solana.com/docs/intro/installation)
- [Anchor](https://github.com/solana-foundation/anchor)
- [solana-developers/program-examples](https://github.com/solana-developers/program-examples)
- [crates.io](https://crates.io/)
