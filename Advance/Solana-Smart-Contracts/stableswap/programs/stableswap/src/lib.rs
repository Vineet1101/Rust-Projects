pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;
pub mod math;
pub mod oracle;
pub mod dynamic_fees;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("4qXgy46E7G1ziEafdAkjra4f3KZCqWxpgRhiEQykgcHq");

#[program]
pub mod stableswap {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
}
