use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum AddDecimalsError {
    #[error("Wrapper underlying tokens account must be empty.")]
    InitNonEmptyAccount = 300,
    #[error("Supply of the wrapper mint is non-zero")]
    InitWrapperSupplyNonZero = 301,
    #[error("Owner of the wrapper underlying tokens account must be the wrapper")]
    InitWrapperUnderlyingOwnerMismatch = 302,
    #[error("Underlying mint does not match underlying tokens account mint")]
    InitWrapperUnderlyingMintMismatch = 303,
    #[error("Mint authority mismatch")]
    InitMintAuthorityMismatch = 304,
    #[error("Initial decimals too high")]
    InitMultiplierOverflow = 305,
    #[error(
        "The number of target decimals must be greater than or equal to the underlying asset's decimals."
    )]
    InitWrapperDecimalsTooLow = 306,
    #[error(
        "Mint amount overflow. This error happens when the token cannot support this many decimals added to the token."
    )]
    MintAmountOverflow = 307,
    #[error("Failed to convert burn amount from withdraw amount.")]
    InvalidBurnAmount = 308,
    #[error("Failed to convert withdraw amount from wrapped amount.")]
    InvalidWithdrawAmount = 309,
    #[error("User does not have enough underlying tokens")]
    InsufficientUnderlyingBalance = 310,
    #[error("User does not have enough wrapped tokens")]
    InsufficientWrappedBalance = 311,
    #[error("Cannot send zero tokens")]
    ZeroAmount = 312,
    #[error("Unknown router action")]
    UnknownAction = 313,
}
impl From<AddDecimalsError> for ProgramError {
    fn from(e: AddDecimalsError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
