use anchor_lang::prelude::*;
use anchor_spl::{
    token_interface::{self, Mint, MintTo, TokenAccount, TokenInterface},
};
use anchor_spl::associated_token::AssociatedToken;
use crate::state::Vault;
use crate::errors::StakeError;

#[derive(Accounts)]
pub struct ClaimReward<'info>{
    #[account(mut)]
    pub user:Signer<'info>,
    pub mint:InterfaceAccount<'info,Mint>,
    #[account(
        init_if_needed,
        payer=user,
        associated_token::mint = mint,
        associated_token::authority = user,
        associated_token::token_program = token_program,
    )]
    pub token_account:InterfaceAccount<'info,TokenAccount>,
    #[account(
        mut,
        seeds=[b"staking",user.key().as_ref()],
        bump=vault.bump
    )]
    pub vault:Account<'info,Vault>,
    pub mint_authority:SystemAccount<'info>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}


impl <'info> ClaimReward<'info>{
    pub fn claim_rewards(&mut self)->Result<()>{
        let vault =&mut self.vault;
        let vault_amount=vault.get_lamports();
        let points=vault.update_points(vault_amount)?;
        
        require!(points>0,StakeError::NoPoints);

        msg!("Rewards :{}",points);
        let cpi_accounts = MintTo{
            mint:self.mint.to_account_info(),
            to:self.token_account.to_account_info(),
            authority:self.mint_authority.to_account_info()
        };

        let cpi_program_id = self.token_program.to_account_info();
        let cpi_context = CpiContext::new(cpi_program_id, cpi_accounts);
        token_interface::mint_to(cpi_context, points)?;
        vault.total_points=0;

        let clock = Clock::get()?;
        vault.timestamp=clock.unix_timestamp;
        Ok(())
    }
}

pub fn handler(ctx:Context<ClaimReward>)->Result<()>{
    ctx.accounts.claim_rewards()?;
    Ok(())
}