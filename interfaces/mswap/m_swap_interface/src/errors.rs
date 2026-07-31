use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum MSwapError {
    #[error("Extension is not whitelisted")]
    InvalidExtension = 6000,
    #[error("Extension is already whitelisted")]
    AlreadyWhitelisted = 6001,
    #[error("Index invalid for length of the array")]
    InvalidIndex = 6002,
    #[error("Signer is not whitelisted")]
    UnauthorizedUnwrapper = 6003,
    #[error("Signer is not authorized to perform this action")]
    NotAuthorized = 6004,
    #[error("Invalid amount")]
    InvalidAmount = 6005,
}
impl From<MSwapError> for ProgramError {
    fn from(e: MSwapError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
