use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};
use crate::constants::MINT_DECIMALS;

#[derive(Accounts)]
pub struct CreateMint<'info>{
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        mint::decimals = MINT_DECIMALS,
        mint::authority = signer.key(),
        mint::freeze_authority = signer.key(),
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}


impl <'info> CreateMint<'info>{
    pub fn create_mint(&self)->Result<()>{
        msg!("Mint Address: {}",self.mint.key());   
        Ok(())
    }
}

pub fn handler(ctx:Context<CreateMint>)->Result<()>{
    ctx.accounts.create_mint()?;
    Ok(())
}