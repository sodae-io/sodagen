use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum TreasuryManagementError {
    #[error("Treasury management is currently frozen")]
    Frozen = 6000,
    #[error("Error in arithmetic")]
    ArithmeticError = 6001,
}
impl From<TreasuryManagementError> for ProgramError {
    fn from(e: TreasuryManagementError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
