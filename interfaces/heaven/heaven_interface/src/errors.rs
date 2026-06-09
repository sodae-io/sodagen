use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum HeavenError {
    #[error("Unsupported token mint")]
    UnsupportedTokenMint = 6000,
    #[error("Invalid token vault balance")]
    InvalidTokenVaultBalance = 6001,
    #[error("Invalid user token")]
    InvalidUserToken = 6002,
    #[error("Invalid taxation mode")]
    InvalidTaxationMode = 6003,
    #[error("Invalid owner")]
    InvalidOwner = 6004,
    #[error("Invalid lock liquidity provider token percentage")]
    InvalidLockLiquidityProviderTokenPercentage = 6005,
    #[error("Cannot create pool with the a disabled protocol config version")]
    CannotCreatePoolWithDisabledProtocolConfigVersion = 6006,
    #[error("Invalid token input amount")]
    InvalidTokenInputAmount = 6007,
    #[error("Invalid swap tax")]
    InvalidSwapTax = 6008,
    #[error("Invalid fee mode")]
    InvalidFeeMode = 6009,
    #[error("Invalid liquidity provider token lock vault")]
    InvalidLiquidityProviderTokenLockVault = 6010,
    #[error("Invalid liquidity provider token vault")]
    InvalidUserLiquidityProviderTokenVault = 6011,
    #[error("Insufficient balance")]
    InsufficientBalance = 6012,
    #[error("Exceeded slippage")]
    ExceededSlippage = 6013,
    #[error("Invalid add liquidity input")]
    InvalidAddLiquidityInput = 6014,
    #[error("Invalid remove liquidity input")]
    InvalidRemoveLiquidityInput = 6015,
    #[error("Add liquidity is disabled")]
    AddLiquidityDisabled = 6016,
    #[error("Remove liquidity is disabled")]
    RemoveLiquidityDisabled = 6017,
    #[error("Swap is disabled")]
    SwapDisabled = 6018,
    #[error("Liquidity pool is not open yet")]
    LiquidityPoolIsNotOpenYet = 6019,
    #[error("Invalid swap in inputs")]
    InvalidSwapInInputs = 6020,
    #[error("Invalid protocol swap fee wallet")]
    InvalidProtocolSwapFeeWallet = 6021,
    #[error("Invalid swap out inputs")]
    InvalidSwapOutInputs = 6022,
    #[error("Invalid post fee amount")]
    InvalidPostFeeAmount = 6023,
    #[error("Exceeded quote token slippage")]
    ExceededQuoteTokenSlippage = 6024,
    #[error("Exceeded base token slippage")]
    ExceededBaseTokenSlippage = 6025,
    #[error("Lp tokens locked")]
    LpTokensLocked = 6026,
    #[error("Invalid protocol base token swap fee vault")]
    InvalidProtocolBaseTokenSwapFeeVault = 6027,
    #[error("Invalid protocol quote token swap fee vault")]
    InvalidProtocolQuoteTokenSwapFeeVault = 6028,
    #[error("Invalid user pool stats account")]
    InvalidUserPoolStatsAccount = 6029,
    #[error("Invalid user global stats account")]
    InvalidUserGlobalStatsAccount = 6030,
    #[error("Cannot update lp lock")]
    CannotUpdateLpLock = 6031,
    #[error("Zero amount")]
    ZeroAmount = 6032,
    #[error("Cannot update lp open time")]
    CannotUpdateLpOpenTime = 6033,
    #[error("Cannot set lock burn lp tokens")]
    CannotSetLockBurnLpTokens = 6034,
    #[error("Invalid tax")]
    InvalidTax = 6035,
    #[error("Invalid chainlink feed account")]
    InvalidChainlinkFeedAccount = 6036,
    #[error("Invalid chainlink program")]
    InvalidChainlinkProgram = 6037,
    #[error("Invalid config version")]
    InvalidConfigVersion = 6038,
    #[error("Cannot update locked taxation")]
    CannotUpdateLockedTaxation = 6039,
    #[error("Cannot claim swap fee")]
    CannotClaimSwapFee = 6040,
    #[error("This pool does not allow non-creator to add lp")]
    NonCreatorCannotAddLp = 6041,
    #[error("Simple amm base token mint cannot have freeze authority")]
    SimpleAmmBaseTokenCannotHaveFreezeAuthority = 6042,
    #[error("Simple amm base token mint cannot have mint authority")]
    SimpleAmmBaseTokenCannotHaveMintAuthority = 6043,
    #[error("Simple amm quote token mint must be WSOL")]
    SimpleAmmQuoteTokenMustBeWsol = 6044,
    #[error("Simple amm base token supply must match input amount")]
    SimpleAmmBaseTokenSupplyMustMatchInputAmount = 6045,
    #[error("Simple amm base token mint must be SPL token")]
    SimpleAmmBaseTokenMustBeSplToken = 6046,
    #[error("Invalid program authority lookup table address")]
    InvalidProgramAuthorityLookupTableAddress = 6047,
    #[error("CannotCreatePoolWithUnsupportedPoolType")]
    CannotCreatePoolWithUnsupportedPoolType = 6048,
    #[error("InvalidLendingMarket")]
    InvalidLendingMarket = 6049,
    #[error("KaminoPriceListRefreshIxNotFound")]
    KaminoPriceListRefreshIxNotFound = 6050,
    #[error("KaminoError")]
    KaminoError = 6051,
    #[error("Reserve rebalancing, please try again.")]
    ReserveRebalancing = 6052,
    #[error("Only single swap allowed")]
    OnlySingleSwapAllowed = 6053,
}
impl From<HeavenError> for ProgramError {
    fn from(e: HeavenError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
