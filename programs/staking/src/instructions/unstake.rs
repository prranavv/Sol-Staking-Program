use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer, transfer};
use crate::errors::StakeError;
use crate::state::Vault;


#[derive(Accounts)]
pub struct Unstake<'info>{
    #[account(mut)]
    pub user:Signer<'info>,
    #[account(
        mut,
        seeds=[b"staking",user.key().as_ref()],
        bump=vault.bump
    )]
    pub vault:Account<'info,Vault>,
    pub system_program:Program<'info,System>
}

impl<'info> Unstake<'info>{
    pub fn unstake(&mut self,amount:u64)->Result<()>{
        require!(amount>0,StakeError::AmountGTZero);
        require!(amount<=self.vault.stake_amount,StakeError::AmountLTVault);
        
        let signer_seeds: &[&[&[u8]]]=&[&[b"staking",self.user.key.as_ref(),&[self.vault.bump]]];

        let vault = &mut self.vault;
        let vault_amount=vault.stake_amount;
        vault.update_points(vault_amount)?;
        let accounts = Transfer{
            from:vault.to_account_info(),
            to:self.user.to_account_info()
        };

        vault.stake_amount=vault.stake_amount.checked_sub(vault_amount).unwrap();
        let cpi_context  =CpiContext::new(self.system_program.to_account_info(), accounts).with_signer(signer_seeds);
        transfer(cpi_context, amount)?;

        Ok(())
    }
}

pub fn handler(ctx:Context<Unstake>,amount:u64)->Result<()>{
    ctx.accounts.unstake(amount)?;
    Ok(())
}