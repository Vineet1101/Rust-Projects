use anchor_lang::prelude::*;

#[error_code]
pub enum StableSwapError {
    #[msg("Custom error message")]
    CustomError,

    MathOverflow,

    InvalidFeeConfig,

    InvalidDepegThreshold,

    EmptyPool,

    ConvergenceFailed,

    ZeroAmount,

    InsufficientInitialLiquidity,

    InvalidVault,

    InsufficientLiquidity,

    SingleSidedWithdrawalNotAllowed

    
}
