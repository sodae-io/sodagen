use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum HumaError {
    #[error("ZeroAmountProvided")]
    ZeroAmountProvided = 6001,
    #[error("ZeroSharesMinted")]
    ZeroSharesMinted = 6002,
    #[error("InvalidNumberOfRemainingAccounts")]
    InvalidNumberOfRemainingAccounts = 6003,
    #[error("InvalidStrategyProgram")]
    InvalidStrategyProgram = 6004,
    #[error("TargetAccountMismatch")]
    TargetAccountMismatch = 6005,
    #[error("InvalidBasisPointHigherThan10000")]
    InvalidBasisPointHigherThan10000 = 6006,
    #[error("RequiredAccountsNotProvided")]
    RequiredAccountsNotProvided = 6007,
    #[error("PoolOwnerRequired")]
    PoolOwnerRequired = 6101,
    #[error("PoolOwnerOrHumaOwnerRequired")]
    PoolOwnerOrHumaOwnerRequired = 6102,
    #[error("PoolOwnerTreasuryRequired")]
    PoolOwnerTreasuryRequired = 6103,
    #[error("PoolOwnerOrSentinelRequired")]
    PoolOwnerOrSentinelRequired = 6104,
    #[error("PoolOperatorRequired")]
    PoolOperatorRequired = 6105,
    #[error("RedemptionRequestProcessorRequired")]
    RedemptionRequestProcessorRequired = 6106,
    #[error("InstantWithdrawalLenderRequired")]
    InstantWithdrawalLenderRequired = 6107,
    #[error("LossAuthorityRequired")]
    LossAuthorityRequired = 6108,
    #[error("HumaOwnerRequired")]
    HumaOwnerRequired = 6109,
    #[error("InvalidHumaConfig")]
    InvalidHumaConfig = 6201,
    #[error("InvalidLiquidityAsset")]
    InvalidLiquidityAsset = 6202,
    #[error("InvalidUnderlyingMint")]
    InvalidUnderlyingMint = 6203,
    #[error("ProtocolIsPaused")]
    ProtocolIsPaused = 6204,
    #[error("PoolIsNotOn")]
    PoolIsNotOn = 6301,
    #[error("PoolIsNotOnOrInPreClosure")]
    PoolIsNotOnOrInPreClosure = 6302,
    #[error("PoolIsNotInPreClosure")]
    PoolIsNotInPreClosure = 6303,
    #[error("PoolIsNotClosed")]
    PoolIsNotClosed = 6304,
    #[error("PoolIsClosed")]
    PoolIsClosed = 6305,
    #[error("PoolNameTooLong")]
    PoolNameTooLong = 6306,
    #[error("ModeNameTooLong")]
    ModeNameTooLong = 6307,
    #[error("TooManyModes")]
    TooManyModes = 6308,
    #[error("TooFewModes")]
    TooFewModes = 6309,
    #[error("DuplicateModes")]
    DuplicateModes = 6310,
    #[error("MinDepositAmountTooLow")]
    MinDepositAmountTooLow = 6311,
    #[error("MinRedemptionSharesTooLow")]
    MinRedemptionSharesTooLow = 6312,
    #[error("LenderMismatch")]
    LenderMismatch = 6313,
    #[error("RedemptionRequestOutOfOrder")]
    RedemptionRequestOutOfOrder = 6314,
    #[error("InsufficientBalanceForRedemptionProcessing")]
    InsufficientBalanceForRedemptionProcessing = 6315,
    #[error("TotalRedemptionLimitExceeded")]
    TotalRedemptionLimitExceeded = 6316,
    #[error("InsufficientSharesForRequest")]
    InsufficientSharesForRequest = 6317,
    #[error("DifferentModesRequired")]
    DifferentModesRequired = 6318,
    #[error("UnprocessedRedemptionRequests")]
    UnprocessedRedemptionRequests = 6319,
    #[error("InsufficientBalanceForWithdrawal")]
    InsufficientBalanceForWithdrawal = 6320,
    #[error("InsufficientAssetsForPoolActivation")]
    InsufficientAssetsForPoolActivation = 6321,
    #[error("PayerMismatch")]
    PayerMismatch = 6322,
    #[error("PlaceholderError323")]
    PlaceholderError323 = 6323,
    #[error("TotalRedemptionLimitBelowInstantWithdrawalLimit")]
    TotalRedemptionLimitBelowInstantWithdrawalLimit = 6324,
    #[error("TooFewInstantWithdrawalFeeConfigs")]
    TooFewInstantWithdrawalFeeConfigs = 6325,
    #[error("TooManyInstantWithdrawalFeeConfigs")]
    TooManyInstantWithdrawalFeeConfigs = 6326,
    #[error("InvalidInstantWithdrawalFeeConfigOutOfOrder")]
    InvalidInstantWithdrawalFeeConfigOutOfOrder = 6327,
    #[error("LastFeeConfigRatioMustBe100Pct")]
    LastFeeConfigRatioMustBe100Pct = 6328,
    #[error("InstantWithdrawalFeeTooHigh")]
    InstantWithdrawalFeeTooHigh = 6329,
    #[error("InstantWithdrawalDisabled")]
    InstantWithdrawalDisabled = 6330,
    #[error("NoInstantWithdrawalFeeConfigForRatio")]
    NoInstantWithdrawalFeeConfigForRatio = 6331,
    #[error("LiquidAssetsDeployedTooHigh")]
    LiquidAssetsDeployedTooHigh = 6332,
    #[error("RedemptionRequestModeMismatch")]
    RedemptionRequestModeMismatch = 6333,
    #[error("InstantWithdrawalLiquiditySourceMismatch")]
    InstantWithdrawalLiquiditySourceMismatch = 6334,
    #[error("InvalidPoolConfig")]
    InvalidPoolConfig = 6335,
    #[error("InvalidModeConfig")]
    InvalidModeConfig = 6401,
    #[error("DepositAmountTooLow")]
    DepositAmountTooLow = 6402,
    #[error("LiquidityCapExceeded")]
    LiquidityCapExceeded = 6403,
    #[error("InsufficientModeSupply")]
    InsufficientModeSupply = 6404,
    #[error("RedemptionSharesTooLow")]
    RedemptionSharesTooLow = 6405,
    #[error("ModeAssetsStale")]
    ModeAssetsStale = 6406,
    #[error("ModeTokenNotReadyToClose")]
    ModeTokenNotReadyToClose = 6407,
    #[error("LenderStateNotReadyToClose")]
    LenderStateNotReadyToClose = 6408,
    #[error("ModeMintNotReadyToClose")]
    ModeMintNotReadyToClose = 6409,
    #[error("ModeStateNotReadyToClose")]
    ModeStateNotReadyToClose = 6410,
    #[error("TokenMetadataMintMismatch")]
    TokenMetadataMintMismatch = 6411,
    #[error("ZeroAssetsSwitched")]
    ZeroAssetsSwitched = 6412,
    #[error("StrategyManagerWalletMismatch")]
    StrategyManagerWalletMismatch = 6501,
    #[error("InsufficientBalanceForDeployment")]
    InsufficientBalanceForDeployment = 6502,
    #[error("DeclaredLossTooHigh")]
    DeclaredLossTooHigh = 6503,
    #[error("NoLossToRecover")]
    NoLossToRecover = 6504,
    #[error("TokenAccountOwnerMismatch")]
    TokenAccountOwnerMismatch = 6505,
    #[error("ManualStrategyNotAllowedAsLiquiditySource")]
    ManualStrategyNotAllowedAsLiquiditySource = 6506,
    #[error("DeploymentTargetIsActiveLiquiditySource")]
    DeploymentTargetIsActiveLiquiditySource = 6507,
    #[error("StrategyWithdrawalAmountMismatch")]
    StrategyWithdrawalAmountMismatch = 6508,
    #[error("ManualDeploymentDailyLimitExceeded")]
    ManualDeploymentDailyLimitExceeded = 6509,
    #[error("ManualDeploymentPerWalletLimitExceeded")]
    ManualDeploymentPerWalletLimitExceeded = 6510,
    #[error("ManualStrategyNotAllowed")]
    ManualStrategyNotAllowed = 6511,
    #[error("ManualStrategyManagerProposalExpired")]
    ManualStrategyManagerProposalExpired = 6512,
    #[error("ManualDeploymentTargetWalletMismatch")]
    ManualDeploymentTargetWalletMismatch = 6513,
    #[error("JupLendFTokenATARequired")]
    JupLendFTokenAtaRequired = 6514,
    #[error("KaminoLendingMarketMismatch")]
    KaminoLendingMarketMismatch = 6515,
}
impl From<HumaError> for ProgramError {
    fn from(e: HumaError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
