use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum FarmsError {
    #[error("Cannot stake 0 amount")]
    StakeZero = 6000,
    #[error("Cannot unstake 0 amount")]
    UnstakeZero = 6001,
    #[error("Nothing to unstake")]
    NothingToUnstake = 6002,
    #[error("No reward to harvest")]
    NoRewardToHarvest = 6003,
    #[error("Reward not present in reward list")]
    NoRewardInList = 6004,
    #[error("Reward already initialized")]
    RewardAlreadyInitialized = 6005,
    #[error("Max number of reward tokens reached")]
    MaxRewardNumberReached = 6006,
    #[error("Reward does not exist")]
    RewardDoesNotExist = 6007,
    #[error("Reward vault exists but the account is wrong")]
    WrongRewardVaultAccount = 6008,
    #[error("Reward vault pubkey does not match staking pool vault")]
    RewardVaultMismatch = 6009,
    #[error("Reward vault authority pubkey does not match staking pool vault")]
    RewardVaultAuthorityMismatch = 6010,
    #[error("Nothing staked, cannot collect any rewards")]
    NothingStaked = 6011,
    #[error("Integer overflow")]
    IntegerOverflow = 6012,
    #[error("Conversion failure")]
    ConversionFailure = 6013,
    #[error("Unexpected account in instruction")]
    UnexpectedAccount = 6014,
    #[error("Operation forbidden")]
    OperationForbidden = 6015,
    #[error("Mathematical operation with overflow")]
    MathOverflow = 6016,
    #[error("Minimum claim duration has not been reached")]
    MinClaimDurationNotReached = 6017,
    #[error("Reward vault has a delegate")]
    RewardsVaultHasDelegate = 6018,
    #[error("Reward vault has a close authority")]
    RewardsVaultHasCloseAuthority = 6019,
    #[error("Farm vault has a delegate")]
    FarmVaultHasDelegate = 6020,
    #[error("Farm vault has a close authority")]
    FarmVaultHasCloseAuthority = 6021,
    #[error("Reward vault has a delegate")]
    RewardsTreasuryVaultHasDelegate = 6022,
    #[error("Reward vault has a close authority")]
    RewardsTreasuryVaultHasCloseAuthority = 6023,
    #[error("User ata and reward vault have different mints")]
    UserAtaRewardVaultMintMissmatch = 6024,
    #[error("User ata and farm token have different mints")]
    UserAtaFarmTokenMintMissmatch = 6025,
    #[error("Token mint and farm token have different mints")]
    TokenFarmTokenMintMissmatch = 6026,
    #[error("Reward ata mint is different than reward mint")]
    RewardAtaRewardMintMissmatch = 6027,
    #[error("Reward ata owner is different than payer")]
    RewardAtaOwnerNotPayer = 6028,
    #[error("Mode to update global_config is invalid")]
    InvalidGlobalConfigMode = 6029,
    #[error("Reward Index is higher than number of rewards")]
    RewardIndexOutOfRange = 6030,
    #[error("No tokens available to withdraw")]
    NothingToWithdraw = 6031,
    #[error("user, user_ref, authority and payer must match for non-delegated farm")]
    UserDelegatedFarmNonDelegatedMissmatch = 6032,
    #[error("Authority must match farm delegate authority")]
    AuthorityFarmDelegateMissmatch = 6033,
    #[error("Farm not delegated, can not complete operation")]
    FarmNotDelegated = 6034,
    #[error("Operation not allowed for delegated farm")]
    FarmDelegated = 6035,
    #[error(
        "Unstake lockup period is not elapsed. Deposit is locked until end of unstake period"
    )]
    UnstakeNotElapsed = 6036,
    #[error("Pending withdrawal already exist and not withdrawn yet")]
    PendingWithdrawalNotWithdrawnYet = 6037,
    #[error("Cannot deposit zero amount directly to farm vault")]
    DepositZero = 6038,
    #[error("Invalid config value")]
    InvalidConfigValue = 6039,
    #[error("Invalid penalty percentage")]
    InvalidPenaltyPercentage = 6040,
    #[error("Early withdrawal not allowed")]
    EarlyWithdrawalNotAllowed = 6041,
    #[error("Invalid locking timestamps")]
    InvalidLockingTimestamps = 6042,
    #[error("Invalid reward rate curve point")]
    InvalidRpsCurvePoint = 6043,
    #[error("Invalid timestamp")]
    InvalidTimestamp = 6044,
    #[error("Deposit cap reached")]
    DepositCapReached = 6045,
    #[error("Missing Scope Prices")]
    MissingScopePrices = 6046,
    #[error("Scope Oracle Price Too Old")]
    ScopeOraclePriceTooOld = 6047,
    #[error("Invalid Oracle Config")]
    InvalidOracleConfig = 6048,
    #[error("Could not deserialize scope")]
    CouldNotDeserializeScope = 6049,
    #[error("Reward ata owner is different than farm admin")]
    RewardAtaOwnerNotAdmin = 6050,
    #[error("Cannot withdraw reward as available amount is zero")]
    WithdrawRewardZeroAvailable = 6051,
    #[error("Cannot withdraw reward as reward schedule is set")]
    RewardScheduleCurveSet = 6052,
    #[error(
        "Cannot initialize farm while having a mint with token22 and requested extensions"
    )]
    UnsupportedTokenExtension = 6053,
    #[error("Invalid authority for updating farm config")]
    InvalidFarmConfigUpdateAuthority = 6054,
    #[error("Invalid authority for transfer ownersip new user state initialization")]
    InvalidTransferOwnershipOldOwner = 6055,
    #[error("Invalid farm state for transfer ownership new user state initialization")]
    InvalidTransferOwnershipFarmState = 6056,
    #[error("Invalid user state for transfer ownership, owner must match delegatee")]
    InvalidTransferOwnershipUserStateOwnerDelegatee = 6057,
    #[error("Invalid farm state locking mode for transfer ownership, must be 0")]
    InvalidTransferOwnershipFarmStateLockingMode = 6058,
    #[error(
        "Invalid farm state withdrawal cooldown period for transfer ownership, must be 0"
    )]
    InvalidTransferOwnershipFarmStateWithdrawCooldownPeriod = 6059,
    #[error(
        "Invalid transfer ownership stake amount, must be equal to unstaked deposits"
    )]
    InvalidTransferOwnershipStakeAmount = 6060,
    #[error("Invalid authority for transfer ownersip new user state initialization")]
    InvalidTransferOwnershipNewOwner = 6061,
    #[error(
        "Invalid farm state deposit warmup period for transfer ownership, must be 0 if old user has stake"
    )]
    InvalidTransferOwnershipFarmStateDepositWarmupPeriod = 6062,
    #[error("Reward User Once feature is disabled")]
    RewardUserOnceFeatureDisabled = 6063,
    #[error("Can not set delegate_authority to default pubkey - farm is delegated")]
    InvalidDelegatedAuthorityUpdate = 6064,
    #[error("User token account owner does not match user state owner")]
    UserTokenAccountOwnerMismatch = 6065,
    #[error("Harvesting is not permissionless, payer does not match user state owner")]
    HarvestingNotPermissionlessPayerMismatch = 6066,
    #[error("Rewards issued cumulative does not match expected value")]
    RewardsIssuedCumulativeMismatch = 6067,
    #[error("Cannot close user state because staked amount is non-zero")]
    CannotCloseUserStateStakeNonZero = 6068,
    #[error("Cannot close user state because there are pending unstake requests")]
    CannotCloseUserStatePendingUnstakes = 6069,
    #[error("Cannot close user state because there are pending deposit requests")]
    CannotCloseUserStatePendingDeposits = 6070,
    #[error("Cannot close user state because there are unharvested rewards")]
    CannotCloseUserStateUnharvestedRewards = 6071,
    #[error("Cannot close user state because signer is not the owner")]
    CannotCloseUserStateSignerNotOwner = 6072,
    #[error(
        "Cannot close user state (delegated) because signer is not the delegate authority"
    )]
    CannotCloseUserStateDelegatedSignerNotDelegateAuthority = 6073,
    #[error("Cannot close user state because rent receiver is not the owner")]
    CannotCloseUserStateRentReceiverNotOwner = 6074,
    #[error(
        "Cannot close user state (delegated) because rent receiver is not the admin"
    )]
    CannotCloseUserStateDelegatedRentReceiverNotAdmin = 6075,
    #[error("User reward token account must be an ATA when payer is not the owner")]
    UserRewardTokenAccountMustBeAta = 6076,
    #[error(
        "Cannot reward user because rewards_issued_cumulative has reached maximum value"
    )]
    RewardsIssuedCumulativeAtMax = 6077,
    #[error("User state user id does not match expected value")]
    UserStateIdMismatch = 6078,
}
impl From<FarmsError> for ProgramError {
    fn from(e: FarmsError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
