use anchor_lang::prelude::*;

#[error_code]
pub enum MarketError {
    #[msg("Question length exceeded")]
    Overflow,

    #[msg("Resolution Time cannot be set to past")]
    ResolutionTimeInPast
}