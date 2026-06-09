use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum BankinecoError {
    #[error("You are not authorized to perform this action.")]
    Unauthorized = 14001,
    #[error("Invalid mint")]
    InvalidBankMint = 14002,
    #[error("Accounting issue")]
    AccountingIssue = 14003,
    #[error("Vault accounting issue")]
    VaultAccountingIssue = 14004,
    #[error("Bank accounting issue")]
    BankAccountingIssue = 14005,
    #[error("Onchain accounting issue")]
    OnchainAccountingIssue = 14006,
    #[error("External accounting issue")]
    ExternalAccountingIssue = 14007,
    #[error("Minting failed due to slippage.")]
    MintSlippage = 14008,
    #[error("Burning failed due to slippage.")]
    BurningSlippage = 14009,
    #[error("Team account already exists")]
    TeamAccountAlreadyExists = 14010,
    #[error("Oracle account already exists")]
    OracleAccountAlreadyExists = 14011,
    #[error("Stale oracle")]
    StaleOracle = 14012,
    #[error("Price change is too large locally")]
    LocalSignificantPriceChange = 14013,
    #[error("Price change is too large")]
    SignificantPriceChange = 14014,
    #[error("Rounding impacts the protocol")]
    RoundingImpact = 14015,
    #[error("Bank is halted")]
    BankIsHalted = 14016,
    #[error("Vault is halted")]
    VaultIsHalted = 14017,
    #[error("Conversion Overflow")]
    ConversionOverflow = 14018,
    #[error("Vault type doesnt match")]
    WrongVaultType = 14019,
    #[error("Invalid argument")]
    InvalidArgument = 14020,
    #[error("User Sanity Check")]
    UserSanityCheck = 14021,
    #[error("Math overflow")]
    MathOverflow = 14022,
    #[error("Invalid MarginFi account - must be correct PDA for this vault")]
    InvalidMarginFiAccount = 14023,
    #[error("Invalid start marker")]
    InvalidStartMarker = 14024,
    #[error("Unable to push oracle update on an empty vault reserve")]
    OracleUpdateOnEmptyReserve = 14025,
    #[error("Invalid remaining accounts length")]
    InvalidRemainingAccountsLength = 14026,
    #[error("Specified withdrawal exceeds available balance")]
    WithdrawalExceedsBalance = 14027,
    #[error("Insufficient pending yield")]
    InsufficientPendingYield = 14028,
    #[error("Not supported")]
    NotSupported = 14029,
    #[error("Exceeds maximum yielding TVL")]
    ExceedsMaxYieldingTvl = 14030,
    #[error("Tranching is not enabled for this bank")]
    TranchingNotEnabled = 14031,
    #[error("Invalid leverage factor - must be at least 10000 (1x)")]
    InvalidLeverageFactor = 14032,
    #[error("Insufficient staked amount")]
    InsufficientStakedAmount = 14033,
    #[error("Junior tranche has no value")]
    JuniorTrancheEmpty = 14034,
    #[error("Invalid tranche config parameter")]
    InvalidTrancheConfig = 14035,
    #[error("Withdrawal queue lockup period has not expired")]
    WithdrawalQueueNotExpired = 14036,
    #[error("Withdrawal queue entry is not in a valid state")]
    InvalidWithdrawalQueue = 14037,
    #[error("Account does not belong to the expected parent")]
    InvalidAccount = 14038,
    #[error("Required optional account is missing")]
    MissingAccount = 14039,
    #[error("Junior tranche is wiped out and cannot accept new stakes")]
    TrancheWipedOut = 14040,
    #[error("Vault does not accept losses — enable losses_accepted before retrying")]
    VaultLossNotAccepted = 14041,
}
impl From<BankinecoError> for ProgramError {
    fn from(e: BankinecoError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
