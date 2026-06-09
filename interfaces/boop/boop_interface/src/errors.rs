use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum BoopError {
    #[error("Amount in too low")]
    AmountInTooLow = 6000,
    #[error("Amount out too low")]
    AmountOutTooLow = 6001,
    #[error("Amount zero")]
    AmountZero = 6002,
    #[error("Config not initialized")]
    ConfigNotInitialized = 6003,
    #[error("Creator is not provided")]
    CreatorIsNotProvided = 6004,
    #[error("Damping term too low")]
    DampingTermTooLow = 6005,
    #[error("First buy must be at most 50% of the total supply")]
    FirstBuyMustBeAtMost50PercentOfTotalSupply = 6006,
    #[error("Graduation fee relative to graduation target is too high")]
    GraduationFeeRelativeToTargetIsTooHigh = 6007,
    #[error("Insufficient tokens to transfer out of the bonding curve")]
    InsufficientTokensOut = 6008,
    #[error("Invalid bonding curve status")]
    InvalidBondingCurveStatus = 6009,
    #[error("Invalid damping term")]
    InvalidDampingTerm = 6010,
    #[error("Invalid mint")]
    InvalidMint = 6011,
    #[error("Invalid protocol fee recipient")]
    InvalidProtocolFeeRecipient = 6012,
    #[error("Invalid sqrt price")]
    InvalidSqrtPrice = 6013,
    #[error("Math overflow")]
    MathOverflow = 6014,
    #[error("Max basis points off graduation price too high")]
    MaxBasisPointsOffGraduationPriceTooHigh = 6015,
    #[error("Max graduation price deviation basis points too high")]
    MaxGraduationPriceDeviationBasisPointsTooHigh = 6016,
    #[error("Max swap amount for pool price correction basis points too high")]
    MaxSwapAmountForPoolPriceCorrectionBasisPointsTooHigh = 6017,
    #[error("Mint is larger than or equal to native mint")]
    MintIsLargerThanOrEqualToNativeMint = 6018,
    #[error("No authority transfer in progress")]
    NoAuthorityTransferInProgress = 6019,
    #[error("LP Token amount is too low")]
    NothingToDeposit = 6020,
    #[error("Nothing to split")]
    NothingToSplit = 6021,
    #[error("Nothing to lock")]
    NothingToLock = 6022,
    #[error("Operator already added")]
    OperatorAlreadyAdded = 6023,
    #[error("Operator does not exist")]
    OperatorDoesNotExist = 6024,
    #[error("Paused")]
    Paused = 6025,
    #[error(
        "Pool is already created and has a price out of range when attempting to deposit liquidity"
    )]
    PoolPriceOutOfRange = 6026,
    #[error("Swap fee basis points too high")]
    SwapFeeBasisPointsTooHigh = 6027,
    #[error("Swap amount exceeds the reasonable limit to leave as liquidity")]
    SwapAmountTooHigh = 6028,
    #[error("Token amount for Raydium liquidity too high")]
    TokenAmountForRaydiumLiquidityTooHigh = 6029,
    #[error("Token for stakers basis points too high")]
    TokenForStakersBasisPointsTooHigh = 6030,
    #[error("Token graduated")]
    TokenGraduated = 6031,
    #[error("Token name too long")]
    TokenNameTooLong = 6032,
    #[error("Token name too short")]
    TokenNameTooShort = 6033,
    #[error("Token symbol too long")]
    TokenSymbolTooLong = 6034,
    #[error("Token symbol too short")]
    TokenSymbolTooShort = 6035,
    #[error("Unauthorized")]
    Unauthorized = 6036,
}
impl From<BoopError> for ProgramError {
    fn from(e: BoopError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
