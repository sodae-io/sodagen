use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum StableswapError {
    #[error("Exchange is currently frozen")]
    Frozen = 6000,
    #[error("Insufficient liquidity in vault")]
    InsufficientLiquidity = 6001,
    #[error("Unauthorized")]
    Unauthorized = 6002,
    #[error("Amount must be greater than zero")]
    ZeroAmount = 6003,
    #[error("Mint decimals must be 6 for stablecoins")]
    InvalidMintDecimals = 6004,
    #[error("Invalid mint provided")]
    InvalidMint = 6005,
    #[error("Slippage exceeded: amount_out is less than minimum_amount_out")]
    SlippageExceeded = 6006,
    #[error("Cannot remove token: vault still has liquidity")]
    VaultHasLiquidity = 6007,
    #[error("Invalid token program provided")]
    InvalidTokenProgram = 6008,
    #[error("Math overflow")]
    MathOverflow = 6009,
    #[error("Insufficient deposited balance")]
    InsufficientDeposits = 6010,
    #[error("Invalid proof")]
    InvalidProof = 6011,
    #[error("Proof expired")]
    ProofExpired = 6012,
    #[error("Invalid pair state account")]
    InvalidPairState = 6013,
    #[error("Insufficient fee balance")]
    InsufficientFeeBalance = 6014,
    #[error("Fee bps must be between 0 and 10,000")]
    InvalidRate = 6015,
}
impl From<StableswapError> for ProgramError {
    fn from(e: StableswapError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
