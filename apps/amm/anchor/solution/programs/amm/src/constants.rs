use anchor_lang::prelude::*;

pub const MAX_POOL_FEE: u16 = 10000;

#[constant]
pub const POOL_AUTH_SEED_PREFIX: &[u8] = b"pool_auth";
#[constant]
pub const POOL_MINT_SEED_PREFIX: &[u8] = b"pool_mint";
