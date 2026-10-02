use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum JupiterLendError {
    #[error("F_TOKEN_DEPOSIT_INSIGNIFICANT")]
    FTokenDepositInsignificant = 6000,
    #[error("F_TOKEN_MIN_AMOUNT_OUT")]
    FTokenMinAmountOut = 6001,
    #[error("F_TOKEN_MAX_AMOUNT")]
    FTokenMaxAmount = 6002,
    #[error("F_TOKEN_INVALID_PARAMS")]
    FTokenInvalidParams = 6003,
    #[error("F_TOKEN_REWARDS_RATE_MODEL_ALREADY_SET")]
    FTokenRewardsRateModelAlreadySet = 6004,
    #[error("F_TOKEN_MAX_AUTH_COUNT")]
    FTokenMaxAuthCountReached = 6005,
    #[error("F_TOKEN_LIQUIDITY_EXCHANGE_PRICE_UNEXPECTED")]
    FTokenLiquidityExchangePriceUnexpected = 6006,
    #[error("F_TOKEN_CPI_TO_LIQUIDITY_FAILED")]
    FTokenCpiToLiquidityFailed = 6007,
    #[error("F_TOKEN_ONLY_AUTH")]
    FTokenOnlyAuth = 6008,
    #[error("F_TOKEN_ONLY_AUTHORITY")]
    FTokenOnlyAuthority = 6009,
    #[error("F_TOKEN_ONLY_REBALANCER")]
    FTokenOnlyRebalancer = 6010,
    #[error("F_TOKEN_USER_SUPPLY_POSITION_REQUIRED")]
    FTokenUserSupplyPositionRequired = 6011,
    #[error("F_TOKEN_LIQUIDITY_PROGRAM_MISMATCH")]
    FTokenLiquidityProgramMismatch = 6012,
}
impl From<JupiterLendError> for ProgramError {
    fn from(e: JupiterLendError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
