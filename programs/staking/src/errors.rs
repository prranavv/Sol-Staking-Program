use anchor_lang::prelude::*;

#[error_code]
pub enum StakeError{
    #[msg("Overflow error")]
    StakeOverflow,
    #[msg("Amount should be greater than 0")]
    AmountGTZero,
    #[msg("Amount should be available in the vault")]
    AmountLTVault,
    #[msg("There is 0 points")]
    NoPoints
}