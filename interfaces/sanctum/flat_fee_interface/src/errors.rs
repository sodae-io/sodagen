use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum FlatFeeError {
    #[error("Invalid program state data")]
    InvalidProgramStateData = 0,
    #[error("Incorrect program state account")]
    IncorrectProgramState = 1,
    #[error("FeeAccount is not initialized for the given LST mint")]
    UnsupportedLstMint = 2,
    #[error("Given signed fee value is out of bound")]
    SignedFeeOutOfBound = 3,
    #[error("Given unsigned fee value is out of bound")]
    UnsignedFeeOutOfBound = 4,
    #[error("Math error")]
    MathError = 5,
}
impl From<FlatFeeError> for ProgramError {
    fn from(e: FlatFeeError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
