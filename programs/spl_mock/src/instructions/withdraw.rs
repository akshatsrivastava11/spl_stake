use anchor_spl::{associated_token::AssociatedToken, token::{transfer,Transfer}, token_interface::{Mint, TokenAccount, TokenInterface}};
use anchor_lang::{prelude::*};
use crate::{StakingPool, UserStaking};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"staking_pool".as_ref()],
        bump
    )]
    pub staking_pool: Account<'info, StakingPool>,
    #[account(
        mut,
        associated_token::mint = staking_mint,
        associated_token::authority = staking_pool
    )]
    pub staking_vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mint::token_program = token_program
    )]
    pub staking_mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        seeds = [b"user_staking".as_ref(), signer.key().as_ref()],
        bump,
        close = signer
    )]
    pub user_staking: Account<'info, UserStaking>,
    #[account(
        mut,
        associated_token::mint = staking_mint,
        associated_token::authority = signer
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>
}

impl<'info> Withdraw<'info> {
    pub fn withdraw(&mut self) -> Result<()> {
        self.transfer_back()
    }

    pub fn transfer_back(&mut self) -> Result<()> {
        let program_id = self.token_program.to_account_info();
        let accounts = Transfer {
            from: self.staking_vault.to_account_info(),
            to: self.user_token_account.to_account_info(),
            authority: self.staking_pool.to_account_info()
        };

        // Create seeds for the staking_pool PDA
        let seeds = &[
            b"staking_pool".as_ref(),
            &[self.staking_pool.bump], // Use the bump stored in the staking_pool account
        ];
        let signer_seeds = &[&seeds[..]];

        // Create CPI context with seeds so the PDA can sign
        let ctx = CpiContext::new_with_signer(program_id, accounts, signer_seeds);
        
        transfer(ctx, self.user_staking.amount_staked)?;
        
        // Update the staking pool's total_staked amount
        self.staking_pool.total_staked = self.staking_pool.total_staked
            .checked_sub(self.user_staking.amount_staked)
            .ok_or(ProgramError::ArithmeticOverflow)?;
            
        Ok(())
    }
}