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
pub struct AddLiquidityParameters {
    pub liquidity_delta: u128,
    pub token_a_amount_threshold: u64,
    pub token_b_amount_threshold: u64,
}
impl AddLiquidityParameters {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let liquidity_delta: u128 = crate::borsh_de_or_default(&mut reader)?;
        let token_a_amount_threshold: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_b_amount_threshold: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            liquidity_delta,
            token_a_amount_threshold,
            token_b_amount_threshold,
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
pub struct BaseFeeInfo {
    pub data: [u8; 32],
}
impl BaseFeeInfo {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let data: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { data })
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
pub struct BaseFeeParameters {
    pub data: [u8; 27],
}
impl BaseFeeParameters {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let data: [u8; 27] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { data })
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
pub struct BaseFeeStruct {
    pub base_fee_info: BaseFeeInfo,
    pub padding_1: u64,
}
impl BaseFeeStruct {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let base_fee_info = if reader.is_empty() {
            Default::default()
        } else {
            <BaseFeeInfo>::deserialize(&mut reader)?
        };
        let padding_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { base_fee_info, padding_1 })
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
pub struct BorshFeeMarketCapScheduler {
    pub cliff_fee_numerator: u64,
    pub number_of_period: u16,
    pub sqrt_price_step_bps: u32,
    pub scheduler_expiration_duration: u32,
    pub reduction_factor: u64,
    pub base_fee_mode: u8,
}
impl BorshFeeMarketCapScheduler {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let cliff_fee_numerator: u64 = crate::borsh_de_or_default(&mut reader)?;
        let number_of_period: u16 = crate::borsh_de_or_default(&mut reader)?;
        let sqrt_price_step_bps: u32 = crate::borsh_de_or_default(&mut reader)?;
        let scheduler_expiration_duration: u32 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let reduction_factor: u64 = crate::borsh_de_or_default(&mut reader)?;
        let base_fee_mode: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            cliff_fee_numerator,
            number_of_period,
            sqrt_price_step_bps,
            scheduler_expiration_duration,
            reduction_factor,
            base_fee_mode,
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
pub struct BorshFeeRateLimiter {
    pub cliff_fee_numerator: u64,
    pub fee_increment_bps: u16,
    pub max_limiter_duration: u32,
    pub max_fee_bps: u32,
    pub reference_amount: u64,
    pub base_fee_mode: u8,
}
impl BorshFeeRateLimiter {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let cliff_fee_numerator: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_increment_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let max_limiter_duration: u32 = crate::borsh_de_or_default(&mut reader)?;
        let max_fee_bps: u32 = crate::borsh_de_or_default(&mut reader)?;
        let reference_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let base_fee_mode: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            cliff_fee_numerator,
            fee_increment_bps,
            max_limiter_duration,
            max_fee_bps,
            reference_amount,
            base_fee_mode,
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
pub struct BorshFeeTimeScheduler {
    pub cliff_fee_numerator: u64,
    pub number_of_period: u16,
    pub period_frequency: u64,
    pub reduction_factor: u64,
    pub base_fee_mode: u8,
}
impl BorshFeeTimeScheduler {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let cliff_fee_numerator: u64 = crate::borsh_de_or_default(&mut reader)?;
        let number_of_period: u16 = crate::borsh_de_or_default(&mut reader)?;
        let period_frequency: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reduction_factor: u64 = crate::borsh_de_or_default(&mut reader)?;
        let base_fee_mode: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            cliff_fee_numerator,
            number_of_period,
            period_frequency,
            reduction_factor,
            base_fee_mode,
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
pub struct DummyParams {
    pub borsh_fee_time_scheduler_params: BorshFeeTimeScheduler,
    pub borsh_fee_rate_limiter_params: BorshFeeRateLimiter,
    pub borsh_fee_market_cap_scheduler_params: BorshFeeMarketCapScheduler,
}
impl DummyParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let borsh_fee_time_scheduler_params = if reader.is_empty() {
            Default::default()
        } else {
            <BorshFeeTimeScheduler>::deserialize(&mut reader)?
        };
        let borsh_fee_rate_limiter_params = if reader.is_empty() {
            Default::default()
        } else {
            <BorshFeeRateLimiter>::deserialize(&mut reader)?
        };
        let borsh_fee_market_cap_scheduler_params = if reader.is_empty() {
            Default::default()
        } else {
            <BorshFeeMarketCapScheduler>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            borsh_fee_time_scheduler_params,
            borsh_fee_rate_limiter_params,
            borsh_fee_market_cap_scheduler_params,
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
pub struct DynamicConfigParameters {
    pub pool_creator_authority: Pubkey,
    pub permission: u128,
}
impl DynamicConfigParameters {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_creator_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let permission: u128 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool_creator_authority,
            permission,
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
pub struct DynamicFeeConfig {
    pub initialized: u8,
    pub padding: [u8; 7],
    pub max_volatility_accumulator: u32,
    pub variable_fee_control: u32,
    pub bin_step: u16,
    pub filter_period: u16,
    pub decay_period: u16,
    pub reduction_factor: u16,
    pub padding_1: [u8; 8],
    pub bin_step_u128: u128,
}
impl DynamicFeeConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let initialized: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 7] = crate::borsh_de_or_default(&mut reader)?;
        let max_volatility_accumulator: u32 = crate::borsh_de_or_default(&mut reader)?;
        let variable_fee_control: u32 = crate::borsh_de_or_default(&mut reader)?;
        let bin_step: u16 = crate::borsh_de_or_default(&mut reader)?;
        let filter_period: u16 = crate::borsh_de_or_default(&mut reader)?;
        let decay_period: u16 = crate::borsh_de_or_default(&mut reader)?;
        let reduction_factor: u16 = crate::borsh_de_or_default(&mut reader)?;
        let padding_1: [u8; 8] = crate::borsh_de_or_default(&mut reader)?;
        let bin_step_u128: u128 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            initialized,
            padding,
            max_volatility_accumulator,
            variable_fee_control,
            bin_step,
            filter_period,
            decay_period,
            reduction_factor,
            padding_1,
            bin_step_u128,
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
pub struct DynamicFeeParameters {
    pub bin_step: u16,
    pub bin_step_u128: u128,
    pub filter_period: u16,
    pub decay_period: u16,
    pub reduction_factor: u16,
    pub max_volatility_accumulator: u32,
    pub variable_fee_control: u32,
}
impl DynamicFeeParameters {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bin_step: u16 = crate::borsh_de_or_default(&mut reader)?;
        let bin_step_u128: u128 = crate::borsh_de_or_default(&mut reader)?;
        let filter_period: u16 = crate::borsh_de_or_default(&mut reader)?;
        let decay_period: u16 = crate::borsh_de_or_default(&mut reader)?;
        let reduction_factor: u16 = crate::borsh_de_or_default(&mut reader)?;
        let max_volatility_accumulator: u32 = crate::borsh_de_or_default(&mut reader)?;
        let variable_fee_control: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            bin_step,
            bin_step_u128,
            filter_period,
            decay_period,
            reduction_factor,
            max_volatility_accumulator,
            variable_fee_control,
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
pub struct DynamicFeeStruct {
    pub initialized: u8,
    pub padding: [u8; 7],
    pub max_volatility_accumulator: u32,
    pub variable_fee_control: u32,
    pub bin_step: u16,
    pub filter_period: u16,
    pub decay_period: u16,
    pub reduction_factor: u16,
    pub last_update_timestamp: u64,
    pub bin_step_u128: u128,
    pub sqrt_price_reference: u128,
    pub volatility_accumulator: u128,
    pub volatility_reference: u128,
}
impl DynamicFeeStruct {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let initialized: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 7] = crate::borsh_de_or_default(&mut reader)?;
        let max_volatility_accumulator: u32 = crate::borsh_de_or_default(&mut reader)?;
        let variable_fee_control: u32 = crate::borsh_de_or_default(&mut reader)?;
        let bin_step: u16 = crate::borsh_de_or_default(&mut reader)?;
        let filter_period: u16 = crate::borsh_de_or_default(&mut reader)?;
        let decay_period: u16 = crate::borsh_de_or_default(&mut reader)?;
        let reduction_factor: u16 = crate::borsh_de_or_default(&mut reader)?;
        let last_update_timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        let bin_step_u128: u128 = crate::borsh_de_or_default(&mut reader)?;
        let sqrt_price_reference: u128 = crate::borsh_de_or_default(&mut reader)?;
        let volatility_accumulator: u128 = crate::borsh_de_or_default(&mut reader)?;
        let volatility_reference: u128 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            initialized,
            padding,
            max_volatility_accumulator,
            variable_fee_control,
            bin_step,
            filter_period,
            decay_period,
            reduction_factor,
            last_update_timestamp,
            bin_step_u128,
            sqrt_price_reference,
            volatility_accumulator,
            volatility_reference,
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
pub struct InitializeCustomizablePoolParameters {
    pub pool_fees: PoolFeeParameters,
    pub sqrt_min_price: u128,
    pub sqrt_max_price: u128,
    pub has_alpha_vault: bool,
    pub liquidity: u128,
    pub sqrt_price: u128,
    pub activation_type: u8,
    pub collect_fee_mode: u8,
    pub activation_point: Option<u64>,
}
impl InitializeCustomizablePoolParameters {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_fees = if reader.is_empty() {
            Default::default()
        } else {
            <PoolFeeParameters>::deserialize(&mut reader)?
        };
        let sqrt_min_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        let sqrt_max_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        let has_alpha_vault: bool = crate::borsh_de_or_default(&mut reader)?;
        let liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let sqrt_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        let activation_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let collect_fee_mode: u8 = crate::borsh_de_or_default(&mut reader)?;
        let activation_point: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool_fees,
            sqrt_min_price,
            sqrt_max_price,
            has_alpha_vault,
            liquidity,
            sqrt_price,
            activation_type,
            collect_fee_mode,
            activation_point,
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
pub struct InitializePoolParameters {
    pub liquidity: u128,
    pub sqrt_price: u128,
    pub activation_point: Option<u64>,
}
impl InitializePoolParameters {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let sqrt_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        let activation_point: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            liquidity,
            sqrt_price,
            activation_point,
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
pub struct InnerVesting {
    pub cliff_point: u64,
    pub period_frequency: u64,
    pub cliff_unlock_liquidity: u128,
    pub liquidity_per_period: u128,
    pub total_released_liquidity: u128,
    pub number_of_period: u16,
    pub padding: [u8; 14],
}
impl InnerVesting {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let cliff_point: u64 = crate::borsh_de_or_default(&mut reader)?;
        let period_frequency: u64 = crate::borsh_de_or_default(&mut reader)?;
        let cliff_unlock_liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let liquidity_per_period: u128 = crate::borsh_de_or_default(&mut reader)?;
        let total_released_liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let number_of_period: u16 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 14] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            cliff_point,
            period_frequency,
            cliff_unlock_liquidity,
            liquidity_per_period,
            total_released_liquidity,
            number_of_period,
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
pub struct PoolFeeParameters {
    pub base_fee: BaseFeeParameters,
    pub compounding_fee_bps: u16,
    pub padding: u8,
    pub dynamic_fee: Option<DynamicFeeParameters>,
}
impl PoolFeeParameters {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let base_fee = if reader.is_empty() {
            Default::default()
        } else {
            <BaseFeeParameters>::deserialize(&mut reader)?
        };
        let compounding_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let padding: u8 = crate::borsh_de_or_default(&mut reader)?;
        let dynamic_fee: Option<DynamicFeeParameters> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            base_fee,
            compounding_fee_bps,
            padding,
            dynamic_fee,
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
pub struct PoolFeesConfig {
    pub base_fee: BaseFeeInfo,
    pub dynamic_fee: DynamicFeeConfig,
    pub protocol_fee_percent: u8,
    pub padding_0: u8,
    pub referral_fee_percent: u8,
    pub padding_1: [u8; 3],
    pub compounding_fee_bps: u16,
    pub padding_2: [u64; 5],
}
impl PoolFeesConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let base_fee = if reader.is_empty() {
            Default::default()
        } else {
            <BaseFeeInfo>::deserialize(&mut reader)?
        };
        let dynamic_fee = if reader.is_empty() {
            Default::default()
        } else {
            <DynamicFeeConfig>::deserialize(&mut reader)?
        };
        let protocol_fee_percent: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding_0: u8 = crate::borsh_de_or_default(&mut reader)?;
        let referral_fee_percent: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding_1: [u8; 3] = crate::borsh_de_or_default(&mut reader)?;
        let compounding_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let padding_2: [u64; 5] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            base_fee,
            dynamic_fee,
            protocol_fee_percent,
            padding_0,
            referral_fee_percent,
            padding_1,
            compounding_fee_bps,
            padding_2,
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
pub struct PoolFeesStruct {
    pub base_fee: BaseFeeStruct,
    pub protocol_fee_percent: u8,
    pub padding_0: u8,
    pub referral_fee_percent: u8,
    pub padding_1: [u8; 3],
    pub compounding_fee_bps: u16,
    pub dynamic_fee: DynamicFeeStruct,
    pub init_sqrt_price: u128,
}
impl PoolFeesStruct {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let base_fee = if reader.is_empty() {
            Default::default()
        } else {
            <BaseFeeStruct>::deserialize(&mut reader)?
        };
        let protocol_fee_percent: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding_0: u8 = crate::borsh_de_or_default(&mut reader)?;
        let referral_fee_percent: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding_1: [u8; 3] = crate::borsh_de_or_default(&mut reader)?;
        let compounding_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let dynamic_fee = if reader.is_empty() {
            Default::default()
        } else {
            <DynamicFeeStruct>::deserialize(&mut reader)?
        };
        let init_sqrt_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            base_fee,
            protocol_fee_percent,
            padding_0,
            referral_fee_percent,
            padding_1,
            compounding_fee_bps,
            dynamic_fee,
            init_sqrt_price,
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
pub struct PoolMetrics {
    pub total_lp_a_fee: u128,
    pub total_lp_b_fee: u128,
    pub total_protocol_a_fee: u64,
    pub total_protocol_b_fee: u64,
    pub padding_0: [u64; 2],
    pub total_position: u64,
    pub padding: u64,
}
impl PoolMetrics {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let total_lp_a_fee: u128 = crate::borsh_de_or_default(&mut reader)?;
        let total_lp_b_fee: u128 = crate::borsh_de_or_default(&mut reader)?;
        let total_protocol_a_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_protocol_b_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let padding_0: [u64; 2] = crate::borsh_de_or_default(&mut reader)?;
        let total_position: u64 = crate::borsh_de_or_default(&mut reader)?;
        let padding: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            total_lp_a_fee,
            total_lp_b_fee,
            total_protocol_a_fee,
            total_protocol_b_fee,
            padding_0,
            total_position,
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
pub struct PositionMetrics {
    pub total_claimed_a_fee: u64,
    pub total_claimed_b_fee: u64,
}
impl PositionMetrics {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let total_claimed_a_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_claimed_b_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            total_claimed_a_fee,
            total_claimed_b_fee,
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
pub struct RemoveLiquidityParameters {
    pub liquidity_delta: u128,
    pub token_a_amount_threshold: u64,
    pub token_b_amount_threshold: u64,
}
impl RemoveLiquidityParameters {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let liquidity_delta: u128 = crate::borsh_de_or_default(&mut reader)?;
        let token_a_amount_threshold: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_b_amount_threshold: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            liquidity_delta,
            token_a_amount_threshold,
            token_b_amount_threshold,
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
    pub initialized: u8,
    pub reward_token_flag: u8,
    pub padding_0: [u8; 6],
    pub dead_liquidity_reward_checkpoint: u64,
    pub mint: Pubkey,
    pub vault: Pubkey,
    pub funder: Pubkey,
    pub reward_duration: u64,
    pub reward_duration_end: u64,
    pub reward_rate: u128,
    pub reward_per_token_stored: [u8; 32],
    pub last_update_time: u64,
    pub cumulative_seconds_with_empty_liquidity_reward: u64,
}
impl RewardInfo {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let initialized: u8 = crate::borsh_de_or_default(&mut reader)?;
        let reward_token_flag: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding_0: [u8; 6] = crate::borsh_de_or_default(&mut reader)?;
        let dead_liquidity_reward_checkpoint: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let funder: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let reward_duration: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reward_duration_end: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reward_rate: u128 = crate::borsh_de_or_default(&mut reader)?;
        let reward_per_token_stored: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        let last_update_time: u64 = crate::borsh_de_or_default(&mut reader)?;
        let cumulative_seconds_with_empty_liquidity_reward: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            initialized,
            reward_token_flag,
            padding_0,
            dead_liquidity_reward_checkpoint,
            mint,
            vault,
            funder,
            reward_duration,
            reward_duration_end,
            reward_rate,
            reward_per_token_stored,
            last_update_time,
            cumulative_seconds_with_empty_liquidity_reward,
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
pub struct SplitAmountInfo {
    pub permanent_locked_liquidity: u128,
    pub unlocked_liquidity: u128,
    pub fee_a: u64,
    pub fee_b: u64,
    pub reward_0: u64,
    pub reward_1: u64,
}
impl SplitAmountInfo {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let permanent_locked_liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let unlocked_liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fee_a: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_b: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reward_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reward_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            permanent_locked_liquidity,
            unlocked_liquidity,
            fee_a,
            fee_b,
            reward_0,
            reward_1,
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
pub struct SplitAmountInfo2 {
    pub permanent_locked_liquidity: u128,
    pub unlocked_liquidity: u128,
    pub vested_liquidity: u128,
    pub fee_a: u64,
    pub fee_b: u64,
    pub reward_0: u64,
    pub reward_1: u64,
}
impl SplitAmountInfo2 {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let permanent_locked_liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let unlocked_liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let vested_liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fee_a: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_b: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reward_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reward_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            permanent_locked_liquidity,
            unlocked_liquidity,
            vested_liquidity,
            fee_a,
            fee_b,
            reward_0,
            reward_1,
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
pub struct SplitPositionInfo {
    pub liquidity: u128,
    pub fee_a: u64,
    pub fee_b: u64,
    pub reward_0: u64,
    pub reward_1: u64,
}
impl SplitPositionInfo {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fee_a: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_b: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reward_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reward_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            liquidity,
            fee_a,
            fee_b,
            reward_0,
            reward_1,
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
pub struct SplitPositionInfo2 {
    pub unlocked_liquidity: u128,
    pub permanent_locked_liquidity: u128,
    pub vested_liquidity: u128,
    pub fee_a: u64,
    pub fee_b: u64,
    pub reward_0: u64,
    pub reward_1: u64,
}
impl SplitPositionInfo2 {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let unlocked_liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let permanent_locked_liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let vested_liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fee_a: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_b: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reward_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reward_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            unlocked_liquidity,
            permanent_locked_liquidity,
            vested_liquidity,
            fee_a,
            fee_b,
            reward_0,
            reward_1,
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
pub struct SplitPositionParameters {
    pub unlocked_liquidity_percentage: u8,
    pub permanent_locked_liquidity_percentage: u8,
    pub fee_a_percentage: u8,
    pub fee_b_percentage: u8,
    pub reward_0_percentage: u8,
    pub reward_1_percentage: u8,
    pub inner_vesting_liquidity_percentage: u8,
    pub padding: [u8; 15],
}
impl SplitPositionParameters {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let unlocked_liquidity_percentage: u8 = crate::borsh_de_or_default(&mut reader)?;
        let permanent_locked_liquidity_percentage: u8 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let fee_a_percentage: u8 = crate::borsh_de_or_default(&mut reader)?;
        let fee_b_percentage: u8 = crate::borsh_de_or_default(&mut reader)?;
        let reward_0_percentage: u8 = crate::borsh_de_or_default(&mut reader)?;
        let reward_1_percentage: u8 = crate::borsh_de_or_default(&mut reader)?;
        let inner_vesting_liquidity_percentage: u8 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let padding: [u8; 15] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            unlocked_liquidity_percentage,
            permanent_locked_liquidity_percentage,
            fee_a_percentage,
            fee_b_percentage,
            reward_0_percentage,
            reward_1_percentage,
            inner_vesting_liquidity_percentage,
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
pub struct SplitPositionParameters2 {
    pub unlocked_liquidity_numerator: u32,
    pub permanent_locked_liquidity_numerator: u32,
    pub fee_a_numerator: u32,
    pub fee_b_numerator: u32,
    pub reward_0_numerator: u32,
    pub reward_1_numerator: u32,
}
impl SplitPositionParameters2 {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let unlocked_liquidity_numerator: u32 = crate::borsh_de_or_default(&mut reader)?;
        let permanent_locked_liquidity_numerator: u32 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let fee_a_numerator: u32 = crate::borsh_de_or_default(&mut reader)?;
        let fee_b_numerator: u32 = crate::borsh_de_or_default(&mut reader)?;
        let reward_0_numerator: u32 = crate::borsh_de_or_default(&mut reader)?;
        let reward_1_numerator: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            unlocked_liquidity_numerator,
            permanent_locked_liquidity_numerator,
            fee_a_numerator,
            fee_b_numerator,
            reward_0_numerator,
            reward_1_numerator,
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
pub struct SplitPositionParameters3 {
    pub unlocked_liquidity_numerator: u32,
    pub permanent_locked_liquidity_numerator: u32,
    pub fee_a_numerator: u32,
    pub fee_b_numerator: u32,
    pub reward_0_numerator: u32,
    pub reward_1_numerator: u32,
    pub inner_vesting_liquidity_numerator: u32,
}
impl SplitPositionParameters3 {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let unlocked_liquidity_numerator: u32 = crate::borsh_de_or_default(&mut reader)?;
        let permanent_locked_liquidity_numerator: u32 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let fee_a_numerator: u32 = crate::borsh_de_or_default(&mut reader)?;
        let fee_b_numerator: u32 = crate::borsh_de_or_default(&mut reader)?;
        let reward_0_numerator: u32 = crate::borsh_de_or_default(&mut reader)?;
        let reward_1_numerator: u32 = crate::borsh_de_or_default(&mut reader)?;
        let inner_vesting_liquidity_numerator: u32 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            unlocked_liquidity_numerator,
            permanent_locked_liquidity_numerator,
            fee_a_numerator,
            fee_b_numerator,
            reward_0_numerator,
            reward_1_numerator,
            inner_vesting_liquidity_numerator,
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
pub struct StaticConfigParameters {
    pub pool_fees: PoolFeeParameters,
    pub sqrt_min_price: u128,
    pub sqrt_max_price: u128,
    pub vault_config_key: Pubkey,
    pub pool_creator_authority: Pubkey,
    pub activation_type: u8,
    pub collect_fee_mode: u8,
    pub permission: u128,
}
impl StaticConfigParameters {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_fees = if reader.is_empty() {
            Default::default()
        } else {
            <PoolFeeParameters>::deserialize(&mut reader)?
        };
        let sqrt_min_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        let sqrt_max_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        let vault_config_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_creator_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let activation_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let collect_fee_mode: u8 = crate::borsh_de_or_default(&mut reader)?;
        let permission: u128 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool_fees,
            sqrt_min_price,
            sqrt_max_price,
            vault_config_key,
            pool_creator_authority,
            activation_type,
            collect_fee_mode,
            permission,
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
pub struct SwapParameters {
    pub amount_in: u64,
    pub minimum_amount_out: u64,
}
impl SwapParameters {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let minimum_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            amount_in,
            minimum_amount_out,
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
pub struct SwapParameters2 {
    pub amount_0: u64,
    pub amount_1: u64,
    pub swap_mode: u8,
}
impl SwapParameters2 {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let swap_mode: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            amount_0,
            amount_1,
            swap_mode,
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
pub struct SwapResult2 {
    pub included_fee_input_amount: u64,
    pub excluded_fee_input_amount: u64,
    pub amount_left: u64,
    pub output_amount: u64,
    pub next_sqrt_price: u128,
    pub claiming_fee: u64,
    pub protocol_fee: u64,
    pub compounding_fee: u64,
    pub referral_fee: u64,
}
impl SwapResult2 {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let included_fee_input_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let excluded_fee_input_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_left: u64 = crate::borsh_de_or_default(&mut reader)?;
        let output_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let next_sqrt_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        let claiming_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let compounding_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let referral_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            included_fee_input_amount,
            excluded_fee_input_amount,
            amount_left,
            output_amount,
            next_sqrt_price,
            claiming_fee,
            protocol_fee,
            compounding_fee,
            referral_fee,
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
pub struct UpdatePoolFeesParameters {
    pub cliff_fee_numerator: Option<u64>,
    pub dynamic_fee: Option<DynamicFeeParameters>,
    pub compounding_fee_bps: Option<u16>,
}
impl UpdatePoolFeesParameters {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let cliff_fee_numerator: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let dynamic_fee: Option<DynamicFeeParameters> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let compounding_fee_bps: Option<u16> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            cliff_fee_numerator,
            dynamic_fee,
            compounding_fee_bps,
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
pub struct UserRewardInfo {
    pub reward_per_token_checkpoint: [u8; 32],
    pub reward_pendings: u64,
    pub total_claimed_rewards: u64,
}
impl UserRewardInfo {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let reward_per_token_checkpoint: [u8; 32] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let reward_pendings: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_claimed_rewards: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            reward_per_token_checkpoint,
            reward_pendings,
            total_claimed_rewards,
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
pub struct VestingParameters {
    pub cliff_point: Option<u64>,
    pub period_frequency: u64,
    pub cliff_unlock_liquidity: u128,
    pub liquidity_per_period: u128,
    pub number_of_period: u16,
}
impl VestingParameters {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let cliff_point: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let period_frequency: u64 = crate::borsh_de_or_default(&mut reader)?;
        let cliff_unlock_liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let liquidity_per_period: u128 = crate::borsh_de_or_default(&mut reader)?;
        let number_of_period: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            cliff_point,
            period_frequency,
            cliff_unlock_liquidity,
            liquidity_per_period,
            number_of_period,
        })
    }
}
