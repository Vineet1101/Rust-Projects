pub mod instructions;
pub mod state;
pub mod constants;
pub mod error;


use anchor_lang::prelude::*;

declare_id!("3UbWs4nuaMKoGkjhLuoLdgVESe9YkjjpDMBaZsz33cVt");

pub mod prediction_market{
    use super::*;

    pub fn create_market(ctx:Context<CreateMarket>,market_id:u64,question:String,resolution_time:i64)->Result<()>{
        instructions::create_market::handler(ctx, market_id,question,resolution_time)
    }
}