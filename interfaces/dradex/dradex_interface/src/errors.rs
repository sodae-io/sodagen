use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum DradexError {
    #[error("Unknown error")]
    Unknown = 6000,
    #[error("Insufficient liquidity added")]
    InsufficientLiquidityAdded = 6001,
    #[error("Insufficient liquidity removed")]
    InsufficientLiquidityRemoved = 6002,
    #[error("Slab too small")]
    SlabTooSmall = 6003,
    #[error("Slab out of space")]
    SlabOutOfSpace = 6004,
    #[error("Invalid quantity")]
    InvalidQuantity = 6005,
    #[error("Invalid side")]
    InvalidSide = 6006,
    #[error("Order not found")]
    OrderNotFound = 6007,
    #[error("Invalid lot size")]
    InvalidLotSize = 6008,
    #[error("Invalid farm start time")]
    InvalidFarmStartTime = 6009,
    #[error("Invalid farm end time")]
    InvalidFarmEndTime = 6010,
    #[error("Invalid farm duration")]
    InvalidFarmDuration = 6011,
    #[error("Invalid amount")]
    InvalidAmount = 6012,
    #[error("Unauthorize operation")]
    UnauthorizedOperation = 6013,
    #[error("Order book is full, please submit a better offer")]
    OrderbookFull = 6014,
    #[error("Order outdated: booted orders not found")]
    BootedOrdersNotFound = 6015,
    #[error("Invalid fee tier")]
    InvalidFeeTier = 6016,
    #[error("Invalid order type")]
    InvalidOrderType = 6017,
    #[error("Order cancelled due to Post Only condition")]
    PostOnlyOrderCancelled = 6018,
    #[error("Slippage tolerance exceeded")]
    SlippageToleranceExceeded = 6019,
    #[error("Invalid trading pair")]
    InvalidTradingPair = 6020,
    #[error("Invalid input")]
    InvalidInput = 6021,
    #[error("Order book too large")]
    OrderBookTooLarge = 6022,
    #[error("Invalid dex user")]
    InvalidDexUser = 6023,
    #[error("Invalid market user")]
    InvalidMarketUser = 6024,
    #[error("Invalid rebate account")]
    InvalidRebateAccount = 6025,
}
impl From<DradexError> for ProgramError {
    fn from(e: DradexError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
