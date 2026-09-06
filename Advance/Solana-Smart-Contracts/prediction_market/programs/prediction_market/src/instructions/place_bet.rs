use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;

use crate::state::{Market, UserPosition};
use crate::error::*;

#[derive(Accounts)]
pub struct PlaceBet<'info>{

    #[account(mut)]
    pub user:Signer<'info>,
    
    #[account(mut)]
    pub market:Account<'info,Market>,

    #[account(
        init_if_needed,
        payer=user,
        space=8+UserPosition::INIT_SPACE,
        seeds=[b"userposition",market.key().as_ref(),user.key().as_ref()],
        bump
    )]
    pub user_position:Account<'info,UserPosition>,

    pub system_program:Program<'info,System>
}


impl<'info> PlaceBet<'info>{
    fn place_bet(&mut self,amount:u64,bet_yes:bool)->Result<()>{
            
        require!(amount>0,MarketError::InvalidBetAmount);
        let clock=Clock::get()?;

        require!(self.market.resolution_time>clock.unix_timestamp,MarketError::BettingClosed);

        let ixn=system_instruction::transfer(&self.user.key(), &self.user_position.key(), amount);

        invoke(&ixn,&[
            self.user.to_account_info(),
            self.user_position.to_account_info(),
            self.system_program.to_account_info()
        ])?;
        let bump=self.user_position.bump;
        if bet_yes{
            self.market.yes_pool.checked_add(amount).ok_or(MarketError::Overflow);
            self.user_position.set_inner(UserPosition { user: *self.user.key, market: self.market.key(), yes_amount: amount, no_amount: 0, claimed: false,bump });
        }else{
            self.market.no_pool.checked_add(amount).ok_or(MarketError::Overflow);
            self.user_position.set_inner(UserPosition { user: *self.user.key, market: self.market.key(), yes_amount: 0, no_amount: amount, claimed: false,bump }); 
        }
        Ok(())
    }
}
