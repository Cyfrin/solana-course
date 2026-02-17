# Troubleshooting Guide

Common issues and solutions when developing Solana programs.

## Build & Compilation Errors

### Error: `solana-program 3.0.0 causing issues with edition 2024`

**Problem**: Build fails with errors about `solana-program` version incompatibility with Rust edition 2024.

**Solution**: Use solana-program version `2.2.0` or later. This course uses `solana-program = "2.2.0"` for all native programs.

```toml
[dependencies]
solana-program = "2.2.0"  # ✅ Correct
```

### Error: `cargo build-sbf` command not found

**Problem**: 
```
error: tool 'cargo' could not find cargo-build-sbf
```

**Solution**: The cargo-build-sbf plugin is included with Solana CLI. Ensure you have the correct Solana version installed:

```shell
solana --version
# Should be: solana-cli 3.0.10 or later

# If not, reinstall:
sh -c "$(curl -sSfL https://release.solana.com/v3.0.10/install)"
```

### Error: `anchor build` fails silently

**Problem**: `anchor build` exits without error but produces no output.

**Solution**: Usually caused by corrupted Anchor cache. Reset Anchor:

```shell
anchor clean
anchor build
```

### Error: `edition2024` feature not recognized

**Problem**:
```
error: failed to resolve: use of undeclared crate or module
```

**Solution**: Ensure your Cargo.toml has the edition2024 feature at the top:

```toml
cargo-features = ["edition2024"]

[package]
name = "my_program"
edition = "2024"
```

## Testing Issues

### Error: `cargo test` fails but `anchor test` works (or vice versa)

**Problem**: Tests pass with one tool but fail with another.

**Solution**: This can happen due to different test environments:
- `cargo test` uses LiteSVM (faster, in-process)
- `anchor test` uses solana-test-validator (slower, separate process)

For consistent testing:

```shell
# Option 1: Use LiteSVM for fast iteration (recommended for development)
cargo test -- --nocapture

# Option 2: Use anchor test validator (more realistic)
anchor test
```

### Error: `anchor test` timeout or hangs

**Problem**: `anchor test` hangs at "Program log:" or takes 2+ minutes.

**Solution**:

1. **Try resetting Anchor**:
```shell
anchor clean
anchor test
```

2. **Check for port conflicts** (validator needs port 8899):
```shell
lsof -i :8899
# If something is using it, either:
# - Kill the process
# - Or use a different port in local config
```

3. **Restart solana-test-validator**:
```shell
pkill solana-test-validator
sleep 2
anchor test
```

### Error: `Your transaction was not confirmed`

**Problem**: Tests fail with "Transaction not confirmed" when deploying to local validator.

**Solution**: Increase blockhash timeout or add retries:

```rust
// Increase block height buffer
let latest_blockhash = client
    .get_latest_blockhash_with_commitment(CommitmentConfig::default())
    .unwrap()
    .0;
```

## Wallet & Airdrop Issues

### Error: `Airdrop request failed`

**Problem**: `solana airdrop 2` fails on Devnet.

**Solution**: Devnet faucet is rate-limited. Try alternatives:

```shell
# Option 1: Try the web faucet
# Go to: https://faucet.solana.com/

# Option 2: Use local validator (unlimited SOL)
solana config set -ul
solana-test-validator

# In another terminal:
solana airdrop 1000  # 1000 SOL locally
```

### Error: `Insufficient funds for transaction fee`

**Problem**: Wallet balance is too low.

**Solution**: Check and request airdrop:

```shell
# Check current balance
solana balance

# Airdrop on Devnet (must set cluster first)
solana config set -ud
solana airdrop 5

# Or use faucet: https://faucet.solana.com/
```

### Error: `Could not confirm transaction`

**Problem**: Transaction submitted but never confirmed.

**Solution**:

1. **Check account state** (wallet might not exist):
```shell
solana account $(solana address)
```

2. **Verify RPC endpoint is working**:
```shell
solana config get  # Check cluster URL
solana ping        # Test connectivity
```

3. **Create fresh keypair if corrupted**:
```shell
solana-keygen new -o ~/.config/solana/id.json --force
```

## Deployment Issues

### Error: `Insufficient lamports to deploy program`

**Problem**: Program size is too large or not enough SOL for fees.

**Solution**:

```shell
# Airdrop more SOL
solana config set -ud
solana airdrop 10

# Check balance
solana balance

# Deploy (will show fee estimate first)
solana program deploy ./target/deploy/my_program.so
```

### Error: `Program already deployed at this address`

**Problem**: Trying to deploy program with same ID that already exists.

**Solution**:

```shell
# Option 1: Close the old program and reclaim SOL
solana program close $PROGRAM_ID

# Option 2: Generate new program keypair
solana-keygen grind --starts-with h1sm:1 &
# Then use new keypair in Anchor.toml
```

### Error: `Cannot deploy: Permission denied`

**Problem**: Can't write to `target/deploy/` directory.

**Solution**:

```shell
# Check permissions
ls -la target/deploy/

# Fix permissions
chmod 755 target/deploy/

# Rebuild
cargo build-sbf
```

## Network & RPC Issues

### Error: `Failed to get cluster version`

**Problem**: Cannot connect to RPC endpoint.

**Solution**:

