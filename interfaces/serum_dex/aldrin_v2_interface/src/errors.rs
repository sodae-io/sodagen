use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum AldrinV2Error {}
impl From<AldrinV2Error> for ProgramError {
    fn from(e: AldrinV2Error) -> Self {
        ProgramError::Custom(e as u32)
    }
}
