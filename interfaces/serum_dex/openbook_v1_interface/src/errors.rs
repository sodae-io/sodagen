use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum OpenbookV1Error {}
impl From<OpenbookV1Error> for ProgramError {
    fn from(e: OpenbookV1Error) -> Self {
        ProgramError::Custom(e as u32)
    }
}
