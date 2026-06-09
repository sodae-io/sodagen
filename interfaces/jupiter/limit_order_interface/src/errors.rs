use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum LimitOrderError {
    #[error("InvalidMakingAmount")]
    InvalidMakingAmount = 6000,
    #[error("InvalidTakingAmount")]
    InvalidTakingAmount = 6001,
    #[error("InvalidMaxTakingAmount")]
    InvalidMaxTakingAmount = 6002,
    #[error("InvalidCalculation")]
    InvalidCalculation = 6003,
    #[error("InvalidInputAccount")]
    InvalidInputAccount = 6004,
    #[error("InvalidOutputAccount")]
    InvalidOutputAccount = 6005,
    #[error("InvalidPair")]
    InvalidPair = 6006,
    #[error("MissingReferral")]
    MissingReferral = 6007,
    #[error("OrderExpired")]
    OrderExpired = 6008,
    #[error("OrderNotExpired")]
    OrderNotExpired = 6009,
    #[error("InvalidKeeper")]
    InvalidKeeper = 6010,
    #[error("MathOverflow")]
    MathOverflow = 6011,
    #[error("ProgramMismatch")]
    ProgramMismatch = 6012,
    #[error("UnknownInstruction")]
    UnknownInstruction = 6013,
    #[error("MissingRepayInstructions")]
    MissingRepayInstructions = 6014,
    #[error("InvalidOrder")]
    InvalidOrder = 6015,
    #[error("InvalidBorrowMakingAmount")]
    InvalidBorrowMakingAmount = 6016,
}
impl From<LimitOrderError> for ProgramError {
    fn from(e: LimitOrderError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
