use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum AmmV3Error {
    #[error("Not approved")]
    NotApproved = 6000,
    #[error("invalid update amm config flag")]
    InvalidUpdateConfigFlag = 6001,
    #[error("Account lack")]
    AccountLack = 6002,
    #[error(
        "Remove liquidity, collect fees owed and reward then you can close position account"
    )]
    ClosePositionErr = 6003,
    #[error("Tick out of range")]
    InvalidTickIndex = 6004,
    #[error("The lower tick must be below the upper tick")]
    TickInvalidOrder = 6005,
    #[error("The tick must be greater, or equal to the minimum tick(-443636)")]
    TickLowerOverflow = 6006,
    #[error("The tick must be lesser than, or equal to the maximum tick(443636)")]
    TickUpperOverflow = 6007,
    #[error("tick % tick_spacing must be zero")]
    TickAndSpacingNotMatch = 6008,
    #[error("Invalid tick array account")]
    InvalidTickArray = 6009,
    #[error("Invalid tick array boundary")]
    InvalidTickArrayBoundary = 6010,
    #[error("Square root price limit overflow")]
    SqrtPriceLimitOverflow = 6011,
    #[error("sqrt_price_x64 out of range")]
    SqrtPriceX64 = 6012,
    #[error("Liquidity sub delta L must be smaller than before")]
    LiquiditySubValueErr = 6013,
    #[error("Liquidity add delta L must be greater, or equal to before")]
    LiquidityAddValueErr = 6014,
    #[error("Both token amount must not be zero while supply liquidity")]
    ForbidBothZeroForSupplyLiquidity = 6015,
    #[error("Liquidity insufficient")]
    LiquidityInsufficient = 6016,
    #[error("Price slippage check")]
    PriceSlippageCheck = 6017,
    #[error("Too little output received")]
    TooLittleOutputReceived = 6018,
    #[error("Too much input paid")]
    TooMuchInputPaid = 6019,
    #[error("Swap special amount can not be zero")]
    ZeroAmountSpecified = 6020,
    #[error("Input pool vault is invalid")]
    InvalidInputPoolVault = 6021,
    #[error("Swap input or output amount is too small")]
    TooSmallInputOrOutputAmount = 6022,
    #[error("Not enough tick array account")]
    NotEnoughTickArrayAccount = 6023,
    #[error("Invalid first tick array account")]
    InvalidFirstTickArrayAccount = 6024,
    #[error("Invalid reward index")]
    InvalidRewardIndex = 6025,
    #[error("The init reward token reach to the max")]
    FullRewardInfo = 6026,
    #[error("The init reward token already in use")]
    RewardTokenAlreadyInUse = 6027,
    #[error(
        "The reward tokens must contain one of pool vault mint except the last reward"
    )]
    ExceptRewardMint = 6028,
    #[error("Invalid reward init param")]
    InvalidRewardInitParam = 6029,
    #[error("Invalid collect reward input account number")]
    InvalidRewardInputAccountNumber = 6030,
    #[error("Invalid reward period")]
    InvalidRewardPeriod = 6031,
    #[error(
        "Modification of emissions is allowed within 72 hours from the end of the previous cycle"
    )]
    NotApproveUpdateRewardEmissions = 6032,
    #[error("uninitialized reward info")]
    UnInitializedRewardInfo = 6033,
    #[error("Not support token_2022 mint extension")]
    NotSupportMint = 6034,
    #[error("Missing tickarray bitmap extension account")]
    MissingTickArrayBitmapExtensionAccount = 6035,
    #[error("Insufficient liquidity for this direction")]
    InsufficientLiquidityForDirection = 6036,
    #[error("Max token overflow")]
    MaxTokenOverflow = 6037,
    #[error("Calculate overflow")]
    CalculateOverflow = 6038,
    #[error("TransferFee calculate not match")]
    TransferFeeCalculateNotMatch = 6039,
    #[error("Order already fully filled, cannot modify")]
    OrderAlreadyFilled = 6040,
    #[error("Invalid order phase")]
    InvalidOrderPhase = 6041,
    #[error("Invalid limit order amount")]
    InvalidLimitOrderAmount = 6042,
    #[error("Tick order phase saturated")]
    OrderPhaseSaturated = 6043,
    #[error("Invalid dynamic fee config params")]
    InvalidDynamicFeeConfigParams = 6044,
    #[error("Invalid fee on which token (must be 0, 1, or 2)")]
    InvalidFeeOn = 6045,
    #[error("sqrt_price_x64 must be greater than 0")]
    ZeroSqrtPrice = 6046,
    #[error("liquidity must be greater than 0")]
    ZeroLiquidity = 6047,
    #[error("base_flag is required when liquidity is zero")]
    MissingBaseFlag = 6048,
    #[error("Mint account is required but not provided")]
    MissingMintAccount = 6049,
    #[error("Token-2022 program is required but not provided")]
    MissingTokenProgram2022 = 6050,
}
impl From<AmmV3Error> for ProgramError {
    fn from(e: AmmV3Error) -> Self {
        ProgramError::Custom(e as u32)
    }
}
