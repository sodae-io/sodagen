use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum TokenMillError {
    #[error("DivisionByZero")]
    DivisionByZero = 6000,
    #[error("AmountOverflow")]
    AmountOverflow = 6001,
    #[error("AmountInOverflow")]
    AmountInOverflow = 6002,
    #[error("AmountOutOverflow")]
    AmountOutOverflow = 6003,
    #[error("LiquidityOverflow0")]
    LiquidityOverflow0 = 6004,
    #[error("LiquidityOverflow1")]
    LiquidityOverflow1 = 6005,
    #[error("PriceOverflow")]
    PriceOverflow = 6006,
    #[error("FeeAmountOverflow")]
    FeeAmountOverflow = 6007,
    #[error("AdminSignatureRequired")]
    AdminSignatureRequired = 6008,
    #[error("CreatorSignatureRequired")]
    CreatorSignatureRequired = 6009,
    #[error("AuthoritySignatureRequired")]
    AuthoritySignatureRequired = 6010,
    #[error("FeeRecipientUpdateOnCd")]
    FeeRecipientUpdateOnCd = 6011,
    #[error("SwapAuthorityAlreadyRemoved")]
    SwapAuthorityAlreadyRemoved = 6012,
    #[error("InvalidFeeReserve")]
    InvalidFeeReserve = 6013,
    #[error("InvalidFee")]
    InvalidFee = 6014,
    #[error("InvalidQuoteTokenMint")]
    InvalidQuoteTokenMint = 6015,
    #[error("InvalidSqrtPriceLimit")]
    InvalidSqrtPriceLimit = 6016,
    #[error("InvalidSqrtPrices")]
    InvalidSqrtPrices = 6017,
    #[error("ZeroDeltaAmount")]
    ZeroDeltaAmount = 6018,
    #[error("SlippageExceeded")]
    SlippageExceeded = 6019,
    #[error("CanOnlyOptInKOTM")]
    CanOnlyOptInKotm = 6020,
}
impl From<TokenMillError> for ProgramError {
    fn from(e: TokenMillError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
