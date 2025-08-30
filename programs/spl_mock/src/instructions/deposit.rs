use anchor_lang::{prelude::{Account, *}};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Transfer},
    token_interface::{Mint, TokenAccount, TokenInterface}
};

use crate::{StakingPool, UserStaking};

#[derive(Accounts)]
pub struct Deposit<'info> {
    // user who is depositing tokens
    #[account(mut)]
    pub signer: Signer<'info>,

    // staking pool PDA (already initialized in InitializeStakingPool)
    #[account(
        mut,
        seeds = [b"staking_pool".as_ref()],
        bump
    )]
    pub staking_pool: Account<'info, StakingPool>,

    // vault ATA owned by staking_pool PDA
    // tokens deposited will be transferred here
    #[account(
        mut,
        associated_token::mint = staking_mint,
        associated_token::authority = staking_pool
    )]
    pub staking_vault: InterfaceAccount<'info, TokenAccount>,

    // staking mint used for deposits
    #[account(
        mint::token_program = token_program,
    )]
    pub staking_mint: InterfaceAccount<'info, Mint>,

    // user staking account (tracks how much user staked)
    // gets initialized on first deposit
    #[account(
        init,
        payer = signer,
        space = 8 + UserStaking::INIT_SPACE,
        seeds = [b"user_staking".as_ref(), signer.key().as_ref()],
        bump
    )]
    pub user_staking: Account<'info, UserStaking>,

    // user's own token account (source of deposit)
    #[account(
        mut,
        associated_token::mint = staking_mint,
        associated_token::authority = signer
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,

    // required programs
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Deposit<'info> {
    // main deposit function → sets up user staking account + transfers tokens
    pub fn deposit(&mut self, amount: u64, bumps: DepositBumps) -> Result<()> {
        self.initialize_user_staking(amount, bumps); // create/record user staking state
        self.deposit_token(amount); // move tokens into staking vault
        self.staking_pool.total_staked+=amount; // update staking pool state
        Ok(())
        }

    // store initial state for user's staking account
    pub fn initialize_user_staking(&mut self, amount: u64, bumps: DepositBumps) -> Result<()> {
        self.user_staking.set_inner(UserStaking {
            user: self.signer.key(),                  // who staked
            staking_pool: self.staking_pool.key(),    // link to pool
            user_token_account: self.user_token_account.key(), // their ATA
            amount_staked: amount,                    // amount they staked
            bump: bumps.user_staking,                 // bump for PDA
        });
        Ok(())
    }

    // actually transfer tokens from user → vault
    pub fn deposit_token(&mut self, amount: u64) -> Result<()> {
        let program_id = self.token_program.to_account_info();

        // setup token transfer accounts
        let accounts = Transfer {
            from: self.user_token_account.to_account_info(),
            to: self.staking_vault.to_account_info(),
            authority: self.signer.to_account_info(),    
        };

        // create CPI context + call transfer
        let ctx = CpiContext::new(program_id, accounts);
        transfer(ctx, amount)
    }

}
