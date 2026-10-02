use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum OnreappError {
    #[error("Math Overflow")]
    MathOverflow = 6000,
    #[error("Max Supply Exceeded")]
    MaxSupplyExceeded = 6001,
    #[error("Max Mint Amount Exceeded")]
    MaxMintAmountExceeded = 6002,
    #[error("Transfer Fee Not Supported")]
    TransferFeeNotSupported = 6003,
    #[error("Zero Price Not Allowed")]
    ZeroPriceNotAllowed = 6004,
    #[error("Decimals Exceed Max")]
    DecimalsExceedMax = 6005,
    #[error("Result Overflow")]
    ResultOverflow = 6006,
    #[error("Expired")]
    Expired = 6007,
    #[error("Wrong Program")]
    WrongProgram = 6008,
    #[error("Wrong User")]
    WrongUser = 6009,
    #[error("Missing Ed25519 Ix")]
    MissingEd25519Ix = 6010,
    #[error("Wrong Ix Program")]
    WrongIxProgram = 6011,
    #[error("Bad Ed25519 Accounts")]
    BadEd25519Accounts = 6012,
    #[error("Malformed Ed25519 Ix")]
    MalformedEd25519Ix = 6013,
    #[error("Multiple Sigs")]
    MultipleSigs = 6014,
    #[error("Wrong Authority")]
    WrongAuthority = 6015,
    #[error("Msg Mismatch")]
    MsgMismatch = 6016,
    #[error("Msg Deserialize")]
    MsgDeserialize = 6017,
    #[error("Invalid Fee")]
    InvalidFee = 6018,
    #[error("Invalid Token In Mint")]
    InvalidTokenInMint = 6019,
    #[error("Invalid Token Out Mint")]
    InvalidTokenOutMint = 6020,
    #[error("Vector Not Found")]
    VectorNotFound = 6021,
    #[error("Start Time In Past")]
    StartTimeInPast = 6022,
    #[error("Invalid Boss")]
    InvalidBoss = 6023,
    #[error("Kill Switch Activated")]
    KillSwitchActivated = 6024,
    #[error("Permissionless Not Allowed")]
    PermissionlessNotAllowed = 6025,
    #[error("Invalid Market Stats Pda")]
    InvalidMarketStatsPda = 6026,
    #[error("Market Stats Not Writable")]
    MarketStatsNotWritable = 6027,
    #[error("Invalid Instructions Sysvar")]
    InvalidInstructionsSysvar = 6028,
    #[error("Invalid Permissionless Token Out Account")]
    InvalidPermissionlessTokenOutAccount = 6029,
    #[error("Invalid User Token Out Account")]
    InvalidUserTokenOutAccount = 6030,
    #[error("Invalid Boss Token In Account")]
    InvalidBossTokenInAccount = 6031,
    #[error("Invalid Time Range")]
    InvalidTimeRange = 6032,
    #[error("Zero Value")]
    ZeroValue = 6033,
    #[error("Duplicate Start Time")]
    DuplicateStartTime = 6034,
    #[error("Too Many Vectors")]
    TooManyVectors = 6035,
    #[error("Invalid A P R")]
    InvalidApr = 6036,
    #[error("Invalid Price Fix Duration")]
    InvalidPriceFixDuration = 6037,
    #[error("Invalid Vault Authority")]
    InvalidVaultAuthority = 6038,
    #[error("Invalid Mint Authority")]
    InvalidMintAuthority = 6039,
    #[error("Offer Not Found")]
    OfferNotFound = 6040,
    #[error("No Active Vector")]
    NoActiveVector = 6041,
    #[error("Overflow Error")]
    OverflowError = 6042,
    #[error("Approval Required")]
    ApprovalRequired = 6043,
    #[error("Account Full")]
    AccountFull = 6044,
    #[error("Invalid Token Program")]
    InvalidTokenProgram = 6045,
    #[error("Invalid Onyc Mint")]
    InvalidOnycMint = 6046,
    #[error("Invalid Market Stats Owner")]
    InvalidMarketStatsOwner = 6047,
    #[error("Invalid Market Stats Data")]
    InvalidMarketStatsData = 6048,
    #[error("Invalid Circulating Supply Excluded Accounts")]
    InvalidCirculatingSupplyExcludedAccounts = 6049,
    #[error("Invalid Circulating Supply Excluded Accounts Owner")]
    InvalidCirculatingSupplyExcludedAccountsOwner = 6050,
    #[error("Invalid Circulating Supply Excluded Accounts Data")]
    InvalidCirculatingSupplyExcludedAccountsData = 6051,
    #[error("Invalid Circulating Supply Excluded Balance")]
    InvalidCirculatingSupplyExcludedBalance = 6052,
    #[error("Invalid Circulating Supply Excluded Balance Owner")]
    InvalidCirculatingSupplyExcludedBalanceOwner = 6053,
    #[error("Invalid Circulating Supply Excluded Balance Data")]
    InvalidCirculatingSupplyExcludedBalanceData = 6054,
    #[error("Missing Excluded Token Account")]
    MissingExcludedTokenAccount = 6055,
    #[error("Too Many Excluded Token Accounts")]
    TooManyExcludedTokenAccounts = 6056,
    #[error("Invalid Excluded Token Account")]
    InvalidExcludedTokenAccount = 6057,
    #[error("Duplicate Excluded Account Owner")]
    DuplicateExcludedAccountOwner = 6058,
    #[error("Overflow")]
    Overflow = 6059,
    #[error("Invalid Main Offer")]
    InvalidMainOffer = 6060,
    #[error("Div By Zero")]
    DivByZero = 6061,
    #[error("Invalid Vault Account")]
    InvalidVaultAccount = 6062,
    #[error("Boss Already Set")]
    BossAlreadySet = 6063,
    #[error("Wrong Boss")]
    WrongBoss = 6064,
    #[error("Wrong Owner")]
    WrongOwner = 6065,
    #[error("Immutable Program")]
    ImmutableProgram = 6066,
    #[error("Wrong Program Data")]
    WrongProgramData = 6067,
    #[error("Missing Program Data")]
    MissingProgramData = 6068,
    #[error("Deserialize Program Data Failed")]
    DeserializeProgramDataFailed = 6069,
    #[error("Not Program Data")]
    NotProgramData = 6070,
    #[error("Invalid Permissionless Account Name")]
    InvalidPermissionlessAccountName = 6071,
    #[error("Both Approvers Filled")]
    BothApproversFilled = 6072,
    #[error("Invalid Approver")]
    InvalidApprover = 6073,
    #[error("Approver Already Exists")]
    ApproverAlreadyExists = 6074,
    #[error("Only Boss Can Disable")]
    OnlyBossCanDisable = 6075,
    #[error("Unauthorized To Enable")]
    UnauthorizedToEnable = 6076,
    #[error("Not An Approver")]
    NotAnApprover = 6077,
    #[error("Invalid State Owner")]
    InvalidStateOwner = 6078,
    #[error("Invalid State Pda")]
    InvalidStatePda = 6079,
    #[error("Invalid State Data")]
    InvalidStateData = 6080,
    #[error("Unauthorized Signer")]
    UnauthorizedSigner = 6081,
    #[error("Lamport Overflow")]
    LamportOverflow = 6082,
    #[error("No Boss Proposal")]
    NoBossProposal = 6083,
    #[error("Not Proposed Boss")]
    NotProposedBoss = 6084,
    #[error("Invalid Boss Address")]
    InvalidBossAddress = 6085,
    #[error("No Change")]
    NoChange = 6086,
    #[error("Admin Already Exists")]
    AdminAlreadyExists = 6087,
    #[error("Max Admins Reached")]
    MaxAdminsReached = 6088,
    #[error("Admin Not Found")]
    AdminNotFound = 6089,
    #[error("Program Not Mint Authority")]
    ProgramNotMintAuthority = 6090,
    #[error("No Mint Authority")]
    NoMintAuthority = 6091,
    #[error("Boss Not Mint Authority")]
    BossNotMintAuthority = 6092,
    #[error("Unauthorized")]
    Unauthorized = 6093,
    #[error("Zero Balance")]
    ZeroBalance = 6094,
    #[error("Insufficient Balance")]
    InsufficientBalance = 6095,
    #[error("Arithmetic Overflow")]
    ArithmeticOverflow = 6096,
    #[error("Invalid Mint")]
    InvalidMint = 6097,
    #[error("Invalid Redemption Offer")]
    InvalidRedemptionOffer = 6098,
    #[error("Arithmetic Underflow")]
    ArithmeticUnderflow = 6099,
    #[error("Invalid Redeemer")]
    InvalidRedeemer = 6100,
    #[error("Invalid Worker")]
    InvalidWorker = 6101,
    #[error("Invalid Redeemer Token Account")]
    InvalidRedeemerTokenAccount = 6102,
    #[error("Offer Mismatch")]
    OfferMismatch = 6103,
    #[error("Offer Mint Mismatch")]
    OfferMintMismatch = 6104,
    #[error("Invalid Redemption Offer Owner")]
    InvalidRedemptionOfferOwner = 6105,
    #[error("Invalid Redemption Offer Data")]
    InvalidRedemptionOfferData = 6106,
    #[error("Invalid Fee Destination Token In Account")]
    InvalidFeeDestinationTokenInAccount = 6107,
    #[error("Invalid Offer Vault Onyc Account")]
    InvalidOfferVaultOnycAccount = 6108,
    #[error("Invalid Vault Token In Account")]
    InvalidVaultTokenInAccount = 6109,
    #[error("Invalid Vault Token Out Account")]
    InvalidVaultTokenOutAccount = 6110,
    #[error("Invalid Amount")]
    InvalidAmount = 6111,
    #[error("Offer Disabled")]
    OfferDisabled = 6112,
    #[error("Redemption Offer Disabled")]
    RedemptionOfferDisabled = 6113,
    #[error("Unauthorized To Disable Offer")]
    UnauthorizedToDisableOffer = 6114,
    #[error("Only Boss Can Enable Offer")]
    OnlyBossCanEnableOffer = 6115,
    #[error("Amount Exceeds Remaining")]
    AmountExceedsRemaining = 6116,
    #[error("Invalid Fee Destination")]
    InvalidFeeDestination = 6117,
    #[error("Invalid Configurable Vault")]
    InvalidConfigurableVault = 6118,
    #[error("Invalid Configurable Vault Owner")]
    InvalidConfigurableVaultOwner = 6119,
    #[error("Invalid Configurable Vault Data")]
    InvalidConfigurableVaultData = 6120,
    #[error("Invalid Configurable Vault Kind")]
    InvalidConfigurableVaultKind = 6121,
    #[error("Missing Configurable Vault Destination")]
    MissingConfigurableVaultDestination = 6122,
    #[error("Invalid Configurable Vault Token Account")]
    InvalidConfigurableVaultTokenAccount = 6123,
    #[error("Invalid Buffer State Account")]
    InvalidBufferStateAccount = 6124,
    #[error("Invalid Timestamp")]
    InvalidTimestamp = 6125,
    #[error("Minimum Out Not Met")]
    MinimumOutNotMet = 6126,
    #[error("Invalid Swap Pair")]
    InvalidSwapPair = 6127,
    #[error("Invalid Prop AMM Pair State")]
    InvalidPropAmmPairState = 6128,
    #[error("Prop AMM Pair Disabled")]
    PropAmmPairDisabled = 6129,
    #[error("Invalid Target Nav")]
    InvalidTargetNav = 6130,
    #[error("Invalid Asset Adjustment Amount")]
    InvalidAssetAdjustmentAmount = 6131,
    #[error("No Burn Needed")]
    NoBurnNeeded = 6132,
    #[error("Insufficient Cache Balance")]
    InsufficientCacheBalance = 6133,
    #[error("Insufficient Fee Balance")]
    InsufficientFeeBalance = 6134,
    #[error("Invalid Fee Recipient")]
    InvalidFeeRecipient = 6135,
    #[error("Invalid Burn Target")]
    InvalidBurnTarget = 6136,
    #[error("Invalid Redemption Request ID")]
    InvalidRedemptionRequestId = 6137,
}
impl From<OnreappError> for ProgramError {
    fn from(e: OnreappError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
