use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum GuacswapError {
    #[error("Delta greater than provider's tokens")]
    DeltaTooBig = 6000,
    #[error("Token amount underflow")]
    TokenUnderflow = 6001,
    #[error("Wrong tokens ratio")]
    WrongRatio = 6002,
    #[error("Too much shares provided")]
    TooMuchShares = 6003,
    #[error("Swap too big")]
    SwapToBig = 6004,
    #[error("Fee exceeded 100%")]
    FeeExceeded = 6005,
    #[error("Scales have to be equal")]
    ScalesNotEqual = 6007,
    #[error("Fees exceeded delta_out")]
    FeeExceededDeltaOut = 6008,
    #[error("Price limit exceeded")]
    PriceLimitExceeded = 6009,
    #[error("Mint mismatch")]
    MintMismatch = 6010,
    #[error("Tokens are the same")]
    TokensAreTheSame = 6011,
    #[error("Cannot add supply to wrong farm")]
    WrongFarm = 6012,
    #[error("Cannot withdraw rewards exceeding supply left")]
    RewardsExceedingSupply = 6013,
    #[error("Farm has not ended, cannot add additional rewards")]
    FarmNotEnded = 6014,
    #[error("Must provide a nonzero amount")]
    ZeroAmount = 6015,
    #[error("Invariant has changed")]
    InvariantDecreased = 6016,
}
impl From<GuacswapError> for ProgramError {
    fn from(e: GuacswapError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
