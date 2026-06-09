use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum JupiterError {
    #[error("Empty route")]
    EmptyRoute = 6000,
    #[error("Slippage tolerance exceeded")]
    SlippageToleranceExceeded = 6001,
    #[error("Invalid calculation")]
    InvalidCalculation = 6002,
    #[error("Missing platform fee account")]
    MissingPlatformFeeAccount = 6003,
    #[error("Invalid slippage")]
    InvalidSlippage = 6004,
    #[error("Not enough percent to 100")]
    NotEnoughPercent = 6005,
    #[error("Token input index is invalid")]
    InvalidInputIndex = 6006,
    #[error("Token output index is invalid")]
    InvalidOutputIndex = 6007,
    #[error("Not Enough Account keys")]
    NotEnoughAccountKeys = 6008,
    #[error("Non zero minimum out amount not supported")]
    NonZeroMinimumOutAmountNotSupported = 6009,
    #[error("Invalid route plan")]
    InvalidRoutePlan = 6010,
    #[error("Invalid referral authority")]
    InvalidReferralAuthority = 6011,
    #[error("Token account doesn't match the ledger")]
    LedgerTokenAccountDoesNotMatch = 6012,
    #[error("Invalid token ledger")]
    InvalidTokenLedger = 6013,
    #[error("Token program ID is invalid")]
    IncorrectTokenProgramId = 6014,
    #[error("Token program not provided")]
    TokenProgramNotProvided = 6015,
    #[error("Swap not supported")]
    SwapNotSupported = 6016,
    #[error("Exact out amount doesn't match")]
    ExactOutAmountNotMatched = 6017,
    #[error("Source mint and destination mint cannot the same")]
    SourceAndDestinationMintCannotBeTheSame = 6018,
    #[error("Invalid mint")]
    InvalidMint = 6019,
    #[error("Invalid program authority")]
    InvalidProgramAuthority = 6020,
    #[error("Invalid output token account")]
    InvalidOutputTokenAccount = 6021,
    #[error("Invalid fee wallet")]
    InvalidFeeWallet = 6022,
    #[error("Invalid authority")]
    InvalidAuthority = 6023,
    #[error("Insufficient funds")]
    InsufficientFunds = 6024,
    #[error("Invalid token account")]
    InvalidTokenAccount = 6025,
    #[error("Bonding curve already completed")]
    BondingCurveAlreadyCompleted = 6026,
}
impl From<JupiterError> for ProgramError {
    fn from(e: JupiterError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
