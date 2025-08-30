use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};
use anchor_spl::associated_token::AssociatedToken;
use crate::StakingPool;

#[derive(Accounts)]
pub struct InitializeStakingPool<'info> {
    // the account paying for initialization (also becomes authority)
    #[account(mut)]
    pub signer: Signer<'info>,
    
    // PDA account that stores the staking pool state
    #[account(
        init,
        payer = signer,
        space = 8 + StakingPool::INIT_SPACE, // discriminator + state size
        seeds = [b"staking_pool".as_ref()],  // PDA seed
        bump
    )]
    pub staking_pool: Account<'info, StakingPool>,
    
    // the mint that represents staking tokens
    // authority is set to staking_pool PDA
    #[account(
        mint::authority = staking_pool,
        mint::decimals = 6,
        mint::token_program = token_program,
    )]
    pub staking_mint: InterfaceAccount<'info, Mint>,
    
    // vault ATA owned by staking_pool PDA
    // tokens deposited by users are stored here
    #[account(
        init,
        payer = signer,
        associated_token::mint = staking_mint,
        associated_token::authority = staking_pool,
        associated_token::token_program = token_program,
    )]
    pub staking_vault: InterfaceAccount<'info, TokenAccount>,
    
    // system + other required programs
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> InitializeStakingPool<'info> {
    pub fn initialize_staking_pool(&mut self, bumps: InitializeStakingPoolBumps) -> Result<()> {
        // set initial state for staking pool account
        self.staking_pool.set_inner(StakingPool {
            authority: self.signer.key(),              // who controls the pool
            total_staked: 0,                           // no tokens staked initially
            staking_mint: self.staking_mint.key(),     // record staking mint
            staking_vault: self.staking_vault.key(),   // record staking vault ATA
            bump: bumps.staking_pool,                  // store bump for PDA
        });
        Ok(())
    }
}
