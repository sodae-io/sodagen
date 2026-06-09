use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum VirtualsProgramError {
    #[error("Invalid mint address")]
    InvalidMintAddress = 6000,
    #[error("Invalid mint params")]
    InvalidMintParams = 6001,
    #[error("Curve error")]
    CurveError = 6002,
    #[error("Fee cannot exceed 100%")]
    InvalidFee = 6003,
    #[error("Name too long. Max length = 20")]
    NameTooLong = 6004,
    #[error("Symbol too long. Max length = 10")]
    SymbolTooLong = 6005,
    #[error("URI too long. Max length = 200")]
    UriTooLong = 6006,
    #[error("Amount must be >0")]
    InvalidAmount = 6007,
    #[error("Slippage exceeded")]
    SlippageExceeded = 6008,
    #[error("Invalid vpool state")]
    InvalidVPoolState = 6009,
    #[error("Invalid name")]
    InvalidName = 6010,
    #[error("Invalid symbol")]
    InvalidSymbol = 6011,
    #[error("Invalid URI")]
    InvalidUri = 6012,
}
impl From<VirtualsProgramError> for ProgramError {
    fn from(e: VirtualsProgramError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
