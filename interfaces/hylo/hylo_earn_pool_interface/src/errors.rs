use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum HyloEarnPoolError {
    #[error("Cannot process deposit yielding zero LP tokens.")]
    ZeroLpDeposit = 6000,
    #[error("Cannot process withdrawal resulting in zero protocol tokens.")]
    ZeroTokenWithdrawal = 6001,
    #[error("Cannot update configuration with identical value.")]
    AdminNoop = 6002,
    #[error("Protocol operations have been paused by admin.")]
    ProtocolPaused = 6003,
    #[error("Earn pool operations have been paused.")]
    EarnPoolPaused = 6004,
    #[error("Earn pool empty due to drawdown. Deposits are temporarily disabled.")]
    DepositDisabled = 6005,
}
impl From<HyloEarnPoolError> for ProgramError {
    fn from(e: HyloEarnPoolError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
