use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum ZapError {
    #[error("Math operation overflow")]
    MathOverflow = 6000,
    #[error("Invalid offset")]
    InvalidOffset = 6001,
    #[error("Invalid zapout parameters")]
    InvalidZapOutParameters = 6002,
    #[error("Type cast error")]
    TypeCastFailed = 6003,
    #[error("Amm program is not supported")]
    AmmIsNotSupported = 6004,
    #[error("Position is not empty")]
    InvalidPosition = 6005,
    #[error("Exceeded slippage tolerance")]
    ExceededSlippage = 6006,
    #[error("Invalid dlmm zap in parameters")]
    InvalidDlmmZapInParameters = 6007,
    #[error("Unsupported fee mode")]
    UnsupportedFeeMode = 6008,
}
impl From<ZapError> for ProgramError {
    fn from(e: ZapError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
