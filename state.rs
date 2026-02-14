use anchor_lang::prelude::*;

#[account]
pub struct Market {
    pub authority: Pubkey,
    pub total_locked: u64,
}

#[account]
pub struct Position {
    pub trader: Pubkey,
    pub size: u64,
}
