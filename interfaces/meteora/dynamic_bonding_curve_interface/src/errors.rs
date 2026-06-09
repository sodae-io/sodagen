use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum DynamicBondingCurveError {
    #[error("Math operation overflow")]
    MathOverflow = 6000,
    #[error("Invalid fee setup")]
    InvalidFee = 6001,
    #[error("Exceeded slippage tolerance")]
    ExceededSlippage = 6002,
    #[error("Exceeded max fee bps")]
    ExceedMaxFeeBps = 6003,
    #[error("Invalid admin")]
    InvalidAdmin = 6004,
    #[error("Amount is zero")]
    AmountIsZero = 6005,
    #[error("Type cast error")]
    TypeCastFailed = 6006,
    #[error("Invalid activation type")]
    InvalidActivationType = 6007,
    #[error("Invalid quote mint")]
    InvalidQuoteMint = 6008,
    #[error("Invalid collect fee mode")]
    InvalidCollectFeeMode = 6009,
    #[error("Invalid migration fee option")]
    InvalidMigrationFeeOption = 6010,
    #[error("Invalid input")]
    InvalidInput = 6011,
    #[error("Not enough liquidity")]
    NotEnoughLiquidity = 6012,
    #[error("Pool is completed")]
    PoolIsCompleted = 6013,
    #[error("Pool is incompleted")]
    PoolIsIncompleted = 6014,
    #[error("Invalid migration option")]
    InvalidMigrationOption = 6015,
    #[error("Invalid token decimals")]
    InvalidTokenDecimals = 6016,
    #[error("Invalid token type")]
    InvalidTokenType = 6017,
    #[error("Invalid fee percentage")]
    InvalidFeePercentage = 6018,
    #[error("Invalid quote threshold")]
    InvalidQuoteThreshold = 6019,
    #[error("Invalid token supply")]
    InvalidTokenSupply = 6020,
    #[error("Invalid curve")]
    InvalidCurve = 6021,
    #[error("Not permit to do this action")]
    NotPermitToDoThisAction = 6022,
    #[error("Invalid owner account")]
    InvalidOwnerAccount = 6023,
    #[error("Invalid config account")]
    InvalidConfigAccount = 6024,
    #[error("Surplus has been withdraw")]
    SurplusHasBeenWithdraw = 6025,
    #[error("Leftover has been withdraw")]
    LeftoverHasBeenWithdraw = 6026,
    #[error("Total base token is exceeded max supply")]
    TotalBaseTokenExceedMaxSupply = 6027,
    #[error("Unsupport native mint token 2022")]
    UnsupportNativeMintToken2022 = 6028,
    #[error("Insufficient liquidity for migration")]
    InsufficientLiquidityForMigration = 6029,
    #[error("Missing pool config in remaining account")]
    MissingPoolConfigInRemainingAccount = 6030,
    #[error("Invalid vesting parameters")]
    InvalidVestingParameters = 6031,
    #[error("Invalid leftover address")]
    InvalidLeftoverAddress = 6032,
    #[error("Liquidity in bonding curve is insufficient")]
    InsufficientLiquidity = 6033,
    #[error("Invalid fee scheduler")]
    InvalidFeeScheduler = 6034,
    #[error("Invalid creator trading fee percentage")]
    InvalidCreatorTradingFeePercentage = 6035,
    #[error("Invalid new creator")]
    InvalidNewCreator = 6036,
    #[error("Invalid token authority option")]
    InvalidTokenAuthorityOption = 6037,
    #[error("Invalid account for the instruction")]
    InvalidAccount = 6038,
    #[error("Invalid migrator fee percentage")]
    InvalidMigratorFeePercentage = 6039,
    #[error("Migration fee has been withdraw")]
    MigrationFeeHasBeenWithdraw = 6040,
    #[error("Invalid base fee mode")]
    InvalidBaseFeeMode = 6041,
    #[error("Invalid fee rate limiter")]
    InvalidFeeRateLimiter = 6042,
    #[error("Fail to validate single swap instruction in rate limiter")]
    FailToValidateSingleSwapInstruction = 6043,
    #[error("Invalid migrated pool fee params")]
    InvalidMigratedPoolFee = 6044,
    #[error("Undertermined error")]
    UndeterminedError = 6045,
    #[error("Rate limiter not supported")]
    RateLimiterNotSupported = 6046,
    #[error("Amount left is not zero")]
    AmountLeftIsNotZero = 6047,
    #[error("Next sqrt price is smaller than start sqrt price")]
    NextSqrtPriceIsSmallerThanStartSqrtPrice = 6048,
    #[error("Invalid min base fee")]
    InvalidMinBaseFee = 6049,
    #[error("Account invariant violation")]
    AccountInvariantViolation = 6050,
    #[error("Invalid pool creation fee")]
    InvalidPoolCreationFee = 6051,
    #[error("Pool creation fee has been claimed")]
    PoolCreationFeeHasBeenClaimed = 6052,
    #[error("Not permit to do this action")]
    Unauthorized = 6053,
    #[error("Pool creation fee is zero")]
    ZeroPoolCreationFee = 6054,
    #[error("Invalid migration locked liquidity")]
    InvalidMigrationLockedLiquidity = 6055,
    #[error("Invalid fee market cap scheduler")]
    InvalidFeeMarketCapScheduler = 6056,
    #[error("Fail to validate first swap with minimum fee")]
    FirstSwapValidationFailed = 6057,
    #[error("Incorrect ATA")]
    IncorrectAta = 6058,
    #[error("Pool has insufficient lamports to perform the operation")]
    InsufficientPoolLamports = 6059,
    #[error("Invalid permission")]
    InvalidPermission = 6060,
    #[error("Invalid withdraw protocol fee zap accounts")]
    InvalidWithdrawProtocolFeeZapAccounts = 6061,
    #[error("SOL,USDC protocol fee cannot be withdrawn via zap")]
    MintRestrictedFromZap = 6062,
    #[error("Invalid zap out parameters")]
    InvalidZapOutParameters = 6063,
    #[error("CPI disabled")]
    CpiDisabled = 6064,
    #[error("Missing zap out instruction")]
    MissingZapOutInstruction = 6065,
    #[error("Invalid zap accounts")]
    InvalidZapAccounts = 6066,
    #[error("Invalid compounding parameters")]
    InvalidCompoundingParameters = 6067,
}
impl From<DynamicBondingCurveError> for ProgramError {
    fn from(e: DynamicBondingCurveError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
