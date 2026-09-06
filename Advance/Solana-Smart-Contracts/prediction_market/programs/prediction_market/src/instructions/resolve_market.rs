use anchor_lang::prelude::*;

use crate::state::Market;
use crate::error::*;

#[derive(Accounts)]
pub struct ResolveMarket<'info>{
    #[account(mut)]
    pub creator:Signer<'info>,
    
    #[account(mut)]
    pub market:Account<'info,Market>,

    pub system_program:Program<'info,System>
}

impl <'info> ResolveMarket<'info>{
    fn resolve_market(&mut self)->Result<()>{
        
        let clock=Clock::get()?;
        require!(clock.unix_timestamp>self.market.resolution_time,MarketError::ResolutionTimeInPast);
        Ok(())
    }
}

pub fn handler()->Result<()>{
Ok(())
}
