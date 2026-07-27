use anchor_lang::prelude::*;

use crate::state::{Market,UserPosition};

#[derive(Accounts)]
pub struct PlaceBet<'info>{

    #[account(mut)]
    pub user:Signer<'info>,
    
    #[account(
        mut,
        seeds=[b"market",creator.key.as_ref(),market_id.to_le_bytes().as_ref()],
        bump=market.bump
    )]
    pub market:Account<'info,Market>,

    #[account(
        init_if_needed,
        payer=user,
        space=8+UserPosition::INIT_SPACE,
        seeds=[b"user_position",market.key().as_ref(),user.key().as_ref()],
        bump
    )]
    pub user_position:Account<'info,UserPosition>,

    pub system_program:Program<'info,System>
}


pub fn handler(ctx:Context<PlaceBet>,)->Result<()>{
    Ok(())
}