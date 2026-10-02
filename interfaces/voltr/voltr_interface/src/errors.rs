use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum VoltrError {
    #[error("Invalid amount provided.")]
    InvalidAmount = 6000,
    #[error("Invalid token mint.")]
    InvalidTokenMint = 6001,
    #[error("Invalid token account.")]
    InvalidTokenAccount = 6002,
    #[error("Invalid account input.")]
    InvalidAccountInput = 6003,
    #[error("Math overflow.")]
    MathOverflow = 6004,
    #[error("Fee exceeds total asset value.")]
    FeeExceedsTotalAssetValue = 6005,
    #[error("Max cap exceeded.")]
    MaxCapExceeded = 6006,
    #[error("Vault not active.")]
    VaultNotActive = 6007,
    #[error("Manager not allowed in remaining.")]
    ManagerNotAllowed = 6008,
    #[error("Operation not allowed.")]
    OperationNotAllowed = 6009,
    #[error("Adaptor epoch invalid.")]
    AdaptorEpochInvalid = 6010,
    #[error("Fee configuration invalid.")]
    InvalidFeeConfiguration = 6011,
    #[error("Withdrawal not yet available.")]
    WithdrawalNotYetAvailable = 6012,
    #[error("Invalid input.")]
    InvalidInput = 6013,
    #[error("Division by zero.")]
    DivisionByZero = 6014,
    #[error("Instant withdraw not allowed.")]
    InstantWithdrawNotAllowed = 6015,
    #[error("Adaptor program not whitelisted.")]
    AdaptorProgramNotWhitelisted = 6016,
    #[error("Invalid adaptor policy.")]
    InvalidAdaptorPolicy = 6017,
}
impl From<VoltrError> for ProgramError {
    fn from(e: VoltrError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
