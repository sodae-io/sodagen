use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum OrcaV1Error {
    #[error("Swap account already in use")]
    AlreadyInUse = 0,
    #[error("Invalid program address generated from bump seed and key")]
    InvalidProgramAddress = 1,
    #[error("Input account owner is not the program address")]
    InvalidOwner = 2,
    #[error("Output pool account owner cannot be the program address")]
    InvalidOutputOwner = 3,
    #[error("Deserialized account is not an SPL Token mint")]
    ExpectedMint = 4,
    #[error("Deserialized account is not an SPL Token account")]
    ExpectedAccount = 5,
    #[error("Input token account empty")]
    EmptySupply = 6,
    #[error("Pool token mint has a non-zero supply")]
    InvalidSupply = 7,
    #[error("Token account has a delegate")]
    InvalidDelegate = 8,
    #[error("InvalidInput")]
    InvalidInput = 9,
    #[error("Address of the provided swap token account is incorrect")]
    IncorrectSwapAccount = 10,
    #[error("Address of the provided pool token mint is incorrect")]
    IncorrectPoolMint = 11,
    #[error("InvalidOutput")]
    InvalidOutput = 12,
    #[error("General calculation failure due to overflow or underflow")]
    CalculationFailure = 13,
    #[error("Invalid instruction")]
    InvalidInstruction = 14,
    #[error("Swap input token accounts have the same mint")]
    RepeatedMint = 15,
    #[error("Swap instruction exceeds desired slippage limit")]
    ExceededSlippage = 16,
    #[error("Token account has a close authority")]
    InvalidCloseAuthority = 17,
    #[error("Pool token mint has a freeze authority")]
    InvalidFreezeAuthority = 18,
    #[error("Pool fee token account incorrect")]
    IncorrectFeeAccount = 19,
    #[error("Given pool token amount results in zero trading tokens")]
    ZeroTradingTokens = 20,
    #[error("Fee calculation failed due to overflow, underflow, or unexpected 0")]
    FeeCalculationFailure = 21,
    #[error("Conversion to u64 failed with an overflow or underflow")]
    ConversionFailure = 22,
    #[error("The provided fee does not match the program owner's constraints")]
    InvalidFee = 23,
    #[error(
        "The provided token program does not match the token program expected by the swap"
    )]
    IncorrectTokenProgramId = 24,
    #[error("The provided curve type is not supported by the program owner")]
    UnsupportedCurveType = 25,
    #[error("The provided curve parameters are invalid")]
    InvalidCurve = 26,
    #[error("The operation cannot be performed on the given curve")]
    UnsupportedCurveOperation = 27,
    #[error("The pool fee account is invalid")]
    InvalidFeeAccount = 28,
}
impl From<OrcaV1Error> for ProgramError {
    fn from(e: OrcaV1Error) -> Self {
        ProgramError::Custom(e as u32)
    }
}
