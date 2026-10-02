use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum RaydiumAmmError {
    #[error("Already in use")]
    AlreadyInUse = 0,
    #[error("Invalid program address")]
    InvalidProgramAddress = 1,
    #[error("Expected mint")]
    ExpectedMint = 2,
    #[error("Expected account")]
    ExpectedAccount = 3,
    #[error("Invalid coin vault")]
    InvalidCoinVault = 4,
    #[error("Invalid PC vault")]
    InvalidPcVault = 5,
    #[error("Invalid token LP")]
    InvalidTokenLp = 6,
    #[error("Invalid destination token coin")]
    InvalidDestTokenCoin = 7,
    #[error("Invalid destination token PC")]
    InvalidDestTokenPc = 8,
    #[error("Invalid pool mint")]
    InvalidPoolMint = 9,
    #[error("Invalid open orders")]
    InvalidOpenOrders = 10,
    #[error("Invalid market")]
    InvalidMarket = 11,
    #[error("Invalid market program")]
    InvalidMarketProgram = 12,
    #[error("Invalid target orders")]
    InvalidTargetOrders = 13,
    #[error("Account must be writable")]
    AccountNeedWriteable = 14,
    #[error("Account must be read-only")]
    AccountNeedReadOnly = 15,
    #[error("Invalid coin mint")]
    InvalidCoinMint = 16,
    #[error("Invalid PC mint")]
    InvalidPcMint = 17,
    #[error("Invalid owner")]
    InvalidOwner = 18,
    #[error("Invalid supply")]
    InvalidSupply = 19,
    #[error("Invalid delegate")]
    InvalidDelegate = 20,
    #[error("Invalid sign account")]
    InvalidSignAccount = 21,
    #[error("Invalid status")]
    InvalidStatus = 22,
    #[error("Invalid instruction")]
    InvalidInstruction = 23,
    #[error("Wrong accounts number")]
    WrongAccountsNumber = 24,
    #[error("Invalid target account owner")]
    InvalidTargetAccountOwner = 25,
    #[error("Invalid target owner")]
    InvalidTargetOwner = 26,
    #[error("Invalid AMM account owner")]
    InvalidAmmAccountOwner = 27,
    #[error("Invalid parameter set")]
    InvalidParamsSet = 28,
    #[error("Invalid input")]
    InvalidInput = 29,
    #[error("Exceeded desired slippage limit")]
    ExceededSlippage = 30,
    #[error("Calculation exchange rate failed")]
    CalculationExRateFailure = 31,
    #[error("Checked subtraction overflow")]
    CheckedSubOverflow = 32,
    #[error("Checked addition overflow")]
    CheckedAddOverflow = 33,
    #[error("Checked multiplication overflow")]
    CheckedMulOverflow = 34,
    #[error("Checked division overflow")]
    CheckedDivOverflow = 35,
    #[error("Empty funds")]
    CheckedEmptyFunds = 36,
    #[error("P&L calculation error")]
    CalcPnlError = 37,
    #[error("Invalid SPL token program")]
    InvalidSplTokenProgram = 38,
    #[error("Take P&L error")]
    TakePnlError = 39,
    #[error("Insufficient funds")]
    InsufficientFunds = 40,
    #[error("Conversion to u64 failed with overflow or underflow")]
    ConversionFailure = 41,
    #[error("User token input does not match AMM")]
    InvalidUserToken = 42,
    #[error("Invalid SRM mint")]
    InvalidSrmMint = 43,
    #[error("Invalid SRM token")]
    InvalidSrmToken = 44,
    #[error("Too many open orders")]
    TooManyOpenOrders = 45,
    #[error("Order at slot is already placed")]
    OrderAtSlotIsPlaced = 46,
    #[error("Invalid system program address")]
    InvalidSysProgramAddress = 47,
    #[error("Invalid fee")]
    InvalidFee = 48,
    #[error("Repeat AMM creation for the market")]
    RepeatCreateAmm = 49,
    #[error("Zero LP not allowed")]
    NotAllowZeroLp = 50,
    #[error("Token account has a close authority")]
    InvalidCloseAuthority = 51,
    #[error("Pool token mint has a freeze authority")]
    InvalidFreezeAuthority = 52,
    #[error("Invalid referrer PC mint")]
    InvalidReferPcMint = 53,
    #[error("Invalid configuration account")]
    InvalidConfigAccount = 54,
    #[error("Repeat configuration account creation")]
    RepeatCreateConfigAccount = 55,
    #[error("Market lot size is too large")]
    MarketLotSizeIsTooLarge = 56,
    #[error("Initial LP amount is too low")]
    InitLpAmountTooLess = 57,
    #[error("Unknown AMM error")]
    UnknownAmmError = 58,
    #[error("Not allowed due to same mint")]
    NotAllowed = 59,
    #[error("Lamports calculate error")]
    LamportsCalculateError = 60,
}
impl From<RaydiumAmmError> for ProgramError {
    fn from(e: RaydiumAmmError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
