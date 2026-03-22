use anchor_lang::prelude::*;
use anchor_spl::associated_token::spl_associated_token_account::solana_program::native_token::LAMPORTS_PER_SOL;
use crate::errors::StakeError;
use crate::constants::*;


#[derive(InitSpace)]
#[account]
pub struct Vault{
    pub bump:u8,
    pub timestamp:i64,
    pub total_points:u64,
    pub stake_amount:u64
}


impl Vault{
    pub fn update_points(&mut self,vault_amount:u64)->Result<u64>{
        let initial_timestamp = self.timestamp;
        let current_timestamp = Clock::get()?.unix_timestamp;
        let seconds_passed=current_timestamp.checked_sub(initial_timestamp).ok_or(StakeError::StakeOverflow)?;
        let points_accumulated = seconds_passed.checked_mul(POINTS_PER_SECOND)
                                                    .ok_or(StakeError::StakeOverflow)?
                                                    .checked_mul(vault_amount as i64)
                                                    .ok_or(StakeError::StakeOverflow)?
                                                    .checked_div(POINTS_PER_REWARD_TOKEN)
                                                    .ok_or(StakeError::StakeOverflow)?
                                                    .checked_div(LAMPORTS_PER_SOL as i64)
                                                    .ok_or(StakeError::StakeOverflow)?;
        self.total_points=self.total_points.checked_add(points_accumulated as u64).ok_or(StakeError::StakeOverflow)?;
        self.timestamp=current_timestamp;
        Ok(self.total_points)
    }
}