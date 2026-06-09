use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum DumpfunError {
    #[error("Insufficient balance")]
    InsufficientBalance = 6000,
    #[error("Insufficient output amount")]
    InsufficientOutput = 6001,
    #[error("Invalid sell lock period")]
    InvalidSellLockPeriod = 6002,
    #[error("Ramp limit exceeded")]
    RampLimitExceeded = 6003,
    #[error("Excessive input amount")]
    ExcessiveInputAmount = 6004,
    #[error("Sell not unlocked")]
    SellNotUnlocked = 6005,
    #[error("Too many ramping limits (max 4 allowed)")]
    TooManyRampingLimits = 6006,
    #[error(
        "Invalid ramping limit time (start_sec must be >= 0 and end_sec > start_sec)"
    )]
    InvalidRampingLimitTime = 6007,
    #[error("Invalid ramping limit bps (must be > 0 and <= 10000)")]
    InvalidRampingLimitBps = 6008,
    #[error("Overlapping ramping limit periods")]
    OverlappingRampingLimits = 6009,
    #[error(
        "Excessive ramping limit time (total time must be <= 1/5 of sell unlock time)"
    )]
    ExcessiveRampingLimitTime = 6010,
    #[error("Ramping limit exceeds unlock time")]
    RampingLimitExceedsUnlockTime = 6011,
    #[error("Invalid virtual SOL reserve")]
    InvalidVirtualSolReserve = 6012,
    #[error("Invalid fee rate")]
    InvalidFeeRate = 6013,
    #[error("Math overflow")]
    MathOverflow = 6014,
    #[error("Invalid token account")]
    InvalidTokenAccount = 6015,
    #[error("Reentrancy")]
    Reentrancy = 6016,
    #[error("Liquidity pool closed")]
    PoolClosed = 6017,
}
impl From<DumpfunError> for ProgramError {
    fn from(e: DumpfunError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
