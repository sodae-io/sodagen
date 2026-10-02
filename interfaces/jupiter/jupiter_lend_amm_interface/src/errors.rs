use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum JupiterLendAmmError {
    #[error("DEX_CPI_TO_ORACLE_FAILED")]
    DexCpiToOracleFailed = 6000,
    #[error("DEX_USER_NOT_DEFINED")]
    DexUserNotDefined = 6001,
    #[error("DEX_USER_MUST_SIGN")]
    DexUserMustSign = 6002,
    #[error("DEX_ADMIN_LIQUIDITY_PROGRAM_MISMATCH")]
    DexAdminLiquidityProgramMismatch = 6003,
    #[error("DEX_ADMIN_ADDRESS_ZERO_NOT_ALLOWED")]
    DexAdminAddressZeroNotAllowed = 6004,
    #[error("DEX_ADMIN_NOT_AN_AUTH")]
    DexAdminNotAnAuth = 6005,
    #[error("DEX_ADMIN_POOL_NOT_INITIALIZED")]
    DexAdminPoolNotInitialized = 6006,
    #[error("DEX_ADMIN_SMART_COL_ALREADY_ON")]
    DexAdminSmartColAlreadyOn = 6007,
    #[error("DEX_ADMIN_SMART_DEBT_ALREADY_ON")]
    DexAdminSmartDebtAlreadyOn = 6008,
    #[error("DEX_ADMIN_CONFIG_OVERFLOW")]
    DexAdminConfigOverflow = 6009,
    #[error("DEX_ADMIN_INVALID_PARAMS")]
    DexAdminInvalidParams = 6010,
    #[error("DEX_ADMIN_USER_NOT_DEFINED")]
    DexAdminUserNotDefined = 6011,
    #[error("DEX_ADMIN_INVALID_PAUSE_TOGGLE")]
    DexAdminInvalidPauseToggle = 6012,
    #[error("DEX_ADMIN_UNEXPECTED_POOL_STATE")]
    DexAdminUnexpectedPoolState = 6013,
    #[error("DEX_ADMIN_ADDRESS_NOT_A_PROGRAM")]
    DexAdminAddressNotAProgram = 6014,
    #[error("DEX_ADMIN_INVALID_TOKEN_DECIMALS")]
    DexAdminInvalidTokenDecimals = 6015,
    #[error("DEX_ADMIN_PREVIOUS_SHIFT_STILL_ACTIVE")]
    DexAdminPreviousShiftStillActive = 6016,
    #[error("DEX_ALREADY_ENTERED")]
    DexAlreadyEntered = 6017,
    #[error("DEX_NOT_AN_AUTH")]
    DexNotAnAuth = 6018,
    #[error("DEX_SMART_COL_NOT_ENABLED")]
    DexSmartColNotEnabled = 6019,
    #[error("DEX_SMART_DEBT_NOT_ENABLED")]
    DexSmartDebtNotEnabled = 6020,
    #[error("DEX_POOL_NOT_INITIALIZED")]
    DexPoolNotInitialized = 6021,
    #[error("DEX_TOKEN_RESERVES_TOO_LOW")]
    DexTokenReservesTooLow = 6022,
    #[error("DEX_NO_SWAP_ROUTE")]
    DexNoSwapRoute = 6023,
    #[error("DEX_NOT_ENOUGH_AMOUNT_OUT")]
    DexNotEnoughAmountOut = 6024,
    #[error("DEX_UTILIZATION_CAP_REACHED")]
    DexUtilizationCapReached = 6025,
    #[error("DEX_USER_SUPPLY_NOT_ON")]
    DexUserSupplyNotOn = 6026,
    #[error("DEX_USER_DEBT_NOT_ON")]
    DexUserDebtNotOn = 6027,
    #[error("DEX_ABOVE_DEPOSIT_MAX")]
    DexAboveDepositMax = 6028,
    #[error("DEX_WITHDRAWAL_LIMIT_REACHED")]
    DexWithdrawalLimitReached = 6029,
    #[error("DEX_BELOW_WITHDRAW_MIN")]
    DexBelowWithdrawMin = 6030,
    #[error("DEX_DEBT_LIMIT_REACHED")]
    DexDebtLimitReached = 6031,
    #[error("DEX_BELOW_BORROW_MIN")]
    DexBelowBorrowMin = 6032,
    #[error("DEX_ABOVE_PAYBACK_MAX")]
    DexAbovePaybackMax = 6033,
    #[error("DEX_INVALID_DEPOSIT_AMOUNTS")]
    DexInvalidDepositAmounts = 6034,
    #[error("DEX_DEPOSIT_AMOUNTS_ZERO")]
    DexDepositAmountsZero = 6035,
    #[error("DEX_SHARES_MINTED_LESS")]
    DexSharesMintedLess = 6036,
    #[error("DEX_WITHDRAWAL_NOT_ENOUGH")]
    DexWithdrawalNotEnough = 6037,
    #[error("DEX_INVALID_WITHDRAW_AMOUNTS")]
    DexInvalidWithdrawAmounts = 6038,
    #[error("DEX_WITHDRAW_AMOUNTS_ZERO")]
    DexWithdrawAmountsZero = 6039,
    #[error("DEX_WITHDRAW_EXCESS_SHARES_BURN")]
    DexWithdrawExcessSharesBurn = 6040,
    #[error("DEX_INVALID_BORROW_AMOUNTS")]
    DexInvalidBorrowAmounts = 6041,
    #[error("DEX_BORROW_AMOUNTS_ZERO")]
    DexBorrowAmountsZero = 6042,
    #[error("DEX_BORROW_EXCESS_SHARES_MINTED")]
    DexBorrowExcessSharesMinted = 6043,
    #[error("DEX_INVALID_PAYBACK_AMOUNTS")]
    DexInvalidPaybackAmounts = 6044,
    #[error("DEX_PAYBACK_AMOUNTS_ZERO")]
    DexPaybackAmountsZero = 6045,
    #[error("DEX_PAYBACK_SHARES_BURNED_LESS")]
    DexPaybackSharesBurnedLess = 6046,
    #[error("DEX_NOTHING_TO_ARBITRAGE")]
    DexNothingToArbitrage = 6047,
    #[error("DEX_ORACLE_UPDATE_HUGE_SWAP_DIFF")]
    DexOracleUpdateHugeSwapDiff = 6048,
    #[error("DEX_TOKEN0_SHOULD_BE_SMALLER_THAN_TOKEN1")]
    DexToken0ShouldBeSmallerThanToken1 = 6049,
    #[error("DEX_SWAP_AND_ARBITRAGE_PAUSED")]
    DexSwapAndArbitragePaused = 6050,
    #[error("DEX_EXCEEDS_AMOUNT_IN_MAX")]
    DexExceedsAmountInMax = 6051,
    #[error("DEX_SWAP_IN_LIMITING_AMOUNTS")]
    DexSwapInLimitingAmounts = 6052,
    #[error("DEX_SWAP_OUT_LIMITING_AMOUNTS")]
    DexSwapOutLimitingAmounts = 6053,
    #[error("DEX_SUPPLY_SHARES_OVERFLOW")]
    DexSupplySharesOverflow = 6054,
    #[error("DEX_BORROW_SHARES_OVERFLOW")]
    DexBorrowSharesOverflow = 6055,
    #[error("DEX_CENTER_PRICE_OUT_OF_RANGE")]
    DexCenterPriceOutOfRange = 6056,
    #[error("DEX_DEBT_RESERVES_TOO_LOW")]
    DexDebtReservesTooLow = 6057,
    #[error("DEX_INVALID_COLLATERAL_RESERVES")]
    DexInvalidCollateralReserves = 6058,
    #[error("DEX_INVALID_DEBT_RESERVES")]
    DexInvalidDebtReserves = 6059,
    #[error("DEX_LIMITING_AMOUNTS_SWAP_AND_NON_PERFECT")]
    DexLimitingAmountsSwapAndNonPerfect = 6060,
    #[error("DEX_MINT_OVERFLOW")]
    DexMintOverflow = 6061,
    #[error("DEX_BURN_OVERFLOW")]
    DexBurnOverflow = 6062,
    #[error("DEX_SHARES_AMOUNT_INSUFFICIENT")]
    DexSharesAmountInsufficient = 6063,
    #[error("DEX_PERFECT_NATIVE_AMOUNTS_ROUND_TO_ZERO")]
    DexPerfectNativeAmountsRoundToZero = 6064,
    #[error("DEX_CPI_TO_LIQUIDITY_FAILED")]
    DexCpiToLiquidityFailed = 6065,
    #[error("DEX_INVALID_TOKEN_ACCOUNT")]
    DexInvalidTokenAccount = 6066,
    #[error("DEX_MISSING_EXTERNAL_CENTER_PRICE")]
    DexMissingExternalCenterPrice = 6067,
    #[error("DEX_INVALID_EXTERNAL_CENTER_PRICE")]
    DexInvalidExternalCenterPrice = 6068,
    #[error("DEX_PAYBACK_AMT_TOO_HIGH")]
    DexPaybackAmtTooHigh = 6069,
    #[error("DEX_SWAP_AND_PAYBACK_TOO_LOW_OR_TOO_HIGH")]
    DexSwapAndPaybackTooLowOrTooHigh = 6070,
    #[error("DEX_INVALID_DEX_ID")]
    DexValidateInvalidDexId = 6071,
    #[error("DEX_INVALID_TOKEN0")]
    DexValidateInvalidToken0 = 6072,
    #[error("DEX_INVALID_TOKEN1")]
    DexValidateInvalidToken1 = 6073,
    #[error("DEX_INVALID_POSITION")]
    DexValidateInvalidPosition = 6074,
    #[error("DEX_INVALID_LIQUIDITY_PROGRAM")]
    DexValidateInvalidLiquidityProgram = 6075,
    #[error("DEX_INVALID_ORACLE_PROGRAM")]
    DexValidateInvalidOracleProgram = 6076,
    #[error("DEX_INVALID_LIQUIDITY_POSITION")]
    DexValidateInvalidLiquidityPosition = 6077,
    #[error("DEX_MISSING_LIQUIDITY_POSITION")]
    DexValidateMissingLiquidityPosition = 6078,
    #[error("DEX_INVALID_RECIPIENT_POSITION_WITHDRAW")]
    DexInvalidRecipientPositionWithdraw = 6079,
    #[error("DEX_INVALID_RECIPIENT_POSITION_BORROW")]
    DexInvalidRecipientPositionBorrow = 6080,
    #[error("DEX_SWAP_IN_RESULT")]
    DexSwapInResult = 6081,
    #[error("DEX_SWAP_OUT_RESULT")]
    DexSwapOutResult = 6082,
}
impl From<JupiterLendAmmError> for ProgramError {
    fn from(e: JupiterLendAmmError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
