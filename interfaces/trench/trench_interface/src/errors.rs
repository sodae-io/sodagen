use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum TrenchError {
    #[error("Name exceeds max length")]
    NameTooLong = 6000,
    #[error("Symbol exceeds max length")]
    SymbolTooLong = 6001,
    #[error("URI exceeds max length")]
    UriTooLong = 6002,
    #[error("Fee recipient does not match config")]
    InvalidFeeRecipient = 6003,
    #[error("Creator vault does not match bonding curve")]
    InvalidCreatorVault = 6004,
    #[error("Slippage tolerance exceeded")]
    SlippageExceeded = 6005,
    #[error("Bonding curve is complete")]
    CurveComplete = 6006,
    #[error("Insufficient token reserves")]
    InsufficientTokenReserves = 6007,
    #[error("Math overflow")]
    MathOverflow = 6008,
    #[error("Fee bps must be <= 10000")]
    InvalidFeeBps = 6009,
    #[error("Invalid curve parameters")]
    InvalidCurveParams = 6010,
    #[error("Invalid oracle price")]
    InvalidOraclePrice = 6011,
    #[error("Oracle price is stale")]
    OraclePriceStale = 6012,
    #[error("Oracle confidence exceeds config")]
    OracleConfidenceTooWide = 6013,
    #[error("Oracle exponent is unsupported")]
    InvalidOracleExponent = 6014,
    #[error("Oracle price is outside configured bounds")]
    OraclePriceOutOfRange = 6015,
    #[error("Curve cannot reach migration threshold")]
    CurveCapacityTooLow = 6016,
    #[error("Buy limit exceeded")]
    BuyLimitExceeded = 6017,
    #[error("No creator rewards available")]
    NoCreatorRewardsAvailable = 6018,
    #[error("Unauthorized")]
    Unauthorized = 6019,
    #[error("No trader cashback available")]
    NoTraderCashbackAvailable = 6020,
    #[error("Invalid governance authority")]
    InvalidGovernanceAuthority = 6021,
    #[error("Deploy cooldown is active")]
    DeployCooldownActive = 6022,
    #[error("This name reached the rolling launch limit")]
    LaunchNameLimitReached = 6023,
    #[error("Same-slot trading is blocked")]
    SameSlotTradingBlocked = 6024,
    #[error("Invalid X user id hash")]
    InvalidXUserIdHash = 6025,
    #[error("A user cannot refer themselves")]
    SelfReferral = 6026,
    #[error("Invalid trader referral account")]
    InvalidTraderReferral = 6027,
    #[error("Invalid referrer profile account")]
    InvalidReferrerProfile = 6028,
    #[error("Invalid referrer payout wallet")]
    InvalidReferrerWallet = 6029,
    #[error("No referral rewards available")]
    NoReferralRewardsAvailable = 6030,
    #[error("Buy would exceed the migration threshold")]
    MigrationThresholdExceeded = 6031,
    #[error("This symbol reached the rolling launch limit")]
    LaunchSymbolLimitReached = 6032,
    #[error("Launch limit tracker data is invalid")]
    InvalidLaunchLimitTracker = 6033,
    #[error("Identity bypass fee exceeds the approved maximum")]
    BypassFeeExceedsMaximum = 6034,
    #[error("Identity bypass history is full")]
    BypassHistoryFull = 6035,
    #[error("Migration is disabled")]
    MigrationDisabled = 6036,
    #[error("Bonding curve is not complete")]
    CurveNotComplete = 6037,
    #[error("Invalid migration configuration")]
    InvalidMigrationConfig = 6038,
    #[error("Invalid Raydium CPMM program")]
    InvalidRaydiumProgram = 6039,
    #[error("Invalid Raydium AMM config")]
    InvalidRaydiumAmmConfig = 6040,
    #[error("Raydium AMM fee configuration changed")]
    RaydiumFeeConfigChanged = 6041,
    #[error("Invalid Raydium permission account")]
    InvalidRaydiumPermission = 6042,
    #[error("Invalid Raydium pool account")]
    InvalidRaydiumPool = 6043,
    #[error("Invalid Raydium pool account ordering")]
    InvalidRaydiumMintOrder = 6044,
    #[error("Invalid wrapped SOL mint")]
    InvalidWrappedSolMint = 6045,
    #[error("Invalid Raydium pool creation fee receiver")]
    InvalidCreatePoolFeeReceiver = 6046,
    #[error("Insufficient migration funding")]
    InsufficientMigrationFunding = 6047,
    #[error("Insufficient bonding curve lamports")]
    InsufficientCurveLamports = 6048,
    #[error("Migration threshold cannot cover Raydium's fee and permanent account rent")]
    MigrationThresholdTooLow = 6049,
    #[error("Invalid migration token amount")]
    InvalidMigrationTokenAmount = 6050,
    #[error("Raydium opening price would not match the completed bonding curve")]
    MigrationPriceDiscontinuity = 6051,
    #[error("Migration staging account was not emptied")]
    MigrationAccountNotEmpty = 6052,
    #[error("LP tokens were not fully burned")]
    LpBurnIncomplete = 6053,
    #[error("Account data is invalid")]
    InvalidAccountData = 6054,
    #[error("Disable migrations before rotating the executor authority")]
    MigrationAuthorityRotationRequiresDisabled = 6055,
    #[error("Reserved legacy migration instruction-pair error")]
    InvalidMigrationInstructionPair = 6056,
    #[error("Migration staging PDA is invalid")]
    InvalidMigrationStagingAccount = 6057,
    #[error("Executor was not fully reimbursed for migration-funded account costs")]
    ExecutorMigrationFundingMismatch = 6058,
    #[error("Migration ready timestamp is invalid")]
    InvalidMigrationReadyAt = 6059,
    #[error("Migration delay is active")]
    MigrationDelayActive = 6060,
    #[error("Invalid creator revenue configuration")]
    InvalidRevenueConfig = 6061,
    #[error("Invalid creator revenue state")]
    InvalidCreatorRevenueState = 6062,
    #[error("Invalid creator revenue governance authority")]
    InvalidRevenueGovernance = 6063,
    #[error("Invalid creator revenue recipient")]
    InvalidRevenueRecipient = 6064,
    #[error("Creator revenue recipient revision is stale")]
    StaleCreatorRecipientRevision = 6065,
    #[error("No creator revenue is available to collect")]
    NoRevenueToCollect = 6066,
    #[error("Raydium creator fees accrued in the wrong asset")]
    InvalidRevenueFeeAsset = 6067,
    #[error("Raydium creator fee collection amount did not match")]
    RevenueCollectionMismatch = 6068,
    #[error("Creator revenue cumulative accounting target decreased")]
    RevenueAccountingTargetDecreased = 6069,
    #[error("Creator revenue cumulative accounting target exceeds collected custody")]
    RevenueAccountingExceedsCustody = 6070,
    #[error("Creator revenue settlement sequence is stale")]
    StaleRevenueSettlementSequence = 6071,
    #[error("Creator revenue accounting cutoff is stale")]
    StaleRevenueAccountingCutoff = 6072,
    #[error("Creator revenue policy version is stale")]
    StaleRevenuePolicyVersion = 6073,
    #[error("No settled creator revenue is available")]
    NoCreatorRevenueAvailable = 6074,
    #[error("No settled protocol revenue is available")]
    NoProtocolRevenueAvailable = 6075,
    #[error("Invalid creator revenue custody account")]
    InvalidRevenueCustody = 6076,
    #[error("Creator revenue payout settlement sequence is stale")]
    StaleRevenuePayoutSequence = 6077,
    #[error("Invalid trade authority")]
    InvalidTradeAuthority = 6078,
    #[error("Trade authority is already whitelisted")]
    TradeAuthorityAlreadyWhitelisted = 6079,
    #[error("Trade authority whitelist is full")]
    TradeAuthorityWhitelistFull = 6080,
    #[error("Trade authority is not whitelisted")]
    TradeAuthorityNotWhitelisted = 6081,
    #[error("Config cannot be migrated")]
    InvalidConfigMigration = 6082,
}
impl From<TrenchError> for ProgramError {
    fn from(e: TrenchError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
