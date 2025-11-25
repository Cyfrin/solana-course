use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

declare_id!("C8ft4mixLcvxcum1JiMMi8SLR8muASoEKvsQG8XQf7JJ");

#[program]
pub mod amm {
    pub use super::instructions::*;
    use super::*;

    pub fn init_pool(ctx: Context<InitPool>, fee: u16) -> Result<()> {
        // instructions::init_pool(ctx, fee)?;
        Ok(())
    }

    pub fn add_liquidity(
        ctx: Context<AddLiquidity>,
        fee: u16,
        amount_a: u64,
        amount_b: u64,
    ) -> Result<()> {
        // instructions::add_liquidity(ctx, fee, amount_a, amount_b)?;
        Ok(())
    }

    pub fn remove_liquidity(
        ctx: Context<RemoveLiquidity>,
        fee: u16,
        shares: u64,
        min_amount_a: u64,
        min_amount_b: u64,
    ) -> Result<()> {
        /*
        instructions::remove_liquidity(
            ctx,
            fee,
            shares,
            min_amount_a,
            min_amount_b,
        )?;
        */
        Ok(())
    }

    pub fn swap(
        ctx: Context<Swap>,
        fee: u16,
        a_for_b: bool,
        amount_in: u64,
        min_amount_out: u64,
    ) -> Result<()> {
        // instructions::swap(ctx, fee, a_for_b, amount_in, min_amount_out)?;
        Ok(())
    }
}
