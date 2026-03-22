use anchor_lang::prelude::*;
use crate::state::Vault;

#[derive(Accounts)]
pub struct CreateStakeAccount<'info>{
    #[account(mut)]
    pub user:Signer<'info>,
    #[account(
        init,
        payer=user,
        space=8+Vault::INIT_SPACE,
        seeds=[b"staking",user.key().as_ref()],
        bump
    )]
    pub vault:Account<'info,Vault>,
    pub system_program:Program<'info,System>
}

impl<'info> CreateStakeAccount<'info>{
    pub fn create_stake_account(&mut self,bump:u8)->Result<()>{
        let time=Clock::get()?;
        let time_stamp = time.unix_timestamp;
        self.vault.set_inner(Vault { bump ,timestamp:time_stamp, total_points:0,stake_amount:0});   
        Ok(())
    }
}

pub fn handler(ctx:Context<CreateStakeAccount>,bump:u8)->Result<()>{
    ctx.accounts.create_stake_account(bump)?;
    Ok(())
}