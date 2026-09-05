use anchor_lang::prelude::*;

use crate::constants::MAX_QUESTION_LEN;



/// Market account for storing market state
#[account]
#[derive(InitSpace)]
pub struct Market{
    /// Creator and resolver of the market
    pub creator:Pubkey,

    /// Unique market ID (per creator)
    pub market_id:u64,

    /// Question
    #[max_len(MAX_QUESTION_LEN)]
    pub question:String,

    /// Time when market will be resolved
    pub resolution_time:i64,

    pub yes_pool:u64,

    pub no_pool:u64,

    /// Tells whether the market has been resolved or not
    pub resolved:bool,

    pub outcome:Option<bool>,

    /// Used to dervie the PDA
    pub bump:u8,

}


#[account]
#[derive(InitSpace)]
pub struct UserPosition{

    /// Public Key of the user who created this market
    pub user:Pubkey,

    /// Market which user is placing bet
    pub market:Pubkey,

    /// Lamports placed on yes bet
    pub yes_amount:u64,

    /// Lamports placed on no bet
    pub no_amount:u64,
    
    /// Whether user has claimed its winnings or not
    pub claimed:bool,
    
    /// PDA bump seed
    pub bump:u8
    
}
