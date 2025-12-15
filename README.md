# solana-course

TODO: clean up

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
- [x] [Solana vs Ethereum](./notes/eth-sol.png)
- [x] [Accounts](./notes/account.png)
- [x] [Programs, instructions and transactions](./notes/program.png)
- [x] [PDA](./notes/pda.png)
- [x] [CPI](https://solana.com/docs/core/cpi)
- [x] [IDL](https://solana.com/developers/guides/advanced/idls)

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
        - [ ] program id
            - how is it derived?
                - Randomly generated private key typically stored under  `target/deploy/my_program.json`
                - Private key needed for upgrade
            - Pub key needed for user interaction
        - [Solana explorer](https://explorer.solana.com/)
        - [ ] [Limitations](https://solana.com/docs/programs/limitations)
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
    - No auto generated IDL

# Wormhole
- [Overview](./notes/wormhole.png)
    - [Overview](https://wormhole.com/docs/products/messaging/overview/)
    - Wormhole is a cross-chain messaging and bridging protocol
    - [Guardians](https://wormhole.com/docs/protocol/infrastructure/guardians/)
        - nodes that validate messages (events) emitted to Wormhole's core contracts
        - Consensus -> 19 nodes (2/3 majority) 13 nodes must approve
            - [Verifiable Action Approvals (VAA)](https://wormhole.com/docs/protocol/infrastructure/vaas/)
            - VAA submitted to destination chain
        - [Dashboard](https://wormhole-foundation.github.io/wormhole-dashboard/#/?endpoint=Mainnet)
    - Executor
        - ets anyone act as a relayer to take a Wormhole message (a VAA) from one chain and execute it on another
    - [NTT](https://wormhole.com/docs/products/token-transfers/overview/)
        - [Burn & mint, hub and spokes](./notes/wormhole-ntt-modes.png)
- NTT
    - Overview (SPL to ETH ERC20)
    - CLI setup (NTT 1.6.0)
        - https://wormhole.com/docs/products/token-transfers/native-token-transfers/get-started/
        - ERC20 / SPL
            - [Deploy ERC20](https://sepolia.etherscan.io/address/0x0c3d43954B0b312D591739980E0A157621B581BC)
            ```shell
            export ERC20=...
            ```
            - Deploy SPL
                ```shell
                solana config set -ud
                solana balance
                solana airdrop 1

                cargo install spl-token-cli
                spl-token --version


                spl-token create-token
                MINT=...

                spl-token create-account $MINT
                ATA=...

                spl-token mint $MINT 1000

                spl-token accounts
                ```

                ```shell
                export SPL=...
                ```

                - Install NTT CLI
                ```shell
                ntt --version
                ```
                - Init NTT project
                - Deploy to EVM
                    ```shell
                    export ETH_PRIVATE_KEY=
                    export SEPOLIA_SCAN_API_KEY=
                    ```
                - Set rate limits
                - Set mint authority to NTTManager
            - Deploy to SVM
                - `solana airdrop 10`
                - Solana and Anchor version
                    ```shell
                    Install latest Solana CLI
                    sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"

                    NTT specific version
                    sh -c "$(curl -sSfL https://release.anza.xyz/v1.18.26/install)"

                    cargo install --git https://github.com/solana-foundation/anchor --tag v0.29.0 anchor-cli

                    solana --version
                    anchor --version
                    ```
                - Demo with NTT SDK
                  https://github.com/wormhole-foundation/demo-ntt-ts-sdk
                  - comment out optional RPC url
                  - set dst address and amount
                  - ts-ignore wh
                  https://wormholescan.io/#/tx/26ZaFXBfHwSzSQL9xEWmGpJheV4kPcSeAQVbTNizaxeo9Cf8Zq5QDDygXGs6roquLRKhaaWYoG6yFnUyxpNw2s1X?network=Testnet
    - Token transfer demo
- Deploy AMM + swap using NTT
    - [Overview](./notes/wormhole-amm.png)
    - Deploy Solana AMM (NTT token + token)
        - [NTT token](https://explorer.solana.com/address/73Rgt8CZCJez89VtJdRd84kfUPVSprS2Sy5V7Skmq1bU?cluster=devnet)
        - [SPL token](https://explorer.solana.com/address/GK4c9bYHnKEDeKMXfve9xWFQ7byjjaWWdGNrzBW2Geep?cluster=devnet)
        - [CSAMM](https://explorer.solana.com/address/9Xsm3WVTBY6ALbUhRTDzt5wVZiNN52BU5kXUR3m6ERZ?cluster=devnet)
            - Deploy (switch Solana CLI to latest)
            ```
            sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"

            solana config set -ud
            solana program deploy ./target/deploy/amm.so
            ```
            - Transfer NTT token from EVM
            - Execute AMM script
                - init pool, add liquidity, swap
    - Deploy EVM AMM (NTT token + token)
        - [NTT token](https://sepolia.etherscan.io/address/0x0c3d43954B0b312D591739980E0A157621B581BC)
        - [ERC20 token](https://sepolia.etherscan.io/address/0x140e1Af0bdd3AcE2D2CbE5B76F1De4A40c340308)
        - [CSAMM](https://sepolia.etherscan.io/address/0xadd4350ce0de140fbb081a4627fde251eb5c1f26)
        - Deploy NTT token
        - Deploy ERC20
        - Deploy CSAMM
        - Add liquidity
            - Mint ERC20 and NTT token
            - ERC20 and NTT token approve CSAMM and add liquidity


# Resources

- [Solana docs](https://solana.com/docs)
- [Solana program](https://www.solana-program.com/)
- [solscan](https://solscan.io/)
- [Solana faucet](https://faucet.solana.com/)
- [GitHub - Solana program](https://github.com/solana-program)
- [GitHub - Anchor](https://github.com/solana-foundation/anchor)
- [Anchor doc](https://www.anchor-lang.com/docs)
- [GitHub - solana-developers/program-examples](https://github.com/solana-developers/program-examples)
- [GitHub - litesvm](https://github.com/LiteSVM/litesvm)
- [docs.rs - litesvm](https://docs.rs/litesvm/latest/litesvm/)
- [Solana explorer](https://explorer.solana.com/)
- [crates.io](https://crates.io/)
- [Solana playground](https://beta.solpg.io/)
- [GitHub - Wormhole](https://github.com/wormhole-foundation)
- [GitHub - Wormhole NTT](https://github.com/wormhole-foundation/native-token-transfers)
- [GitHub - Wormhole TypeScript SDK](https://github.com/wormhole-foundation/wormhole-sdk-ts)
- [Wormhole - NTT](https://wormhole.com/docs/products/token-transfers/native-token-transfers/overview/)
- [Wormhole Advanced Tech Workshop: NTT](https://www.youtube.com/watch?v=ltZmeyjUxRk)
- [Remix](https://remix.ethereum.org)
