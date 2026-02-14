use anchor_lang::prelude::*;

pub mod state;
pub mod errors;
pub mod instructions;

use instructions::*;

declare_id!("InfuserProgram111111111111111111111111");

#[program]
pub mod infuser {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn open_position(ctx: Context<OpenPosition>, size: u64) -> Result<()> {
        open_position::handler(ctx, size)
    }

    pub fn lock_fee(ctx: Context<LockFee>, amount: u64) -> Result<()> {
        lock_fee::handler(ctx, amount)
    }
}
