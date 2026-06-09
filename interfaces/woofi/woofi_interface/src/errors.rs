use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum WoofiError {
    #[error("Unable to divide by zero")]
    DivideByZero = 6000,
    #[error("Unable to cast number into BigInt")]
    NumberCastError = 6001,
    #[error("Exceeded max fee rate")]
    FeeRateMaxExceeded = 6002,
    #[error("Mathematical operation with overflow")]
    MathOverflow = 6003,
    #[error("Muldiv overflow")]
    MulDivOverflow = 6004,
    #[error("Exceeded max protocol fee")]
    ProtocolFeeMaxExceeded = 6005,
    #[error("Protocol fee not enough")]
    ProtocolFeeNotEnough = 6006,
    #[error("Exceeded max rebate fee")]
    RebateFeeMaxExceeded = 6007,
    #[error("Rebate fee not enough")]
    RebateFeeNotEnough = 6008,
    #[error("Exceeded max reserve")]
    ReserveMaxExceeded = 6009,
    #[error("Reserve not enough")]
    ReserveNotEnough = 6010,
    #[error("Reserve less than fee")]
    ReserveLessThanFee = 6011,
    #[error("Too Many Authorities")]
    TooManyAuthorities = 6012,
    #[error("Woo oracle bound exceed limit")]
    WooOracleBoundLimit = 6013,
    #[error("Woo oracle is not feasible")]
    WooOracleNotFeasible = 6014,
    #[error("Woo oracle price is not valid")]
    WooOraclePriceNotValid = 6015,
    #[error("Woo oracle price below range MIN")]
    WooOraclePriceRangeMin = 6016,
    #[error("Woo oracle price exceed range MAX")]
    WooOraclePriceRangeMax = 6017,
    #[error("Woo oracle spread exceed 1E18")]
    WooOracleSpreadExceed = 6018,
    #[error("Woo pp exceed max notional value")]
    WooPoolExceedMaxNotionalValue = 6019,
    #[error("Woo pp exceed max gamma")]
    WooPoolExceedMaxGamma = 6020,
    #[error("Src Balance < LP Deposit Amount.")]
    NotEnoughBalance = 6021,
    #[error("Not enough out")]
    NotEnoughOut = 6022,
    #[error("Amount out below minimum threshold")]
    AmountOutBelowMinimum = 6023,
    #[error("Amount exceeds max balance cap")]
    BalanceCapExceeds = 6024,
    #[error("Swap pool invalid")]
    SwapPoolInvalid = 6025,
    #[error("invalid authority")]
    InvalidAuthority = 6026,
}
impl From<WoofiError> for ProgramError {
    fn from(e: WoofiError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
