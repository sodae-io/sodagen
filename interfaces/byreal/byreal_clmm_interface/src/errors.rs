use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum ByrealClmmError {
    #[error("LOK")]
    Lok = 6000,
    #[error("Not approved")]
    NotApproved = 6001,
    #[error("invalid update amm config flag")]
    InvalidUpdateConfigFlag = 6002,
    #[error("Account lack")]
    AccountLack = 6003,
    #[error(
        "Remove liquitity, collect fees owed and reward then you can close position account"
    )]
    ClosePositionErr = 6004,
    #[error("Minting amount should be greater than 0")]
    ZeroMintAmount = 6005,
    #[error("Tick out of range")]
    InvalidTickIndex = 6006,
    #[error("The lower tick must be below the upper tick")]
    TickInvalidOrder = 6007,
    #[error("The tick must be greater, or equal to the minimum tick(-443636)")]
    TickLowerOverflow = 6008,
    #[error("The tick must be lesser than, or equal to the maximum tick(443636)")]
    TickUpperOverflow = 6009,
    #[error("tick % tick_spacing must be zero")]
    TickAndSpacingNotMatch = 6010,
    #[error("Invalid tick array account")]
    InvalidTickArray = 6011,
    #[error("Invalid tick array boundary")]
    InvalidTickArrayBoundary = 6012,
    #[error("Square root price limit overflow")]
    SqrtPriceLimitOverflow = 6013,
    #[error("sqrt_price_x64 out of range")]
    SqrtPriceX64 = 6014,
    #[error("Liquidity sub delta L must be smaller than before")]
    LiquiditySubValueErr = 6015,
    #[error("Liquidity add delta L must be greater, or equal to before")]
    LiquidityAddValueErr = 6016,
    #[error("Invalid liquidity when update position")]
    InvalidLiquidity = 6017,
    #[error("Both token amount must not be zero while supply liquidity")]
    ForbidBothZeroForSupplyLiquidity = 6018,
    #[error("Liquidity insufficient")]
    LiquidityInsufficient = 6019,
    #[error("Transaction too old")]
    TransactionTooOld = 6020,
    #[error("Price slippage check")]
    PriceSlippageCheck = 6021,
    #[error("Too little output received")]
    TooLittleOutputReceived = 6022,
    #[error("Too much input paid")]
    TooMuchInputPaid = 6023,
    #[error("Swap special amount can not be zero")]
    ZeroAmountSpecified = 6024,
    #[error("Input pool vault is invalid")]
    InvalidInputPoolVault = 6025,
    #[error("Swap input or output amount is too small")]
    TooSmallInputOrOutputAmount = 6026,
    #[error("Not enought tick array account")]
    NotEnoughTickArrayAccount = 6027,
    #[error("Invalid first tick array account")]
    InvalidFirstTickArrayAccount = 6028,
    #[error("Invalid reward index")]
    InvalidRewardIndex = 6029,
    #[error("The init reward token reach to the max")]
    FullRewardInfo = 6030,
    #[error("The init reward token already in use")]
    RewardTokenAlreadyInUse = 6031,
    #[error(
        "The reward tokens must contain one of pool vault mint except the last reward"
    )]
    ExceptRewardMint = 6032,
    #[error("Invalid reward init param")]
    InvalidRewardInitParam = 6033,
    #[error("Invalid collect reward desired amount")]
    InvalidRewardDesiredAmount = 6034,
    #[error("Invalid collect reward input account number")]
    InvalidRewardInputAccountNumber = 6035,
    #[error("Invalid reward period")]
    InvalidRewardPeriod = 6036,
    #[error(
        "Modification of emissiones is allowed within 72 hours from the end of the previous cycle"
    )]
    NotApproveUpdateRewardEmissiones = 6037,
    #[error("uninitialized reward info")]
    UnInitializedRewardInfo = 6038,
    #[error("Not support token_2022 mint extension")]
    NotSupportMint = 6039,
    #[error("Missing tickarray bitmap extension account")]
    MissingTickArrayBitmapExtensionAccount = 6040,
    #[error("Insufficient liquidity for this direction")]
    InsufficientLiquidityForDirection = 6041,
    #[error("Max token overflow")]
    MaxTokenOverflow = 6042,
    #[error("Calculate overflow")]
    CalculateOverflow = 6043,
    #[error("TransferFee calculate not match")]
    TransferFeeCalculateNotMatch = 6044,
    #[error("invalid account owner")]
    IllegalAccountOwner = 6045,
    #[error("Invalid account")]
    InvalidAccount = 6046,
    #[error("Invalid decay fee params")]
    DecayFeeNeitherOnSellMint0NorMint1 = 6047,
    #[error("Swap dynamic fee is enabled, must use swap_v3_dyn instruction")]
    SwapDynamicFeeEnabled = 6048,
    #[error("Invalid pyth oracle account")]
    InvalidPythOracleAccount = 6049,
    #[error("Pyth price is stale")]
    PythPriceStale = 6050,
    #[error("Invalid swap dynamic fee params")]
    InvalidSwapDynamicFeeParams = 6051,
}
impl From<ByrealClmmError> for ProgramError {
    fn from(e: ByrealClmmError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
