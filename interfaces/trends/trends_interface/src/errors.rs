use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum TrendsError {
    #[error("Invalid zero amount")]
    InvalidZeroAmount = 6000,
    #[error("Math overflow / underflow")]
    MathErr = 6001,
    #[error("Insufficient output amount")]
    InsufficientOutputAmount = 6002,
    #[error("Invalid input token mint")]
    InvalidInputTokenMint = 6003,
    #[error("Invalid output token mint")]
    InvalidOutputTokenMint = 6004,
    #[error("Invalid quote mint")]
    InvalidQuoteMint = 6005,
    #[error("Invariant violation: K")]
    InvariantViolation = 6006,
    #[error("Pool has reached migration threshold, no more swaps allowed")]
    PoolCompleted = 6007,
    #[error("Pool has not reached migration threshold yet")]
    MigrationNotReady = 6008,
    #[error("Pool has already been migrated")]
    AlreadyMigrated = 6009,
    #[error("AMM config is not in the whitelist")]
    InvalidAmmConfig = 6010,
    #[error("Invalid destination")]
    InvalidDestination = 6011,
}
impl From<TrendsError> for ProgramError {
    fn from(e: TrendsError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
