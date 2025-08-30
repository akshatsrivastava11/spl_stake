use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};
use anchor_spl::associated_token::AssociatedToken;
use crate::StakingPool;

#[derive(Accounts)]
pub struct InitializeStakingPool<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    
    #[account(
        init,
        payer = signer,
        space = 8 + StakingPool::INIT_SPACE,
        seeds = [b"staking_pool".as_ref()],
        bump
    )]
    pub staking_pool: Account<'info, StakingPool>,
    
    #[account(
        mint::authority = staking_pool,
        mint::decimals = 6,
        mint::token_program = token_program,
    )]
    pub staking_mint: InterfaceAccount<'info, Mint>,
    
    #[account(
        init,
        payer = signer,
        associated_token::mint = staking_mint,
        associated_token::authority = staking_pool,
        associated_token::token_program = token_program,
    )]
    pub staking_vault: InterfaceAccount<'info, TokenAccount>,
    
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> InitializeStakingPool<'info> {
    pub fn initialize_staking_pool(&mut self, bumps: InitializeStakingPoolBumps) -> Result<()> {
        self.staking_pool.set_inner(StakingPool {
            authority: self.signer.key(),
            total_staked: 0,
            staking_mint: self.staking_mint.key(),
            staking_vault: self.staking_vault.key(),
            bump: bumps.staking_pool,
        });
        Ok(())
    }
}