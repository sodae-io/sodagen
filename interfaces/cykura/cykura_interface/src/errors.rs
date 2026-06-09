use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum CykuraError {
    #[error("LOK")]
    Lok = 6000,
    #[error("Minting amount should be greater than 0")]
    ZeroMintAmount = 6001,
    #[error("TLU")]
    Tlu = 6002,
    #[error("TMS")]
    Tms = 6003,
    #[error("TLM")]
    Tlm = 6004,
    #[error("TUM")]
    Tum = 6005,
    #[error("M0")]
    M0 = 6006,
    #[error("M1")]
    M1 = 6007,
    #[error("OS")]
    Os = 6008,
    #[error("AS")]
    As = 6009,
    #[error("SPL")]
    Spl = 6010,
    #[error("IIA")]
    Iia = 6011,
    #[error("NP")]
    Np = 6012,
    #[error("LO")]
    Lo = 6013,
    #[error("R")]
    R = 6014,
    #[error("T")]
    T = 6015,
    #[error("LS")]
    Ls = 6016,
    #[error("LA")]
    La = 6017,
    #[error("Transaction too old")]
    TransactionTooOld = 6018,
    #[error("Price slippage check")]
    PriceSlippageCheck = 6019,
    #[error("Not approved")]
    NotApproved = 6020,
    #[error("Too little received")]
    TooLittleReceived = 6021,
}
impl From<CykuraError> for ProgramError {
    fn from(e: CykuraError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
