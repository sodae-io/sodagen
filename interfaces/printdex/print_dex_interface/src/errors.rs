use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum PrintDexError {
    #[error("The dex is currently shut down")]
    DexDisabled = 6000,
    #[error("Slippage exceeded the maximum allowed amount")]
    SlippageExceeded = 6001,
    #[error("Both amounts must be greater than 0")]
    InvalidAmount = 6002,
    #[error("The pool is already initialized")]
    PoolAlreadyInitialized = 6003,
    #[error("The pool does not exist")]
    PoolDoesntExist = 6004,
    #[error("Buffer Overflow or Undeflow Error Occurred")]
    OverflowError = 6005,
    #[error("No liquidity would have been minted")]
    NoLiquidityMinted = 6006,
    #[error("No liquidity would have been burned")]
    NoLiquidityBurned = 6007,
}
impl From<PrintDexError> for ProgramError {
    fn from(e: PrintDexError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
