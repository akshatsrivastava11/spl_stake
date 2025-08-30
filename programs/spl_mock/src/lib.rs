use anchor_lang::prelude::*;
pub mod state;
pub use state::*;
pub mod instructions;
pub use instructions::*;
declare_id!("AJNgNbxzPSsuCkJjSp5TyKVmfGc6v3TWr5288Tu4cFiy");

#[program]
pub mod spl_mock {




    use super::*;

    pub fn initialize_staking_pool(ctx:Context<InitializeStakingPool>)->Result<()>{
        ctx.accounts.initialize_staking_pool(ctx.bumps);
        Ok(())
    }
    pub fn deposit(ctx:Context<Deposit>,amount:u64)->Result<()>{
        ctx.accounts.deposit(amount, ctx.bumps);
        Ok(())
    }
    pub fn withdraw(ctx:Context<Withdraw>)->Result<()>{
        ctx.accounts.withdraw();
        Ok(())
    }

}
