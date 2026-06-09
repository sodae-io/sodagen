use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum OnreappError {
    #[error("The approval message has expired.")]
    Expired = 6000,
    #[error("The approval message is for the wrong program.")]
    WrongProgram = 6001,
    #[error("The approval message is for the wrong user.")]
    WrongUser = 6002,
    #[error("Missing Ed25519 instruction.")]
    MissingEd25519Ix = 6003,
    #[error("The instruction is for the wrong program.")]
    WrongIxProgram = 6004,
    #[error("Ed25519 instruction has accounts.")]
    BadEd25519Accounts = 6005,
    #[error("Malformed Ed25519 instruction.")]
    MalformedEd25519Ix = 6006,
    #[error("Multiple signatures found in Ed25519 instruction.")]
    MultipleSigs = 6007,
    #[error("The authority public key does not match.")]
    WrongAuthority = 6008,
    #[error(
        "The message in the Ed25519 instruction does not match the approval message."
    )]
    MsgMismatch = 6009,
    #[error("Failed to deserialize the approval message.")]
    MsgDeserialize = 6010,
}
impl From<OnreappError> for ProgramError {
    fn from(e: OnreappError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
