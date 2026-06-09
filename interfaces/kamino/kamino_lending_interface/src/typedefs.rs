use borsh::{BorshDeserialize, BorshSerialize};
#[allow(unused_imports)]
use crate::*;
use solana_pubkey::Pubkey;
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct ReserveConfigCustomizationArgs {
    pub override_fixed_rate_bps: u8,
    pub fixed_borrow_rate_bps: u32,
    pub override_debt_term_seconds: u8,
    pub debt_term_seconds: u64,
    pub clear_elevation_groups: u8,
}
impl ReserveConfigCustomizationArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let override_fixed_rate_bps: u8 = crate::borsh_de_or_default(&mut reader)?;
        let fixed_borrow_rate_bps: u32 = crate::borsh_de_or_default(&mut reader)?;
        let override_debt_term_seconds: u8 = crate::borsh_de_or_default(&mut reader)?;
        let debt_term_seconds: u64 = crate::borsh_de_or_default(&mut reader)?;
        let clear_elevation_groups: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            override_fixed_rate_bps,
            fixed_borrow_rate_bps,
            override_debt_term_seconds,
            debt_term_seconds,
            clear_elevation_groups,
        })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct BorrowOrderConfigArgs {
    pub remaining_debt_amount: u64,
    pub max_borrow_rate_bps: u32,
    pub min_debt_term_seconds: u64,
    pub fillable_until_timestamp: u64,
    pub enable_auto_rollover_on_filled_borrows: bool,
}
impl BorrowOrderConfigArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let remaining_debt_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_borrow_rate_bps: u32 = crate::borsh_de_or_default(&mut reader)?;
        let min_debt_term_seconds: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fillable_until_timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        let enable_auto_rollover_on_filled_borrows: bool = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            remaining_debt_amount,
            max_borrow_rate_bps,
            min_debt_term_seconds,
            fillable_until_timestamp,
            enable_auto_rollover_on_filled_borrows,
        })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub enum UpdateConfigMode {
    #[default]
    UpdateLoanToValuePct,
    UpdateMaxLiquidationBonusBps,
    UpdateLiquidationThresholdPct,
    UpdateProtocolLiquidationFee,
    UpdateProtocolTakeRate,
    UpdateFeesOriginationFee,
    UpdateFeesFlashLoanFee,
    DeprecatedUpdateFeesReferralFeeBps,
    UpdateDepositLimit,
    UpdateBorrowLimit,
    UpdateTokenInfoLowerHeuristic,
    UpdateTokenInfoUpperHeuristic,
    UpdateTokenInfoExpHeuristic,
    UpdateTokenInfoTwapDivergence,
    UpdateTokenInfoScopeTwap,
    UpdateTokenInfoScopeChain,
    UpdateTokenInfoName,
    UpdateTokenInfoPriceMaxAge,
    UpdateTokenInfoTwapMaxAge,
    UpdateScopePriceFeed,
    UpdatePythPrice,
    UpdateSwitchboardFeed,
    UpdateSwitchboardTwapFeed,
    UpdateBorrowRateCurve,
    DeprecatedUpdateEntireReserveConfig,
    UpdateDebtWithdrawalCap,
    UpdateDepositWithdrawalCap,
    DeprecatedUpdateDebtWithdrawalCapCurrentTotal,
    DeprecatedUpdateDepositWithdrawalCapCurrentTotal,
    UpdateBadDebtLiquidationBonusBps,
    UpdateMinLiquidationBonusBps,
    UpdateDeleveragingMarginCallPeriod,
    UpdateBorrowFactor,
    DeprecatedUpdateAssetTier,
    UpdateElevationGroup,
    UpdateDeleveragingThresholdDecreaseBpsPerDay,
    DeprecatedUpdateMultiplierSideBoost,
    DeprecatedUpdateMultiplierTagBoost,
    UpdateReserveStatus,
    UpdateFarmCollateral,
    UpdateFarmDebt,
    UpdateDisableUsageAsCollateralOutsideEmode,
    UpdateBlockBorrowingAboveUtilizationPct,
    UpdateBlockPriceUsage,
    UpdateBorrowLimitOutsideElevationGroup,
    UpdateBorrowLimitsInElevationGroupAgainstThisReserve,
    UpdateHostFixedInterestRateBps,
    UpdateAutodeleverageEnabled,
    UpdateDeleveragingBonusIncreaseBpsPerDay,
    UpdateProtocolOrderExecutionFee,
    UpdateProposerAuthorityLock,
    UpdateMinDeleveragingBonusBps,
    UpdateBlockCTokenUsage,
    UpdateDebtMaturityTimestamp,
    UpdateDebtTermSeconds,
    UpdateEarlyRepayRemainingInterestPct,
    UpdateReserveEmergencyMode,
    UpdateRewardsAmountPerSlot,
    UpdateReservePermissionedOps,
}
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub enum UpdateLendingMarketConfigValue {
    Bool(bool),
    U8(u8),
    U8Array([u8; 8]),
    U16(u16),
    U64(u64),
    U128(u128),
    Pubkey(Pubkey),
    ElevationGroup(ElevationGroup),
    Name([u8; 32]),
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub enum UpdateLendingMarketMode {
    #[default]
    UpdateOwner,
    UpdateEmergencyMode,
    UpdateLiquidationCloseFactor,
    UpdateLiquidationMaxValue,
    DeprecatedUpdateGlobalUnhealthyBorrow,
    UpdateGlobalAllowedBorrow,
    UpdateEmergencyCouncil,
    UpdateMinFullLiquidationThreshold,
    UpdateInsolvencyRiskLtv,
    UpdateElevationGroup,
    UpdateReferralFeeBps,
    DeprecatedUpdateMultiplierPoints,
    UpdatePriceRefreshTriggerToMaxAgePct,
    UpdateAutodeleverageEnabled,
    UpdateBorrowingDisabled,
    UpdateMinNetValueObligationPostAction,
    UpdateMinValueLtvSkipPriorityLiqCheck,
    UpdateMinValueBfSkipPriorityLiqCheck,
    UpdatePaddingFields,
    UpdateName,
    UpdateIndividualAutodeleverageMarginCallPeriodSecs,
    UpdateInitialDepositAmount,
    UpdateObligationOrderExecutionEnabled,
    UpdateImmutableFlag,
    UpdateObligationOrderCreationEnabled,
    UpdateProposerAuthority,
    UpdatePriceTriggeredLiquidationDisabled,
    UpdateMatureReserveDebtLiquidationEnabled,
    UpdateObligationBorrowDebtTermLiquidationEnabled,
    UpdateBorrowOrderCreationEnabled,
    UpdateBorrowOrderExecutionEnabled,
    UpdateMinBorrowOrderFillValue,
    UpdateWithdrawTicketIssuanceEnabled,
    UpdateWithdrawTicketRedemptionEnabled,
    UpdateMinWithdrawQueuedLiquidityValue,
    UpdateFixedTermRolloverWindowDurationSeconds,
    UpdateOpenTermRolloverWindowDurationSeconds,
    UpdateObligationBorrowRolloverConfigurationEnabled,
    UpdateTermBasedFullLiquidationDurationSecs,
    UpdateObligationBorrowMigrationToFixedExecutionEnabled,
    UpdateMinPartialRolloverValue,
    UpdateWithdrawTicketCancellationEnabled,
    UpdatePermissioningAuthority,
    UpdatePermissionedOps,
    DeprecatedUpdateReserveRewardsMaxAprPct,
    UpdateReserveRewardsMaxAprBps,
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub enum UpdateGlobalConfigMode {
    #[default]
    PendingAdmin,
    FeeCollector,
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct LastUpdate {
    pub slot: u64,
    pub stale: u8,
    pub price_status: u8,
    pub placeholder: [u8; 6],
}
impl LastUpdate {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        let stale: u8 = crate::borsh_de_or_default(&mut reader)?;
        let price_status: u8 = crate::borsh_de_or_default(&mut reader)?;
        let placeholder: [u8; 6] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            slot,
            stale,
            price_status,
            placeholder,
        })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct ElevationGroup {
    pub max_liquidation_bonus_bps: u16,
    pub id: u8,
    pub ltv_pct: u8,
    pub liquidation_threshold_pct: u8,
    pub allow_new_loans: u8,
    pub max_reserves_as_collateral: u8,
    pub padding0: u8,
    pub debt_reserve: Pubkey,
    pub padding1: [u64; 4],
}
impl ElevationGroup {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let max_liquidation_bonus_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let id: u8 = crate::borsh_de_or_default(&mut reader)?;
        let ltv_pct: u8 = crate::borsh_de_or_default(&mut reader)?;
        let liquidation_threshold_pct: u8 = crate::borsh_de_or_default(&mut reader)?;
        let allow_new_loans: u8 = crate::borsh_de_or_default(&mut reader)?;
        let max_reserves_as_collateral: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding0: u8 = crate::borsh_de_or_default(&mut reader)?;
        let debt_reserve: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let padding1: [u64; 4] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            max_liquidation_bonus_bps,
            id,
            ltv_pct,
            liquidation_threshold_pct,
            allow_new_loans,
            max_reserves_as_collateral,
            padding0,
            debt_reserve,
            padding1,
        })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct BorrowOrder {
    pub debt_liquidity_mint: Pubkey,
    pub remaining_debt_amount: u64,
    pub filled_debt_destination: Pubkey,
    pub min_debt_term_seconds: u64,
    pub fillable_until_timestamp: u64,
    pub placed_at_timestamp: u64,
    pub last_updated_at_timestamp: u64,
    pub requested_debt_amount: u64,
    pub max_borrow_rate_bps: u32,
    pub active: u8,
    pub enable_auto_rollover_on_filled_borrows: u8,
    pub padding1: [u8; 2],
    pub end_padding: [u64; 5],
}
impl BorrowOrder {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let debt_liquidity_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let remaining_debt_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let filled_debt_destination: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let min_debt_term_seconds: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fillable_until_timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        let placed_at_timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_updated_at_timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        let requested_debt_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_borrow_rate_bps: u32 = crate::borsh_de_or_default(&mut reader)?;
        let active: u8 = crate::borsh_de_or_default(&mut reader)?;
        let enable_auto_rollover_on_filled_borrows: u8 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let padding1: [u8; 2] = crate::borsh_de_or_default(&mut reader)?;
        let end_padding: [u64; 5] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            debt_liquidity_mint,
            remaining_debt_amount,
            filled_debt_destination,
            min_debt_term_seconds,
            fillable_until_timestamp,
            placed_at_timestamp,
            last_updated_at_timestamp,
            requested_debt_amount,
            max_borrow_rate_bps,
            active,
            enable_auto_rollover_on_filled_borrows,
            padding1,
            end_padding,
        })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct FixedTermBorrowRolloverConfig {
    pub auto_rollover_enabled: u8,
    pub open_term_allowed: u8,
    pub migration_to_fixed_enabled: u8,
    pub alignment_padding: [u8; 1],
    pub max_borrow_rate_bps: u32,
    pub min_debt_term_seconds: u64,
}
impl FixedTermBorrowRolloverConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let auto_rollover_enabled: u8 = crate::borsh_de_or_default(&mut reader)?;
        let open_term_allowed: u8 = crate::borsh_de_or_default(&mut reader)?;
        let migration_to_fixed_enabled: u8 = crate::borsh_de_or_default(&mut reader)?;
        let alignment_padding: [u8; 1] = crate::borsh_de_or_default(&mut reader)?;
        let max_borrow_rate_bps: u32 = crate::borsh_de_or_default(&mut reader)?;
        let min_debt_term_seconds: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            auto_rollover_enabled,
            open_term_allowed,
            migration_to_fixed_enabled,
            alignment_padding,
            max_borrow_rate_bps,
            min_debt_term_seconds,
        })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct InitObligationArgs {
    pub tag: u8,
    pub id: u8,
}
impl InitObligationArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let tag: u8 = crate::borsh_de_or_default(&mut reader)?;
        let id: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { tag, id })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct ObligationCollateral {
    pub deposit_reserve: Pubkey,
    pub deposited_amount: u64,
    pub market_value_sf: u128,
    pub borrowed_amount_against_this_collateral_in_elevation_group: u64,
    pub padding: [u64; 9],
}
impl ObligationCollateral {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let deposit_reserve: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let deposited_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let market_value_sf: u128 = crate::borsh_de_or_default(&mut reader)?;
        let borrowed_amount_against_this_collateral_in_elevation_group: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let padding: [u64; 9] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            deposit_reserve,
            deposited_amount,
            market_value_sf,
            borrowed_amount_against_this_collateral_in_elevation_group,
            padding,
        })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct ObligationLiquidity {
    pub borrow_reserve: Pubkey,
    pub cumulative_borrow_rate_bsf: BigFractionBytes,
    pub last_borrowed_at_timestamp: u64,
    pub borrowed_amount_sf: u128,
    pub market_value_sf: u128,
    pub borrow_factor_adjusted_market_value_sf: u128,
    pub borrowed_amount_outside_elevation_groups: u64,
    pub fixed_term_borrow_rollover_config: FixedTermBorrowRolloverConfig,
    pub borrowed_amount_at_expiration: u64,
    pub padding2: [u64; 4],
}
impl ObligationLiquidity {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let borrow_reserve: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let cumulative_borrow_rate_bsf = if reader.is_empty() {
            Default::default()
        } else {
            <BigFractionBytes>::deserialize(&mut reader)?
        };
        let last_borrowed_at_timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        let borrowed_amount_sf: u128 = crate::borsh_de_or_default(&mut reader)?;
        let market_value_sf: u128 = crate::borsh_de_or_default(&mut reader)?;
        let borrow_factor_adjusted_market_value_sf: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let borrowed_amount_outside_elevation_groups: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let fixed_term_borrow_rollover_config = if reader.is_empty() {
            Default::default()
        } else {
            <FixedTermBorrowRolloverConfig>::deserialize(&mut reader)?
        };
        let borrowed_amount_at_expiration: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let padding2: [u64; 4] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            borrow_reserve,
            cumulative_borrow_rate_bsf,
            last_borrowed_at_timestamp,
            borrowed_amount_sf,
            market_value_sf,
            borrow_factor_adjusted_market_value_sf,
            borrowed_amount_outside_elevation_groups,
            fixed_term_borrow_rollover_config,
            borrowed_amount_at_expiration,
            padding2,
        })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct ObligationOrder {
    pub condition_threshold_sf: u128,
    pub opportunity_parameter_sf: u128,
    pub min_execution_bonus_bps: u16,
    pub max_execution_bonus_bps: u16,
    pub condition_type: u8,
    pub opportunity_type: u8,
    pub padding1: [u8; 10],
    pub padding2: [u128; 5],
}
impl ObligationOrder {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let condition_threshold_sf: u128 = crate::borsh_de_or_default(&mut reader)?;
        let opportunity_parameter_sf: u128 = crate::borsh_de_or_default(&mut reader)?;
        let min_execution_bonus_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let max_execution_bonus_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let condition_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let opportunity_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding1: [u8; 10] = crate::borsh_de_or_default(&mut reader)?;
        let padding2: [u128; 5] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            condition_threshold_sf,
            opportunity_parameter_sf,
            min_execution_bonus_bps,
            max_execution_bonus_bps,
            condition_type,
            opportunity_type,
            padding1,
            padding2,
        })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub enum UpdateObligationConfigMode {
    #[default]
    FixedTermRolloverEnabled,
    FixedTermRolloverMaxBorrowRateBps,
    FixedTermRolloverMinDebtTermSeconds,
    FixedTermRolloverOpenTermAllowed,
    MigrationToFixedEnabled,
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct BigFractionBytes {
    pub value: [u64; 4],
    pub padding: [u64; 2],
}
impl BigFractionBytes {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let value: [u64; 4] = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u64; 2] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { value, padding })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub enum FeeCalculation {
    #[default]
    Exclusive,
    Inclusive,
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct ReserveCollateral {
    pub mint_pubkey: Pubkey,
    pub mint_total_supply: u64,
    pub supply_vault: Pubkey,
    pub padding1: [u128; 32],
    pub padding2: [u128; 32],
}
impl ReserveCollateral {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mint_pubkey: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint_total_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let supply_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let padding1: [u128; 32] = crate::borsh_de_or_default(&mut reader)?;
        let padding2: [u128; 32] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            mint_pubkey,
            mint_total_supply,
            supply_vault,
            padding1,
            padding2,
        })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct ReserveConfig {
    pub status: u8,
    pub padding_deprecated_asset_tier: u8,
    pub host_fixed_interest_rate_bps: u16,
    pub min_deleveraging_bonus_bps: u16,
    pub block_ctoken_usage: u8,
    pub early_repay_remaining_interest_pct: u8,
    pub emergency_mode: u8,
    pub reserved1: [u8; 4],
    pub protocol_order_execution_fee_pct: u8,
    pub protocol_take_rate_pct: u8,
    pub protocol_liquidation_fee_pct: u8,
    pub loan_to_value_pct: u8,
    pub liquidation_threshold_pct: u8,
    pub min_liquidation_bonus_bps: u16,
    pub max_liquidation_bonus_bps: u16,
    pub bad_debt_liquidation_bonus_bps: u16,
    pub deleveraging_margin_call_period_secs: u64,
    pub deleveraging_threshold_decrease_bps_per_day: u64,
    pub fees: ReserveFees,
    pub borrow_rate_curve: BorrowRateCurve,
    pub borrow_factor_pct: u64,
    pub deposit_limit: u64,
    pub borrow_limit: u64,
    pub token_info: TokenInfo,
    pub deposit_withdrawal_cap: WithdrawalCaps,
    pub debt_withdrawal_cap: WithdrawalCaps,
    pub elevation_groups: [u8; 20],
    pub disable_usage_as_coll_outside_emode: u8,
    pub utilization_limit_block_borrowing_above_pct: u8,
    pub autodeleverage_enabled: u8,
    pub proposer_authority_locked: u8,
    pub borrow_limit_outside_elevation_group: u64,
    pub borrow_limit_against_this_collateral_in_elevation_group: [u64; 32],
    pub deleveraging_bonus_increase_bps_per_day: u64,
    pub debt_maturity_timestamp: u64,
    pub debt_term_seconds: u64,
    pub rewards_amount_per_slot: u64,
    pub permissioned_ops: u64,
}
impl ReserveConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let status: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding_deprecated_asset_tier: u8 = crate::borsh_de_or_default(&mut reader)?;
        let host_fixed_interest_rate_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let min_deleveraging_bonus_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let block_ctoken_usage: u8 = crate::borsh_de_or_default(&mut reader)?;
        let early_repay_remaining_interest_pct: u8 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let emergency_mode: u8 = crate::borsh_de_or_default(&mut reader)?;
        let reserved1: [u8; 4] = crate::borsh_de_or_default(&mut reader)?;
        let protocol_order_execution_fee_pct: u8 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let protocol_take_rate_pct: u8 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_liquidation_fee_pct: u8 = crate::borsh_de_or_default(&mut reader)?;
        let loan_to_value_pct: u8 = crate::borsh_de_or_default(&mut reader)?;
        let liquidation_threshold_pct: u8 = crate::borsh_de_or_default(&mut reader)?;
        let min_liquidation_bonus_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let max_liquidation_bonus_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let bad_debt_liquidation_bonus_bps: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let deleveraging_margin_call_period_secs: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let deleveraging_threshold_decrease_bps_per_day: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let fees = if reader.is_empty() {
            Default::default()
        } else {
            <ReserveFees>::deserialize(&mut reader)?
        };
        let borrow_rate_curve = if reader.is_empty() {
            Default::default()
        } else {
            <BorrowRateCurve>::deserialize(&mut reader)?
        };
        let borrow_factor_pct: u64 = crate::borsh_de_or_default(&mut reader)?;
        let deposit_limit: u64 = crate::borsh_de_or_default(&mut reader)?;
        let borrow_limit: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_info = if reader.is_empty() {
            Default::default()
        } else {
            <TokenInfo>::deserialize(&mut reader)?
        };
        let deposit_withdrawal_cap = if reader.is_empty() {
            Default::default()
        } else {
            <WithdrawalCaps>::deserialize(&mut reader)?
        };
        let debt_withdrawal_cap = if reader.is_empty() {
            Default::default()
        } else {
            <WithdrawalCaps>::deserialize(&mut reader)?
        };
        let elevation_groups: [u8; 20] = crate::borsh_de_or_default(&mut reader)?;
        let disable_usage_as_coll_outside_emode: u8 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let utilization_limit_block_borrowing_above_pct: u8 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let autodeleverage_enabled: u8 = crate::borsh_de_or_default(&mut reader)?;
        let proposer_authority_locked: u8 = crate::borsh_de_or_default(&mut reader)?;
        let borrow_limit_outside_elevation_group: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let borrow_limit_against_this_collateral_in_elevation_group: [u64; 32] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let deleveraging_bonus_increase_bps_per_day: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let debt_maturity_timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        let debt_term_seconds: u64 = crate::borsh_de_or_default(&mut reader)?;
        let rewards_amount_per_slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        let permissioned_ops: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            status,
            padding_deprecated_asset_tier,
            host_fixed_interest_rate_bps,
            min_deleveraging_bonus_bps,
            block_ctoken_usage,
            early_repay_remaining_interest_pct,
            emergency_mode,
            reserved1,
            protocol_order_execution_fee_pct,
            protocol_take_rate_pct,
            protocol_liquidation_fee_pct,
            loan_to_value_pct,
            liquidation_threshold_pct,
            min_liquidation_bonus_bps,
            max_liquidation_bonus_bps,
            bad_debt_liquidation_bonus_bps,
            deleveraging_margin_call_period_secs,
            deleveraging_threshold_decrease_bps_per_day,
            fees,
            borrow_rate_curve,
            borrow_factor_pct,
            deposit_limit,
            borrow_limit,
            token_info,
            deposit_withdrawal_cap,
            debt_withdrawal_cap,
            elevation_groups,
            disable_usage_as_coll_outside_emode,
            utilization_limit_block_borrowing_above_pct,
            autodeleverage_enabled,
            proposer_authority_locked,
            borrow_limit_outside_elevation_group,
            borrow_limit_against_this_collateral_in_elevation_group,
            deleveraging_bonus_increase_bps_per_day,
            debt_maturity_timestamp,
            debt_term_seconds,
            rewards_amount_per_slot,
            permissioned_ops,
        })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub enum ReserveFarmKind {
    #[default]
    Collateral,
    Debt,
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct ReserveFees {
    pub origination_fee_sf: u64,
    pub flash_loan_fee_sf: u64,
    pub padding: [u8; 8],
}
impl ReserveFees {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let origination_fee_sf: u64 = crate::borsh_de_or_default(&mut reader)?;
        let flash_loan_fee_sf: u64 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 8] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            origination_fee_sf,
            flash_loan_fee_sf,
            padding,
        })
    }
}
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct ReserveLiquidity {
    pub mint_pubkey: Pubkey,
    pub supply_vault: Pubkey,
    pub fee_vault: Pubkey,
    pub total_available_amount: u64,
    pub borrowed_amount_sf: u128,
    pub market_price_sf: u128,
    pub market_price_last_updated_ts: u64,
    pub mint_decimals: u64,
    pub deposit_limit_crossed_timestamp: u64,
    pub borrow_limit_crossed_timestamp: u64,
    pub cumulative_borrow_rate_bsf: BigFractionBytes,
    pub accumulated_protocol_fees_sf: u128,
    pub accumulated_referrer_fees_sf: u128,
    pub pending_referrer_fees_sf: u128,
    pub absolute_referral_rate_sf: u128,
    pub token_program: Pubkey,
    pub rewards_amount_available: u64,
    #[serde(with = "crate::big_array_serde")]
    pub padding2: [u64; 50],
    pub padding3: [u128; 32],
}
impl ReserveLiquidity {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mint_pubkey: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let supply_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let total_available_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let borrowed_amount_sf: u128 = crate::borsh_de_or_default(&mut reader)?;
        let market_price_sf: u128 = crate::borsh_de_or_default(&mut reader)?;
        let market_price_last_updated_ts: u64 = crate::borsh_de_or_default(&mut reader)?;
        let mint_decimals: u64 = crate::borsh_de_or_default(&mut reader)?;
        let deposit_limit_crossed_timestamp: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let borrow_limit_crossed_timestamp: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let cumulative_borrow_rate_bsf = if reader.is_empty() {
            Default::default()
        } else {
            <BigFractionBytes>::deserialize(&mut reader)?
        };
        let accumulated_protocol_fees_sf: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let accumulated_referrer_fees_sf: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let pending_referrer_fees_sf: u128 = crate::borsh_de_or_default(&mut reader)?;
        let absolute_referral_rate_sf: u128 = crate::borsh_de_or_default(&mut reader)?;
        let token_program: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let rewards_amount_available: u64 = crate::borsh_de_or_default(&mut reader)?;
        let padding2 = <[u64; 50] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let padding3: [u128; 32] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            mint_pubkey,
            supply_vault,
            fee_vault,
            total_available_amount,
            borrowed_amount_sf,
            market_price_sf,
            market_price_last_updated_ts,
            mint_decimals,
            deposit_limit_crossed_timestamp,
            borrow_limit_crossed_timestamp,
            cumulative_borrow_rate_bsf,
            accumulated_protocol_fees_sf,
            accumulated_referrer_fees_sf,
            pending_referrer_fees_sf,
            absolute_referral_rate_sf,
            token_program,
            rewards_amount_available,
            padding2,
            padding3,
        })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub enum ReserveStatus {
    #[default]
    Active,
    Obsolete,
    Hidden,
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct WithdrawQueue {
    pub queued_collateral_amount: u64,
    pub next_issued_ticket_sequence_number: u64,
    pub next_withdrawable_ticket_sequence_number: u64,
}
impl WithdrawQueue {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let queued_collateral_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let next_issued_ticket_sequence_number: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let next_withdrawable_ticket_sequence_number: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            queued_collateral_amount,
            next_issued_ticket_sequence_number,
            next_withdrawable_ticket_sequence_number,
        })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct WithdrawalCaps {
    pub config_capacity: i64,
    pub current_total: i64,
    pub last_interval_start_timestamp: u64,
    pub config_interval_length_seconds: u64,
}
impl WithdrawalCaps {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let config_capacity: i64 = crate::borsh_de_or_default(&mut reader)?;
        let current_total: i64 = crate::borsh_de_or_default(&mut reader)?;
        let last_interval_start_timestamp: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let config_interval_length_seconds: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            config_capacity,
            current_total,
            last_interval_start_timestamp,
            config_interval_length_seconds,
        })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct PriceHeuristic {
    pub lower: u64,
    pub upper: u64,
    pub exp: u64,
}
impl PriceHeuristic {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let lower: u64 = crate::borsh_de_or_default(&mut reader)?;
        let upper: u64 = crate::borsh_de_or_default(&mut reader)?;
        let exp: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { lower, upper, exp })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct PythConfiguration {
    pub price: Pubkey,
}
impl PythConfiguration {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let price: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { price })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct ScopeConfiguration {
    pub price_feed: Pubkey,
    pub price_chain: [u16; 4],
    pub twap_chain: [u16; 4],
}
impl ScopeConfiguration {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let price_feed: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let price_chain: [u16; 4] = crate::borsh_de_or_default(&mut reader)?;
        let twap_chain: [u16; 4] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            price_feed,
            price_chain,
            twap_chain,
        })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct SwitchboardConfiguration {
    pub price_aggregator: Pubkey,
    pub twap_aggregator: Pubkey,
}
impl SwitchboardConfiguration {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let price_aggregator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let twap_aggregator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            price_aggregator,
            twap_aggregator,
        })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct TokenInfo {
    pub name: [u8; 32],
    pub heuristic: PriceHeuristic,
    pub max_twap_divergence_bps: u64,
    pub max_age_price_seconds: u64,
    pub max_age_twap_seconds: u64,
    pub scope_configuration: ScopeConfiguration,
    pub switchboard_configuration: SwitchboardConfiguration,
    pub pyth_configuration: PythConfiguration,
    pub block_price_usage: u8,
    pub reserved: [u8; 7],
    pub padding: [u64; 19],
}
impl TokenInfo {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let name: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        let heuristic = if reader.is_empty() {
            Default::default()
        } else {
            <PriceHeuristic>::deserialize(&mut reader)?
        };
        let max_twap_divergence_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_age_price_seconds: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_age_twap_seconds: u64 = crate::borsh_de_or_default(&mut reader)?;
        let scope_configuration = if reader.is_empty() {
            Default::default()
        } else {
            <ScopeConfiguration>::deserialize(&mut reader)?
        };
        let switchboard_configuration = if reader.is_empty() {
            Default::default()
        } else {
            <SwitchboardConfiguration>::deserialize(&mut reader)?
        };
        let pyth_configuration = if reader.is_empty() {
            Default::default()
        } else {
            <PythConfiguration>::deserialize(&mut reader)?
        };
        let block_price_usage: u8 = crate::borsh_de_or_default(&mut reader)?;
        let reserved: [u8; 7] = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u64; 19] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            name,
            heuristic,
            max_twap_divergence_bps,
            max_age_price_seconds,
            max_age_twap_seconds,
            scope_configuration,
            switchboard_configuration,
            pyth_configuration,
            block_price_usage,
            reserved,
            padding,
        })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub enum ProgressCallbackType {
    #[default]
    None,
    KlendQueueAccountingHandlerOnKvault,
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct BorrowRateCurve {
    pub points: [CurvePoint; 11],
}
impl BorrowRateCurve {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let points: [CurvePoint; 11] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { points })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct CurvePoint {
    pub utilization_rate_bps: u32,
    pub borrow_rate_bps: u32,
}
impl CurvePoint {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let utilization_rate_bps: u32 = crate::borsh_de_or_default(&mut reader)?;
        let borrow_rate_bps: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            utilization_rate_bps,
            borrow_rate_bps,
        })
    }
}
