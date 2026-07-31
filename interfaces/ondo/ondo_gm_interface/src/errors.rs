use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum OndoGmError {
    #[error("Missing or mismatched secp256k1 verification instruction")]
    MissingOrMismatchedSecpIx = 6000,
    #[error("Malformed secp256k1 instruction")]
    MalformedSecpIx = 6001,
    #[error("Wrong signature count")]
    WrongSigCount = 6002,
    #[error("Expected 32-byte hash")]
    WrongDigestLen = 6003,
    #[error("Digest mismatch")]
    DigestMismatch = 6004,
    #[error("Recovered address mismatch")]
    AddressMismatch = 6005,
}
impl From<OndoGmError> for ProgramError {
    fn from(e: OndoGmError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
