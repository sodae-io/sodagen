use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum InvariantError {
    #[error("Amount is zero")]
    ZeroAmount = 6000,
    #[error("Output would be zero")]
    ZeroOutput = 6001,
    #[error("Not the expected tick")]
    WrongTick = 6002,
    #[error("Price limit is on the wrong side of price")]
    WrongLimit = 6003,
    #[error("Tick index not divisible by spacing or over limit")]
    InvalidTickIndex = 6004,
    #[error("Invalid tick_lower or tick_upper")]
    InvalidTickInterval = 6005,
    #[error("There is no more tick in that direction")]
    NoMoreTicks = 6006,
    #[error("Correct tick not found in context")]
    TickNotFound = 6007,
    #[error("Price would cross swap limit")]
    PriceLimitReached = 6008,
    #[error("Invalid tick liquidity")]
    InvalidTickLiquidity = 6009,
    #[error("Disable empty position pokes")]
    EmptyPositionPokes = 6010,
    #[error("Invalid tick liquidity")]
    InvalidPositionLiquidity = 6011,
    #[error("Invalid pool liquidity")]
    InvalidPoolLiquidity = 6012,
    #[error("Invalid position index")]
    InvalidPositionIndex = 6013,
    #[error("Position liquidity would be zero")]
    PositionWithoutLiquidity = 6014,
    #[error("You are not admin")]
    Unauthorized = 6015,
    #[error("Invalid pool token addresses")]
    InvalidPoolTokenAddresses = 6016,
    #[error("Time cannot be negative")]
    NegativeTime = 6017,
    #[error("Oracle is already initialized")]
    OracleAlreadyInitialized = 6018,
    #[error("Absolute price limit was reached")]
    LimitReached = 6019,
    #[error("Invalid protocol fee")]
    InvalidProtocolFee = 6020,
    #[error("Swap amount out is 0")]
    NoGainSwap = 6021,
    #[error("Provided token account is different than expected")]
    InvalidTokenAccount = 6022,
    #[error("Admin address is different than expected")]
    InvalidAdmin = 6023,
    #[error("Provided authority is different than expected")]
    InvalidAuthority = 6024,
    #[error("Provided token owner is different than expected")]
    InvalidOwner = 6025,
    #[error("Provided token account mint is different than expected mint token")]
    InvalidMint = 6026,
    #[error("Provided tickmap is different than expected")]
    InvalidTickmap = 6027,
    #[error("Provided tickmap owner is different than program ID")]
    InvalidTickmapOwner = 6028,
    #[error("Recipient list address and owner list address should be different")]
    InvalidListOwner = 6029,
    #[error("Invalid tick spacing")]
    InvalidTickSpacing = 6030,
}
impl From<InvariantError> for ProgramError {
    fn from(e: InvariantError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
