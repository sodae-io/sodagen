use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum VaultError {
    #[error("Vault is disabled")]
    VaultIsDisabled = 6000,
    #[error("Exceeded slippage tolerance")]
    ExceededSlippage = 6001,
    #[error("Strategy is not existed")]
    StrategyIsNotExisted = 6002,
    #[error("UnAuthorized")]
    UnAuthorized = 6003,
    #[error("Math operation overflow")]
    MathOverflow = 6004,
    #[error("Protocol is not supported")]
    ProtocolIsNotSupported = 6005,
    #[error("Reserve does not support token mint")]
    UnMatchReserve = 6006,
    #[error("lockedProfitDegradation is invalid")]
    InvalidLockedProfitDegradation = 6007,
    #[error("Maximum number of strategies have been reached")]
    MaxStrategyReached = 6008,
    #[error("Strategy existed")]
    StrategyExisted = 6009,
    #[error("Invalid unmint amount")]
    InvalidUnmintAmount = 6010,
    #[error("Invalid accounts for strategy")]
    InvalidAccountsForStrategy = 6011,
    #[error("Invalid bump")]
    InvalidBump = 6012,
    #[error("Amount must be greater than 0")]
    AmountMustGreaterThanZero = 6013,
    #[error("Mango is not supported anymore")]
    MangoIsNotSupportedAnymore = 6014,
    #[error("Strategy is not supported")]
    StrategyIsNotSupported = 6015,
    #[error("Pay amount is exceeded")]
    PayAmountIsExeeced = 6016,
    #[error("Fee vault is not set")]
    FeeVaultIsNotSet = 6017,
    #[error("deposit amount in lending is not matched")]
    LendingAssertionViolation = 6018,
    #[error("Cannot remove strategy becase we have some in lending")]
    HaveMoneyInLending = 6019,
}
impl From<VaultError> for ProgramError {
    fn from(e: VaultError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
