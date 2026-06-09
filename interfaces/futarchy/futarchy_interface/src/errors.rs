use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum FutarchyError {
    #[error(
        "Amms must have been created within 5 minutes (counted in slots) of proposal initialization"
    )]
    AmmTooOld = 6000,
    #[error("An amm has an `initial_observation` that doesn't match the `dao`'s config")]
    InvalidInitialObservation = 6001,
    #[error(
        "An amm has a `max_observation_change_per_update` that doesn't match the `dao`'s config"
    )]
    InvalidMaxObservationChange = 6002,
    #[error("An amm has a `start_delay_slots` that doesn't match the `dao`'s config")]
    InvalidStartDelaySlots = 6003,
    #[error("One of the vaults has an invalid `settlement_authority`")]
    InvalidSettlementAuthority = 6004,
    #[error("Proposal is too young to be executed or rejected")]
    ProposalTooYoung = 6005,
    #[error(
        "Markets too young for proposal to be finalized. TWAP might need to be cranked"
    )]
    MarketsTooYoung = 6006,
    #[error("This proposal has already been finalized")]
    ProposalAlreadyFinalized = 6007,
    #[error(
        "A conditional vault has an invalid nonce. A nonce should encode the proposal number"
    )]
    InvalidVaultNonce = 6008,
    #[error("This proposal can't be executed because it isn't in the passed state")]
    ProposalNotPassed = 6009,
    #[error("More liquidity needs to be in the AMM to launch this proposal")]
    InsufficientLiquidity = 6010,
    #[error(
        "Proposal duration must be longer 1 day and longer than 2 times the TWAP start delay"
    )]
    ProposalDurationTooShort = 6011,
    #[error("Pass threshold must be less than 10%")]
    PassThresholdTooHigh = 6012,
    #[error("Question must have exactly 2 outcomes for binary futarchy")]
    QuestionMustBeBinary = 6013,
    #[error("Squads proposal must be in Active status")]
    InvalidSquadsProposalStatus = 6014,
    #[error("Casting overflow. If you're seeing this, please report this")]
    CastingOverflow = 6015,
    #[error("Insufficient balance")]
    InsufficientBalance = 6016,
    #[error("Cannot remove zero liquidity")]
    ZeroLiquidityRemove = 6017,
    #[error("Swap slippage exceeded")]
    SwapSlippageExceeded = 6018,
    #[error("Assert failed")]
    AssertFailed = 6019,
    #[error("Invalid admin")]
    InvalidAdmin = 6020,
    #[error("Proposal is not in draft state")]
    ProposalNotInDraftState = 6021,
    #[error("Insufficient token balance")]
    InsufficientTokenBalance = 6022,
    #[error("Invalid amount")]
    InvalidAmount = 6023,
    #[error("Insufficient stake to launch proposal")]
    InsufficientStakeToLaunch = 6024,
    #[error("Staker not found in proposal")]
    StakerNotFound = 6025,
    #[error("Pool must be in spot state")]
    PoolNotInSpotState = 6026,
    #[error(
        "If you're providing liquidity, you must provide both base and quote token accounts"
    )]
    InvalidDaoCreateLiquidity = 6027,
    #[error("Invalid stake account")]
    InvalidStakeAccount = 6028,
    #[error(
        "An invariant was violated. You should get in contact with the MetaDAO team if you see this"
    )]
    InvariantViolated = 6029,
    #[error("Proposal needs to be active to perform a conditional swap")]
    ProposalNotActive = 6030,
    #[error(
        "This Squads transaction should only contain calls to update spending limits"
    )]
    InvalidTransaction = 6031,
    #[error("Proposal has already been sponsored")]
    ProposalAlreadySponsored = 6032,
    #[error("Team sponsored pass threshold must be between -10% and 10%")]
    InvalidTeamSponsoredPassThreshold = 6033,
    #[error("Target K must be greater than the current K")]
    InvalidTargetK = 6034,
    #[error("Failed to compile transaction message for Squads vault transaction")]
    InvalidTransactionMessage = 6035,
    #[error("Base mint and quote mint must be different")]
    InvalidMint = 6036,
    #[error("Proposal is not ready to be unstaked")]
    ProposalNotReadyToUnstake = 6037,
    #[error("Optimistic governance is disabled")]
    OptimisticGovernanceDisabled = 6038,
    #[error("An active optimistic proposal is already enqueued")]
    ActiveOptimisticProposalAlreadyEnqueued = 6039,
    #[error("Optimistic proposal has already passed")]
    OptimisticProposalAlreadyPassed = 6040,
    #[error("Invalid spending limit mint. Must be the same as the DAO's quote mint")]
    InvalidSpendingLimitMint = 6041,
    #[error("No active optimistic proposal")]
    NoActiveOptimisticProposal = 6042,
}
impl From<FutarchyError> for ProgramError {
    fn from(e: FutarchyError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
