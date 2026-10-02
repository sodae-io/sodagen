use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum PumpError {
    #[error("The given account is not authorized to execute this instruction.")]
    NotAuthorized = 6000,
    #[error("The program is already initialized.")]
    AlreadyInitialized = 6001,
    #[error("slippage: Too much SOL required to buy the given amount of tokens.")]
    TooMuchSolRequired = 6002,
    #[error("slippage: Too little SOL received to sell the given amount of tokens.")]
    TooLittleSolReceived = 6003,
    #[error("The mint does not match the bonding curve.")]
    MintDoesNotMatchBondingCurve = 6004,
    #[error("The bonding curve has completed and liquidity migrated to raydium.")]
    BondingCurveComplete = 6005,
    #[error("The bonding curve has not completed.")]
    BondingCurveNotComplete = 6006,
    #[error("The program is not initialized.")]
    NotInitialized = 6007,
    #[error("Withdraw too frequent")]
    WithdrawTooFrequent = 6008,
    #[error("new_size should be > current_size")]
    NewSizeShouldBeGreaterThanCurrentSize = 6009,
    #[error("Account type not supported")]
    AccountTypeNotSupported = 6010,
    #[error("initial_real_token_reserves should be less than token_total_supply")]
    InitialRealTokenReservesShouldBeLessThanTokenTotalSupply = 6011,
    #[error(
        "initial_virtual_token_reserves should be greater than initial_real_token_reserves"
    )]
    InitialVirtualTokenReservesShouldBeGreaterThanInitialRealTokenReserves = 6012,
    #[error("fee_basis_points greater than maximum")]
    FeeBasisPointsGreaterThanMaximum = 6013,
    #[error("Withdraw authority cannot be set to System Program ID")]
    AllZerosWithdrawAuthority = 6014,
    #[error("pool_migration_fee should be less than final_real_sol_reserves")]
    PoolMigrationFeeShouldBeLessThanFinalRealSolReserves = 6015,
    #[error("pool_migration_fee should be greater than creator_fee + MAX_MIGRATE_FEES")]
    PoolMigrationFeeShouldBeGreaterThanCreatorFeePlusMaxMigrateFees = 6016,
    #[error("Migrate instruction is disabled")]
    DisabledWithdraw = 6017,
    #[error("Migrate instruction is disabled")]
    DisabledMigrate = 6018,
    #[error("Invalid creator pubkey")]
    InvalidCreator = 6019,
    #[error("Buy zero amount")]
    BuyZeroAmount = 6020,
    #[error("Not enough tokens to buy")]
    NotEnoughTokensToBuy = 6021,
    #[error("Sell zero amount")]
    SellZeroAmount = 6022,
    #[error("Not enough tokens to sell")]
    NotEnoughTokensToSell = 6023,
    #[error("Overflow")]
    Overflow = 6024,
    #[error("Truncation")]
    Truncation = 6025,
    #[error("Division by zero")]
    DivisionByZero = 6026,
    #[error("Not enough remaining accounts")]
    NotEnoughRemainingAccounts = 6027,
    #[error("All fee recipients should be non-zero")]
    AllFeeRecipientsShouldBeNonZero = 6028,
    #[error("Unsorted or not unique fee recipients")]
    UnsortedNotUniqueFeeRecipients = 6029,
    #[error("Creator should not be zero")]
    CreatorShouldNotBeZero = 6030,
    #[error("StartTimeInThePast")]
    StartTimeInThePast = 6031,
    #[error("EndTimeInThePast")]
    EndTimeInThePast = 6032,
    #[error("EndTimeBeforeStartTime")]
    EndTimeBeforeStartTime = 6033,
    #[error("TimeRangeTooLarge")]
    TimeRangeTooLarge = 6034,
    #[error("EndTimeBeforeCurrentDay")]
    EndTimeBeforeCurrentDay = 6035,
    #[error("SupplyUpdateForFinishedRange")]
    SupplyUpdateForFinishedRange = 6036,
    #[error("DayIndexAfterEndIndex")]
    DayIndexAfterEndIndex = 6037,
    #[error("DayInActiveRange")]
    DayInActiveRange = 6038,
    #[error("InvalidIncentiveMint")]
    InvalidIncentiveMint = 6039,
    #[error("Buy: Not enough SOL to cover for rent exemption.")]
    BuyNotEnoughSolToCoverRent = 6040,
    #[error("Buy: Not enough SOL to cover for fees.")]
    BuyNotEnoughSolToCoverFees = 6041,
    #[error("Slippage: Would buy less tokens than expected min_tokens_out")]
    BuySlippageBelowMinTokensOut = 6042,
    #[error("NameTooLong")]
    NameTooLong = 6043,
    #[error("SymbolTooLong")]
    SymbolTooLong = 6044,
    #[error("UriTooLong")]
    UriTooLong = 6045,
    #[error("CreateV2Disabled")]
    CreateV2Disabled = 6046,
    #[error("CpitializeMayhemFailed")]
    CpitializeMayhemFailed = 6047,
    #[error("MayhemModeDisabled")]
    MayhemModeDisabled = 6048,
    #[error("creator has been migrated to sharing config")]
    CreatorMigratedToSharingConfig = 6049,
    #[error(
        "creator_vault has been migrated to sharing config, use pump:distribute_creator_fees instead"
    )]
    UnableToDistributeCreatorVaultMigratedToSharingConfig = 6050,
    #[error("Sharing config is not active")]
    SharingConfigNotActive = 6051,
    #[error(
        "The recipient account is executable, so it cannot receive lamports, remove it from the team first"
    )]
    UnableToDistributeCreatorFeesToExecutableRecipient = 6052,
    #[error("Bonding curve creator does not match sharing config")]
    BondingCurveAndSharingConfigCreatorMismatch = 6053,
    #[error(
        "Remaining accounts do not match shareholders, make sure to pass exactly the same pubkeys in the same order"
    )]
    ShareholdersAndRemainingAccountsMismatch = 6054,
    #[error("Share bps must be greater than 0")]
    InvalidShareBps = 6055,
    #[error("Cashback is not enabled")]
    CashbackNotEnabled = 6056,
    #[error("Buyback fee recipient not authorized")]
    BuybackFeeRecipientNotAuthorized = 6057,
    #[error("AllBuybackFeeRecipientsShouldBeNonZero")]
    AllBuybackFeeRecipientsShouldBeNonZero = 6058,
    #[error("NotUniqueBuybackFeeRecipients")]
    NotUniqueBuybackFeeRecipients = 6059,
    #[error("buyback_basis_points must be <= 10_000")]
    BuybackBasisPointsOutOfRange = 6060,
    #[error("buyback fee recipients require exactly 8 remaining accounts (or none)")]
    WrongBuybackFeeRecipientsCount = 6061,
    #[error("BuybackFeeRecipientMissing")]
    BuybackFeeRecipientMissing = 6062,
    #[error("Unsupported quote mint")]
    UnsupportedQuoteMint = 6063,
    #[error("Create v2: quote token program must be SPL Token or Token-2022")]
    InvalidQuoteTokenProgram = 6064,
    #[error(
        "Create v2: associated quote bonding curve address does not match derivation"
    )]
    InvalidAssociatedQuoteBondingCurve = 6065,
    #[error("Quote mint whitelist is full")]
    QuoteMintWhitelistFull = 6066,
    #[error("Quote mint is already whitelisted")]
    QuoteMintAlreadyWhitelisted = 6067,
    #[error("Quote mint is not in the whitelist")]
    QuoteMintNotWhitelisted = 6068,
    #[error(
        "Quote mint cannot be added or removed via whitelist (default or native SOL mint)"
    )]
    QuoteMintNotEligibleForWhitelist = 6069,
    #[error("Unable to distribute creator fees to uninitialized account")]
    UnableToDistributeCreatorFeesToUninitializedAccount = 6070,
    #[error("Mayhem mode quote mint not allowed")]
    MayhemModeQuoteMintNotAllowed = 6071,
    #[error("Cashback trade is missing the required remaining accounts")]
    MissingCashbackAccounts = 6072,
    #[error("Cashback user_volume_accumulator account is invalid")]
    InvalidCashbackAccumulator = 6073,
    #[error("bonding_curve_v2 remaining account is missing or invalid")]
    InvalidBondingCurveV2 = 6074,
    #[error(
        "quote_control remaining account does not match derivation or is uninitialized"
    )]
    InvalidQuoteControl = 6075,
    #[error("Cashback recipient token account is not owned by user")]
    InvalidCashbackRecipient = 6076,
    #[error("Configurable creator fees are disabled")]
    CreatorFeeNotConfigurable = 6077,
    #[error("Creator fee basis points must be between 1 and the configured maximum")]
    CreatorFeeBpsOutOfRange = 6078,
    #[error("Creator fee is not editable for this bonding curve")]
    CreatorFeeNotEditable = 6079,
    #[error("Creator fee cannot be configured for a cashback coin")]
    CreatorFeeNotAllowedForCashbackCoin = 6080,
    #[error("Bonding curve has already migrated")]
    BondingCurveAlreadyMigrated = 6081,
    #[error("Cashback coins can no longer be created")]
    CashbackDeprecated = 6082,
    #[error("The creator of a holder-reward coin cannot be changed")]
    HolderRewardCreatorImmutable = 6083,
    #[error("Holder-reward coins are disabled")]
    HolderRewardDisabled = 6084,
    #[error("Holder-reward amounts and recipient accounts do not match")]
    HolderRewardRecipientsMismatch = 6085,
    #[error("The holder-rewards PDA cannot be left below its rent-exempt minimum")]
    HolderRewardsRentFloor = 6086,
    #[error("A holder-rewards token account is required on a token quote")]
    HolderRewardTokenAccountMissing = 6087,
    #[error("CTO is not allowed on a mayhem-mode coin")]
    CtoNotAllowedForMayhemCoin = 6088,
    #[error("new_creator is required unless converting to holder rewards")]
    CtoNewCreatorRequired = 6089,
    #[error("new_creator must be omitted when converting to holder rewards")]
    CtoNewCreatorNotAllowed = 6090,
    #[error(
        "Creator fee is not configurable on a SOL or whitelisted quote; the fee schedule applies"
    )]
    CreatorFeeNotConfigurableForQuote = 6091,
    #[error(
        "current_creator must be passed writable so the outgoing creator can be paid"
    )]
    CtoCreatorAccountNotWritable = 6092,
    #[error(
        "A frozen sharing-config vault account holds a balance; thaw it before the holder conversion"
    )]
    CtoSharedVaultFrozen = 6093,
}
impl From<PumpError> for ProgramError {
    fn from(e: PumpError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
