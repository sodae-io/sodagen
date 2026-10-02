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
pub enum CollectFeeOn {
    #[default]
    FromInput,
    Token0Only,
    Token1Only,
}
impl TryFrom<u8> for CollectFeeOn {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::FromInput),
            1u8 => Ok(Self::Token0Only),
            2u8 => Ok(Self::Token1Only),
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
pub struct CreateCustomizableParams {
    pub sqrt_price_x64: u128,
    pub collect_fee_on: CollectFeeOn,
    pub enable_dynamic_fee: bool,
}
impl CreateCustomizableParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let sqrt_price_x64: u128 = crate::borsh_de_or_default(&mut reader)?;
        let collect_fee_on: CollectFeeOn = crate::borsh_de_or_default(&mut reader)?;
        let enable_dynamic_fee: bool = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            sqrt_price_x64,
            collect_fee_on,
            enable_dynamic_fee,
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
pub struct DynamicFeeInfo {
    pub filter_period: u16,
    pub decay_period: u16,
    pub reduction_factor: u16,
    pub dynamic_fee_control: u32,
    pub max_volatility_accumulator: u32,
    pub tick_spacing_index_reference: i32,
    pub volatility_reference: u32,
    pub volatility_accumulator: u32,
    pub last_update_timestamp: u64,
    #[serde(with = "crate::big_array_serde")]
    pub padding: [u8; 46],
}
impl DynamicFeeInfo {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let filter_period: u16 = crate::borsh_de_or_default(&mut reader)?;
        let decay_period: u16 = crate::borsh_de_or_default(&mut reader)?;
        let reduction_factor: u16 = crate::borsh_de_or_default(&mut reader)?;
        let dynamic_fee_control: u32 = crate::borsh_de_or_default(&mut reader)?;
        let max_volatility_accumulator: u32 = crate::borsh_de_or_default(&mut reader)?;
        let tick_spacing_index_reference: i32 = crate::borsh_de_or_default(&mut reader)?;
        let volatility_reference: u32 = crate::borsh_de_or_default(&mut reader)?;
        let volatility_accumulator: u32 = crate::borsh_de_or_default(&mut reader)?;
        let last_update_timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        let padding = <[u8; 46] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            filter_period,
            decay_period,
            reduction_factor,
            dynamic_fee_control,
            max_volatility_accumulator,
            tick_spacing_index_reference,
            volatility_reference,
            volatility_accumulator,
            last_update_timestamp,
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
pub struct InitializeRewardParam {
    pub open_time: u64,
    pub end_time: u64,
    pub emissions_per_second_x64: u128,
}
impl InitializeRewardParam {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let open_time: u64 = crate::borsh_de_or_default(&mut reader)?;
        let end_time: u64 = crate::borsh_de_or_default(&mut reader)?;
        let emissions_per_second_x64: u128 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            open_time,
            end_time,
            emissions_per_second_x64,
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
pub struct Observation {
    pub block_timestamp: u32,
    pub tick_cumulative: i64,
    pub padding: [u64; 4],
}
impl Observation {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let block_timestamp: u32 = crate::borsh_de_or_default(&mut reader)?;
        let tick_cumulative: i64 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u64; 4] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            block_timestamp,
            tick_cumulative,
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
pub struct PositionRewardInfo {
    pub growth_inside_last_x64: u128,
    pub reward_amount_owed: u64,
}
impl PositionRewardInfo {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let growth_inside_last_x64: u128 = crate::borsh_de_or_default(&mut reader)?;
        let reward_amount_owed: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            growth_inside_last_x64,
            reward_amount_owed,
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
    pub reward_total_emitted: u64,
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
        let reward_total_emitted: u64 = crate::borsh_de_or_default(&mut reader)?;
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
            reward_total_emitted,
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
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct TickState {
    pub tick: i32,
    pub liquidity_net: i128,
    pub liquidity_gross: u128,
    pub fee_growth_outside_0_x64: u128,
    pub fee_growth_outside_1_x64: u128,
    pub reward_growths_outside_x64: [u128; 3],
    pub order_phase: u64,
    pub orders_amount: u64,
    pub part_filled_orders_remaining: u64,
    pub unfilled_ratio_x64: u128,
    pub padding: [u32; 3],
}
impl TickState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let tick: i32 = crate::borsh_de_or_default(&mut reader)?;
        let liquidity_net: i128 = crate::borsh_de_or_default(&mut reader)?;
        let liquidity_gross: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fee_growth_outside_0_x64: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fee_growth_outside_1_x64: u128 = crate::borsh_de_or_default(&mut reader)?;
        let reward_growths_outside_x64: [u128; 3] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let order_phase: u64 = crate::borsh_de_or_default(&mut reader)?;
        let orders_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let part_filled_orders_remaining: u64 = crate::borsh_de_or_default(&mut reader)?;
        let unfilled_ratio_x64: u128 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u32; 3] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            tick,
            liquidity_net,
            liquidity_gross,
            fee_growth_outside_0_x64,
            fee_growth_outside_1_x64,
            reward_growths_outside_x64,
            order_phase,
            orders_amount,
            part_filled_orders_remaining,
            unfilled_ratio_x64,
            padding,
        })
    }
}
