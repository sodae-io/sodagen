use solana_program_error::ProgramError;
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum PumpAmmError {
    #[error("FeeBasisPointsExceedsMaximum")]
    FeeBasisPointsExceedsMaximum = 6000,
    #[error("ZeroBaseAmount")]
    ZeroBaseAmount = 6001,
    #[error("ZeroQuoteAmount")]
    ZeroQuoteAmount = 6002,
    #[error("TooLittlePoolTokenLiquidity")]
    TooLittlePoolTokenLiquidity = 6003,
    #[error("ExceededSlippage")]
    ExceededSlippage = 6004,
    #[error("InvalidAdmin")]
    InvalidAdmin = 6005,
    #[error("UnsupportedBaseMint")]
    UnsupportedBaseMint = 6006,
    #[error("UnsupportedQuoteMint")]
    UnsupportedQuoteMint = 6007,
    #[error("InvalidBaseMint")]
    InvalidBaseMint = 6008,
    #[error("InvalidQuoteMint")]
    InvalidQuoteMint = 6009,
    #[error("InvalidLpMint")]
    InvalidLpMint = 6010,
    #[error("AllProtocolFeeRecipientsShouldBeNonZero")]
    AllProtocolFeeRecipientsShouldBeNonZero = 6011,
    #[error("UnsortedNotUniqueProtocolFeeRecipients")]
    UnsortedNotUniqueProtocolFeeRecipients = 6012,
    #[error("InvalidProtocolFeeRecipient")]
    InvalidProtocolFeeRecipient = 6013,
    #[error("InvalidPoolBaseTokenAccount")]
    InvalidPoolBaseTokenAccount = 6014,
    #[error("InvalidPoolQuoteTokenAccount")]
    InvalidPoolQuoteTokenAccount = 6015,
    #[error("BuyMoreBaseAmountThanPoolReserves")]
    BuyMoreBaseAmountThanPoolReserves = 6016,
    #[error("DisabledCreatePool")]
    DisabledCreatePool = 6017,
    #[error("DisabledDeposit")]
    DisabledDeposit = 6018,
    #[error("DisabledWithdraw")]
    DisabledWithdraw = 6019,
    #[error("DisabledBuy")]
    DisabledBuy = 6020,
    #[error("DisabledSell")]
    DisabledSell = 6021,
    #[error("SameMint")]
    SameMint = 6022,
    #[error("Overflow")]
    Overflow = 6023,
    #[error("Truncation")]
    Truncation = 6024,
    #[error("DivisionByZero")]
    DivisionByZero = 6025,
    #[error("NewSizeLessThanCurrentSize")]
    NewSizeLessThanCurrentSize = 6026,
    #[error("AccountTypeNotSupported")]
    AccountTypeNotSupported = 6027,
    #[error("OnlyCanonicalPumpPoolsCanHaveCoinCreator")]
    OnlyCanonicalPumpPoolsCanHaveCoinCreator = 6028,
    #[error("InvalidAdminSetCoinCreatorAuthority")]
    InvalidAdminSetCoinCreatorAuthority = 6029,
    #[error("StartTimeInThePast")]
    StartTimeInThePast = 6030,
    #[error("EndTimeInThePast")]
    EndTimeInThePast = 6031,
    #[error("EndTimeBeforeStartTime")]
    EndTimeBeforeStartTime = 6032,
    #[error("TimeRangeTooLarge")]
    TimeRangeTooLarge = 6033,
    #[error("EndTimeBeforeCurrentDay")]
    EndTimeBeforeCurrentDay = 6034,
    #[error("SupplyUpdateForFinishedRange")]
    SupplyUpdateForFinishedRange = 6035,
    #[error("DayIndexAfterEndIndex")]
    DayIndexAfterEndIndex = 6036,
    #[error("DayInActiveRange")]
    DayInActiveRange = 6037,
    #[error("InvalidIncentiveMint")]
    InvalidIncentiveMint = 6038,
    #[error("buy: Not enough quote tokens to cover for fees.")]
    BuyNotEnoughQuoteTokensToCoverFees = 6039,
    #[error("buy: slippage - would buy less tokens than expected min_base_amount_out")]
    BuySlippageBelowMinBaseAmountOut = 6040,
    #[error("MayhemModeDisabled")]
    MayhemModeDisabled = 6041,
    #[error("OnlyPumpPoolsMayhemMode")]
    OnlyPumpPoolsMayhemMode = 6042,
    #[error("MayhemModeInDesiredState")]
    MayhemModeInDesiredState = 6043,
    #[error("NotEnoughRemainingAccounts")]
    NotEnoughRemainingAccounts = 6044,
    #[error("InvalidSharingConfigBaseMint")]
    InvalidSharingConfigBaseMint = 6045,
    #[error("InvalidSharingConfigCoinCreator")]
    InvalidSharingConfigCoinCreator = 6046,
    #[error(
        "coin creator has been migrated to sharing config, use pump_fees::reset_fee_sharing_config instead"
    )]
    CoinCreatorMigratedToSharingConfig = 6047,
    #[error(
        "creator_vault has been migrated to sharing config, use pump:distribute_creator_fees instead"
    )]
    CreatorVaultMigratedToSharingConfig = 6048,
    #[error("Cashback is disabled")]
    CashbackNotEnabled = 6049,
    #[error("OnlyPumpPoolsCashback")]
    OnlyPumpPoolsCashback = 6050,
    #[error("CashbackNotInDesiredState")]
    CashbackNotInDesiredState = 6051,
    #[error("TokensInVaultLessThanCashbackEarned")]
    TokensInVaultLessThanCashbackEarned = 6052,
    #[error("Buyback fee recipient not authorized")]
    BuybackFeeRecipientNotAuthorized = 6053,
    #[error("AllBuybackFeeRecipientsShouldBeNonZero")]
    AllBuybackFeeRecipientsShouldBeNonZero = 6054,
    #[error("NotUniqueBuybackFeeRecipients")]
    NotUniqueBuybackFeeRecipients = 6055,
    #[error("buyback_basis_points must be <= 10_000")]
    BuybackBasisPointsOutOfRange = 6056,
    #[error("buyback fee recipients require exactly 8 remaining accounts (or none)")]
    WrongBuybackFeeRecipientsCount = 6057,
    #[error("BuybackFeeRecipientMissing")]
    BuybackFeeRecipientMissing = 6058,
    #[error("Cashback trade is missing the required remaining accounts")]
    MissingCashbackAccounts = 6059,
    #[error("Cashback user_volume_accumulator account is invalid")]
    InvalidCashbackAccumulator = 6060,
    #[error("Cashback user_volume_accumulator ATA is missing or invalid")]
    InvalidCashbackAccumulatorAta = 6061,
    #[error("pool_v2 remaining account is missing or invalid")]
    InvalidPoolV2 = 6062,
    #[error(
        "BOOST: sell output exceeds the real quote vault. effective = real + virtual is pricing-only; payout is capped at real_vault, so quote min(out, real_vault)"
    )]
    InsufficientRealQuoteReserves = 6063,
    #[error("BOOST: deposit/withdraw don't apply to boost pools")]
    BoostPoolLiquidityUnsupported = 6064,
    #[error("BOOST: pool cannot be boosted (no virtual reserves)")]
    PoolCannotBoost = 6065,
    #[error("BOOST: boost is disabled")]
    BoostDisabled = 6066,
    #[error("BOOST: lp_supply must never drop below the circulating LP mint supply")]
    SeedLockViolation = 6067,
    #[error("Configurable creator fee is disabled")]
    CreatorFeeNotConfigurable = 6068,
    #[error("Creator fee basis points must be between 1 and the configured maximum")]
    CreatorFeeBpsOutOfRange = 6069,
    #[error("Creator fee is not editable for this pool")]
    CreatorFeeNotEditable = 6070,
    #[error("Cashback coins cannot have a creator fee")]
    CreatorFeeNotAllowedForCashbackCoin = 6071,
    #[error("Sharing config is not active")]
    SharingConfigNotActive = 6072,
    #[error("Not authorized")]
    NotAuthorized = 6073,
}
impl From<PumpAmmError> for ProgramError {
    fn from(e: PumpAmmError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
