use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum OrderEngineError {
    #[error("InvalidCalculation")]
    InvalidCalculation = 6000,
    #[error("MissingTemporaryWrappedSolTokenAccount")]
    MissingTemporaryWrappedSolTokenAccount = 6001,
    #[error("Token2022MintExtensionNotSupported")]
    Token2022MintExtensionNotSupported = 6002,
}
impl From<OrderEngineError> for ProgramError {
    fn from(e: OrderEngineError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
