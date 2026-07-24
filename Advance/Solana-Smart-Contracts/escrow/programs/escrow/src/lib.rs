pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use instructions::*;
pub use state::*;

declare_id!("BpJapcebzXJctnLYYaG4fJrdDD7L8wXZ3vzKmmJh3RpS");

#[program]
pub mod escrow {
    use super::*;

    pub fn make(ctx: Context<Make>,seed:u64,receive_amount:u64,deposit_amount:u64) -> Result<()> {
        instructions::make::handler(ctx,seed,receive_amount,deposit_amount)
    }

    pub fn take(ctx:Context<Take>)->Result<()>{
        instructions::take::handler(ctx);
        Ok(())
    }

    pub fn refund(ctx:Context<Refund>)->Result<()>{
        instructions::refund::handler(ctx);
        Ok(())
    }
}
