use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserStaking{
    pub user:Pubkey,
    pub user_token_account:Pubkey,
    pub staking_pool:Pubkey,
    pub amount_staked:u64,
    pub bump:u8
}

