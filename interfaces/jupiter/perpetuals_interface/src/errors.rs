use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum PerpetualsError {
    #[error("Overflow in arithmetic operation")]
    MathOverflow = 6000,
    #[error("Unsupported price oracle")]
    UnsupportedOracle = 6001,
    #[error("Invalid oracle account")]
    InvalidOracleAccount = 6002,
    #[error("Stale oracle price")]
    StaleOraclePrice = 6003,
    #[error("Invalid oracle price")]
    InvalidOraclePrice = 6004,
    #[error("Instruction is not allowed in production")]
    InvalidEnvironment = 6005,
    #[error("Invalid collateral account")]
    InvalidCollateralAccount = 6006,
    #[error("Invalid collateral amount")]
    InvalidCollateralAmount = 6007,
    #[error("Collateral slippage")]
    CollateralSlippage = 6008,
    #[error("Invalid position state")]
    InvalidPositionState = 6009,
    #[error("Invalid perpetuals config")]
    InvalidPerpetualsConfig = 6010,
    #[error("Invalid pool config")]
    InvalidPoolConfig = 6011,
    #[error("Invalid instruction")]
    InvalidInstruction = 6012,
    #[error("Invalid custody config")]
    InvalidCustodyConfig = 6013,
    #[error("Invalid custody balance")]
    InvalidCustodyBalance = 6014,
    #[error("Invalid argument")]
    InvalidArgument = 6015,
    #[error("Invalid position request")]
    InvalidPositionRequest = 6016,
    #[error("Invalid position request input ata")]
    InvalidPositionRequestInputAta = 6017,
    #[error("Invalid mint")]
    InvalidMint = 6018,
    #[error("Insufficient token amount")]
    InsufficientTokenAmount = 6019,
    #[error("Insufficient token amount returned")]
    InsufficientAmountReturned = 6020,
    #[error("Price slippage limit exceeded")]
    MaxPriceSlippage = 6021,
    #[error("Position leverage limit exceeded")]
    MaxLeverage = 6022,
    #[error("Custody amount limit exceeded")]
    CustodyAmountLimit = 6023,
    #[error("Pool amount limit exceeded")]
    PoolAmountLimit = 6024,
    #[error("Personal pool amount limit exceeded")]
    PersonalPoolAmountLimit = 6025,
    #[error("Token is not supported")]
    UnsupportedToken = 6026,
    #[error("Instruction is not allowed at this time")]
    InstructionNotAllowed = 6027,
    #[error("Jupiter Program ID mismatch")]
    JupiterProgramMismatch = 6028,
    #[error("Program ID mismatch")]
    ProgramMismatch = 6029,
    #[error("Address mismatch")]
    AddressMismatch = 6030,
    #[error("Missing keeper ATA")]
    KeeperAtaMissing = 6031,
    #[error("Swap amount mismatch")]
    SwapAmountMismatch = 6032,
    #[error("CPI not allowed")]
    CpiNotAllowed = 6033,
    #[error("Invalid Keeper")]
    InvalidKeeper = 6034,
    #[error("Exceed execution period")]
    ExceedExecutionPeriod = 6035,
    #[error("Invalid Request Type")]
    InvalidRequestType = 6036,
    #[error("Invalid Trigger Price")]
    InvalidTriggerPrice = 6037,
    #[error("Trigger Price Slippage")]
    TriggerPriceSlippage = 6038,
    #[error("Missing Trigger Price")]
    MissingTriggerPrice = 6039,
    #[error("Missing Price Slippage")]
    MissingPriceSlippage = 6040,
    #[error("Invalid Price Calc Mode")]
    InvalidPriceCalcMode = 6041,
    #[error("Request Updated Too Recent")]
    RequestUpdatedTooRecent = 6042,
    #[error("Exceed Token Weightage")]
    ExceedTokenWeightage = 6043,
    #[error("Oracle Publish Time Too Early")]
    OraclePublishTimeTooEarly = 6044,
    #[error("Pull Oracle Publish Time Too Early")]
    PullOraclePublishTimeTooEarly = 6045,
    #[error("Stale Pull Oracle Price")]
    StalePullOraclePrice = 6046,
    #[error("Invalid Pull Oracle Price")]
    InvalidPullOraclePrice = 6047,
    #[error("Pull Oracle Not Verified")]
    PullOracleNotVerified = 6048,
    #[error("Price Diff Between Pull and Push Oracle is Too Large")]
    PriceDiffTooLarge = 6049,
    #[error("Invalid Doves Oracle Price")]
    InvalidDovesOraclePrice = 6050,
    #[error("Invalid Request Time")]
    InvalidRequestTime = 6051,
    #[error("Position Updated Too Recent")]
    PositionUpdatedTooRecent = 6052,
    #[error("Ledger token account does not match")]
    LedgerTokenAccountDoesNotMatch = 6053,
    #[error("Invalid token ledger")]
    InvalidTokenLedger = 6054,
    #[error("Oracle Price Difference Too Large")]
    OraclePriceDifferenceTooLarge = 6055,
    #[error("Invalid Oracle Signer")]
    InvalidOracleSigner = 6056,
    #[error("Invalid Oracle Timestamp")]
    InvalidOracleTimestamp = 6057,
    #[error("New Max Global Long Size Too Low")]
    InvalidMaxGlobalLongSize = 6058,
    #[error("New Max Global Short Size Too Low")]
    InvalidMaxGlobalShortSize = 6059,
    #[error("Borrows disabled for this custody")]
    BorrowsDisabled = 6060,
    #[error("Borrows limit exceeded for this custody")]
    BorrowLimitsExceeded = 6061,
    #[error("Withdraw exceeds margin of the custody")]
    WithdrawExceedsMarginLimits = 6062,
    #[error("Cannot liquidate")]
    CannotLiquidate = 6063,
    #[error("Cannot delegate stake to deactivated account")]
    CannotDelegateStake = 6064,
    #[error("Max total staked amount exceeded")]
    ExceededMaxTotalStakedAmount = 6065,
    #[error("LP token price change exceeds limit")]
    LpTokenPriceChangeLimitExceeded = 6066,
}
impl From<PerpetualsError> for ProgramError {
    fn from(e: PerpetualsError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
