use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum CarrotError {
    #[error("Asset Already Exists")]
    AssetAlreadyExists = 6000,
    #[error("Asset Not Found")]
    AssetNotFound = 6001,
    #[error("Oracle Not Found")]
    OracleNotFound = 6002,
    #[error("Oracle Returned Stale Price")]
    OracleStalePrice = 6003,
    #[error("Strategy Not Found")]
    StrategyNotFound = 6004,
    #[error("Ata Not Found")]
    AtaNotFound = 6005,
    #[error("Strategy Already Exists")]
    StrategyAlreadyExists = 6006,
    #[error("Invalid Strategy Type")]
    InvalidStrategyType = 6007,
    #[error("Strategy Balance Calculation Error")]
    StrategyBalanceCalculationError = 6008,
    #[error("Vault is Paused")]
    VaultIsPaused = 6009,
    #[error("Invalid Vault Authority")]
    InvalidVaultAuthority = 6010,
    #[error("Account Migration Error")]
    MigrationError = 6011,
    #[error("Invalid Price Confidence Interval")]
    InvalidPriceConf = 6012,
    #[error("Invalid Strategy Accounts")]
    InvalidStrategyAccounts = 6013,
    #[error("Strategy Not Empty")]
    StrategyNotEmpty = 6014,
    #[error("Asset Not Empty")]
    AssetNotEmpty = 6015,
    #[error("Invalid Fee Bps")]
    InvalidFeeBps = 6016,
    #[error("Invalid Return Data")]
    InvalidReturnData = 6017,
    #[error("Invalid Coin Supply")]
    InvalidCoinSupply = 6018,
    #[error("Invalid NAV")]
    InvalidNav = 6019,
    #[error("Math Error")]
    MathError = 6020,
    #[error("Chest Low Balance")]
    ChestLowBalance = 6021,
}
impl From<CarrotError> for ProgramError {
    fn from(e: CarrotError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
