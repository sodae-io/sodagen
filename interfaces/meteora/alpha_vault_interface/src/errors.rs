use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum AlphaVaultError {
    #[error("Time point is not in future")]
    TimePointNotInFuture = 6000,
    #[error("Token mint is incorrect")]
    IncorrectTokenMint = 6001,
    #[error("Pair is not permissioned")]
    IncorrectPairType = 6002,
    #[error("Pool has started")]
    PoolHasStarted = 6003,
    #[error("This action is not permitted in this time point")]
    NotPermitThisActionInThisTimePoint = 6004,
    #[error("The sale is on going, cannot withdraw")]
    TheSaleIsOngoing = 6005,
    #[error("Escrow is not closable")]
    EscrowIsNotClosable = 6006,
    #[error("Time point orders are incorrect")]
    TimePointOrdersAreIncorrect = 6007,
    #[error("Escrow has refunded")]
    EscrowHasRefunded = 6008,
    #[error("Math operation overflow")]
    MathOverflow = 6009,
    #[error("Max buying cap is zero")]
    MaxBuyingCapIsZero = 6010,
    #[error("Max amount is too small")]
    MaxAmountIsTooSmall = 6011,
    #[error("Pool type is not supported")]
    PoolTypeIsNotSupported = 6012,
    #[error("Invalid admin")]
    InvalidAdmin = 6013,
    #[error("Vault mode is incorrect")]
    VaultModeIsIncorrect = 6014,
    #[error("Max depositing cap is invalid")]
    MaxDepositingCapIsInValid = 6015,
    #[error("Vesting duration is invalid")]
    VestingDurationIsInValid = 6016,
    #[error("Deposit amount is zero")]
    DepositAmountIsZero = 6017,
    #[error("Pool owner is mismatched")]
    PoolOwnerIsMismatched = 6018,
    #[error("Withdraw amount is zero")]
    WithdrawAmountIsZero = 6019,
    #[error("Depositing duration is invalid")]
    DepositingDurationIsInvalid = 6020,
    #[error("Depositing time point is invalid")]
    DepositingTimePointIsInvalid = 6021,
    #[error("Individual depositing cap is zero")]
    IndividualDepositingCapIsZero = 6022,
    #[error("Invalid fee receiver account")]
    InvalidFeeReceiverAccount = 6023,
    #[error("Not permissioned vault")]
    NotPermissionedVault = 6024,
    #[error("Not permit to do this action")]
    NotPermitToDoThisAction = 6025,
    #[error("Invalid Merkle proof")]
    InvalidProof = 6026,
    #[error("Invalid activation type")]
    InvalidActivationType = 6027,
    #[error("Activation type is mismatched")]
    ActivationTypeIsMismatched = 6028,
    #[error("Pool is not connected to the alpha vault")]
    InvalidPool = 6029,
    #[error("Invalid creator")]
    InvalidCreator = 6030,
    #[error("Permissioned vault cannot charge escrow fee")]
    PermissionedVaultCannotChargeEscrowFee = 6031,
    #[error("Escrow fee too high")]
    EscrowFeeTooHigh = 6032,
    #[error("Lock duration is invalid")]
    LockDurationInvalid = 6033,
    #[error("Max buying cap is too small")]
    MaxBuyingCapIsTooSmall = 6034,
    #[error("Max depositing cap is too small")]
    MaxDepositingCapIsTooSmall = 6035,
    #[error("Invalid whitelist wallet mode")]
    InvalidWhitelistWalletMode = 6036,
    #[error("Invalid crank fee whitelist")]
    InvalidCrankFeeWhitelist = 6037,
    #[error("Missing fee receiver")]
    MissingFeeReceiver = 6038,
    #[error("Discriminator is mismatched")]
    DiscriminatorIsMismatched = 6039,
}
impl From<AlphaVaultError> for ProgramError {
    fn from(e: AlphaVaultError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
