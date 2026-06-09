use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum MastermindError {
    #[error("Unauthorized: Incorrect signer or program ID")]
    Unauthorized = 6000,
    #[error("State not initialized")]
    StateNotInitialized = 6001,
    #[error("Initial authority pubkey cannot be zero")]
    ZeroAuthority = 6002,
    #[error("Initial pSOL pubkey cannot be zero")]
    ZeroPsol = 6003,
    #[error("Initial fee receiver pubkey cannot be zero")]
    ZeroFeeReceiver = 6004,
    #[error("Invalid fee basis points")]
    InvalidFeeBps = 6005,
    #[error("Invalid virtual reserves")]
    InvalidVirtualReserves = 6006,
    #[error("Invalid curve intialization params")]
    InvalidCurveIntialization = 6007,
    #[error("Curve is not completed")]
    CurveNotCompleted = 6008,
    #[error("Curve SOL must be wrapped")]
    CurveSolNotWrapped = 6009,
    #[error("Curve is completed")]
    CurveAlreadyCompleted = 6010,
    #[error("Curve is time locked or migrated")]
    CurveTimeLockedOrMigrated = 6011,
    #[error("Curve is already migrated")]
    CurveAlreadyMigrated = 6012,
    #[error("Token output exceed limit")]
    WrongTokenOutput = 6013,
    #[error("K")]
    InvariantKMismatch = 6014,
    #[error("Slippage exceeded")]
    SlippageExceeded = 6015,
}
impl From<MastermindError> for ProgramError {
    fn from(e: MastermindError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
