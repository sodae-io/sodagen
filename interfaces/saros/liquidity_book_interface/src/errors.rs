use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum LiquidityBookError {
    #[error("Bin not found within bin array")]
    BinNotFound = 6000,
    #[error("Invalid authority")]
    InvalidAuthority = 6001,
    #[error("Invalid amounts length")]
    InvalidAmounts = 6002,
    #[error("Bin array index mismatch")]
    BinArrayIndexMismatch = 6003,
    #[error("Invalid amount out")]
    InvalidAmountOut = 6004,
    #[error("Invalid amount in")]
    InvalidAmountIn = 6005,
    #[error("Invalid distribution")]
    InvalidDistribution = 6006,
    #[error("Liquidity overflow")]
    LiquidityOverflow = 6007,
    #[error("Liquidity underflow")]
    LiquidityUnderflow = 6008,
    #[error("Zero shares")]
    ZeroShares = 6009,
    #[error("Not the owner of the position")]
    NotPositionOwner = 6010,
    #[error("Invalid static fee parameters")]
    InvalidStaticFeeParameters = 6011,
    #[error("Invalid LB config provided")]
    InvalidConfig = 6012,
    #[error("Pair and position mismatch")]
    PairPositionMismatch = 6013,
    #[error("Pair and lower bin array mismatch")]
    PairLowerBinArrayMismatch = 6014,
    #[error("Pair and upper bin array mismatch")]
    PairUpperBinArrayMismatch = 6015,
    #[error("Inactive bin step config")]
    InactiveBinStepConfig = 6016,
    #[error("Closed bin step config")]
    ClosedBinStepConfig = 6017,
    #[error("Invalid quote asset badge provided")]
    InvalidQuoteAssetBadge = 6018,
    #[error("Transfer fee calculation error")]
    TransferFeeCalculationError = 6019,
    #[error("Get amount overflow error")]
    GetAmountOverflow = 6020,
    #[error("Amount overflow error")]
    AmountOverflow = 6021,
    #[error("Amount underflow error")]
    AmountUnderflow = 6022,
    #[error("Active id overflow error")]
    ActiveIdOverflow = 6023,
    #[error("Active id overflow error")]
    ActiveIdUnderflow = 6024,
    #[error("Invalid bin range")]
    InvalidBinRange = 6025,
    #[error("Pair Token Mismatch")]
    PairTokenMismatch = 6026,
    #[error("Invalid Hook Provided")]
    InvalidHook = 6027,
    #[error("Token Account X Mismatch")]
    UserVaultXMismatch = 6028,
    #[error("Token Account Y Mismatch")]
    UserVaultYMismatch = 6029,
}
impl From<LiquidityBookError> for ProgramError {
    fn from(e: LiquidityBookError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
