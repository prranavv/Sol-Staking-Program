use anchor_lang::prelude::{program::invoke, system_instruction::transfer, *};
use crate::{errors::StakeError, state::Vault};

#[derive(Accounts)]
pub struct Stake<'info>{
    #[account(mut)]
    pub user:Signer<'info>,
    #[account(
        mut,
        seeds=[b"staking",user.key().as_ref()],
        bump = vault.bump
    )]
    pub vault:Account<'info,Vault>,
    pub system_program:Program<'info,System>
}

impl <'info> Stake<'info>{
    pub fn stake(&mut self,amount:u64)->Result<()>{
        require!(amount>0,StakeError::AmountGTZero);

        let vault_amount=self.vault.to_account_info().lamports();
        let vault = &mut self.vault;

        vault.update_points(vault_amount)?; 
        //Transfer the lamports from user to the vault
        let instruction = transfer(&self.user.key(), &self.vault.key(), amount);
        let accounts = [self.user.to_account_info(),self.vault.to_account_info(),self.system_program.to_account_info()];

        invoke(&instruction, &accounts)?;
        
        Ok(())
    }
}

pub fn handler(ctx:Context<Stake>,amount:u64)->Result<()>{
    ctx.accounts.stake(amount)?;
    Ok(())
}