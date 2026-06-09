use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum JitoVaultError {
    #[error("Bad epoch length")]
    BadEpochLength = 0,
    #[error("VaultSlashUnderflow")]
    VaultSlashUnderflow = 1000,
    #[error("VaultInitialAmountFailed")]
    VaultInitialAmountFailed = 1001,
    #[error("VaultInsufficientFunds")]
    VaultInsufficientFunds = 1002,
    #[error("VaultOverflow")]
    VaultOverflow = 1003,
    #[error("VaultOperatorAdminInvalid")]
    VaultOperatorAdminInvalid = 1004,
    #[error("VaultAdminInvalid")]
    VaultAdminInvalid = 1005,
    #[error("VaultCapacityAdminInvalid")]
    VaultCapacityAdminInvalid = 1006,
    #[error("VaultMintBurnAdminInvalid")]
    VaultMintBurnAdminInvalid = 1007,
    #[error("VaultDelegationAdminInvalid")]
    VaultDelegationAdminInvalid = 1008,
    #[error("VaultDelegateAssetAdminInvalid")]
    VaultDelegateAssetAdminInvalid = 1009,
    #[error("VaultCapacityExceeded")]
    VaultCapacityExceeded = 1010,
    #[error("VaultSlasherAdminInvalid")]
    VaultSlasherAdminInvalid = 1011,
    #[error("VaultNcnAdminInvalid")]
    VaultNcnAdminInvalid = 1012,
    #[error("VaultFeeAdminInvalid")]
    VaultFeeAdminInvalid = 1013,
    #[error("ConfigAdminInvalid")]
    ConfigAdminInvalid = 1014,
    #[error("ConfigFeeAdminInvalid")]
    ConfigFeeAdminInvalid = 1015,
    #[error("VaultFeeCapExceeded")]
    VaultFeeCapExceeded = 1016,
    #[error("VaultFeeChangeTooSoon")]
    VaultFeeChangeTooSoon = 1017,
    #[error("VaultFeeBumpTooLarge")]
    VaultFeeBumpTooLarge = 1018,
    #[error("VaultUnderflow")]
    VaultUnderflow = 1019,
    #[error("VaultUpdateNeeded")]
    VaultUpdateNeeded = 1020,
    #[error("VaultIsUpdated")]
    VaultIsUpdated = 1021,
    #[error("VaultOperatorDelegationUpdateNeeded")]
    VaultOperatorDelegationUpdateNeeded = 1022,
    #[error("VaultOperatorDelegationIsUpdated")]
    VaultOperatorDelegationIsUpdated = 1023,
    #[error("VaultUpdateIncorrectIndex")]
    VaultUpdateIncorrectIndex = 1024,
    #[error("VaultUpdateStateNotFinishedUpdating")]
    VaultUpdateStateNotFinishedUpdating = 1025,
    #[error("VaultSecurityOverflow")]
    VaultSecurityOverflow = 1026,
    #[error("VaultSlashIncomplete")]
    VaultSlashIncomplete = 1027,
    #[error("VaultSecurityUnderflow")]
    VaultSecurityUnderflow = 1028,
    #[error("SlippageError")]
    SlippageError = 1029,
    #[error("VaultStakerWithdrawalTicketNotWithdrawable")]
    VaultStakerWithdrawalTicketNotWithdrawable = 1030,
    #[error("VaultNcnSlasherTicketFailedCooldown")]
    VaultNcnSlasherTicketFailedCooldown = 1031,
    #[error("VaultNcnSlasherTicketFailedWarmup")]
    VaultNcnSlasherTicketFailedWarmup = 1032,
    #[error("VaultNcnTicketFailedCooldown")]
    VaultNcnTicketFailedCooldown = 1033,
    #[error("VaultNcnTicketFailedWarmup")]
    VaultNcnTicketFailedWarmup = 1034,
    #[error("VaultNcnTicketUnslashable")]
    VaultNcnTicketUnslashable = 1035,
    #[error("OperatorVaultTicketUnslashable")]
    OperatorVaultTicketUnslashable = 1036,
    #[error("NcnOperatorStateUnslashable")]
    NcnOperatorStateUnslashable = 1037,
    #[error("VaultNcnSlasherTicketUnslashable")]
    VaultNcnSlasherTicketUnslashable = 1038,
    #[error("NcnVaultTicketUnslashable")]
    NcnVaultTicketUnslashable = 1039,
    #[error("NcnVaultSlasherTicketUnslashable")]
    NcnVaultSlasherTicketUnslashable = 1040,
    #[error("VaultMaxSlashedPerOperatorExceeded")]
    VaultMaxSlashedPerOperatorExceeded = 1041,
    #[error("VaultStakerWithdrawalTicketInvalidStaker")]
    VaultStakerWithdrawalTicketInvalidStaker = 1042,
    #[error("SlasherOverflow")]
    SlasherOverflow = 1043,
    #[error("NcnOverflow")]
    NcnOverflow = 1044,
    #[error("OperatorOverflow")]
    OperatorOverflow = 1045,
    #[error("VaultDelegationZero")]
    VaultDelegationZero = 1046,
    #[error("VaultCooldownZero")]
    VaultCooldownZero = 1047,
    #[error("VaultBurnZero")]
    VaultBurnZero = 1048,
    #[error("VaultEnqueueWithdrawalAmountZero")]
    VaultEnqueueWithdrawalAmountZero = 1049,
    #[error("VaultMintZero")]
    VaultMintZero = 1050,
    #[error("VaultIsPaused")]
    VaultIsPaused = 1051,
    #[error("InvalidDepositor")]
    InvalidDepositor = 1052,
    #[error("InvalidDepositTokenAccount")]
    InvalidDepositTokenAccount = 1053,
    #[error("NoSupportedMintBalanceChange")]
    NoSupportedMintBalanceChange = 1054,
    #[error("InvalidEpochLength")]
    InvalidEpochLength = 1055,
    #[error("VaultRewardFeeDeltaTooLarge")]
    VaultRewardFeeDeltaTooLarge = 1056,
    #[error("VaultRewardFeeIsZero")]
    VaultRewardFeeIsZero = 1057,
    #[error("VrtOutCannotBeZero")]
    VrtOutCannotBeZero = 1058,
    #[error("NonZeroAdditionalAssetsNeededForWithdrawalAtEndOfUpdate")]
    NonZeroAdditionalAssetsNeededForWithdrawalAtEndOfUpdate = 1059,
    #[error("ArithmeticOverflow")]
    ArithmeticOverflow = 3000,
    #[error("ArithmeticUnderflow")]
    ArithmeticUnderflow = 3001,
    #[error("DivisionByZero")]
    DivisionByZero = 3002,
}
impl From<JitoVaultError> for ProgramError {
    fn from(e: JitoVaultError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
