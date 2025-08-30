use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace,)]
pub struct StakingPool{
    pub authority: Pubkey,
    pub staking_mint:Pubkey,
    pub staking_vault:Pubkey,
    pub total_staked:u64,
    pub bump:u8
}
