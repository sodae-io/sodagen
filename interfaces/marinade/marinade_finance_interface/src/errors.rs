use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum MarinadeFinanceError {
    #[error("Wrong reserve owner. Must be a system account")]
    WrongReserveOwner = 6000,
    #[error("Reserve must have no data, but has data")]
    NonEmptyReserveData = 6001,
    #[error("Invalid initial reserve lamports")]
    InvalidInitialReserveLamports = 6002,
    #[error("Zero validator chunk size")]
    ZeroValidatorChunkSize = 6003,
    #[error("Too big validator chunk size")]
    TooBigValidatorChunkSize = 6004,
    #[error("Zero credit chunk size")]
    ZeroCreditChunkSize = 6005,
    #[error("Too big credit chunk size")]
    TooBigCreditChunkSize = 6006,
    #[error("Too low credit fee")]
    TooLowCreditFee = 6007,
    #[error("Invalid mint authority")]
    InvalidMintAuthority = 6008,
    #[error("Non empty initial mint supply")]
    MintHasInitialSupply = 6009,
    #[error("Invalid owner fee state")]
    InvalidOwnerFeeState = 6010,
    #[error(
        "Invalid program id. For using program from another account please update id in the code"
    )]
    InvalidProgramId = 6011,
    #[error("Unexpected account")]
    UnexpectedAccount = 6012,
    #[error("Calculation failure")]
    CalculationFailure = 6013,
    #[error("You can't deposit a stake-account with lockup")]
    StakeAccountWithLockup = 6014,
    #[error("Min stake is too low")]
    MinStakeIsTooLow = 6015,
    #[error("Lp max fee is too high")]
    LpMaxFeeIsTooHigh = 6016,
    #[error("Basis points overflow")]
    BasisPointsOverflow = 6017,
    #[error("LP min fee > LP max fee")]
    LpFeesAreWrongWayRound = 6018,
    #[error("Liquidity target too low")]
    LiquidityTargetTooLow = 6019,
    #[error("Ticket not due. Wait more epochs")]
    TicketNotDue = 6020,
    #[error("Ticket not ready. Wait a few hours and try again")]
    TicketNotReady = 6021,
    #[error("Wrong Ticket Beneficiary")]
    WrongBeneficiary = 6022,
    #[error("Stake Account not updated yet")]
    StakeAccountNotUpdatedYet = 6023,
    #[error("Stake Account not delegated")]
    StakeNotDelegated = 6024,
    #[error("Stake Account is emergency unstaking")]
    StakeAccountIsEmergencyUnstaking = 6025,
    #[error("Insufficient Liquidity in the Liquidity Pool")]
    InsufficientLiquidity = 6026,
    #[error("NotUsed6027")]
    NotUsed6027 = 6027,
    #[error("Invalid admin authority")]
    InvalidAdminAuthority = 6028,
    #[error("Invalid validator system manager")]
    InvalidValidatorManager = 6029,
    #[error("Invalid stake list account discriminator")]
    InvalidStakeListDiscriminator = 6030,
    #[error("Invalid validator list account discriminator")]
    InvalidValidatorListDiscriminator = 6031,
    #[error("Treasury cut is too high")]
    TreasuryCutIsTooHigh = 6032,
    #[error("Reward fee is too high")]
    RewardsFeeIsTooHigh = 6033,
    #[error("Staking is capped")]
    StakingIsCapped = 6034,
    #[error("Liquidity is capped")]
    LiquidityIsCapped = 6035,
    #[error("Update window is too low")]
    UpdateWindowIsTooLow = 6036,
    #[error("Min withdraw is too high")]
    MinWithdrawIsTooHigh = 6037,
    #[error("Withdraw amount is too low")]
    WithdrawAmountIsTooLow = 6038,
    #[error("Deposit amount is too low")]
    DepositAmountIsTooLow = 6039,
    #[error("Not enough user funds")]
    NotEnoughUserFunds = 6040,
    #[error("Wrong token owner or delegate")]
    WrongTokenOwnerOrDelegate = 6041,
    #[error("Too early for stake delta")]
    TooEarlyForStakeDelta = 6042,
    #[error("Required delegated stake")]
    RequiredDelegatedStake = 6043,
    #[error("Required active stake")]
    RequiredActiveStake = 6044,
    #[error("Required deactivating stake")]
    RequiredDeactivatingStake = 6045,
    #[error("Depositing not activated stake")]
    DepositingNotActivatedStake = 6046,
    #[error("Too low delegation in the depositing stake")]
    TooLowDelegationInDepositingStake = 6047,
    #[error("Wrong deposited stake balance")]
    WrongStakeBalance = 6048,
    #[error("Wrong validator account or index")]
    WrongValidatorAccountOrIndex = 6049,
    #[error("Wrong stake account or index")]
    WrongStakeAccountOrIndex = 6050,
    #[error("Delta stake is positive so we must stake instead of unstake")]
    UnstakingOnPositiveDelta = 6051,
    #[error("Delta stake is negative so we must unstake instead of stake")]
    StakingOnNegativeDelta = 6052,
    #[error("Moving stake during an epoch is capped")]
    MovingStakeIsCapped = 6053,
    #[error("Stake must be uninitialized")]
    StakeMustBeUninitialized = 6054,
    #[error("Destination stake must be delegated")]
    DestinationStakeMustBeDelegated = 6055,
    #[error("Destination stake must not be deactivating")]
    DestinationStakeMustNotBeDeactivating = 6056,
    #[error("Destination stake must be updated")]
    DestinationStakeMustBeUpdated = 6057,
    #[error("Invalid destination stake delegation")]
    InvalidDestinationStakeDelegation = 6058,
    #[error("Source stake must be delegated")]
    SourceStakeMustBeDelegated = 6059,
    #[error("Source stake must not be deactivating")]
    SourceStakeMustNotBeDeactivating = 6060,
    #[error("Source stake must be updated")]
    SourceStakeMustBeUpdated = 6061,
    #[error("Invalid source stake delegation")]
    InvalidSourceStakeDelegation = 6062,
    #[error("Invalid delayed unstake ticket")]
    InvalidDelayedUnstakeTicket = 6063,
    #[error("Reusing delayed unstake ticket")]
    ReusingDelayedUnstakeTicket = 6064,
    #[error("Emergency unstaking from non zero scored validator")]
    EmergencyUnstakingFromNonZeroScoredValidator = 6065,
    #[error("Wrong validator duplication flag")]
    WrongValidatorDuplicationFlag = 6066,
    #[error("Redepositing marinade stake")]
    RedepositingMarinadeStake = 6067,
    #[error("Removing validator with balance")]
    RemovingValidatorWithBalance = 6068,
    #[error("Redelegate will put validator over stake target")]
    RedelegateOverTarget = 6069,
    #[error("Source and Dest Validators are the same")]
    SourceAndDestValidatorsAreTheSame = 6070,
    #[error("Some mSOL tokens was minted outside of marinade contract")]
    UnregisteredMsolMinted = 6071,
    #[error("Some LP tokens was minted outside of marinade contract")]
    UnregisteredLpMinted = 6072,
    #[error("List index out of bounds")]
    ListIndexOutOfBounds = 6073,
    #[error("List overflow")]
    ListOverflow = 6074,
    #[error("Requested pause and already Paused")]
    AlreadyPaused = 6075,
    #[error("Requested resume, but not Paused")]
    NotPaused = 6076,
    #[error("Emergency Pause is Active")]
    ProgramIsPaused = 6077,
    #[error("Invalid pause authority")]
    InvalidPauseAuthority = 6078,
    #[error("Selected Stake account has not enough funds")]
    SelectedStakeAccountHasNotEnoughFunds = 6079,
    #[error("Basis point CENTS overflow")]
    BasisPointCentsOverflow = 6080,
    #[error("Withdraw stake account is not enabled")]
    WithdrawStakeAccountIsNotEnabled = 6081,
    #[error("Withdraw stake account fee is too high")]
    WithdrawStakeAccountFeeIsTooHigh = 6082,
    #[error("Delayed unstake fee is too high")]
    DelayedUnstakeFeeIsTooHigh = 6083,
    #[error("Withdraw stake account value is too low")]
    WithdrawStakeLamportsIsTooLow = 6084,
    #[error("Stake account remainder too low")]
    StakeAccountRemainderTooLow = 6085,
    #[error("Capacity of the list must be not less than it's current size")]
    ShrinkingListWithDeletingContents = 6086,
    #[error("Upgrading invariant violation")]
    UpgradingInvariantViolation = 6087,
    #[error("Delinquent upgrader is not done")]
    DelinquentUpgraderIsNotDone = 6088,
    #[error("Canonical stake account for this validator already exists")]
    CanonicalStakeAccountAlreadyCreated = 6089,
    #[error("Invalid canonical stake account address")]
    InvalidCanonicalStakeAccountAddress = 6090,
    #[error("Deposit stake account fee is too high")]
    DepositStakeAccountFeeIsTooHigh = 6091,
    #[error("Deposit SOL fee is too high")]
    DepositSolFeeIsTooHigh = 6092,
}
impl From<MarinadeFinanceError> for ProgramError {
    fn from(e: MarinadeFinanceError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
