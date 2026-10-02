use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum CpAmmError {
    #[error("Math operation overflow")]
    MathOverflow = 6000,
    #[error("Invalid fee setup")]
    InvalidFee = 6001,
    #[error("Exceeded slippage tolerance")]
    ExceededSlippage = 6002,
    #[error("Pool disabled")]
    PoolDisabled = 6003,
    #[error("Exceeded max fee bps")]
    ExceedMaxFeeBps = 6004,
    #[error("Invalid admin")]
    InvalidAdmin = 6005,
    #[error("Amount is zero")]
    AmountIsZero = 6006,
    #[error("Type cast error")]
    TypeCastFailed = 6007,
    #[error("Unable to modify activation point")]
    UnableToModifyActivationPoint = 6008,
    #[error("Invalid authority to create the pool")]
    InvalidAuthorityToCreateThePool = 6009,
    #[error("Invalid activation type")]
    InvalidActivationType = 6010,
    #[error("Invalid activation point")]
    InvalidActivationPoint = 6011,
    #[error("Quote token must be SOL,USDC")]
    InvalidQuoteMint = 6012,
    #[error("Invalid fee curve")]
    InvalidFeeCurve = 6013,
    #[error("Invalid Price Range")]
    InvalidPriceRange = 6014,
    #[error("Trade is over price range")]
    PriceRangeViolation = 6015,
    #[error("Invalid parameters")]
    InvalidParameters = 6016,
    #[error("Invalid collect fee mode")]
    InvalidCollectFeeMode = 6017,
    #[error("Invalid input")]
    InvalidInput = 6018,
    #[error("Cannot create token badge on supported mint")]
    CannotCreateTokenBadgeOnSupportedMint = 6019,
    #[error("Invalid token badge")]
    InvalidTokenBadge = 6020,
    #[error("Invalid minimum liquidity")]
    InvalidMinimumLiquidity = 6021,
    #[error("Invalid vesting information")]
    InvalidVestingInfo = 6022,
    #[error("Insufficient liquidity")]
    InsufficientLiquidity = 6023,
    #[error("Invalid vesting account")]
    InvalidVestingAccount = 6024,
    #[error("Invalid pool status")]
    InvalidPoolStatus = 6025,
    #[error("Unsupported native mint token2022")]
    UnsupportNativeMintToken2022 = 6026,
    #[error("Invalid reward index")]
    InvalidRewardIndex = 6027,
    #[error("Invalid reward duration")]
    InvalidRewardDuration = 6028,
    #[error("Reward already initialized")]
    RewardInitialized = 6029,
    #[error("Reward not initialized")]
    RewardUninitialized = 6030,
    #[error("Invalid reward vault")]
    InvalidRewardVault = 6031,
    #[error("Must withdraw ineligible reward")]
    MustWithdrawnIneligibleReward = 6032,
    #[error("Reward duration is the same")]
    IdenticalRewardDuration = 6033,
    #[error("Reward campaign in progress")]
    RewardCampaignInProgress = 6034,
    #[error("Identical funder")]
    IdenticalFunder = 6035,
    #[error("Invalid funder")]
    InvalidFunder = 6036,
    #[error("Reward not ended")]
    RewardNotEnded = 6037,
    #[error("Fee inverse is incorrect")]
    FeeInverseIsIncorrect = 6038,
    #[error("Position is not empty")]
    PositionIsNotEmpty = 6039,
    #[error("Invalid pool creator authority")]
    InvalidPoolCreatorAuthority = 6040,
    #[error("Invalid config type")]
    InvalidConfigType = 6041,
    #[error("Invalid pool creator")]
    InvalidPoolCreator = 6042,
    #[error("Reward vault is frozen, must skip reward to proceed")]
    RewardVaultFrozenSkipRequired = 6043,
    #[error("Invalid parameters for split position")]
    InvalidSplitPositionParameters = 6044,
    #[error("Unsupported split position has vesting lock")]
    UnsupportPositionHasVestingLock = 6045,
    #[error("Same position")]
    SamePosition = 6046,
    #[error("Invalid base fee mode")]
    InvalidBaseFeeMode = 6047,
    #[error("Invalid fee rate limiter")]
    InvalidFeeRateLimiter = 6048,
    #[error("Fail to validate single swap instruction in rate limiter")]
    FailToValidateSingleSwapInstruction = 6049,
    #[error("Invalid fee scheduler")]
    InvalidFeeTimeScheduler = 6050,
    #[error("Undetermined error")]
    UndeterminedError = 6051,
    #[error("Invalid pool version")]
    InvalidPoolVersion = 6052,
    #[error("Invalid authority to do that action")]
    InvalidAuthority = 6053,
    #[error("Invalid permission")]
    InvalidPermission = 6054,
    #[error("Invalid fee market cap scheduler")]
    InvalidFeeMarketCapScheduler = 6055,
    #[error("Cannot update base fee")]
    CannotUpdateBaseFee = 6056,
    #[error("Invalid dynamic fee parameters")]
    InvalidDynamicFeeParameters = 6057,
    #[error("Invalid update pool fees parameters")]
    InvalidUpdatePoolFeesParameters = 6058,
    #[error("Missing operator account")]
    MissingOperatorAccount = 6059,
    #[error("Incorrect ATA")]
    IncorrectAta = 6060,
    #[error("Invalid zap out parameters")]
    InvalidZapOutParameters = 6061,
    #[error("Invalid withdraw protocol fee zap accounts")]
    InvalidWithdrawProtocolFeeZapAccounts = 6062,
    #[error("SOL,USDC protocol fee cannot be withdrawn via zap")]
    MintRestrictedFromZap = 6063,
    #[error("CPI disabled")]
    CpiDisabled = 6064,
    #[error("Missing zap out instruction")]
    MissingZapOutInstruction = 6065,
    #[error("Invalid zap accounts")]
    InvalidZapAccounts = 6066,
    #[error("Invalid compounding fee bps")]
    InvalidCompoundingFeeBps = 6067,
    #[error("Invalid claim protocol fee accounts")]
    InvalidClaimProtocolFeeAccounts = 6068,
    #[error("Transfer fee excluded amount is zero")]
    TransferFeeExcludedAmountIsZero = 6069,
    #[error("Delegated amount is not zero")]
    DelegatedAmountNonZero = 6070,
    #[error("Deprecated base fee mode")]
    DeprecatedBaseFeeMode = 6071,
    #[error("Invalid config permission")]
    InvalidConfigPermission = 6072,
}
impl From<CpAmmError> for ProgramError {
    fn from(e: CpAmmError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
