use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum RaydiumLaunchpadError {
    #[error("Not approved")]
    NotApproved = 6000,
    #[error("Input account owner is not the program address")]
    InvalidOwner = 6001,
    #[error("InvalidInput")]
    InvalidInput = 6002,
    #[error("The input params are not match with curve type in config")]
    InputNotMatchCurveConfig = 6003,
    #[error("Exceeds desired slippage limit")]
    ExceededSlippage = 6004,
    #[error("Pool funding")]
    PoolFunding = 6005,
    #[error("Pool migrated")]
    PoolMigrated = 6006,
    #[error("Migrate type not match")]
    MigrateTypeNotMatch = 6007,
    #[error("Math overflow")]
    MathOverflow = 6008,
    #[error("No assets to collect")]
    NoAssetsToCollect = 6009,
    #[error("Vesting ratio too high")]
    VestingRatioTooHigh = 6010,
    #[error("Vesting setting ended")]
    VestingSettingEnded = 6011,
    #[error("Vesting not started")]
    VestingNotStarted = 6012,
    #[error("No vesting schedule")]
    NoVestingSchedule = 6013,
    #[error("The platform info input is invalid")]
    InvalidPlatformInfo = 6014,
    #[error("Pool not migrated")]
    PoolNotMigrated = 6015,
    #[error("The input cp swap config account is invalid")]
    InvalidCpSwapConfig = 6016,
    #[error("No support extension")]
    NoSupportExtension = 6017,
    #[error("Not enough remaining accounts")]
    NotEnoughRemainingAccounts = 6018,
    #[error("TransferFee calculate not match")]
    TransferFeeCalculateNotMatch = 6019,
    #[error("Curve param is not exist")]
    CurveParamIsNotExist = 6020,
    #[error(
        "Total locked amount must great or equal to the platform vesting share amount"
    )]
    InvalidTotalLockedAmount = 6021,
    #[error("Platform is not authorized to use this global config")]
    PlatformGlobalAccessDenied = 6022,
    #[error("Invalid platform-global access account")]
    InvalidPlatformGlobalAccess = 6023,
}
impl From<RaydiumLaunchpadError> for ProgramError {
    fn from(e: RaydiumLaunchpadError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
