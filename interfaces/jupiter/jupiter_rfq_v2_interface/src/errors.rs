use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum JupiterRfqV2Error {
    #[error("Arithmetic overflow")]
    Overflow = 6000,
    #[error("No fill occurred")]
    NoFill = 6001,
    #[error("Orderbook is stale")]
    StaleOrderbook = 6002,
    #[error("Orderbook levels must be sorted best-to-worst for the given side")]
    InvalidLevelOrdering = 6003,
    #[error("Invalid caller")]
    InvalidCaller = 6004,
    #[error("Maker authority appears in other instructions")]
    MakerAppearsInOtherInstruction = 6005,
}
impl From<JupiterRfqV2Error> for ProgramError {
    fn from(e: JupiterRfqV2Error) -> Self {
        ProgramError::Custom(e as u32)
    }
}
