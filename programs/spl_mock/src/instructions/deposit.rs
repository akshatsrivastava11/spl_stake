use anchor_lang::{prelude::{Account, *}};
use anchor_spl::{associated_token::AssociatedToken, token::{transfer, Transfer}, token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::{StakingPool, UserStaking};

#[derive(Accounts)]
pub struct Deposit<'info>{
    #[account(mut)]
    pub signer:Signer<'info>,
    #[account(
        mut,
        seeds=[b"staking_pool".as_ref()],
        bump
    )]
    pub staking_pool:Account<'info,StakingPool>,
    #[account(
        mut,
        associated_token::mint=staking_mint,
        associated_token::authority=staking_pool
    )]
    pub staking_vault:InterfaceAccount<'info,TokenAccount>,
    #[account(
        mint::token_program=token_program,
    )]
    pub staking_mint:InterfaceAccount<'info,Mint>,
    #[account(
        init,
        payer=signer,
        space=8+UserStaking::INIT_SPACE,
        seeds=[b"user_staking".as_ref(),signer.key().as_ref()],
        bump
    )]
    pub user_staking:Account<'info,UserStaking>,
    #[account(
        mut,
        associated_token::mint=staking_mint,
        associated_token::authority=signer
    )]
    pub user_token_account:InterfaceAccount<'info,TokenAccount>,
    pub system_program:Program<'info,System>,
    pub token_program:Interface<'info,TokenInterface>,
    pub associated_token_program:Program<'info,AssociatedToken>,
}

impl<'info>Deposit<'info>{
    pub fn deposit(&mut self,amount:u64,bumps:DepositBumps)->Result<()>{
        self.initialize_user_staking(amount, bumps);
        self.deposit_token(amount)
    }
    pub fn initialize_user_staking(&mut self,amount:u64,bumps:DepositBumps)->Result<()>{
        self.user_staking.set_inner(UserStaking{
            user:self.signer.key(),
            staking_pool:self.staking_pool.key(),
            user_token_account:self.user_token_account.key(),
            amount_staked:amount,
            bump:bumps.user_staking
        });
        Ok(())
    }
    pub fn deposit_token(&mut self,amount:u64)->Result<()>{
        let program_id=self.token_program.to_account_info();
        let accounts=Transfer{
            from:self.user_token_account.to_account_info(),
            to:self.staking_vault.to_account_info(),
            authority:self.signer.to_account_info(),    
        };
        let ctx=CpiContext::new(program_id, accounts);
        transfer(ctx, amount)
    }
}