use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum GfxSslV1Error {
    #[error("[G100] The pool is suspended")]
    Suspended = 6000,
    #[error("[G101] Not admin")]
    NotAdmin = 6001,
    #[error("[G102] Mints are not sorted")]
    MintsNotSorted = 6002,
    #[error("[G103] The risk token mint is wrong")]
    WrongRtMint = 6003,
    #[error("[G104] The required oracle is not present")]
    OracleNotPresent = 6004,
    #[error("[G105] The oracle is not in a healthy state (status)")]
    OracleNotHealthyStatus = 6005,
    #[error("[G106] The oracle is not in a healthy state (delay)")]
    OracleNotHealthyDelay = 6006,
    #[error("[G107] The oracle is not in a healthy state (confidence)")]
    OracleNotHealthyConfidence = 6007,
    #[error("[G108] SlippageTooLarge")]
    SlippageTooLarge = 6008,
    #[error("[G109] Percentage out of range")]
    PercentageOutOfRange = 6009,
    #[error("[G110] Swap instruction is not executed in order")]
    SwapIxNotInOrder = 6010,
    #[error("[G111] Mint does not match the pair")]
    MintNotMatchPair = 6011,
    #[error("[G112] Fee collector account incorrect")]
    FeeCollectorIncorrect = 6012,
}
impl From<GfxSslV1Error> for ProgramError {
    fn from(e: GfxSslV1Error) -> Self {
        ProgramError::Custom(e as u32)
    }
}
