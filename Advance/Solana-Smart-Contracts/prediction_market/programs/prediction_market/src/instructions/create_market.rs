use anchor_lang::prelude::*;

use crate::{constants::MAX_QUESTION_LEN, state::Market,error::*};

#[derive(Accounts)]
#[instruction(market_id:u64,question:String)]
pub struct InitializeMarket<'info>{
    #[account(mut)]
    pub creator:Signer<'info>,
    
    #[account(
        init,
        space=8+Market::INIT_SPACE,
        payer=creator,
        seeds=[b"market",market.key.as_ref(),market_id.to_le_bytes().as_ref()],
        bump
    )]
    pub market:Account<'info,Market>,

    pub system_program:Program<'info,System>
}

impl<'info> InitializeMarket<'info>{

    fn  initialize_market(&mut self,question:String,market_id:u64,resolution_time:i64)->Result<()>{
        require!(question.len()<=MAX_QUESTION_LEN,MarketError::Overflow);

        let clock=Clock::get()?;
        require!(resolution_time>clock.unix_timestamp,MarketError::ResolutionTimeInPast);
        
        let bump=self.market.bump;
        self.market.set_inner(Market{
            creator:*self.creator.key,market_id,question,resolution_time,yes_pool:0,no_pool:0u64,resolved:false,outcome:None,bump
        });

        Ok(())
    }
}
