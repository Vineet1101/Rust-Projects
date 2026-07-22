pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("BSY9hxuBeATfDyhPgr1i9LPvQvN7A7yDqfMnNfSnocca");

#[program]
pub mod escrow {
    use super::*;

    pub fn make(ctx: Context<Initialize>) -> Result<()> {
        make::handler(ctx)
    }

    pub fn take(ctx:Context<>)->Result<()>{
        Ok(())
    }

    pub fn refund(ctx:Context<>)->Result<()>{
        Ok(())
    }
}
