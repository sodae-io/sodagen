use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum LimoError {
    #[error("Order can't be canceled")]
    OrderCanNotBeCanceled = 6000,
    #[error("Order not active")]
    OrderNotActive = 6001,
    #[error("Invalid admin authority")]
    InvalidAdminAuthority = 6002,
    #[error("Invalid pda authority")]
    InvalidPdaAuthority = 6003,
    #[error("Invalid config option")]
    InvalidConfigOption = 6004,
    #[error("Order owner account is not the order owner")]
    InvalidOrderOwner = 6005,
    #[error("Out of range integral conversion attempted")]
    OutOfRangeIntegralConversion = 6006,
    #[error("Invalid boolean flag, valid values are 0 and 1")]
    InvalidFlag = 6007,
    #[error("Mathematical operation with overflow")]
    MathOverflow = 6008,
    #[error("Order input amount invalid")]
    OrderInputAmountInvalid = 6009,
    #[error("Order output amount invalid")]
    OrderOutputAmountInvalid = 6010,
    #[error("Host fee bps must be between 0 and 10000")]
    InvalidHostFee = 6011,
    #[error("Conversion between integers failed")]
    IntegerOverflow = 6012,
    #[error("Tip balance less than accounted tip")]
    InvalidTipBalance = 6013,
    #[error("Tip transfer amount is less than expected")]
    InvalidTipTransferAmount = 6014,
    #[error("Host tup amount is less than accounted for")]
    InvalidHostTipBalance = 6015,
    #[error("Order within flash operation - all otehr actions are blocked")]
    OrderWithinFlashOperation = 6016,
    #[error("CPI not allowed")]
    CpiNotAllowed = 6017,
    #[error("Flash take_order is blocked")]
    FlashTakeOrderBlocked = 6018,
    #[error(
        "Some unexpected instructions are present in the tx. Either before or after the flash ixs, or some ix target the same program between"
    )]
    FlashTxWithUnexpectedIxs = 6019,
    #[error("Flash ixs initiated without the closing ix in the transaction")]
    FlashIxsNotEnded = 6020,
    #[error("Flash ixs ended without the starting ix in the transaction")]
    FlashIxsNotStarted = 6021,
    #[error("Some accounts differ between the two flash ixs")]
    FlashIxsAccountMismatch = 6022,
    #[error("Some args differ between the two flash ixs")]
    FlashIxsArgsMismatch = 6023,
    #[error("Order is not within flash operation")]
    OrderNotWithinFlashOperation = 6024,
    #[error("Emergency mode is enabled")]
    EmergencyModeEnabled = 6025,
    #[error("Creating new ordersis blocked")]
    CreatingNewOrdersBlocked = 6026,
    #[error("Orders taking is blocked")]
    OrderTakingBlocked = 6027,
    #[error("Order input amount larger than the remaining")]
    OrderInputAmountTooLarge = 6028,
    #[error(
        "Permissionless order taking not enabled, please provide permission account"
    )]
    PermissionRequiredPermissionlessNotEnabled = 6029,
    #[error("Permission address does not match order address")]
    PermissionDoesNotMatchOrder = 6030,
    #[error("Invalid ata address")]
    InvalidAtaAddress = 6031,
    #[error("Maker output ata required when output mint is not WSOL")]
    MakerOutputAtaRequired = 6032,
    #[error("Intermediary output token account required when output mint is WSOL")]
    IntermediaryOutputTokenAccountRequired = 6033,
    #[error("Not enough balance for rent")]
    NotEnoughBalanceForRent = 6034,
    #[error("Order can not be closed - Not enough time passed since last update")]
    NotEnoughTimePassedSinceLastUpdate = 6035,
    #[error("Order input and output mints are the same")]
    OrderSameMint = 6036,
    #[error("Mint has a token (2022) extension that is not supported")]
    UnsupportedTokenExtension = 6037,
    #[error("Can't have an spl token mint with a t22 account")]
    InvalidTokenAccount = 6038,
    #[error("The order type is invalid")]
    OrderTypeInvalid = 6039,
    #[error("Token account is not initialized")]
    UninitializedTokenAccount = 6040,
    #[error("Account is not owned by the token program")]
    InvalidTokenAccountOwner = 6041,
    #[error("Account is not a valid token account")]
    InvalidAccount = 6042,
    #[error("Token account has incorrect mint")]
    InvalidTokenMint = 6043,
    #[error("Token account has incorrect authority")]
    InvalidTokenAuthority = 6044,
    #[error("The provided parameter type is invalid")]
    InvalidParameterType = 6045,
    #[error("The counterparty is not the taker")]
    CounterpartyDisallowed = 6046,
    #[error("The swap input amount is larger than the maximum allowed")]
    SwapInputAmountTooLarge = 6047,
    #[error("The swap output amount is smaller than the minimum allowed")]
    SwapOutputAmountTooSmall = 6048,
    #[error("The swap input balance change is positive, expected negative")]
    SwapInputInvalidBalanceChange = 6049,
    #[error("The swap output balance change is negative, expected positive")]
    SwapOutputInvalidBalanceChange = 6050,
}
impl From<LimoError> for ProgramError {
    fn from(e: LimoError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
