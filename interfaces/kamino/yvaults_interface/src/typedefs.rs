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
pub struct PositionRewardInfo {
    pub growth_inside_checkpoint: u128,
    pub amount_owed: u64,
}
impl PositionRewardInfo {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let growth_inside_checkpoint: u128 = crate::borsh_de_or_default(&mut reader)?;
        let amount_owed: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            growth_inside_checkpoint,
            amount_owed,
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
pub struct WhirlpoolRewardInfo {
    pub mint: Pubkey,
    pub vault: Pubkey,
    pub authority: Pubkey,
    pub emissions_per_second_x64: u128,
    pub growth_global_x64: u128,
}
impl WhirlpoolRewardInfo {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let emissions_per_second_x64: u128 = crate::borsh_de_or_default(&mut reader)?;
        let growth_global_x64: u128 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            mint,
            vault,
            authority,
            emissions_per_second_x64,
            growth_global_x64,
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
pub struct RewardInfo {
    pub reward_state: u8,
    pub open_time: u64,
    pub end_time: u64,
    pub last_update_time: u64,
    pub emissions_per_second_x64: u128,
    pub reward_total_emissioned: u64,
    pub reward_claimed: u64,
    pub token_mint: Pubkey,
    pub token_vault: Pubkey,
    pub authority: Pubkey,
    pub reward_growth_global_x64: u128,
}
impl RewardInfo {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let reward_state: u8 = crate::borsh_de_or_default(&mut reader)?;
        let open_time: u64 = crate::borsh_de_or_default(&mut reader)?;
        let end_time: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_update_time: u64 = crate::borsh_de_or_default(&mut reader)?;
        let emissions_per_second_x64: u128 = crate::borsh_de_or_default(&mut reader)?;
        let reward_total_emissioned: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reward_claimed: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let reward_growth_global_x64: u128 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            reward_state,
            open_time,
            end_time,
            last_update_time,
            emissions_per_second_x64,
            reward_total_emissioned,
            reward_claimed,
            token_mint,
            token_vault,
            authority,
            reward_growth_global_x64,
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
pub struct RebalanceRaw {
    #[serde(with = "crate::big_array_serde")]
    pub params: [u8; 128],
    #[serde(with = "crate::big_array_serde")]
    pub state: [u8; 256],
    pub reference_price_type: u8,
}
impl RebalanceRaw {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let params = <[u8; 128] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let state = <[u8; 256] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let reference_price_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            params,
            state,
            reference_price_type,
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
pub struct CollateralInfo {
    pub mint: Pubkey,
    pub lower_heuristic: u64,
    pub upper_heuristic: u64,
    pub exp_heuristic: u64,
    pub max_twap_divergence_bps: u64,
    pub scope_twap_price_chain: [u16; 4],
    pub scope_price_chain: [u16; 4],
    pub name: [u8; 32],
    pub max_age_price_seconds: u64,
    pub max_age_twap_seconds: u64,
    pub max_ignorable_amount_as_reward: u64,
    pub disabled: u8,
    pub padding0: [u8; 7],
    pub scope_staking_rate_chain: [u16; 4],
    pub scope_feed: Pubkey,
    pub padding: [u64; 4],
}
impl CollateralInfo {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lower_heuristic: u64 = crate::borsh_de_or_default(&mut reader)?;
        let upper_heuristic: u64 = crate::borsh_de_or_default(&mut reader)?;
        let exp_heuristic: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_twap_divergence_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let scope_twap_price_chain: [u16; 4] = crate::borsh_de_or_default(&mut reader)?;
        let scope_price_chain: [u16; 4] = crate::borsh_de_or_default(&mut reader)?;
        let name: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        let max_age_price_seconds: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_age_twap_seconds: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_ignorable_amount_as_reward: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let disabled: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding0: [u8; 7] = crate::borsh_de_or_default(&mut reader)?;
        let scope_staking_rate_chain: [u16; 4] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let scope_feed: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u64; 4] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            mint,
            lower_heuristic,
            upper_heuristic,
            exp_heuristic,
            max_twap_divergence_bps,
            scope_twap_price_chain,
            scope_price_chain,
            name,
            max_age_price_seconds,
            max_age_twap_seconds,
            max_ignorable_amount_as_reward,
            disabled,
            padding0,
            scope_staking_rate_chain,
            scope_feed,
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
pub struct CollateralInfoParams {
    pub mint: Pubkey,
    pub lower_heuristic: u64,
    pub upper_heuristic: u64,
    pub exp_heuristic: u64,
    pub max_twap_divergence_bps: u64,
    pub scope_twap_price_chain: [u16; 4],
    pub scope_price_chain: [u16; 4],
    pub name: [u8; 32],
    pub max_age_price_seconds: u64,
    pub max_age_twap_seconds: u64,
    pub max_ignorable_amount_as_reward: u64,
    pub disabled: u8,
    pub scope_staking_rate_chain: [u16; 4],
    pub scope_feed: Pubkey,
}
impl CollateralInfoParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lower_heuristic: u64 = crate::borsh_de_or_default(&mut reader)?;
        let upper_heuristic: u64 = crate::borsh_de_or_default(&mut reader)?;
        let exp_heuristic: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_twap_divergence_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let scope_twap_price_chain: [u16; 4] = crate::borsh_de_or_default(&mut reader)?;
        let scope_price_chain: [u16; 4] = crate::borsh_de_or_default(&mut reader)?;
        let name: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        let max_age_price_seconds: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_age_twap_seconds: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_ignorable_amount_as_reward: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let disabled: u8 = crate::borsh_de_or_default(&mut reader)?;
        let scope_staking_rate_chain: [u16; 4] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let scope_feed: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            mint,
            lower_heuristic,
            upper_heuristic,
            exp_heuristic,
            max_twap_divergence_bps,
            scope_twap_price_chain,
            scope_price_chain,
            name,
            max_age_price_seconds,
            max_age_twap_seconds,
            max_ignorable_amount_as_reward,
            disabled,
            scope_staking_rate_chain,
            scope_feed,
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
pub struct KaminoRewardInfo {
    pub decimals: u64,
    pub reward_vault: Pubkey,
    pub reward_mint: Pubkey,
    pub reward_collateral_id: u64,
    pub last_issuance_ts: u64,
    pub reward_per_second: u64,
    pub amount_uncollected: u64,
    pub amount_issued_cumulative: u64,
    pub amount_available: u64,
}
impl KaminoRewardInfo {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let decimals: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reward_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let reward_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let reward_collateral_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_issuance_ts: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reward_per_second: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_uncollected: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_issued_cumulative: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_available: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            decimals,
            reward_vault,
            reward_mint,
            reward_collateral_id,
            last_issuance_ts,
            reward_per_second,
            amount_uncollected,
            amount_issued_cumulative,
            amount_available,
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
pub struct Price {
    pub value: u64,
    pub exp: u64,
}
impl Price {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let value: u64 = crate::borsh_de_or_default(&mut reader)?;
        let exp: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { value, exp })
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
pub struct RebalanceAutodriftParams {
    pub init_drift_ticks_per_epoch: u32,
    pub ticks_below_mid: i32,
    pub ticks_above_mid: i32,
    pub frontrun_multiplier_bps: u16,
    pub staking_rate_a_source: StakingRateSource,
    pub staking_rate_b_source: StakingRateSource,
    pub init_drift_direction: DriftDirection,
    pub enforce_initial_drift_direction: u8,
    pub max_ticks_per_rebalance: u32,
    pub max_ticks_per_epoch: u32,
}
impl RebalanceAutodriftParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let init_drift_ticks_per_epoch: u32 = crate::borsh_de_or_default(&mut reader)?;
        let ticks_below_mid: i32 = crate::borsh_de_or_default(&mut reader)?;
        let ticks_above_mid: i32 = crate::borsh_de_or_default(&mut reader)?;
        let frontrun_multiplier_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let staking_rate_a_source: StakingRateSource = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let staking_rate_b_source: StakingRateSource = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let init_drift_direction: DriftDirection = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let enforce_initial_drift_direction: u8 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let max_ticks_per_rebalance: u32 = crate::borsh_de_or_default(&mut reader)?;
        let max_ticks_per_epoch: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            init_drift_ticks_per_epoch,
            ticks_below_mid,
            ticks_above_mid,
            frontrun_multiplier_bps,
            staking_rate_a_source,
            staking_rate_b_source,
            init_drift_direction,
            enforce_initial_drift_direction,
            max_ticks_per_rebalance,
            max_ticks_per_epoch,
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
pub struct RebalanceAutodriftWindow {
    pub staking_rate_a: Option<Price>,
    pub staking_rate_b: Option<Price>,
    pub epoch: u64,
    pub theoretical_tick: i32,
    pub strat_mid_tick: i32,
}
impl RebalanceAutodriftWindow {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let staking_rate_a: Option<Price> = crate::borsh_de_or_default(&mut reader)?;
        let staking_rate_b: Option<Price> = crate::borsh_de_or_default(&mut reader)?;
        let epoch: u64 = crate::borsh_de_or_default(&mut reader)?;
        let theoretical_tick: i32 = crate::borsh_de_or_default(&mut reader)?;
        let strat_mid_tick: i32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            staking_rate_a,
            staking_rate_b,
            epoch,
            theoretical_tick,
            strat_mid_tick,
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
pub struct RebalanceAutodriftEpochCapAnchor {
    pub epoch: u64,
    pub strat_mid_tick: i32,
}
impl RebalanceAutodriftEpochCapAnchor {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let epoch: u64 = crate::borsh_de_or_default(&mut reader)?;
        let strat_mid_tick: i32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { epoch, strat_mid_tick })
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
pub struct RebalanceAutodriftState {
    pub last_window: RebalanceAutodriftWindow,
    pub current_window: RebalanceAutodriftWindow,
    pub step: RebalanceAutodriftStep,
    pub epoch_cap_anchor: RebalanceAutodriftEpochCapAnchor,
}
impl RebalanceAutodriftState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let last_window = if reader.is_empty() {
            Default::default()
        } else {
            <RebalanceAutodriftWindow>::deserialize(&mut reader)?
        };
        let current_window = if reader.is_empty() {
            Default::default()
        } else {
            <RebalanceAutodriftWindow>::deserialize(&mut reader)?
        };
        let step: RebalanceAutodriftStep = crate::borsh_de_or_default(&mut reader)?;
        let epoch_cap_anchor = if reader.is_empty() {
            Default::default()
        } else {
            <RebalanceAutodriftEpochCapAnchor>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            last_window,
            current_window,
            step,
            epoch_cap_anchor,
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
pub struct RebalanceDriftParams {
    pub start_mid_tick: i32,
    pub ticks_below_mid: i32,
    pub ticks_above_mid: i32,
    pub seconds_per_tick: u64,
    pub direction: DriftDirection,
}
impl RebalanceDriftParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let start_mid_tick: i32 = crate::borsh_de_or_default(&mut reader)?;
        let ticks_below_mid: i32 = crate::borsh_de_or_default(&mut reader)?;
        let ticks_above_mid: i32 = crate::borsh_de_or_default(&mut reader)?;
        let seconds_per_tick: u64 = crate::borsh_de_or_default(&mut reader)?;
        let direction: DriftDirection = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            start_mid_tick,
            ticks_below_mid,
            ticks_above_mid,
            seconds_per_tick,
            direction,
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
pub struct RebalanceDriftState {
    pub step: RebalanceDriftStep,
    pub last_drift_timestamp: u64,
    pub last_mid_tick: i32,
}
impl RebalanceDriftState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let step: RebalanceDriftStep = crate::borsh_de_or_default(&mut reader)?;
        let last_drift_timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_mid_tick: i32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            step,
            last_drift_timestamp,
            last_mid_tick,
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
pub struct RebalanceExpanderState {
    pub initial_pool_price: u128,
    pub expansion_count: u16,
}
impl RebalanceExpanderState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let initial_pool_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        let expansion_count: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            initial_pool_price,
            expansion_count,
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
pub struct RebalanceManualState {}
impl RebalanceManualState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        *__buf = reader;
        Ok(Self {})
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
pub struct PeriodicRebalanceState {
    pub last_rebalance_timestamp: u64,
}
impl PeriodicRebalanceState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let last_rebalance_timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { last_rebalance_timestamp })
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
pub struct RebalancePricePercentageWithResetState {
    pub last_rebalance_lower_reset_pool_price: u128,
    pub last_rebalance_upper_reset_pool_price: u128,
}
impl RebalancePricePercentageWithResetState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let last_rebalance_lower_reset_pool_price: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let last_rebalance_upper_reset_pool_price: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            last_rebalance_lower_reset_pool_price,
            last_rebalance_upper_reset_pool_price,
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
pub struct RebalancePricePercentageState {
    pub last_rebalance_lower_pool_price: u128,
    pub last_rebalance_upper_pool_price: u128,
}
impl RebalancePricePercentageState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let last_rebalance_lower_pool_price: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let last_rebalance_upper_pool_price: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            last_rebalance_lower_pool_price,
            last_rebalance_upper_pool_price,
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
pub struct RebalanceTakeProfitState {
    pub step: RebalanceTakeProfitStep,
}
impl RebalanceTakeProfitState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let step: RebalanceTakeProfitStep = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { step })
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
pub enum BinAddLiquidityStrategy {
    Uniform {
        current_bin_index: i32,
        lower_bin_index: i32,
        upper_bin_index: i32,
        amount_x_to_deposit: u64,
        amount_y_to_deposit: u64,
        x_current_bin: u64,
        y_current_bin: u64,
    },
    CurrentTick(i32),
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
pub enum SimulationPrice {
    #[default]
    PoolPrice,
    SqrtPrice(u128),
    TickIndex(i32),
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
pub enum DexSpecificPrice {
    SqrtPrice(u128),
    Q6464(u128),
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
pub enum RemoveLiquidityMode {
    Liquidity(u128),
    Bps(u16),
    All,
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
pub enum WithdrawalCapAccumulatorAction {
    #[default]
    KeepAccumulator,
    ResetAccumulator,
}
impl TryFrom<u8> for WithdrawalCapAccumulatorAction {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::KeepAccumulator),
            1u8 => Ok(Self::ResetAccumulator),
            _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
        }
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
pub enum RebalanceEffects {
    NewRange(i32, i32),
    WithdrawAndFreeze,
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
pub enum SwapLimit {
    Bps(u64),
    Absolute { src_amount_to_swap: u64, dst_amount_to_vault: u64, a_to_b: bool },
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
pub enum MintingMethod {
    #[default]
    PriceBased,
    Proportional,
}
impl TryFrom<u8> for MintingMethod {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::PriceBased),
            1u8 => Ok(Self::Proportional),
            _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
        }
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
pub enum GlobalConfigOption {
    #[default]
    EmergencyMode,
    BlockDeposit,
    BlockInvest,
    BlockWithdraw,
    BlockCollectFees,
    BlockCollectRewards,
    BlockSwapRewards,
    BlockSwapUnevenVaults,
    WithdrawalFeeBps,
    DeprecatedSwapDiscountBps,
    ActionsAuthority,
    DeprecatedTreasuryFeeVaults,
    DeprecatedAdminAuthority,
    BlockEmergencySwap,
    BlockLocalAdmin,
    UpdateTokenInfos,
    ScopeProgramId,
    UpdateScopePriceId,
    MinPerformanceFeeBps,
    MinSwapUnevenSlippageToleranceBps,
    MinReferencePriceSlippageToleranceBps,
    ActionsAfterRebalanceDelaySeconds,
    TreasuryFeeVaultReceiver,
    AddScopePriceId,
    MaxDeviationFromRefPriceOnInvestBps,
    InvestCooldownSeconds,
    MinInvestTriggerValueUsd,
    PendingAdminAuthority,
    CapMaxDeviationFromRefPriceOnInvestBps,
    CapMaxPriceDeviationBps,
    EmergencyCouncil,
    UnfreezeAuthority,
    DepositMaxPoolPriceOracleDeviationBps,
    MaxInvestBpsPerOperation,
    MaxInvestUsdPerOperation,
    MaxPoolDeviationFromTwapBps,
    RefPriceMaxTtlSeconds,
    MaxDeviationFromSnapshotPriceBps,
    CapMaxDeviationFromSnapshotPriceBps,
    MinRefPriceAgeSeconds,
}
impl TryFrom<u8> for GlobalConfigOption {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::EmergencyMode),
            1u8 => Ok(Self::BlockDeposit),
            2u8 => Ok(Self::BlockInvest),
            3u8 => Ok(Self::BlockWithdraw),
            4u8 => Ok(Self::BlockCollectFees),
            5u8 => Ok(Self::BlockCollectRewards),
            6u8 => Ok(Self::BlockSwapRewards),
            7u8 => Ok(Self::BlockSwapUnevenVaults),
            8u8 => Ok(Self::WithdrawalFeeBps),
            9u8 => Ok(Self::DeprecatedSwapDiscountBps),
            10u8 => Ok(Self::ActionsAuthority),
            11u8 => Ok(Self::DeprecatedTreasuryFeeVaults),
            12u8 => Ok(Self::DeprecatedAdminAuthority),
            13u8 => Ok(Self::BlockEmergencySwap),
            14u8 => Ok(Self::BlockLocalAdmin),
            15u8 => Ok(Self::UpdateTokenInfos),
            16u8 => Ok(Self::ScopeProgramId),
            17u8 => Ok(Self::UpdateScopePriceId),
            18u8 => Ok(Self::MinPerformanceFeeBps),
            19u8 => Ok(Self::MinSwapUnevenSlippageToleranceBps),
            20u8 => Ok(Self::MinReferencePriceSlippageToleranceBps),
            21u8 => Ok(Self::ActionsAfterRebalanceDelaySeconds),
            22u8 => Ok(Self::TreasuryFeeVaultReceiver),
            23u8 => Ok(Self::AddScopePriceId),
            24u8 => Ok(Self::MaxDeviationFromRefPriceOnInvestBps),
            25u8 => Ok(Self::InvestCooldownSeconds),
            26u8 => Ok(Self::MinInvestTriggerValueUsd),
            27u8 => Ok(Self::PendingAdminAuthority),
            28u8 => Ok(Self::CapMaxDeviationFromRefPriceOnInvestBps),
            29u8 => Ok(Self::CapMaxPriceDeviationBps),
            30u8 => Ok(Self::EmergencyCouncil),
            31u8 => Ok(Self::UnfreezeAuthority),
            32u8 => Ok(Self::DepositMaxPoolPriceOracleDeviationBps),
            33u8 => Ok(Self::MaxInvestBpsPerOperation),
            34u8 => Ok(Self::MaxInvestUsdPerOperation),
            35u8 => Ok(Self::MaxPoolDeviationFromTwapBps),
            36u8 => Ok(Self::RefPriceMaxTtlSeconds),
            37u8 => Ok(Self::MaxDeviationFromSnapshotPriceBps),
            38u8 => Ok(Self::CapMaxDeviationFromSnapshotPriceBps),
            39u8 => Ok(Self::MinRefPriceAgeSeconds),
            _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
        }
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
pub enum StrategyConfigOption {
    #[default]
    UpdateDepositCap,
    UpdateDepositCapIxn,
    UpdateWithdrawalCapACapacity,
    UpdateWithdrawalCapAInterval,
    UpdateWithdrawalCapACurrentTotal,
    UpdateWithdrawalCapBCapacity,
    UpdateWithdrawalCapBInterval,
    UpdateWithdrawalCapBCurrentTotal,
    UpdateMaxDeviationBps,
    UpdateSwapVaultMaxSlippage,
    UpdateStrategyType,
    UpdateDepositFee,
    UpdateWithdrawFee,
    UpdateCollectFeesFee,
    UpdateReward0Fee,
    UpdateReward1Fee,
    UpdateReward2Fee,
    UpdateAdminAuthority,
    KaminoRewardIndex0Ts,
    KaminoRewardIndex1Ts,
    KaminoRewardIndex2Ts,
    KaminoRewardIndex0RewardPerSecond,
    KaminoRewardIndex1RewardPerSecond,
    KaminoRewardIndex2RewardPerSecond,
    UpdateDepositBlocked,
    UpdateRaydiumProtocolPositionOrBaseVaultAuthority,
    UpdateRaydiumPoolConfigOrBaseVaultAuthority,
    UpdateInvestBlocked,
    UpdateWithdrawBlocked,
    UpdateLocalAdminBlocked,
    DeprecatedUpdateCollateralIdA,
    DeprecatedUpdateCollateralIdB,
    UpdateFlashVaultSwap,
    AllowDepositWithoutInvest,
    UpdateSwapVaultMaxSlippageFromRef,
    ResetReferencePrices,
    UpdateStrategyCreationState,
    UpdateIsCommunity,
    UpdateRebalanceType,
    UpdateRebalanceParams,
    UpdateDepositMintingMethod,
    UpdateLookupTable,
    UpdateReferencePriceType,
    UpdateReward0Amount,
    UpdateReward1Amount,
    UpdateReward2Amount,
    UpdateFarm,
    UpdateRebalancesCapCapacity,
    UpdateRebalancesCapInterval,
    UpdateRebalancesCapCurrentTotal,
    DeprecatedUpdateSwapUnevenAuthority,
    UpdatePendingStrategyAdmin,
    UpdateMaxDeviationFromRefPriceOnInvestBps,
    UpdatePendingRange,
    ResetPendingRange,
    UpdateStrategyEmergencyMode,
    UpdateDepositMaxPoolPriceOracleDeviationBps,
    UpdateRewardDiscountBps,
    UpdateMaxPoolDeviationFromTwapBps,
    UpdateMaxDeviationFromSnapshotPriceBps,
}
impl TryFrom<u8> for StrategyConfigOption {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::UpdateDepositCap),
            1u8 => Ok(Self::UpdateDepositCapIxn),
            2u8 => Ok(Self::UpdateWithdrawalCapACapacity),
            3u8 => Ok(Self::UpdateWithdrawalCapAInterval),
            4u8 => Ok(Self::UpdateWithdrawalCapACurrentTotal),
            5u8 => Ok(Self::UpdateWithdrawalCapBCapacity),
            6u8 => Ok(Self::UpdateWithdrawalCapBInterval),
            7u8 => Ok(Self::UpdateWithdrawalCapBCurrentTotal),
            8u8 => Ok(Self::UpdateMaxDeviationBps),
            9u8 => Ok(Self::UpdateSwapVaultMaxSlippage),
            10u8 => Ok(Self::UpdateStrategyType),
            11u8 => Ok(Self::UpdateDepositFee),
            12u8 => Ok(Self::UpdateWithdrawFee),
            13u8 => Ok(Self::UpdateCollectFeesFee),
            14u8 => Ok(Self::UpdateReward0Fee),
            15u8 => Ok(Self::UpdateReward1Fee),
            16u8 => Ok(Self::UpdateReward2Fee),
            17u8 => Ok(Self::UpdateAdminAuthority),
            18u8 => Ok(Self::KaminoRewardIndex0Ts),
            19u8 => Ok(Self::KaminoRewardIndex1Ts),
            20u8 => Ok(Self::KaminoRewardIndex2Ts),
            21u8 => Ok(Self::KaminoRewardIndex0RewardPerSecond),
            22u8 => Ok(Self::KaminoRewardIndex1RewardPerSecond),
            23u8 => Ok(Self::KaminoRewardIndex2RewardPerSecond),
            24u8 => Ok(Self::UpdateDepositBlocked),
            25u8 => Ok(Self::UpdateRaydiumProtocolPositionOrBaseVaultAuthority),
            26u8 => Ok(Self::UpdateRaydiumPoolConfigOrBaseVaultAuthority),
            27u8 => Ok(Self::UpdateInvestBlocked),
            28u8 => Ok(Self::UpdateWithdrawBlocked),
            29u8 => Ok(Self::UpdateLocalAdminBlocked),
            30u8 => Ok(Self::DeprecatedUpdateCollateralIdA),
            31u8 => Ok(Self::DeprecatedUpdateCollateralIdB),
            32u8 => Ok(Self::UpdateFlashVaultSwap),
            33u8 => Ok(Self::AllowDepositWithoutInvest),
            34u8 => Ok(Self::UpdateSwapVaultMaxSlippageFromRef),
            35u8 => Ok(Self::ResetReferencePrices),
            36u8 => Ok(Self::UpdateStrategyCreationState),
            37u8 => Ok(Self::UpdateIsCommunity),
            38u8 => Ok(Self::UpdateRebalanceType),
            39u8 => Ok(Self::UpdateRebalanceParams),
            40u8 => Ok(Self::UpdateDepositMintingMethod),
            41u8 => Ok(Self::UpdateLookupTable),
            42u8 => Ok(Self::UpdateReferencePriceType),
            43u8 => Ok(Self::UpdateReward0Amount),
            44u8 => Ok(Self::UpdateReward1Amount),
            45u8 => Ok(Self::UpdateReward2Amount),
            46u8 => Ok(Self::UpdateFarm),
            47u8 => Ok(Self::UpdateRebalancesCapCapacity),
            48u8 => Ok(Self::UpdateRebalancesCapInterval),
            49u8 => Ok(Self::UpdateRebalancesCapCurrentTotal),
            50u8 => Ok(Self::DeprecatedUpdateSwapUnevenAuthority),
            51u8 => Ok(Self::UpdatePendingStrategyAdmin),
            52u8 => Ok(Self::UpdateMaxDeviationFromRefPriceOnInvestBps),
            53u8 => Ok(Self::UpdatePendingRange),
            54u8 => Ok(Self::ResetPendingRange),
            55u8 => Ok(Self::UpdateStrategyEmergencyMode),
            56u8 => Ok(Self::UpdateDepositMaxPoolPriceOracleDeviationBps),
            57u8 => Ok(Self::UpdateRewardDiscountBps),
            58u8 => Ok(Self::UpdateMaxPoolDeviationFromTwapBps),
            59u8 => Ok(Self::UpdateMaxDeviationFromSnapshotPriceBps),
            _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
        }
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
pub enum StrategyStatus {
    #[default]
    Uninitialized,
    Active,
    Frozen,
    Rebalancing,
    NoPosition,
}
impl TryFrom<u8> for StrategyStatus {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Uninitialized),
            1u8 => Ok(Self::Active),
            2u8 => Ok(Self::Frozen),
            3u8 => Ok(Self::Rebalancing),
            4u8 => Ok(Self::NoPosition),
            _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
        }
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
pub enum StrategyType {
    #[default]
    Stable,
    Pegged,
    Volatile,
}
impl TryFrom<u8> for StrategyType {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Stable),
            1u8 => Ok(Self::Pegged),
            2u8 => Ok(Self::Volatile),
            _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
        }
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
pub enum CreationStatus {
    #[default]
    Ignored,
    Shadow,
    Live,
    Deprecated,
    Staging,
}
impl TryFrom<u8> for CreationStatus {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Ignored),
            1u8 => Ok(Self::Shadow),
            2u8 => Ok(Self::Live),
            3u8 => Ok(Self::Deprecated),
            4u8 => Ok(Self::Staging),
            _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
        }
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
pub enum ExecutiveWithdrawAction {
    #[default]
    Freeze,
    Unfreeze,
    Rebalance,
}
impl TryFrom<u8> for ExecutiveWithdrawAction {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Freeze),
            1u8 => Ok(Self::Unfreeze),
            2u8 => Ok(Self::Rebalance),
            _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
        }
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
pub enum ReferencePriceType {
    #[default]
    Pool,
    Twap,
}
impl TryFrom<u8> for ReferencePriceType {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Pool),
            1u8 => Ok(Self::Twap),
            _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
        }
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
pub enum LiquidityCalculationMode {
    #[default]
    Deposit,
    Withdraw,
}
impl TryFrom<u8> for LiquidityCalculationMode {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Deposit),
            1u8 => Ok(Self::Withdraw),
            _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
        }
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
pub enum UpdateCollateralInfoMode {
    #[default]
    CollateralId,
    LowerHeuristic,
    UpperHeuristic,
    ExpHeuristic,
    TwapDivergence,
    UpdateScopeTwap,
    UpdateScopeChain,
    UpdateName,
    UpdatePriceMaxAge,
    UpdateTwapMaxAge,
    UpdateDisabled,
    UpdateStakingRateChain,
    UpdateMaxIgnorableAmountAsReward,
    UpdateScopeFeed,
}
impl TryFrom<u8> for UpdateCollateralInfoMode {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::CollateralId),
            1u8 => Ok(Self::LowerHeuristic),
            2u8 => Ok(Self::UpperHeuristic),
            3u8 => Ok(Self::ExpHeuristic),
            4u8 => Ok(Self::TwapDivergence),
            5u8 => Ok(Self::UpdateScopeTwap),
            6u8 => Ok(Self::UpdateScopeChain),
            7u8 => Ok(Self::UpdateName),
            8u8 => Ok(Self::UpdatePriceMaxAge),
            9u8 => Ok(Self::UpdateTwapMaxAge),
            10u8 => Ok(Self::UpdateDisabled),
            11u8 => Ok(Self::UpdateStakingRateChain),
            12u8 => Ok(Self::UpdateMaxIgnorableAmountAsReward),
            13u8 => Ok(Self::UpdateScopeFeed),
            _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
        }
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
pub enum BalanceStatus {
    #[default]
    Balanced,
    Unbalanced,
}
impl TryFrom<u8> for BalanceStatus {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Balanced),
            1u8 => Ok(Self::Unbalanced),
            _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
        }
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
pub enum RebalanceAutodriftStep {
    #[default]
    Uninitialized,
    Autodrifting,
}
impl TryFrom<u8> for RebalanceAutodriftStep {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Uninitialized),
            1u8 => Ok(Self::Autodrifting),
            _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
        }
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
pub enum StakingRateSource {
    #[default]
    Constant,
    Scope,
}
impl TryFrom<u8> for StakingRateSource {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Constant),
            1u8 => Ok(Self::Scope),
            _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
        }
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
pub enum DriftDirection {
    #[default]
    Increasing,
    Decreasing,
}
impl TryFrom<u8> for DriftDirection {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Increasing),
            1u8 => Ok(Self::Decreasing),
            _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
        }
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
pub enum RebalanceDriftStep {
    #[default]
    Uninitialized,
    Drifting,
}
impl TryFrom<u8> for RebalanceDriftStep {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Uninitialized),
            1u8 => Ok(Self::Drifting),
            _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
        }
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
pub enum ExpanderStep {
    ExpandOrContract(u16),
    Recenter,
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
pub enum RebalanceTakeProfitToken {
    #[default]
    A,
    B,
}
impl TryFrom<u8> for RebalanceTakeProfitToken {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::A),
            1u8 => Ok(Self::B),
            _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
        }
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
pub enum RebalanceTakeProfitStep {
    #[default]
    Uninitialized,
    TakingProfit,
    Finished,
}
impl TryFrom<u8> for RebalanceTakeProfitStep {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Uninitialized),
            1u8 => Ok(Self::TakingProfit),
            2u8 => Ok(Self::Finished),
            _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
        }
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
pub enum RebalanceAction {
    NewPriceRange(DexSpecificPrice, DexSpecificPrice),
    NewTickRange(i32, i32),
    WithdrawAndFreeze,
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
pub enum RebalanceType {
    #[default]
    Manual,
    PricePercentage,
    PricePercentageWithReset,
    Drift,
    TakeProfit,
    PeriodicRebalance,
    Expander,
    Autodrift,
}
impl TryFrom<u8> for RebalanceType {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Manual),
            1u8 => Ok(Self::PricePercentage),
            2u8 => Ok(Self::PricePercentageWithReset),
            3u8 => Ok(Self::Drift),
            4u8 => Ok(Self::TakeProfit),
            5u8 => Ok(Self::PeriodicRebalance),
            6u8 => Ok(Self::Expander),
            7u8 => Ok(Self::Autodrift),
            _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
        }
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
pub enum CollateralTestToken {
    #[default]
    Usdc,
    Usdh,
    Sol,
    Eth,
    Btc,
    Msol,
    Stsol,
    Usdt,
    Orca,
    Mnde,
    Hbb,
    Jsol,
    Ush,
    Dai,
    Ldo,
    Scnsol,
    Uxd,
    Hdg,
    Dust,
    Usdr,
    Ratio,
    Uxp,
    Jitosol,
    Ray,
    Bonk,
    Samo,
    LaineSol,
    Bsol,
}
impl TryFrom<u8> for CollateralTestToken {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Usdc),
            1u8 => Ok(Self::Usdh),
            2u8 => Ok(Self::Sol),
            3u8 => Ok(Self::Eth),
            4u8 => Ok(Self::Btc),
            5u8 => Ok(Self::Msol),
            6u8 => Ok(Self::Stsol),
            7u8 => Ok(Self::Usdt),
            8u8 => Ok(Self::Orca),
            9u8 => Ok(Self::Mnde),
            10u8 => Ok(Self::Hbb),
            11u8 => Ok(Self::Jsol),
            12u8 => Ok(Self::Ush),
            13u8 => Ok(Self::Dai),
            14u8 => Ok(Self::Ldo),
            15u8 => Ok(Self::Scnsol),
            16u8 => Ok(Self::Uxd),
            17u8 => Ok(Self::Hdg),
            18u8 => Ok(Self::Dust),
            19u8 => Ok(Self::Usdr),
            20u8 => Ok(Self::Ratio),
            21u8 => Ok(Self::Uxp),
            22u8 => Ok(Self::Jitosol),
            23u8 => Ok(Self::Ray),
            24u8 => Ok(Self::Bonk),
            25u8 => Ok(Self::Samo),
            26u8 => Ok(Self::LaineSol),
            27u8 => Ok(Self::Bsol),
            _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
        }
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
pub enum ScopePriceIdTest {
    #[default]
    Sol,
    Eth,
    Btc,
    Srm,
    Ray,
    Ftt,
    Msol,
    ScnSolSol,
    Bnb,
    Avax,
    DaoSolSol,
    SaberMsolSol,
    Usdh,
    StSol,
    CsolSol,
    CethEth,
    CbtcBtc,
    CmsolSol,
    WstEth,
    Ldo,
    Usdc,
    CusdcUsdc,
    Usdt,
    Orca,
    Mnde,
    Hbb,
    CorcaOrca,
    CslndSlnd,
    CsrmSrm,
    CrayRay,
    CfttFtt,
    CstsolStsol,
    Slnd,
    Dai,
    JsolSol,
    Ush,
    Uxd,
    UsdhTwap,
    UshTwap,
    UxdTwap,
    Hdg,
    Dust,
    Usdr,
    UsdrTwap,
    Ratio,
    Uxp,
    Kuxdusdcorca,
    JitosolSol,
    SolEma,
    EthEma,
    BtcEma,
    SrmEma,
    RayEma,
    FttEma,
    MsolEma,
    BnbEma,
    AvaxEma,
    StsolEma,
    UsdcEma,
    UsdtEma,
    SlndEma,
    DaiEma,
    WstEthTwap,
    DustTwap,
    Bonk,
    BonkTwap,
    Samo,
    SamoTwap,
    Bsol,
    LaineSol,
}
impl TryFrom<u8> for ScopePriceIdTest {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Sol),
            1u8 => Ok(Self::Eth),
            2u8 => Ok(Self::Btc),
            3u8 => Ok(Self::Srm),
            4u8 => Ok(Self::Ray),
            5u8 => Ok(Self::Ftt),
            6u8 => Ok(Self::Msol),
            7u8 => Ok(Self::ScnSolSol),
            8u8 => Ok(Self::Bnb),
            9u8 => Ok(Self::Avax),
            10u8 => Ok(Self::DaoSolSol),
            11u8 => Ok(Self::SaberMsolSol),
            12u8 => Ok(Self::Usdh),
            13u8 => Ok(Self::StSol),
            14u8 => Ok(Self::CsolSol),
            15u8 => Ok(Self::CethEth),
            16u8 => Ok(Self::CbtcBtc),
            17u8 => Ok(Self::CmsolSol),
            18u8 => Ok(Self::WstEth),
            19u8 => Ok(Self::Ldo),
            20u8 => Ok(Self::Usdc),
            21u8 => Ok(Self::CusdcUsdc),
            22u8 => Ok(Self::Usdt),
            23u8 => Ok(Self::Orca),
            24u8 => Ok(Self::Mnde),
            25u8 => Ok(Self::Hbb),
            26u8 => Ok(Self::CorcaOrca),
            27u8 => Ok(Self::CslndSlnd),
            28u8 => Ok(Self::CsrmSrm),
            29u8 => Ok(Self::CrayRay),
            30u8 => Ok(Self::CfttFtt),
            31u8 => Ok(Self::CstsolStsol),
            32u8 => Ok(Self::Slnd),
            33u8 => Ok(Self::Dai),
            34u8 => Ok(Self::JsolSol),
            35u8 => Ok(Self::Ush),
            36u8 => Ok(Self::Uxd),
            37u8 => Ok(Self::UsdhTwap),
            38u8 => Ok(Self::UshTwap),
            39u8 => Ok(Self::UxdTwap),
            40u8 => Ok(Self::Hdg),
            41u8 => Ok(Self::Dust),
            42u8 => Ok(Self::Usdr),
            43u8 => Ok(Self::UsdrTwap),
            44u8 => Ok(Self::Ratio),
            45u8 => Ok(Self::Uxp),
            46u8 => Ok(Self::Kuxdusdcorca),
            47u8 => Ok(Self::JitosolSol),
            48u8 => Ok(Self::SolEma),
            49u8 => Ok(Self::EthEma),
            50u8 => Ok(Self::BtcEma),
            51u8 => Ok(Self::SrmEma),
            52u8 => Ok(Self::RayEma),
            53u8 => Ok(Self::FttEma),
            54u8 => Ok(Self::MsolEma),
            55u8 => Ok(Self::BnbEma),
            56u8 => Ok(Self::AvaxEma),
            57u8 => Ok(Self::StsolEma),
            58u8 => Ok(Self::UsdcEma),
            59u8 => Ok(Self::UsdtEma),
            60u8 => Ok(Self::SlndEma),
            61u8 => Ok(Self::DaiEma),
            62u8 => Ok(Self::WstEthTwap),
            63u8 => Ok(Self::DustTwap),
            64u8 => Ok(Self::Bonk),
            65u8 => Ok(Self::BonkTwap),
            66u8 => Ok(Self::Samo),
            67u8 => Ok(Self::SamoTwap),
            68u8 => Ok(Self::Bsol),
            69u8 => Ok(Self::LaineSol),
            _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
        }
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
pub enum DEX {
    #[default]
    Orca,
    Raydium,
    Meteora,
}
impl TryFrom<u8> for DEX {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Orca),
            1u8 => Ok(Self::Raydium),
            2u8 => Ok(Self::Meteora),
            _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
        }
    }
}
