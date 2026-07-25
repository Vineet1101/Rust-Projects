pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use instructions::*;
pub use state::*;

declare_id!("BSY9hxuBeATfDyhPgr1i9LPvQvN7A7yDqfMnNfSnocca");

#[program]
pub mod escrow {
    use super::*;

    pub fn make(ctx: Context<Make>,seed:u64,receive_amount:u64,deposit_amount:u64) -> Result<()> {
        instructions::make::handler(ctx,seed,receive_amount,deposit_amount)
    }

    pub fn take(ctx:Context<Take>)->Result<()>{
        instructions::take::take(ctx);
        Ok(())
    }

    pub fn refund(ctx:Context<Refund>)->Result<()>{
        instructions::refund::refund(ctx);
        Ok(())
    }
}
