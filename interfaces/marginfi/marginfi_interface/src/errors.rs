use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum MarginfiError {
    #[error("Internal Marginfi logic error")]
    InternalLogicError = 6000,
    #[error("Invalid bank index")]
    BankNotFound = 6001,
    #[error("Lending account balance not found")]
    LendingAccountBalanceNotFound = 6002,
    #[error("Bank deposit capacity exceeded")]
    BankAssetCapacityExceeded = 6003,
    #[error("Invalid transfer")]
    InvalidTransfer = 6004,
    #[error("Missing Oracle, Bank, LST mint, or Sol Pool")]
    MissingPythOrBankAccount = 6005,
    #[error("Missing Pyth account")]
    MissingPythAccount = 6006,
    #[error("Missing Bank account")]
    MissingBankAccount = 6007,
    #[error("Invalid Bank account")]
    InvalidBankAccount = 6008,
    #[error("RiskEngine rejected due to either bad health or stale oracles")]
    RiskEngineInitRejected = 6009,
    #[error("Lending account balance slots are full")]
    LendingAccountBalanceSlotsFull = 6010,
    #[error("Bank already exists")]
    BankAlreadyExists = 6011,
    #[error("Amount to liquidate must be positive")]
    ZeroLiquidationAmount = 6012,
    #[error("Account is not bankrupt")]
    AccountNotBankrupt = 6013,
    #[error("Account balance is not bad debt")]
    BalanceNotBadDebt = 6014,
    #[error("Invalid group config")]
    InvalidConfig = 6015,
    #[error("Bank paused")]
    BankPaused = 6016,
    #[error("Bank is ReduceOnly mode")]
    BankReduceOnly = 6017,
    #[error("Bank is missing")]
    BankAccountNotFound = 6018,
    #[error("Operation is deposit-only")]
    OperationDepositOnly = 6019,
    #[error("Operation is withdraw-only")]
    OperationWithdrawOnly = 6020,
    #[error("Operation is borrow-only")]
    OperationBorrowOnly = 6021,
    #[error("Operation is repay-only")]
    OperationRepayOnly = 6022,
    #[error("No asset found")]
    NoAssetFound = 6023,
    #[error("No liability found")]
    NoLiabilityFound = 6024,
    #[error("Invalid oracle setup")]
    InvalidOracleSetup = 6025,
    #[error("Invalid bank utilization ratio")]
    IllegalUtilizationRatio = 6026,
    #[error("Bank borrow cap exceeded")]
    BankLiabilityCapacityExceeded = 6027,
    #[error("Invalid Price")]
    InvalidPrice = 6028,
    #[error("Account can have only one liability when account is under isolated risk")]
    IsolatedAccountIllegalState = 6029,
    #[error("Emissions already setup")]
    EmissionsAlreadySetup = 6030,
    #[error("Oracle is not set")]
    OracleNotSetup = 6031,
    #[error("Invalid switchboard decimal conversion")]
    InvalidSwitchboardDecimalConversion = 6032,
    #[error("Cannot close balance because of outstanding emissions")]
    CannotCloseOutstandingEmissions = 6033,
    #[error("Update emissions error")]
    EmissionsUpdateError = 6034,
    #[error("Account disabled")]
    AccountDisabled = 6035,
    #[error("Account can't temporarily open 3 balances, please close a balance first")]
    AccountTempActiveBalanceLimitExceeded = 6036,
    #[error("Illegal action during flashloan")]
    AccountInFlashloan = 6037,
    #[error("Illegal flashloan")]
    IllegalFlashloan = 6038,
    #[error("Illegal flag")]
    IllegalFlag = 6039,
    #[error("Illegal balance state")]
    IllegalBalanceState = 6040,
    #[error("Illegal account authority transfer")]
    IllegalAccountAuthorityTransfer = 6041,
    #[error("Unauthorized")]
    Unauthorized = 6042,
    #[error("Invalid account authority")]
    IllegalAction = 6043,
    #[error("Token22 Banks require mint account as first remaining account")]
    T22MintRequired = 6044,
    #[error("Invalid ATA for global fee account")]
    InvalidFeeAta = 6045,
    #[error("Use add pool permissionless instead")]
    AddedStakedPoolManually = 6046,
    #[error("Staked SOL accounts can only deposit staked assets and borrow SOL")]
    AssetTagMismatch = 6047,
    #[error("Stake pool validation failed: check the stake pool, mint, or sol pool")]
    StakePoolValidationFailed = 6048,
    #[error("Switchboard oracle: stale price")]
    SwitchboardStalePrice = 6049,
    #[error("Pyth Push oracle: stale price")]
    PythPushStalePrice = 6050,
    #[error("Oracle error: wrong number of accounts")]
    WrongNumberOfOracleAccounts = 6051,
    #[error("Oracle error: wrong account keys")]
    WrongOracleAccountKeys = 6052,
    #[error("Vacated2")]
    Vacated2 = 6053,
    #[error("Vacated3")]
    Vacated3 = 6054,
    #[error("Oracle max confidence exceeded: try again later")]
    OracleMaxConfidenceExceeded = 6055,
    #[error("Pyth Push oracle: insufficient verification level")]
    PythPushInsufficientVerificationLevel = 6056,
    #[error("Zero asset price")]
    ZeroAssetPrice = 6057,
    #[error("Zero liability price")]
    ZeroLiabilityPrice = 6058,
    #[error("Switchboard oracle: wrong account owner")]
    SwitchboardWrongAccountOwner = 6059,
    #[error("Pyth Push oracle: invalid account")]
    PythPushInvalidAccount = 6060,
    #[error("Switchboard oracle: invalid account")]
    SwitchboardInvalidAccount = 6061,
    #[error("Math error")]
    MathError = 6062,
    #[error("Invalid emissions destination account")]
    InvalidEmissionsDestinationAccount = 6063,
    #[error("Asset and liability bank cannot be the same")]
    SameAssetAndLiabilityBanks = 6064,
    #[error("Trying to withdraw more assets than available")]
    OverliquidationAttempt = 6065,
    #[error("Liability bank has no liabilities")]
    NoLiabilitiesInLiabilityBank = 6066,
    #[error("Liability bank has assets")]
    AssetsInLiabilityBank = 6067,
    #[error("Account is healthy and cannot be liquidated")]
    HealthyAccount = 6068,
    #[error("Liability payoff too severe, exhausted liability")]
    ExhaustedLiability = 6069,
    #[error("Liability payoff too severe, liability balance has assets")]
    TooSeverePayoff = 6070,
    #[error("Liquidation too severe, account above maintenance requirement")]
    TooSevereLiquidation = 6071,
    #[error("Liquidation would worsen account health")]
    WorseHealthPostLiquidation = 6072,
    #[error("Exceeded the maximum allowed integration positions")]
    IntegrationPositionLimitExceeded = 6073,
    #[error("Maximum initial leverage exceeded")]
    MaxInitLeverageExceeded = 6074,
    #[error("The Emode config was invalid")]
    BadEmodeConfig = 6075,
    #[error("TWAP window size does not match expected duration")]
    PythPushInvalidWindowSize = 6076,
    #[error("Invalid fees destination account")]
    InvalidFeesDestinationAccount = 6077,
    #[error("Banks cannot close when they have open positions or emissions outstanding")]
    BankCannotClose = 6078,
    #[error("Account already migrated")]
    AccountAlreadyMigrated = 6079,
    #[error("Protocol is paused")]
    ProtocolPaused = 6080,
    #[error("Metadata is too long")]
    MetadataTooLong = 6081,
    #[error("Pause limit exceeded")]
    PauseLimitExceeded = 6082,
    #[error("Protocol is not paused")]
    ProtocolNotPaused = 6083,
    #[error(
        "Bank killed by bankruptcy: bank shutdown and value of all holdings is zero"
    )]
    BankKilledByBankruptcy = 6084,
    #[error(
        "Liquidation state issue. Check start before end, end last, and both unique"
    )]
    UnexpectedLiquidationState = 6085,
    #[error(
        "Liquidation start must be first instruction (other than compute program ixes)"
    )]
    StartNotFirst = 6086,
    #[error("Only one liquidation event allowed per tx")]
    StartRepeats = 6087,
    #[error("The end instruction must be the last ix in the tx")]
    EndNotLast = 6088,
    #[error("Tried to call an instruction that is forbidden during liquidation")]
    ForbiddenIx = 6089,
    #[error("Seized too much of the asset relative to liability repaid")]
    LiquidationPremiumTooHigh = 6090,
    #[error("Start and end liquidation and flashloan must be top-level instructions")]
    NotAllowedInCpi = 6091,
    #[error("Stake pool supply is zero: cannot compute price")]
    ZeroSupplyInStakePool = 6092,
    #[error("Invalid group: account constraint violated")]
    InvalidGroup = 6093,
    #[error("Invalid liquidity vault: account constraint violated")]
    InvalidLiquidityVault = 6094,
    #[error("Invalid liquidation record: account constraint violated")]
    InvalidLiquidationRecord = 6095,
    #[error("Invalid liquidation receiver: account constraint violated")]
    InvalidLiquidationReceiver = 6096,
    #[error("Invalid emissions mint: account constraint violated")]
    InvalidEmissionsMint = 6097,
    #[error("Invalid mint: account constraint violated")]
    InvalidMint = 6098,
    #[error("Invalid fee wallet: account constraint violated")]
    InvalidFeeWallet = 6099,
    #[error("Fixed oracle price must be zero or greater")]
    FixedOraclePriceNegative = 6100,
    #[error("Daily withdrawal limit exceeded: try again later")]
    DailyWithdrawalLimitExceeded = 6101,
    #[error("Cannot set daily withdrawal limit to zero")]
    ZeroWithdrawalLimit = 6102,
    #[error("Account is frozen by the group admin")]
    AccountFrozen = 6103,
    #[error("Cannot reference duplicate balances")]
    DuplicateBalance = 6104,
    #[error("Invalid amount of balances referenced")]
    InvalidBalanceCount = 6105,
    #[error("Liquidator not allowed to close order")]
    LiquidatorOrderCloseNotAllowed = 6106,
    #[error("Order trigger is yet to be met")]
    OrderTriggerNotMet = 6107,
    #[error(
        "Order execution state issue. Check the necessary invariants i.e not in flashloan or disabled e.t.c"
    )]
    UnexpectedOrderExecutionState = 6108,
    #[error("Order liability not closed")]
    OrderLiabilityNotClosed = 6109,
    #[error("Invalid asset or liabilities count")]
    InvalidAssetOrLiabilitiesCount = 6110,
    #[error("Account health can only worsen if account is healthy")]
    WorseHealthPostExecution = 6111,
    #[error("TP must be > 0, SL must be > 0 and TP > SL if both are set")]
    InvalidOrderTakeProfitOrStopLoss = 6112,
    #[error("Max slippage must be less than 100%")]
    InvalidSlippage = 6113,
    #[error("Executor withdrew too much: slippage or max fee constraint violated")]
    OrderExecutionOverWithdrawal = 6114,
    #[error("Bank hourly rate limit exceeded: try again later")]
    BankHourlyRateLimitExceeded = 6115,
    #[error("Bank daily rate limit exceeded: try again later")]
    BankDailyRateLimitExceeded = 6116,
    #[error("Group hourly rate limit exceeded: try again later")]
    GroupHourlyRateLimitExceeded = 6117,
    #[error("Group daily rate limit exceeded: try again later")]
    GroupDailyRateLimitExceeded = 6118,
    #[error("Invalid rate limit price: pass oracle or pre-crank cache")]
    InvalidRateLimitPrice = 6119,
    #[error("Group rate limiter admin update must include inflow and/or outflow")]
    GroupRateLimiterUpdateEmpty = 6120,
    #[error("Group rate limiter admin update slot range is invalid")]
    GroupRateLimiterUpdateInvalidSlotRange = 6121,
    #[error("Group rate limiter admin update cannot reference future slots")]
    GroupRateLimiterUpdateFutureSlot = 6122,
    #[error("Group rate limiter admin update is too stale")]
    GroupRateLimiterUpdateStale = 6123,
    #[error("Group rate limiter admin update slot progression is out of order")]
    GroupRateLimiterUpdateOutOfOrderSlot = 6124,
    #[error("Group rate limiter admin update sequence is out of order")]
    GroupRateLimiterUpdateOutOfOrderSeq = 6125,
    #[error("Deleverage withdrawal admin update must include outflow")]
    DeleverageWithdrawalUpdateEmpty = 6126,
    #[error("Deleverage withdrawal admin update slot range is invalid")]
    DeleverageWithdrawalUpdateInvalidSlotRange = 6127,
    #[error("Deleverage withdrawal admin update cannot reference future slots")]
    DeleverageWithdrawalUpdateFutureSlot = 6128,
    #[error("Deleverage withdrawal admin update is too stale")]
    DeleverageWithdrawalUpdateStale = 6129,
    #[error("Deleverage withdrawal admin update slot progression is out of order")]
    DeleverageWithdrawalUpdateOutOfOrderSlot = 6130,
    #[error("Deleverage withdrawal admin update sequence is out of order")]
    DeleverageWithdrawalUpdateOutOfOrderSeq = 6131,
    #[error(
        "Wrong asset tag for standard instructions, expected DEFAULT, SOL, or STAKED asset tag"
    )]
    WrongAssetTagForStandardInstructions = 6200,
    #[error("Wrong asset tag for Kamino instructions, expected KAMINO asset tag")]
    WrongAssetTagForKaminoInstructions = 6201,
    #[error("Cannot create a kamino bank with this instruction, use add_bank_kamino")]
    CantAddPool = 6202,
    #[error("Kamino reserve mint address doesn't match the bank mint address")]
    KaminoReserveMintAddressMismatch = 6203,
    #[error(
        "Deposit failed: obligation deposit amount increase did not match the expected increase, left - actual, right - expected"
    )]
    KaminoDepositFailed = 6204,
    #[error(
        "Withdraw failed: token vault increase did not match the expected increase, left - actual, right - expected"
    )]
    KaminoWithdrawFailed = 6205,
    #[error(
        "Kamino Reserve data is stale - run refresh_reserve on kamino program first"
    )]
    ReserveStale = 6206,
    #[error("Kamino obligation must have exactly one active deposit, at index 0")]
    InvalidObligationDepositCount = 6207,
    #[error("Kamino obligation deposit doesn't match the expected reserve")]
    ObligationDepositReserveMismatch = 6208,
    #[error("Failed to meet minimum deposit amount requirement for init obligation")]
    ObligationInitDepositInsufficient = 6209,
    #[error("Kamino reserve validation failed")]
    KaminoReserveValidationFailed = 6210,
    #[error(
        "Invalid oracle setup: only KaminoPythPush and KaminoSwitchboardPull are supported"
    )]
    KaminoInvalidOracleSetup = 6211,
    #[error("Maximum Maintenance leverage exceeded")]
    MaxMaintLeverageExceeded = 6212,
    #[error("Invalid Kamino reserve: account constraint violated")]
    InvalidKaminoReserve = 6213,
    #[error("Invalid Kamino obligation: account constraint violated")]
    InvalidKaminoObligation = 6214,
    #[error(
        "Invalid oracle setup: only DriftPythPull and DriftSwitchboardPull are supported"
    )]
    DriftInvalidOracleSetup = 6300,
    #[error("Drift spot market mint does not match bank mint")]
    DriftSpotMarketMintMismatch = 6301,
    #[error("Wrong bank asset tag for Drift operation")]
    WrongBankAssetTagForDriftOperation = 6302,
    #[error("Cannot use standard operations on Drift assets")]
    CantUseStandardOperationsOnDriftAssets = 6303,
    #[error("Drift spot market validation failed")]
    DriftSpotMarketValidationFailed = 6304,
    #[error(
        "Drift user has invalid spot positions (only first position can have balance)"
    )]
    DriftInvalidSpotPositions = 6305,
    #[error("Drift spot position market does not match bank's configured market")]
    DriftSpotPositionMarketMismatch = 6306,
    #[error("Drift position has invalid balance type (must be deposit)")]
    DriftInvalidPositionType = 6307,
    #[error("Drift scaled balance change does not match expected amount")]
    DriftScaledBalanceMismatch = 6308,
    #[error("Drift withdrawal failed - token amount mismatch")]
    DriftWithdrawFailed = 6309,
    #[error("Drift user initial deposit insufficient (minimum 10 units required)")]
    DriftUserInitDepositInsufficient = 6310,
    #[error("Invalid drift account")]
    InvalidDriftAccount = 6311,
    #[error("Drift authority mismatch")]
    DriftAuthorityMismatch = 6312,
    #[error("Invalid harvest position index - must be between 2 and 7")]
    DriftInvalidHarvestPositionIndex = 6313,
    #[error("Drift position is empty")]
    DriftPositionEmpty = 6314,
    #[error("Drift position has invalid balance type")]
    DriftInvalidBalanceType = 6315,
    #[error("No admin deposits found in Drift positions 2-7 for this market")]
    DriftNoAdminDeposit = 6316,
    #[error("Cannot harvest from the same market as the bank's main drift spot market")]
    DriftHarvestSameMarket = 6317,
    #[error("Drift account bricked: too many active deposits from admin operations")]
    DriftBrickedAccount = 6318,
    #[error("Drift reward oracle required when 2+ active deposits exist")]
    DriftMissingRewardOracle = 6319,
    #[error("Drift reward spot market required when 2+ active deposits exist")]
    DriftMissingRewardSpotMarket = 6320,
    #[error(
        "Drift account has admin deposits that require reward accounts to be provided"
    )]
    DriftMissingRewardAccounts = 6321,
    #[error("Drift spot market is stale, interest needs to be updated")]
    DriftSpotMarketStale = 6322,
    #[error("Invalid Drift spot market: account constraint violated")]
    InvalidDriftSpotMarket = 6323,
    #[error("Invalid Drift user: account constraint violated")]
    InvalidDriftUser = 6324,
    #[error("Invalid Drift user stats: account constraint violated")]
    InvalidDriftUserStats = 6325,
    #[error("Drift cannot support tokens with more than 19 decimals")]
    DriftUnsupportedTokenDecimals = 6326,
    #[error(
        "Invalid oracle setup: only SolendPythPull and SolendSwitchboardPull are supported"
    )]
    SolendInvalidOracleSetup = 6400,
    #[error("Solend reserve validation failed")]
    SolendReserveValidationFailed = 6401,
    #[error("Solend obligation owner mismatch")]
    SolendObligationOwnerMismatch = 6402,
    #[error("Wrong bank asset tag for Solend operation")]
    WrongBankAssetTagForSolendOperation = 6403,
    #[error("Cannot use standard operations on Solend assets")]
    CantUseStandardOperationsOnSolendAssets = 6404,
    #[error("Solend reserve mismatch")]
    SolendReserveMismatch = 6405,
    #[error("Solend reserve mint mismatch")]
    SolendReserveMintMismatch = 6406,
    #[error(
        "Solend obligation has invalid deposits (only first position can have balance)"
    )]
    SolendInvalidDepositPositions = 6407,
    #[error("Solend deposit position reserve does not match bank's configured reserve")]
    SolendDepositPositionReserveMismatch = 6408,
    #[error("Solend cToken balance change does not match expected amount")]
    SolendCTokenBalanceMismatch = 6409,
    #[error("Solend withdrawal failed - token amount mismatch")]
    SolendWithdrawFailed = 6410,
    #[error("Solend reserve is stale")]
    SolendReserveStale = 6411,
    #[error("Solend deposit failed - collateral amount mismatch")]
    SolendDepositFailed = 6412,
    #[error("Invalid Solend account owner")]
    InvalidSolendAccount = 6413,
    #[error("Invalid Solend account version")]
    InvalidSolendAccountVersion = 6414,
    #[error("Invalid Solend reserve: account constraint violated")]
    InvalidSolendReserve = 6415,
    #[error("Invalid Solend obligation: account constraint violated")]
    InvalidSolendObligation = 6416,
    #[error(
        "Invalid oracle setup: only JuplendPythPull and JuplendSwitchboardPull are supported"
    )]
    JuplendInvalidOracleSetup = 6500,
    #[error("Juplend lending state validation failed")]
    JuplendLendingValidationFailed = 6501,
    #[error("Wrong bank asset tag for Juplend operation")]
    WrongBankAssetTagForJuplendOperation = 6502,
    #[error("Cannot use standard operations on Juplend assets")]
    CantUseStandardOperationsOnJuplendAssets = 6503,
    #[error("Juplend lending state is stale")]
    JuplendLendingStale = 6504,
    #[error("Invalid Juplend lending: account constraint violated")]
    InvalidJuplendLending = 6505,
    #[error("Juplend lending mint mismatch")]
    JuplendLendingMintMismatch = 6506,
    #[error("Juplend bank is already activated")]
    JuplendBankAlreadyActivated = 6507,
    #[error("Invalid Juplend fToken vault")]
    InvalidJuplendFTokenVault = 6508,
    #[error("Juplend deposit failed")]
    JuplendDepositFailed = 6509,
    #[error("Juplend withdraw failed")]
    JuplendWithdrawFailed = 6510,
    #[error("Juplend init position deposit insufficient")]
    JuplendInitPositionDepositInsufficient = 6511,
    #[error("Invalid Juplend withdraw intermediary ATA")]
    InvalidJuplendWithdrawIntermediaryAta = 6512,
}
impl From<MarginfiError> for ProgramError {
    fn from(e: MarginfiError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