```shell
# Check current cluster
solana config get

# Test RPC connectivity
solana ping

# Try switching clusters
solana config set --url devnet
solana ping

# Or use alternative RPC:
solana config set --url https://api.devnet.solana.com
```

### Error: `Blockheight exceeded`

**Problem**: Transaction signature expired or blockhash stale.

**Solution**: This is normal when network is slow. Retry the transaction:

```shell
# For local validator, usually means a process is stuck:
pkill solana-test-validator
solana-test-validator &
```

### Error: `You are not on a known cluster`

**Problem**: RPC URL is invalid or network is down.

**Solution**: Use official Solana clusters:

```shell
# Devnet (testing with airdrop)
solana config set -ud

# Localhost (local validator)
solana config set -ul

# Testnet (less stable)
solana config set -ut

# Mainnet (real SOL - BE CAREFUL)
solana config set -um
```

## Program Execution Issues

### Error: `Program failed to complete: custom program error: 0x1`

**Problem**: Program returned a custom error (error code 1).

**Solution**: Check program logs:

```shell
# For local test validator, add logging to your program:
msg!("Debug info: {}", value);

# Run test with output:
cargo test -- --nocapture

# For devnet, check Solana Explorer:
# https://explorer.solana.com/?cluster=devnet
# Paste transaction signature to see logs
```

### Error: `Program log: panicked at ...`

**Problem**: Program panicked, showing Rust panic message.

**Solution**: 

```rust
// Replace panics with proper error handling:
require!(condition, error::Error::InvalidCondition);  // Anchor
assert!(condition);  // Only in tests
```

Add detailed error codes:

```rust
#[error_code]
pub enum Error {
    #[msg("Invalid amount provided")]
    InvalidAmount,
    
    #[msg("Expired lock")]
    ExpiredLock,
}
```

### Error: `Program returned error: "Unknown"`

**Problem**: Vague error with no details.

**Solution**: Add logging to identify where it fails:

```rust
pub fn process_instruction(...) -> ProgramResult {
    msg!("Instruction started");
    
    msg!("Processing account 1");
    // ... code ...
    
    msg!("About to call CPI");
    // ... CPI code ...
    
    msg!("Instruction completed");
    Ok(())
}
```

## Anchor-Specific Issues

### Error: `TypeScript test compilation failed`

**Problem**: TypeScript tests in `tests/` folder have errors.

**Solution**:

1. Check TypeScript version:
```shell
npx tsc --version
```

2. Regenerate IDL and types:
```shell
anchor build
# This generates idl/program_name.json
```

3. Update imports in test file:
```typescript
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import type { MyProgram } from "../target/types/my_program";
```

### Error: `IDL file not found: target/idl/program.json`

**Problem**: IDL wasn't generated during build.

**Solution**:

```shell
# Ensure [features] includes idl-build
# In your Cargo.toml:
[features]
idl-build = ["anchor-lang/idl-build"]

# Rebuild
anchor clean
anchor build
```

## Version Compatibility Matrix

| Component | Version | Notes |
|-----------|---------|-------|
| Rust | 1.87.0+ | Latest stable recommended |
| Solana CLI | 3.0.10+ | `solana --version` |
| Anchor CLI | 0.31.1+ | `anchor --version` |
| solana-program | 2.2.0 | Native programs |
| anchor-lang | 0.31.1 | Anchor programs |
| solana-sdk | 2.3 | Testing |

Verify versions:

```shell
rustc --version      # Should be 1.87.0+
solana --version     # Should be solana-cli 3.0.10+
anchor --version     # Should be anchor-cli 0.31.1+
```

## Tips for Debugging

### 1. Read Error Messages Carefully

Most errors include the problem and location. Look for:
- Line numbers in Rust errors
- Transaction signature links in Devnet errors
- Account info in transaction failures

### 2. Use the Solana Explorer

For deployed programs:

```
https://explorer.solana.com/?cluster=devnet
```

Paste transaction signature to see:
- Program logs
- Account changes
- Fees spent
- Error messages

### 3. Enable All Logging

```rust
// Add to your program
msg!("Entering function X with param: {}", param);
println!("Debug: {:#?}", accounts);  // In tests only
```

### 4. Test Locally First

Always test on local validator before Devnet:

```shell
solana config set -ul
solana-test-validator  # Terminal 1

# Terminal 2
cargo test -- --nocapture
```

### 5. Check Account Ownership

Common issue - verify account is owned by your program:

```rust
if account.owner != program_id {
    return Err(ProgramError::InvalidAccountOwner);
}
```

### 6. Clear Cache on Strange Behavior

```shell
# Rust build cache
cargo clean

# Anchor cache
anchor clean

# Local validator state
rm -rf test-ledger/
```

## Still Stuck? Getting Help

1. **Check course resources**:
   - [Solana Docs](https://solana.com/docs)
   - [Anchor Documentation](https://www.anchor-lang.com/docs)
   - [Solana Cookbook](https://solanacookbook.com/)

2. **Community support**:
   - [Discord](https://discord.gg/cyfrin) - Cyfrin community
   - [Solana Discord](https://discord.gg/solana) - Solana foundation
   - [Stack Exchange](https://solana.stackexchange.com/) - Q&A

3. **Search existing issues**:
   - GitHub issue tracker for [Anchor](https://github.com/coral-xyz/anchor/issues)
   - [solana-program releases](https://github.com/solana-labs/solana/releases)

