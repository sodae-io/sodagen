use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum RaydiumCpSwapError {
    #[error("Not approved")]
    NotApproved = 6000,
    #[error("Input account owner is not the program address")]
    InvalidOwner = 6001,
    #[error("Input token account empty")]
    EmptySupply = 6002,
    #[error("InvalidInput")]
    InvalidInput = 6003,
    #[error("Address of the provided lp token mint is incorrect")]
    IncorrectLpMint = 6004,
    #[error("Exceeds desired slippage limit")]
    ExceededSlippage = 6005,
    #[error("Given pool token amount results in zero trading tokens")]
    ZeroTradingTokens = 6006,
    #[error("Not support token_2022 mint extension")]
    NotSupportMint = 6007,
    #[error("invaild vault")]
    InvalidVault = 6008,
    #[error("Init lp amount is too less(Because 100 amount lp will be locked)")]
    InitLpAmountTooLess = 6009,
    #[error("TransferFee calculate not match")]
    TransferFeeCalculateNotMatch = 6010,
    #[error("Math overflow")]
    MathOverflow = 6011,
    #[error("Insufficient vault")]
    InsufficientVault = 6012,
    #[error("Invalid fee model")]
    InvalidFeeModel = 6013,
    #[error("Fee is zero")]
    NoFeeCollect = 6014,
    #[error("Lamports calculate error")]
    LamportsCalculateError = 6015,
}
impl From<RaydiumCpSwapError> for ProgramError {
    fn from(e: RaydiumCpSwapError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
