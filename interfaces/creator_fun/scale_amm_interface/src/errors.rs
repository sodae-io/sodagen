use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum ScaleAmmError {
    #[error("Pool Disabled")]
    PoolDisabled = 6000,
    #[error("Owner must be a system account (wallet), not a PDA")]
    OwnerMustBeSystemAccount = 6001,
    #[error("Invalid Fee Decay")]
    InvalidFeeDecay = 6002,
    #[error("Initial token reserves must be greater than 0")]
    InvalidInitialTokenReserves = 6003,
    #[error("Invalid Mint")]
    InvalidMint = 6004,
    #[error("Invalid Token Account")]
    InvalidTokenAccount = 6005,
    #[error("Insufficient Funds")]
    InsufficientFunds = 6006,
    #[error("Shift must be greater than 0")]
    InvalidShift = 6007,
    #[error("Fees must be between 0 and 10000 basis points")]
    InvalidFees = 6008,
    #[error("MathOverflow")]
    MathOverflow = 6009,
    #[error("Insufficient output")]
    InsufficientOutput = 6010,
    #[error("Insufficient input")]
    InsufficientInput = 6011,
    #[error("Illegal Claimant")]
    IllegalClaimant = 6012,
    #[error("Pool Empty")]
    PoolEmpty = 6013,
    #[error("Pool base token is restricted by platform config")]
    InvalidPlatformBaseToken = 6014,
    #[error("Invalid platform fee")]
    InvalidPlatformFee = 6015,
    #[error("Invalid fee beneficiary")]
    InvalidBeneficiary = 6016,
    #[error("Missing beneficiary accounts")]
    MissingBeneficiaryAccount = 6017,
    #[error("Unauthorized program authority")]
    UnauthorizedAuthority = 6018,
    #[error("Swapper not allowed during protected window")]
    SnipingProtection = 6019,
}
impl From<ScaleAmmError> for ProgramError {
    fn from(e: ScaleAmmError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
