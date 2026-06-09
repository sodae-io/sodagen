use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum NumeraireError {
    #[error("Attempt to call an owner only function not by the owner")]
    OnlyOwner = 6000,
    #[error("Token account data is not as expected")]
    InvalidTokenAccountData = 6001,
    #[error("Account data is not as expected")]
    InvalidAccountData = 6002,
    #[error("Token pair decimals mismatch")]
    DecimalsMismatch = 6003,
    #[error("Token should not have this extension")]
    InvalidTokenExtension = 6004,
    #[error("Wrong account(s) passed as input")]
    IncorrectAccounts = 6005,
    #[error("Wrong authority passed as input")]
    IncorrectAuthority = 6006,
    #[error("Delegate not allowed")]
    InvalidDelegate = 6007,
    #[error("This feature is currently unsupported or unenabled")]
    UnsupportedFeature = 6008,
    #[error("This function is currently paused")]
    FunctionPaused = 6009,
    #[error("Fee must be less than 100 basis points")]
    InvalidFee = 6010,
    #[error("Token mints must be ordered by pubkey")]
    InvalidPoolCreate = 6011,
    #[error("Curve params must be positive")]
    InvalidCurveParams = 6012,
    #[error("Pool has too little liquidity for action")]
    InsufficientLiquidity = 6013,
    #[error(
        "Liquidity add does not make pool balanced (or curve params are asymmetric)"
    )]
    LiquidityAddUnbalanced = 6014,
    #[error("Input is more than trader balance")]
    InsufficientBalance = 6015,
    #[error("Input is below the minimum expected")]
    InputTooSmall = 6016,
    #[error("Input is more than available liquidity")]
    InputTooBig = 6017,
    #[error("Output is below the minimum expected")]
    OutputTooSmall = 6018,
    #[error("Invariant computation overflowed")]
    InvariantOverflow = 6019,
    #[error("Invariant does not hold")]
    InvariantViolated = 6020,
    #[error("Depositing too little liquidity")]
    LiquidityAddTooSmall = 6021,
    #[error("Only Token Program 2022 and/or Token Program are supported")]
    UnsupportedTokenProgram = 6022,
    #[error("Some pool weights are zero or nonzero and shouldn't be")]
    InvalidPoolWeights = 6023,
    #[error("A swap math operation overflowed")]
    SwapOverflowError = 6024,
    #[error("An add/remove liquidity math operation overflowed")]
    LiquidityMathOverflow = 6025,
    #[error("Some add/remove balance deltas are nonzero and shouldn't be")]
    InvalidBalanceDeltas = 6026,
    #[error("The provided pool params were not well formed")]
    InvalidPoolParams = 6027,
    #[error("The hints provided do not bound the swap amounts")]
    InvalidHints = 6028,
    #[error("Unexpected fee result")]
    FeeError = 6029,
    #[error("A vault math operation overflowed")]
    VaultOverflowError = 6030,
    #[error("Output is above the maximum expected")]
    OutputTooBig = 6031,
    #[error("This cannot happen")]
    Unreachable = 6032,
    #[error("This action is not allowed")]
    InvalidAction = 6033,
}
impl From<NumeraireError> for ProgramError {
    fn from(e: NumeraireError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
