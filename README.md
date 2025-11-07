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

# Core concepts
- Solana vs Ethereum
- Accounts
    - data
    - lamports
    - owner
    - rent
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
- [GitHub - Anchor](https://github.com/solana-foundation/anchor)
- [Anchor doc](https://www.anchor-lang.com/docs)
- [GitHub - solana-developers/program-examples](https://github.com/solana-developers/program-examples)
- [Solana explorer](https://explorer.solana.com/)
- [crates.io](https://crates.io/)
- [Solana playground](https://beta.solpg.io/)
