use borsh::{BorshDeserialize, BorshSerialize};
#[allow(unused_imports)]
use crate::*;
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
pub struct PoolState {
    pub market_price: u128,
    pub base_reserve: u64,
    pub quote_reserve: u64,
    pub target_base_reserve: u128,
    pub target_quote_reserve: u128,
    pub base_supply: u64,
    pub quote_supply: u64,
    pub total_traded_base: u128,
    pub total_traded_quote: u128,
    pub accumulated_trade_reward: u64,
    pub last_update_timestamp: u64,
    pub market_price_last_update_slot: u64,
    pub low_price: u128,
    pub high_price: u128,
    pub current_day_traded_quote: u64,
    pub last_day_traded_quote: u64,
    pub current_week_traded_quote: u64,
    pub last_week_traded_quote: u64,
    pub reserved_u64: [u64; 12],
}
impl PoolState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let market_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        let base_reserve: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_reserve: u64 = crate::borsh_de_or_default(&mut reader)?;
        let target_base_reserve: u128 = crate::borsh_de_or_default(&mut reader)?;
        let target_quote_reserve: u128 = crate::borsh_de_or_default(&mut reader)?;
        let base_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_traded_base: u128 = crate::borsh_de_or_default(&mut reader)?;
        let total_traded_quote: u128 = crate::borsh_de_or_default(&mut reader)?;
        let accumulated_trade_reward: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_update_timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        let market_price_last_update_slot: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let low_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        let high_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        let current_day_traded_quote: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_day_traded_quote: u64 = crate::borsh_de_or_default(&mut reader)?;
        let current_week_traded_quote: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_week_traded_quote: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reserved_u64: [u64; 12] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            market_price,
            base_reserve,
            quote_reserve,
            target_base_reserve,
            target_quote_reserve,
            base_supply,
            quote_supply,
            total_traded_base,
            total_traded_quote,
            accumulated_trade_reward,
            last_update_timestamp,
            market_price_last_update_slot,
            low_price,
            high_price,
            current_day_traded_quote,
            last_day_traded_quote,
            current_week_traded_quote,
            last_week_traded_quote,
            reserved_u64,
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
pub struct FarmPosition {
    pub deposited_amount: u64,
    pub rewards_owed: u64,
    pub cumulative_interest: u64,
    pub last_update_ts: i64,
    pub next_claim_ts: i64,
    pub latest_deposit_slot: u64,
}
impl FarmPosition {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let deposited_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let rewards_owed: u64 = crate::borsh_de_or_default(&mut reader)?;
        let cumulative_interest: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_update_ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let next_claim_ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let latest_deposit_slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            deposited_amount,
            rewards_owed,
            cumulative_interest,
            last_update_ts,
            next_claim_ts,
            latest_deposit_slot,
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
pub struct FarmConfig {
    pub base_apr_numerator: u64,
    pub base_apr_denominator: u64,
    pub quote_apr_numerator: u64,
    pub quote_apr_denominator: u64,
    pub min_claim_period: u32,
    pub is_paused: bool,
    pub max_staked_base_share: u64,
    pub max_staked_quote_share: u64,
    pub end_timestamp: u64,
    pub reserved_u64: [u64; 32],
}
impl FarmConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let base_apr_numerator: u64 = crate::borsh_de_or_default(&mut reader)?;
        let base_apr_denominator: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_apr_numerator: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_apr_denominator: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_claim_period: u32 = crate::borsh_de_or_default(&mut reader)?;
        let is_paused: bool = crate::borsh_de_or_default(&mut reader)?;
        let max_staked_base_share: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_staked_quote_share: u64 = crate::borsh_de_or_default(&mut reader)?;
        let end_timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reserved_u64: [u64; 32] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            base_apr_numerator,
            base_apr_denominator,
            quote_apr_numerator,
            quote_apr_denominator,
            min_claim_period,
            is_paused,
            max_staked_base_share,
            max_staked_quote_share,
            end_timestamp,
            reserved_u64,
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
pub struct SwapConfig {
    pub is_paused: bool,
    pub enable_confidence_interval: bool,
    pub max_swap_percentage: u8,
    pub min_reserve_limit_percentage: u8,
    pub serum_market_token_ratio_limit_percentage: u8,
    pub admin_trade_fee_numerator: u32,
    pub admin_trade_fee_denominator: u32,
    pub admin_withdraw_fee_numerator: u32,
    pub admin_withdraw_fee_denominator: u32,
    pub trade_fee_numerator: u32,
    pub trade_fee_denominator: u32,
    pub withdraw_fee_numerator: u32,
    pub withdraw_fee_denominator: u32,
    pub trade_reward_numerator: u32,
    pub trade_reward_denominator: u32,
    pub referral_reward_numerator: u32,
    pub referral_reward_denominator: u32,
    pub max_stable_price_diff_numerator: u32,
    pub max_stable_price_diff_denominator: u32,
    pub trade_reward_cap: u64,
    pub trade_reward_max_reserve: u64,
    pub slope: u128,
    pub disable_stable_price_diff_check: bool,
    pub disable_quote_token_price_check: bool,
    pub rebate_numerator: u32,
    pub rebate_denominator: u32,
    pub virtual_reserve_percentage: u16,
    pub reserved_u8: [u8; 4],
    pub reserved_u64: [u64; 14],
}
impl SwapConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let is_paused: bool = crate::borsh_de_or_default(&mut reader)?;
        let enable_confidence_interval: bool = crate::borsh_de_or_default(&mut reader)?;
        let max_swap_percentage: u8 = crate::borsh_de_or_default(&mut reader)?;
        let min_reserve_limit_percentage: u8 = crate::borsh_de_or_default(&mut reader)?;
        let serum_market_token_ratio_limit_percentage: u8 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let admin_trade_fee_numerator: u32 = crate::borsh_de_or_default(&mut reader)?;
        let admin_trade_fee_denominator: u32 = crate::borsh_de_or_default(&mut reader)?;
        let admin_withdraw_fee_numerator: u32 = crate::borsh_de_or_default(&mut reader)?;
        let admin_withdraw_fee_denominator: u32 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let trade_fee_numerator: u32 = crate::borsh_de_or_default(&mut reader)?;
        let trade_fee_denominator: u32 = crate::borsh_de_or_default(&mut reader)?;
        let withdraw_fee_numerator: u32 = crate::borsh_de_or_default(&mut reader)?;
        let withdraw_fee_denominator: u32 = crate::borsh_de_or_default(&mut reader)?;
        let trade_reward_numerator: u32 = crate::borsh_de_or_default(&mut reader)?;
        let trade_reward_denominator: u32 = crate::borsh_de_or_default(&mut reader)?;
        let referral_reward_numerator: u32 = crate::borsh_de_or_default(&mut reader)?;
        let referral_reward_denominator: u32 = crate::borsh_de_or_default(&mut reader)?;
        let max_stable_price_diff_numerator: u32 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let max_stable_price_diff_denominator: u32 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let trade_reward_cap: u64 = crate::borsh_de_or_default(&mut reader)?;
        let trade_reward_max_reserve: u64 = crate::borsh_de_or_default(&mut reader)?;
        let slope: u128 = crate::borsh_de_or_default(&mut reader)?;
        let disable_stable_price_diff_check: bool = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let disable_quote_token_price_check: bool = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let rebate_numerator: u32 = crate::borsh_de_or_default(&mut reader)?;
        let rebate_denominator: u32 = crate::borsh_de_or_default(&mut reader)?;
        let virtual_reserve_percentage: u16 = crate::borsh_de_or_default(&mut reader)?;
        let reserved_u8: [u8; 4] = crate::borsh_de_or_default(&mut reader)?;
        let reserved_u64: [u64; 14] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            is_paused,
            enable_confidence_interval,
            max_swap_percentage,
            min_reserve_limit_percentage,
            serum_market_token_ratio_limit_percentage,
            admin_trade_fee_numerator,
            admin_trade_fee_denominator,
            admin_withdraw_fee_numerator,
            admin_withdraw_fee_denominator,
            trade_fee_numerator,
            trade_fee_denominator,
            withdraw_fee_numerator,
            withdraw_fee_denominator,
            trade_reward_numerator,
            trade_reward_denominator,
            referral_reward_numerator,
            referral_reward_denominator,
            max_stable_price_diff_numerator,
            max_stable_price_diff_denominator,
            trade_reward_cap,
            trade_reward_max_reserve,
            slope,
            disable_stable_price_diff_check,
            disable_quote_token_price_check,
            rebate_numerator,
            rebate_denominator,
            virtual_reserve_percentage,
            reserved_u8,
            reserved_u64,
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
pub enum SwapDirection {
    #[default]
    SellBase,
    SellQuote,
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
pub enum SwapType {
    #[default]
    NormalSwap,
    StableSwap,
    SerumSwap,
}
