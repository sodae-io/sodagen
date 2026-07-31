use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum OmnipairError {
    #[error("Invalid deployer")]
    InvalidDeployer = 6000,
    #[error("Argument missing")]
    ArgumentMissing = 6001,
    #[error("Invalid swap fee bps")]
    InvalidSwapFeeBps = 6002,
    #[error("Invalid interest fee bps")]
    InvalidInterestFeeBps = 6003,
    #[error("Invalid half life")]
    InvalidHalfLife = 6004,
    #[error("Invalid futarchy authority")]
    InvalidFutarchyAuthority = 6005,
    #[error("Invalid reduce-only authority")]
    InvalidReduceOnlyAuthority = 6006,
    #[error("Invalid argument")]
    InvalidArgument = 6007,
    #[error("Amount cannot be zero")]
    AmountZero = 6008,
    #[error("Insufficient amount0 in")]
    InsufficientAmount0In = 6009,
    #[error("Insufficient amount1 in")]
    InsufficientAmount1In = 6010,
    #[error("Borrowing power exceeded")]
    BorrowingPowerExceeded = 6011,
    #[error("Invalid token account")]
    InvalidTokenAccount = 6012,
    #[error("Invalid token program")]
    InvalidTokenProgram = 6013,
    #[error("Borrow exceeds reserve")]
    BorrowExceedsReserve = 6014,
    #[error("Insufficient amount0")]
    InsufficientAmount0 = 6015,
    #[error("Insufficient amount1")]
    InsufficientAmount1 = 6016,
    #[error("Insufficient output amount")]
    InsufficientOutputAmount = 6017,
    #[error("Output amount below minimum requested (slippage exceeded)")]
    SlippageExceeded = 6018,
    #[error("Insufficient liquidity")]
    InsufficientLiquidity = 6019,
    #[error("Insufficient cash reserve0")]
    InsufficientCashReserve0 = 6020,
    #[error("Insufficient cash reserve1")]
    InsufficientCashReserve1 = 6021,
    #[error("Arithmetic overflow")]
    Overflow = 6022,
    #[error("Undercollateralized")]
    Undercollateralized = 6023,
    #[error("Insufficient balance for collateral")]
    InsufficientBalanceForCollateral = 6024,
    #[error("Insufficient amount")]
    InsufficientAmount = 6025,
    #[error("User balance insufficient to cover requested amount")]
    InsufficientBalance = 6026,
    #[error("Insufficient debt")]
    InsufficientDebt = 6027,
    #[error("User position not initialized")]
    UserPositionNotInitialized = 6028,
    #[error("Zero debt amount")]
    ZeroDebtAmount = 6029,
    #[error("Not undercollateralized")]
    NotUndercollateralized = 6030,
    #[error("Broken invariant")]
    BrokenInvariant = 6031,
    #[error("Math overflow during invariant calculation")]
    InvariantOverflow = 6032,
    #[error("Math overflow during fee calculation.")]
    FeeMathOverflow = 6033,
    #[error("Math overflow during output amount calculation.")]
    OutputAmountOverflow = 6034,
    #[error("Math overflow during reserve calculation.")]
    ReserveOverflow = 6035,
    #[error("Math underflow during reserve calculation.")]
    ReserveUnderflow = 6036,
    #[error("Math underflow during cash reserve calculation.")]
    CashReserveUnderflow = 6037,
    #[error("Math overflow during denominator calculation.")]
    DenominatorOverflow = 6038,
    #[error("Math overflow during liquidity calculation")]
    LiquidityMathOverflow = 6039,
    #[error("Math overflow during liquidity square root calculation")]
    LiquiditySqrtOverflow = 6040,
    #[error("Math underflow during liquidity calculation")]
    LiquidityUnderflow = 6041,
    #[error("Math overflow during liquidity conversion")]
    LiquidityConversionOverflow = 6042,
    #[error("Math overflow during supply calculation")]
    SupplyOverflow = 6043,
    #[error("Math underflow during supply calculation")]
    SupplyUnderflow = 6044,
    #[error("Math overflow during debt calculation")]
    DebtMathOverflow = 6045,
    #[error("Math overflow during debt share calculation")]
    DebtShareMathOverflow = 6046,
    #[error("Math overflow during debt share division")]
    DebtShareDivisionOverflow = 6047,
    #[error("Math overflow during debt utilization calculation")]
    DebtUtilizationOverflow = 6048,
    #[error("Invalid mint")]
    InvalidMint = 6049,
    #[error("Invalid mint length")]
    InvalidMintLen = 6050,
    #[error("Invalid distribution - percentages must sum to 100%")]
    InvalidDistribution = 6051,
    #[error("Invalid LP mint key")]
    InvalidLpMintKey = 6052,
    #[error("Invalid LP name")]
    InvalidLpName = 6053,
    #[error("Invalid LP symbol")]
    InvalidLpSymbol = 6054,
    #[error("Invalid LP URI")]
    InvalidLpUri = 6055,
    #[error("Account not empty")]
    AccountNotEmpty = 6056,
    #[error("Invalid mint authority")]
    InvalidMintAuthority = 6057,
    #[error("Frozen LP mint")]
    FrozenLpMint = 6058,
    #[error("Non-zero supply")]
    NonZeroSupply = 6059,
    #[error("Wrong LP decimals")]
    WrongLpDecimals = 6060,
    #[error("Invalid vault - token_in_vault and token_out_vault must be different")]
    InvalidVaultSameAccount = 6061,
    #[error("Invalid vault")]
    InvalidVault = 6062,
    #[error("Invalid params hash - hash does not match computed parameters")]
    InvalidParamsHash = 6063,
    #[error("Invalid version")]
    InvalidVersion = 6064,
    #[error("Invalid token order")]
    InvalidTokenOrder = 6065,
    #[error("Invalid rate model - rate_model does not match pair.rate_model")]
    InvalidRateModel = 6066,
    #[error("Invalid pair - pair does not match user_position.pair")]
    InvalidPair = 6067,
    #[error("Invalid utilization bounds - must satisfy: MIN <= start < end <= MAX")]
    InvalidUtilBounds = 6068,
    #[error(
        "Invalid rate parameters - check half_life_ms, min_rate_bps, max_rate_bps, initial_rate_bps bounds"
    )]
    InvalidRateParams = 6069,
    #[error("Operation blocked: reduce-only mode is active")]
    ReduceOnlyMode = 6070,
    #[error("Cannot remove collateral in reduce-only mode while debt exists")]
    ReduceOnlyHasDebt = 6071,
    #[error("Operation blocked: same-transaction liquidity delta detected")]
    LiquidityDeltaCircuitBreaker = 6072,
    #[error("Operation blocked: liquidity delta instruction must be top-level")]
    LiquidityDeltaCircuitBreakerCpi = 6073,
    #[error("Invalid instructions sysvar")]
    InvalidInstructionsSysvar = 6074,
    #[error("Insufficient post-withdraw debt coverage")]
    InsufficientPostWithdrawDebtCoverage = 6075,
    #[error("Invalid recipient - address does not match configured revenue recipient")]
    InvalidRecipient = 6076,
}
impl From<OmnipairError> for ProgramError {
    fn from(e: OmnipairError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
