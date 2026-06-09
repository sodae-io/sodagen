use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum JitoRestakingError {
    #[error("Bad epoch length")]
    BadEpochLength = 0,
    #[error("NcnOperatorAdminInvalid")]
    NcnOperatorAdminInvalid = 1000,
    #[error("NcnCooldownOperatorFailed")]
    NcnCooldownOperatorFailed = 1001,
    #[error("NcnSlasherAdminInvalid")]
    NcnSlasherAdminInvalid = 1002,
    #[error("NcnVaultAdminInvalid")]
    NcnVaultAdminInvalid = 1003,
    #[error("NcnAdminInvalid")]
    NcnAdminInvalid = 1004,
    #[error("NcnDelegateAdminInvalid")]
    NcnDelegateAdminInvalid = 1005,
    #[error("NcnVaultSlasherTicketFailedCooldown")]
    NcnVaultSlasherTicketFailedCooldown = 1006,
    #[error("NcnVaultTicketFailedCooldown")]
    NcnVaultTicketFailedCooldown = 1007,
    #[error("NcnWarmupOperatorFailed")]
    NcnWarmupOperatorFailed = 1008,
    #[error("NcnVaultSlasherTicketFailedWarmup")]
    NcnVaultSlasherTicketFailedWarmup = 1009,
    #[error("NcnVaultTicketFailedWarmup")]
    NcnVaultTicketFailedWarmup = 1010,
    #[error("OperatorNcnAdminInvalid")]
    OperatorNcnAdminInvalid = 2000,
    #[error("OperatorVaultAdminInvalid")]
    OperatorVaultAdminInvalid = 2001,
    #[error("OperatorAdminInvalid")]
    OperatorAdminInvalid = 2002,
    #[error("OperatorDelegateAdminInvalid")]
    OperatorDelegateAdminInvalid = 2003,
    #[error("OperatorCooldownNcnFailed")]
    OperatorCooldownNcnFailed = 2004,
    #[error("OperatorVaultTicketFailedCooldown")]
    OperatorVaultTicketFailedCooldown = 2005,
    #[error("OperatorVaultTicketFailedWarmup")]
    OperatorVaultTicketFailedWarmup = 2006,
    #[error("OperatorWarmupNcnFailed")]
    OperatorWarmupNcnFailed = 2007,
    #[error("OperatorFeeCapExceeded")]
    OperatorFeeCapExceeded = 2008,
    #[error("NcnOverflow")]
    NcnOverflow = 2009,
    #[error("OperatorOverflow")]
    OperatorOverflow = 2010,
    #[error("VaultOverflow")]
    VaultOverflow = 2011,
    #[error("SlasherOverflow")]
    SlasherOverflow = 2012,
    #[error("InvalidEpochLength")]
    InvalidEpochLength = 2013,
    #[error("ConfigAdminInvalid")]
    ConfigAdminInvalid = 2014,
    #[error("ArithmeticOverflow")]
    ArithmeticOverflow = 3000,
    #[error("ArithmeticUnderflow")]
    ArithmeticUnderflow = 3001,
    #[error("DivisionByZero")]
    DivisionByZero = 3002,
}
impl From<JitoRestakingError> for ProgramError {
    fn from(e: JitoRestakingError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
