use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum AldrinError {}
impl From<AldrinError> for ProgramError {
    fn from(e: AldrinError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
