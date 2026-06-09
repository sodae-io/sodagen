use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum DynamicFeeSharingError {
    #[error("Math operation overflow")]
    MathOverflow = 6000,
    #[error("Mint is not supported")]
    InvalidMint = 6001,
    #[error("Fee vault parameters are invalid")]
    InvalidFeeVaultParameters = 6002,
    #[error("Amount is zero")]
    AmountIsZero = 6003,
    #[error("Invalid user index")]
    InvalidUserIndex = 6004,
    #[error("Invalid user address")]
    InvalidUserAddress = 6005,
    #[error("Exceeded number of users allowed")]
    ExceededUser = 6006,
    #[error("Invalid fee vault")]
    InvalidFeeVault = 6007,
    #[error("Invalid signer")]
    InvalidSigner = 6008,
    #[error("Invalid action")]
    InvalidAction = 6009,
}
impl From<DynamicFeeSharingError> for ProgramError {
    fn from(e: DynamicFeeSharingError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
