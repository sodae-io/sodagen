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
    #[error("Invalid platform allow config account")]
    InvalidPlatformAllowConfig = 6022,
    #[error("Calculation overflowed")]
    CalculateOverflow = 6023,
    #[error("Invalid platform curve rule account")]
    InvalidPlatformCurveRule = 6024,
    #[error("The curve param does not match any check group of the platform curve rule")]
    CurveParamNotMatchPlatformRule = 6025,
    #[error("The curve rule group is not exist")]
    CurveRuleGroupNotExist = 6026,
    #[error("The number of curve rule groups exceeds the limit")]
    CurveRuleGroupsExceeded = 6027,
    #[error("The curve rule constraint is invalid")]
    InvalidCurveRuleConstraint = 6028,
    #[error("The signer is neither the curve rule manager nor the platform admin")]
    InvalidCurveRuleAuthority = 6029,
    #[error("The curve rule field is not supported by the curve of the global config")]
    CurveRuleFieldNotSupportedByCurve = 6030,
    #[error("Lamports calculate error")]
    LamportsCalculateError = 6031,
}
impl From<RaydiumLaunchpadError> for ProgramError {
    fn from(e: RaydiumLaunchpadError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
