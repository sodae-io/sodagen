use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum CloneError {
    #[error("Unauthorized")]
    Unauthorized = 6000,
    #[error("Invalid Mint Collateral Ratio")]
    InvalidMintCollateralRatio = 6001,
    #[error("Integer Type Conversion Error")]
    IntTypeConversionError = 6002,
    #[error("Pool Not Found")]
    PoolNotFound = 6003,
    #[error("Bump not found")]
    BumpNotFound = 6004,
    #[error("Invalid Token Amount")]
    InvalidTokenAmount = 6005,
    #[error("Expected Account Not Found")]
    ExpectedAccountNotFound = 6006,
    #[error("Outdated Oracle")]
    OutdatedOracle = 6007,
    #[error("Checked Math Error")]
    CheckedMathError = 6008,
    #[error("Mint Position Unable to Liquidate")]
    BorrowPositionUnableToLiquidate = 6009,
    #[error("Health Score Too Low")]
    HealthScoreTooLow = 6010,
    #[error("Invalid input collateral account")]
    InvalidInputCollateralAccount = 6011,
    #[error("Invalid Account loader owner")]
    InvalidAccountLoaderOwner = 6012,
    #[error("Invalid input position index")]
    InvalidInputPositionIndex = 6013,
    #[error("Invalid token account balance")]
    InvalidTokenAccountBalance = 6014,
    #[error("Inequality comparison violated")]
    InequalityComparisonViolated = 6015,
    #[error("Comet Not Empty")]
    CometNotEmpty = 6016,
    #[error("Not Subject to Liquidation")]
    NotSubjectToLiquidation = 6017,
    #[error("Liquidation amount too large")]
    LiquidationAmountTooLarge = 6018,
    #[error("No remaining accounts supplied")]
    NoRemainingAccountsSupplied = 6019,
    #[error("Invalid over-collateralization ratios")]
    InvalidOvercollateralizationRatios = 6020,
    #[error("Incorrect oracle address provided")]
    IncorrectOracleAddress = 6021,
    #[error("Value is in an incorrect range")]
    InvalidValueRange = 6022,
    #[error("Asset stable requirement violated")]
    InvalidAssetStability = 6023,
    #[error("Slippage tolerance exceeded")]
    SlippageToleranceExceeded = 6024,
    #[error("Collateral must be all in onUSD")]
    RequireOnlyonUsdCollateral = 6025,
    #[error("Positions must be all closed")]
    RequireAllPositionsClosed = 6026,
    #[error("Failed to Load Pyth Price Feed")]
    FailedToLoadPyth = 6027,
    #[error("Status Prevents Action")]
    StatusPreventsAction = 6028,
    #[error("Pool is empty")]
    PoolEmpty = 6029,
    #[error("No liquidity to withdraw")]
    NoLiquidityToWithdraw = 6030,
    #[error("Invalid Status")]
    InvalidStatus = 6031,
    #[error("Auth Array Full")]
    AuthArrayFull = 6032,
    #[error("Auth Not Found")]
    AuthNotFound = 6033,
    #[error("Invalid oracle index")]
    InvalidOracleIndex = 6034,
    #[error("Invalid Payment Type")]
    InvalidPaymentType = 6035,
    #[error("Invalid Conversion")]
    InvalidConversion = 6036,
    #[error("Auth Already Exists")]
    AuthAlreadyExists = 6037,
}
impl From<CloneError> for ProgramError {
    fn from(e: CloneError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
