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
pub enum AccountsType {
    #[default]
    TransferHookX,
    TransferHookY,
    TransferHookReward,
    TransferHookMultiReward(u8),
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
pub struct PoolFeesStruct {
    pub base_fee: BaseFeeStruct,
    pub protocol_fee_percent: u8,
    pub partner_fee_percent: u8,
    pub referral_fee_percent: u8,
    pub padding_0: [u8; 5],
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
        let partner_fee_percent: u8 = crate::borsh_de_or_default(&mut reader)?;
        let referral_fee_percent: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding_0: [u8; 5] = crate::borsh_de_or_default(&mut reader)?;
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
            partner_fee_percent,
            referral_fee_percent,
            padding_0,
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
    pub total_partner_a_fee: u64,
    pub total_partner_b_fee: u64,
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
        let total_partner_a_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_partner_b_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_position: u64 = crate::borsh_de_or_default(&mut reader)?;
        let padding: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            total_lp_a_fee,
            total_lp_b_fee,
            total_protocol_a_fee,
            total_protocol_b_fee,
            total_partner_a_fee,
            total_partner_b_fee,
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
pub struct ProtocolFee {
    pub amount_x: u64,
    pub amount_y: u64,
}
impl ProtocolFee {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount_x: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_y: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { amount_x, amount_y })
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
pub struct RemainingAccountsInfo {
    pub slices: Vec<RemainingAccountsSlice>,
}
impl RemainingAccountsInfo {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let slices: Vec<RemainingAccountsSlice> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self { slices })
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
pub struct RemainingAccountsSlice {
    pub accounts_type: AccountsType,
    pub length: u8,
}
impl RemainingAccountsSlice {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let accounts_type: AccountsType = crate::borsh_de_or_default(&mut reader)?;
        let length: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { accounts_type, length })
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
pub struct StaticParameters {
    pub base_factor: u16,
    pub filter_period: u16,
    pub decay_period: u16,
    pub reduction_factor: u16,
    pub variable_fee_control: u32,
    pub max_volatility_accumulator: u32,
    pub min_bin_id: i32,
    pub max_bin_id: i32,
    pub protocol_share: u16,
    pub base_fee_power_factor: u8,
    pub _padding: [u8; 5],
}
impl StaticParameters {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let base_factor: u16 = crate::borsh_de_or_default(&mut reader)?;
        let filter_period: u16 = crate::borsh_de_or_default(&mut reader)?;
        let decay_period: u16 = crate::borsh_de_or_default(&mut reader)?;
        let reduction_factor: u16 = crate::borsh_de_or_default(&mut reader)?;
        let variable_fee_control: u32 = crate::borsh_de_or_default(&mut reader)?;
        let max_volatility_accumulator: u32 = crate::borsh_de_or_default(&mut reader)?;
        let min_bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let max_bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_share: u16 = crate::borsh_de_or_default(&mut reader)?;
        let base_fee_power_factor: u8 = crate::borsh_de_or_default(&mut reader)?;
        let _padding: [u8; 5] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            base_factor,
            filter_period,
            decay_period,
            reduction_factor,
            variable_fee_control,
            max_volatility_accumulator,
            min_bin_id,
            max_bin_id,
            protocol_share,
            base_fee_power_factor,
            _padding,
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
pub enum StrategyType {
    #[default]
    Spot,
    Curve,
    BidAsk,
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
pub struct VariableParameters {
    pub volatility_accumulator: u32,
    pub volatility_reference: u32,
    pub index_reference: i32,
    pub _padding: [u8; 4],
    pub last_update_timestamp: i64,
    pub _padding_1: [u8; 8],
}
impl VariableParameters {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let volatility_accumulator: u32 = crate::borsh_de_or_default(&mut reader)?;
        let volatility_reference: u32 = crate::borsh_de_or_default(&mut reader)?;
        let index_reference: i32 = crate::borsh_de_or_default(&mut reader)?;
        let _padding: [u8; 4] = crate::borsh_de_or_default(&mut reader)?;
        let last_update_timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        let _padding_1: [u8; 8] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            volatility_accumulator,
            volatility_reference,
            index_reference,
            _padding,
            last_update_timestamp,
            _padding_1,
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
pub struct ZapOutParameters {
    pub percentage: u8,
    pub offset_amount_in: u16,
    pub pre_user_token_balance: u64,
    pub max_swap_amount: u64,
    pub payload_data: Vec<u8>,
}
impl ZapOutParameters {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let percentage: u8 = crate::borsh_de_or_default(&mut reader)?;
        let offset_amount_in: u16 = crate::borsh_de_or_default(&mut reader)?;
        let pre_user_token_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_swap_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let payload_data: Vec<u8> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            percentage,
            offset_amount_in,
            pre_user_token_balance,
            max_swap_amount,
            payload_data,
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
pub struct CpAmmStatePoolRewardInfo {
    pub initialized: u8,
    pub reward_token_flag: u8,
    pub _padding_0: [u8; 6],
    pub _padding_1: [u8; 8],
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
impl CpAmmStatePoolRewardInfo {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let initialized: u8 = crate::borsh_de_or_default(&mut reader)?;
        let reward_token_flag: u8 = crate::borsh_de_or_default(&mut reader)?;
        let _padding_0: [u8; 6] = crate::borsh_de_or_default(&mut reader)?;
        let _padding_1: [u8; 8] = crate::borsh_de_or_default(&mut reader)?;
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
            _padding_0,
            _padding_1,
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
pub struct DlmmDlmmTypesRewardInfo {
    pub mint: Pubkey,
    pub vault: Pubkey,
    pub funder: Pubkey,
    pub reward_duration: u64,
    pub reward_duration_end: u64,
    pub reward_rate: u128,
    pub last_update_time: u64,
    pub cumulative_seconds_with_empty_liquidity_reward: u64,
}
impl DlmmDlmmTypesRewardInfo {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let funder: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let reward_duration: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reward_duration_end: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reward_rate: u128 = crate::borsh_de_or_default(&mut reader)?;
        let last_update_time: u64 = crate::borsh_de_or_default(&mut reader)?;
        let cumulative_seconds_with_empty_liquidity_reward: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            mint,
            vault,
            funder,
            reward_duration,
            reward_duration_end,
            reward_rate,
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
pub struct DlmmRewardInfo {
    pub mint: Pubkey,
    pub vault: Pubkey,
    pub funder: Pubkey,
    pub reward_duration: u64,
    pub reward_duration_end: u64,
    pub reward_rate: u128,
    pub last_update_time: u64,
    pub cumulative_seconds_with_empty_liquidity_reward: u64,
}
impl DlmmRewardInfo {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let funder: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let reward_duration: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reward_duration_end: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reward_rate: u128 = crate::borsh_de_or_default(&mut reader)?;
        let last_update_time: u64 = crate::borsh_de_or_default(&mut reader)?;
        let cumulative_seconds_with_empty_liquidity_reward: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            mint,
            vault,
            funder,
            reward_duration,
            reward_duration_end,
            reward_rate,
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
pub struct CpAmmRewardInfo {
    pub initialized: u8,
    pub reward_token_flag: u8,
    pub _padding_0: [u8; 6],
    pub _padding_1: [u8; 8],
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
impl CpAmmRewardInfo {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let initialized: u8 = crate::borsh_de_or_default(&mut reader)?;
        let reward_token_flag: u8 = crate::borsh_de_or_default(&mut reader)?;
        let _padding_0: [u8; 6] = crate::borsh_de_or_default(&mut reader)?;
        let _padding_1: [u8; 8] = crate::borsh_de_or_default(&mut reader)?;
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
            _padding_0,
            _padding_1,
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
