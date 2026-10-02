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
    #[error("Vault oracle is stale")]
    StaleOracle = 6009,
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
    #[error("Vault tranching is not enabled")]
    TranchingNotEnabled = 6025,
    #[error("Invalid leverage factor - must be at least 10000 (1x)")]
    InvalidLeverageFactor = 14032,
    #[error("Insufficient staked amount")]
    InsufficientStakedAmount = 14033,
    #[error("Junior tranche has no value")]
    JuniorTrancheEmpty = 14034,
    #[error("Invalid tranche configuration")]
    InvalidTrancheConfig = 6028,
    #[error("Withdrawal queue is not yet expired")]
    WithdrawalQueueNotExpired = 6031,
    #[error("Invalid withdrawal queue")]
    InvalidWithdrawalQueue = 6032,
    #[error("Account does not belong to the expected parent")]
    InvalidAccount = 14038,
    #[error("Required optional account is missing")]
    MissingAccount = 14039,
    #[error("Junior tranche is wiped out and cannot accept new stakes")]
    TrancheWipedOut = 14040,
    #[error("Vault does not accept losses — enable losses_accepted before retrying")]
    VaultLossNotAccepted = 14041,
    #[error("Capital loss exceeds available yielding reserve")]
    CapitalLossExceedsReserve = 14042,
    #[error("Rolling withdrawal limit exceeded")]
    RollingWithdrawalLimitExceeded = 14043,
    #[error("Legacy vault has already been migrated")]
    VaultAlreadyMigrated = 14044,
    #[error("Legacy vault migration is incomplete or invalid")]
    InvalidMigrationState = 14045,
    #[error("Mint authority does not match the expected migration source")]
    InvalidMigrationMintAuthority = 14046,
    #[error(
        "Legacy bank or tranche migration is incomplete — source accounting and escrow must be fully drained before mint authority transfer"
    )]
    BankMigrationIncomplete = 14047,
    #[error("Supplied share mint already has non-zero supply")]
    MintSupplyNonZero = 6000,
    #[error("Supplied mint is not accepted by this vault")]
    InvalidAssetMint = 6001,
    #[error("Deposit amount must be greater than zero")]
    ZeroDepositAmount = 6002,
    #[error("Deposit would exceed the vault's TVL limit")]
    ExceedsTvlLimit = 6003,
    #[error("Burn amount must be greater than zero")]
    ZeroBurnAmount = 6004,
    #[error("Asset amount must be greater than zero")]
    ZeroAssetAmount = 6005,
    #[error("Insufficient vault liquidity to satisfy withdrawal")]
    InsufficientVaultLiquidity = 6006,
    #[error("Mint authority is not the curator")]
    MintAuthorityNotCurator = 6007,
    #[error("Vault total supply is zero")]
    ZeroTotalSupply = 6008,
    #[error("Invalid oracle update")]
    InvalidOracleUpdate = 6010,
    #[error("Initial mint share price must be greater than zero")]
    InvalidInitialMintSharePrice = 6011,
    #[error("Oracle NAV update exceeds the vault's maximum acceptable APY")]
    MaxApyExceeded = 6012,
    #[error("Invalid APY configuration")]
    InvalidApyConfig = 6013,
    #[error("Invalid fee configuration")]
    InvalidFeeConfig = 6014,
    #[error("Vault has live supply, TVL, or holdings")]
    VaultHasLiveAssets = 6015,
    #[error("Deposit would mint zero shares")]
    DepositMintsZeroShares = 6016,
    #[error("Insufficient accrued APY balance")]
    InsufficientAccruedApyBalance = 6017,
    #[error("Invalid role configuration")]
    InvalidRole = 6018,
    #[error("Invalid protocol interaction CPI")]
    InvalidProtocolInteraction = 6019,
    #[error("Reserved")]
    Reserved7020 = 6020,
    #[error("Invalid external liquidity configuration")]
    InvalidExternalLiquidity = 6021,
    #[error("Swap did not change both source and destination balances")]
    SwapBalanceUnchanged = 6022,
    #[error("Protocol interaction did not change the affected token balance")]
    ProtocolBalanceUnchanged = 6023,
    #[error("Swap TVL loss exceeds the allowed slippage")]
    ExcessiveSwapSlippage = 6024,
    #[error("Vault tranching is already enabled")]
    TranchingAlreadyEnabled = 6026,
    #[error("Tranche state account is required")]
    TrancheStateRequired = 6027,
    #[error("Invalid tranche mint")]
    InvalidTrancheMint = 6029,
    #[error("Vault losses are not enabled")]
    LossesDisabled = 6030,
    #[error(
        "Destination asset has no price set; a consensus oracle price is required before swapping"
    )]
    AssetPriceNotSet = 6033,
    #[error("Asset holding still has a balance and cannot be removed")]
    AssetHoldingNotEmpty = 6034,
    #[error("The base asset holding cannot be removed")]
    CannotRemoveBaseHolding = 6035,
    #[error("Caller is not authorized to trigger an asset price update")]
    UnauthorizedPriceUpdater = 6036,
    #[error("Invalid asset-price staleness threshold")]
    InvalidStalenessConfig = 6037,
    #[error("Asset was rebalanced too recently")]
    AssetRebalanceRateLimitExceeded = 6038,
    #[error("Share token program does not match the vault/tranche mint configuration")]
    InvalidShareTokenProgram = 6039,
    #[error("Caller is not on the vault-creator whitelist")]
    UnauthorizedVaultCreator = 6040,
    #[error("Share mint decimals must match the deposit asset mint decimals")]
    InvalidShareMintDecimals = 6041,
    #[error("Invalid fee-exemption PDA signer or seeds")]
    InvalidFeeExemption = 6042,
}
impl From<BankinecoError> for ProgramError {
    fn from(e: BankinecoError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
