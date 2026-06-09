use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum EndoavsProgramError {
    #[error("Unsupported Asset")]
    UnsupportedAsset = 6000,
    #[error("The chosen name is too long")]
    NameTooLong = 6001,
    #[error("Wrong fee recipient address")]
    CreateFeeRecipientMismatch = 6002,
    #[error("The chosen url is too long")]
    UrlTooLong = 6003,
    #[error("One of the constraints is violated")]
    ConstraintViolation = 6004,
    #[error("The token symbol is invalid. It has to end with sSOL")]
    InvalidTokenSymbol = 6005,
}
impl From<EndoavsProgramError> for ProgramError {
    fn from(e: EndoavsProgramError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
