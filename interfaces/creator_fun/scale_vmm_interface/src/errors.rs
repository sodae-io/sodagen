use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum ScaleVmmError {
    #[error("Pair Disabled")]
    PairDisabled = 6000,
    #[error("Pair already graduated")]
    PairGraduated = 6001,
    #[error("Invalid Mint")]
    InvalidMint = 6002,
    #[error("Invalid Token Account")]
    InvalidTokenAccount = 6003,
    #[error("Initial token reserves must be greater than 0")]
    InvalidInitialTokenReserves = 6004,
    #[error("Shift must be greater than 0")]
    InvalidShift = 6005,
    #[error("Fees must be between 0 and 10000 basis points")]
    InvalidFees = 6006,
    #[error("Math overflow")]
    MathOverflow = 6007,
    #[error("Insufficient output")]
    InsufficientOutput = 6008,
    #[error("Insufficient input")]
    InsufficientInput = 6009,
    #[error("Pool Empty")]
    PoolEmpty = 6010,
    #[error("Invalid curve type")]
    InvalidCurve = 6011,
    #[error("Invalid fee beneficiary")]
    InvalidBeneficiary = 6012,
    #[error("Missing beneficiary accounts")]
    MissingBeneficiaryAccount = 6013,
    #[error("Unauthorized authority")]
    UnauthorizedAuthority = 6014,
    #[error("Pair base token is restricted by platform config")]
    InvalidPlatformBaseToken = 6015,
    #[error("Invalid platform fee")]
    InvalidPlatformFee = 6016,
    #[error("Invalid graduation threshold")]
    InvalidGraduationThreshold = 6017,
}
impl From<ScaleVmmError> for ProgramError {
    fn from(e: ScaleVmmError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
