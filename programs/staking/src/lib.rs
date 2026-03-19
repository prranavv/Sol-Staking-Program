use anchor_lang::prelude::*;
pub mod instructions;
pub mod state;
pub mod constants;
pub mod errors;

pub use instructions::*;

declare_id!("5RqZov6TkQYw4ASzf84WPdMqSrjnc53ZfzysNHm9HDJh");

#[program]
pub mod staking {
    use super::*;

    pub fn create_stake_account(ctx:Context<CreateStakeAccount>)->Result<()>{
        let bump=ctx.bumps.vault;
        create_stake_account::handler(ctx, bump)?;
        Ok(())
    }
    pub fn stake(ctx: Context<Stake>,amount:u64) -> Result<()> {
        stake::handler(ctx,amount)?;
        Ok(())
    }

    pub fn unstake(ctx:Context<Unstake>,amount:u64)->Result<()>{
        unstake::handler(ctx, amount)?;
        Ok(())
    }

    pub fn claim_rewards(ctx:Context<ClaimReward>)->Result<()>{
        claim_rewards::handler(ctx)?;
        Ok(())
    }

    pub fn create_mint(ctx:Context<CreateMint>)->Result<()>{
        create_mint::handler(ctx)?;
        Ok(())
    }
}