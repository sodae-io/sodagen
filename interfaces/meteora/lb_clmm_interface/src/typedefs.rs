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
    TransferHookReferral,
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
pub enum ActivationType {
    #[default]
    Slot,
    Timestamp,
}
impl TryFrom<u8> for ActivationType {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Slot),
            1u8 => Ok(Self::Timestamp),
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
pub struct AddLiquidityParams {
    pub min_delta_id: i32,
    pub max_delta_id: i32,
    pub x0: u64,
    pub y0: u64,
    pub delta_x: u64,
    pub delta_y: u64,
    pub bit_flag: u8,
    pub favor_x_in_active_id: bool,
    pub padding: [u8; 16],
}
impl AddLiquidityParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let min_delta_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let max_delta_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let x0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let y0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let delta_x: u64 = crate::borsh_de_or_default(&mut reader)?;
        let delta_y: u64 = crate::borsh_de_or_default(&mut reader)?;
        let bit_flag: u8 = crate::borsh_de_or_default(&mut reader)?;
        let favor_x_in_active_id: bool = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 16] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            min_delta_id,
            max_delta_id,
            x0,
            y0,
            delta_x,
            delta_y,
            bit_flag,
            favor_x_in_active_id,
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
pub struct AddLiquiditySingleSidePreciseParameter {
    pub bins: Vec<CompressedBinDepositAmount>,
    pub decompress_multiplier: u64,
}
impl AddLiquiditySingleSidePreciseParameter {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bins: Vec<CompressedBinDepositAmount> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let decompress_multiplier: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            bins,
            decompress_multiplier,
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
pub struct AddLiquiditySingleSidePreciseParameter2 {
    pub bins: Vec<CompressedBinDepositAmount>,
    pub decompress_multiplier: u64,
    pub max_amount: u64,
}
impl AddLiquiditySingleSidePreciseParameter2 {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bins: Vec<CompressedBinDepositAmount> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let decompress_multiplier: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            bins,
            decompress_multiplier,
            max_amount,
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
pub struct BaseFeeParameter {
    pub protocol_share: u16,
    pub base_factor: u16,
    pub base_fee_power_factor: u8,
}
impl BaseFeeParameter {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let protocol_share: u16 = crate::borsh_de_or_default(&mut reader)?;
        let base_factor: u16 = crate::borsh_de_or_default(&mut reader)?;
        let base_fee_power_factor: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            protocol_share,
            base_factor,
            base_fee_power_factor,
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
pub struct Bin {
    pub amount_x: u64,
    pub amount_y: u64,
    pub price: u128,
    pub liquidity_supply: u128,
    pub fulfilled_order_amount_x: u64,
    pub fulfilled_order_amount_y: u64,
    pub limit_order_fee_ask_side: u64,
    pub limit_order_fee_bid_side: u64,
    pub fee_amount_x_per_token_stored: u128,
    pub fee_amount_y_per_token_stored: u128,
    pub open_order_amount: u64,
    pub total_processing_order_amount: u64,
    pub processed_order_remaining_amount: u64,
    pub order_age: u32,
    pub limit_order_ask_side: u8,
    pub padding_1: [u8; 3],
}
impl Bin {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount_x: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_y: u64 = crate::borsh_de_or_default(&mut reader)?;
        let price: u128 = crate::borsh_de_or_default(&mut reader)?;
        let liquidity_supply: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fulfilled_order_amount_x: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fulfilled_order_amount_y: u64 = crate::borsh_de_or_default(&mut reader)?;
        let limit_order_fee_ask_side: u64 = crate::borsh_de_or_default(&mut reader)?;
        let limit_order_fee_bid_side: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_amount_x_per_token_stored: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let fee_amount_y_per_token_stored: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let open_order_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_processing_order_amount: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let processed_order_remaining_amount: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let order_age: u32 = crate::borsh_de_or_default(&mut reader)?;
        let limit_order_ask_side: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding_1: [u8; 3] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            amount_x,
            amount_y,
            price,
            liquidity_supply,
            fulfilled_order_amount_x,
            fulfilled_order_amount_y,
            limit_order_fee_ask_side,
            limit_order_fee_bid_side,
            fee_amount_x_per_token_stored,
            fee_amount_y_per_token_stored,
            open_order_amount,
            total_processing_order_amount,
            processed_order_remaining_amount,
            order_age,
            limit_order_ask_side,
            padding_1,
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
pub struct BinLimitOrderAmount {
    pub id: i32,
    pub amount: u64,
}
impl BinLimitOrderAmount {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { id, amount })
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
pub struct BinLiquidityDistribution {
    pub bin_id: i32,
    pub distribution_x: u16,
    pub distribution_y: u16,
}
impl BinLiquidityDistribution {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let distribution_x: u16 = crate::borsh_de_or_default(&mut reader)?;
        let distribution_y: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            bin_id,
            distribution_x,
            distribution_y,
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
pub struct BinLiquidityDistributionByWeight {
    pub bin_id: i32,
    pub weight: u16,
}
impl BinLiquidityDistributionByWeight {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let weight: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { bin_id, weight })
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
pub struct BinLiquidityReduction {
    pub bin_id: i32,
    pub bps_to_remove: u16,
}
impl BinLiquidityReduction {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let bps_to_remove: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { bin_id, bps_to_remove })
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
pub struct CompressedBinDepositAmount {
    pub bin_id: i32,
    pub amount: u32,
}
impl CompressedBinDepositAmount {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let amount: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { bin_id, amount })
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
pub struct CustomizableParams {
    pub active_id: i32,
    pub bin_step: u16,
    pub base_factor: u16,
    pub activation_type: u8,
    pub has_alpha_vault: bool,
    pub activation_point: Option<u64>,
    pub creator_pool_on_off_control: bool,
    pub base_fee_power_factor: u8,
    pub concrete_function_type: u8,
    pub collect_fee_mode: u8,
    #[serde(with = "crate::big_array_serde")]
    pub padding: [u8; 60],
}
impl CustomizableParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let active_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let bin_step: u16 = crate::borsh_de_or_default(&mut reader)?;
        let base_factor: u16 = crate::borsh_de_or_default(&mut reader)?;
        let activation_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let has_alpha_vault: bool = crate::borsh_de_or_default(&mut reader)?;
        let activation_point: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let creator_pool_on_off_control: bool = crate::borsh_de_or_default(&mut reader)?;
        let base_fee_power_factor: u8 = crate::borsh_de_or_default(&mut reader)?;
        let concrete_function_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let collect_fee_mode: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding = <[u8; 60] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            active_id,
            bin_step,
            base_factor,
            activation_type,
            has_alpha_vault,
            activation_point,
            creator_pool_on_off_control,
            base_fee_power_factor,
            concrete_function_type,
            collect_fee_mode,
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
pub struct DummyIx {
    pub pair_status: PairStatus,
    pub pair_type: PairType,
    pub activation_type: ActivationType,
    pub token_program_flag: TokenProgramFlags,
    pub resize_side: ResizeSide,
    pub rounding: Rounding,
}
impl DummyIx {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pair_status: PairStatus = crate::borsh_de_or_default(&mut reader)?;
        let pair_type: PairType = crate::borsh_de_or_default(&mut reader)?;
        let activation_type: ActivationType = crate::borsh_de_or_default(&mut reader)?;
        let token_program_flag: TokenProgramFlags = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let resize_side: ResizeSide = crate::borsh_de_or_default(&mut reader)?;
        let rounding: Rounding = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pair_status,
            pair_type,
            activation_type,
            token_program_flag,
            resize_side,
            rounding,
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
pub struct DynamicFeeParameter {
    pub filter_period: u16,
    pub decay_period: u16,
    pub reduction_factor: u16,
    pub variable_fee_control: u32,
    pub max_volatility_accumulator: u32,
}
impl DynamicFeeParameter {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let filter_period: u16 = crate::borsh_de_or_default(&mut reader)?;
        let decay_period: u16 = crate::borsh_de_or_default(&mut reader)?;
        let reduction_factor: u16 = crate::borsh_de_or_default(&mut reader)?;
        let variable_fee_control: u32 = crate::borsh_de_or_default(&mut reader)?;
        let max_volatility_accumulator: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            filter_period,
            decay_period,
            reduction_factor,
            variable_fee_control,
            max_volatility_accumulator,
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
pub struct FeeInfo {
    pub fee_x_per_token_complete: u128,
    pub fee_y_per_token_complete: u128,
    pub fee_x_pending: u64,
    pub fee_y_pending: u64,
}
impl FeeInfo {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let fee_x_per_token_complete: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fee_y_per_token_complete: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fee_x_pending: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_y_pending: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            fee_x_per_token_complete,
            fee_y_per_token_complete,
            fee_x_pending,
            fee_y_pending,
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
pub struct InitPermissionPairIx {
    pub active_id: i32,
    pub bin_step: u16,
    pub base_factor: u16,
    pub base_fee_power_factor: u8,
    pub activation_type: u8,
    pub padding0: u16,
    pub concrete_function_type: u8,
    pub collect_fee_mode: u8,
}
impl InitPermissionPairIx {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let active_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let bin_step: u16 = crate::borsh_de_or_default(&mut reader)?;
        let base_factor: u16 = crate::borsh_de_or_default(&mut reader)?;
        let base_fee_power_factor: u8 = crate::borsh_de_or_default(&mut reader)?;
        let activation_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding0: u16 = crate::borsh_de_or_default(&mut reader)?;
        let concrete_function_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let collect_fee_mode: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            active_id,
            bin_step,
            base_factor,
            base_fee_power_factor,
            activation_type,
            padding0,
            concrete_function_type,
            collect_fee_mode,
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
pub struct InitPresetParametersIx {
    pub index: u16,
    pub bin_step: u16,
    pub base_factor: u16,
    pub filter_period: u16,
    pub decay_period: u16,
    pub reduction_factor: u16,
    pub variable_fee_control: u32,
    pub max_volatility_accumulator: u32,
    pub protocol_share: u16,
    pub base_fee_power_factor: u8,
    pub concrete_function_type: u8,
    pub collect_fee_mode: u8,
}
impl InitPresetParametersIx {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let bin_step: u16 = crate::borsh_de_or_default(&mut reader)?;
        let base_factor: u16 = crate::borsh_de_or_default(&mut reader)?;
        let filter_period: u16 = crate::borsh_de_or_default(&mut reader)?;
        let decay_period: u16 = crate::borsh_de_or_default(&mut reader)?;
        let reduction_factor: u16 = crate::borsh_de_or_default(&mut reader)?;
        let variable_fee_control: u32 = crate::borsh_de_or_default(&mut reader)?;
        let max_volatility_accumulator: u32 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_share: u16 = crate::borsh_de_or_default(&mut reader)?;
        let base_fee_power_factor: u8 = crate::borsh_de_or_default(&mut reader)?;
        let concrete_function_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let collect_fee_mode: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            index,
            bin_step,
            base_factor,
            filter_period,
            decay_period,
            reduction_factor,
            variable_fee_control,
            max_volatility_accumulator,
            protocol_share,
            base_fee_power_factor,
            concrete_function_type,
            collect_fee_mode,
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
pub struct InitializeLbPair2Params {
    pub active_id: i32,
    #[serde(with = "crate::big_array_serde")]
    pub padding: [u8; 96],
}
impl InitializeLbPair2Params {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let active_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let padding = <[u8; 96] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self { active_id, padding })
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
pub struct LimitOrderBinData {
    pub amount: u64,
    pub age: u32,
    pub padding_0: [u8; 4],
    pub bin_id: i32,
    pub is_ask: u8,
    pub padding_1: [u8; 11],
}
impl LimitOrderBinData {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let age: u32 = crate::borsh_de_or_default(&mut reader)?;
        let padding_0: [u8; 4] = crate::borsh_de_or_default(&mut reader)?;
        let bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let is_ask: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding_1: [u8; 11] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            amount,
            age,
            padding_0,
            bin_id,
            is_ask,
            padding_1,
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
pub struct LiquidityOneSideParameter {
    pub amount: u64,
    pub active_id: i32,
    pub max_active_bin_slippage: i32,
    pub bin_liquidity_dist: Vec<BinLiquidityDistributionByWeight>,
}
impl LiquidityOneSideParameter {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let active_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let max_active_bin_slippage: i32 = crate::borsh_de_or_default(&mut reader)?;
        let bin_liquidity_dist: Vec<BinLiquidityDistributionByWeight> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            amount,
            active_id,
            max_active_bin_slippage,
            bin_liquidity_dist,
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
pub struct LiquidityParameter {
    pub amount_x: u64,
    pub amount_y: u64,
    pub bin_liquidity_dist: Vec<BinLiquidityDistribution>,
}
impl LiquidityParameter {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount_x: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_y: u64 = crate::borsh_de_or_default(&mut reader)?;
        let bin_liquidity_dist: Vec<BinLiquidityDistribution> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            amount_x,
            amount_y,
            bin_liquidity_dist,
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
pub struct LiquidityParameterByStrategy {
    pub amount_x: u64,
    pub amount_y: u64,
    pub active_id: i32,
    pub max_active_bin_slippage: i32,
    pub strategy_parameters: StrategyParameters,
}
impl LiquidityParameterByStrategy {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount_x: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_y: u64 = crate::borsh_de_or_default(&mut reader)?;
        let active_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let max_active_bin_slippage: i32 = crate::borsh_de_or_default(&mut reader)?;
        let strategy_parameters = <StrategyParameters>::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            amount_x,
            amount_y,
            active_id,
            max_active_bin_slippage,
            strategy_parameters,
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
pub struct LiquidityParameterByStrategyOneSide {
    pub amount: u64,
    pub active_id: i32,
    pub max_active_bin_slippage: i32,
    pub strategy_parameters: StrategyParameters,
}
impl LiquidityParameterByStrategyOneSide {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let active_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let max_active_bin_slippage: i32 = crate::borsh_de_or_default(&mut reader)?;
        let strategy_parameters = <StrategyParameters>::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            amount,
            active_id,
            max_active_bin_slippage,
            strategy_parameters,
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
pub struct LiquidityParameterByWeight {
    pub amount_x: u64,
    pub amount_y: u64,
    pub active_id: i32,
    pub max_active_bin_slippage: i32,
    pub bin_liquidity_dist: Vec<BinLiquidityDistributionByWeight>,
}
impl LiquidityParameterByWeight {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount_x: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_y: u64 = crate::borsh_de_or_default(&mut reader)?;
        let active_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let max_active_bin_slippage: i32 = crate::borsh_de_or_default(&mut reader)?;
        let bin_liquidity_dist: Vec<BinLiquidityDistributionByWeight> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            amount_x,
            amount_y,
            active_id,
            max_active_bin_slippage,
            bin_liquidity_dist,
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
pub enum PairStatus {
    #[default]
    Enabled,
    Disabled,
}
impl TryFrom<u8> for PairStatus {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Enabled),
            1u8 => Ok(Self::Disabled),
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
pub enum PairType {
    #[default]
    Permissionless,
    Permission,
    CustomizablePermissionless,
    PermissionlessV2,
}
impl TryFrom<u8> for PairType {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Permissionless),
            1u8 => Ok(Self::Permission),
            2u8 => Ok(Self::CustomizablePermissionless),
            3u8 => Ok(Self::PermissionlessV2),
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
pub struct PlaceLimitOrderParams {
    pub is_ask_side: bool,
    pub padding: [u8; 16],
    pub relative_bin: Option<RelativeBin>,
    pub bins: Vec<BinLimitOrderAmount>,
}
impl PlaceLimitOrderParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let is_ask_side: bool = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 16] = crate::borsh_de_or_default(&mut reader)?;
        let relative_bin: Option<RelativeBin> = crate::borsh_de_or_default(&mut reader)?;
        let bins: Vec<BinLimitOrderAmount> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            is_ask_side,
            padding,
            relative_bin,
            bins,
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
pub struct PositionBinData {
    pub liquidity_share: u128,
    pub reward_info: UserRewardInfo,
    pub fee_info: FeeInfo,
}
impl PositionBinData {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let liquidity_share: u128 = crate::borsh_de_or_default(&mut reader)?;
        let reward_info = if reader.is_empty() {
            Default::default()
        } else {
            <UserRewardInfo>::deserialize(&mut reader)?
        };
        let fee_info = if reader.is_empty() {
            Default::default()
        } else {
            <FeeInfo>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            liquidity_share,
            reward_info,
            fee_info,
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
pub struct RebalanceLiquidityParams {
    pub active_id: i32,
    pub max_active_bin_slippage: u16,
    pub should_claim_fee: bool,
    pub should_claim_reward: bool,
    pub min_withdraw_x_amount: u64,
    pub max_deposit_x_amount: u64,
    pub min_withdraw_y_amount: u64,
    pub max_deposit_y_amount: u64,
    pub shrink_mode: u8,
    pub padding: [u8; 31],
    pub removes: Vec<RemoveLiquidityParams>,
    pub adds: Vec<AddLiquidityParams>,
}
impl RebalanceLiquidityParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let active_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let max_active_bin_slippage: u16 = crate::borsh_de_or_default(&mut reader)?;
        let should_claim_fee: bool = crate::borsh_de_or_default(&mut reader)?;
        let should_claim_reward: bool = crate::borsh_de_or_default(&mut reader)?;
        let min_withdraw_x_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_deposit_x_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_withdraw_y_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_deposit_y_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let shrink_mode: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 31] = crate::borsh_de_or_default(&mut reader)?;
        let removes: Vec<RemoveLiquidityParams> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let adds: Vec<AddLiquidityParams> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            active_id,
            max_active_bin_slippage,
            should_claim_fee,
            should_claim_reward,
            min_withdraw_x_amount,
            max_deposit_x_amount,
            min_withdraw_y_amount,
            max_deposit_y_amount,
            shrink_mode,
            padding,
            removes,
            adds,
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
pub struct RelativeBin {
    pub active_id: i32,
    pub max_active_bin_slippage: i32,
}
impl RelativeBin {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let active_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let max_active_bin_slippage: i32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            active_id,
            max_active_bin_slippage,
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
pub struct RemoveLiquidityParams {
    pub min_bin_id: Option<i32>,
    pub max_bin_id: Option<i32>,
    pub bps: u16,
    pub padding: [u8; 16],
}
impl RemoveLiquidityParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let min_bin_id: Option<i32> = crate::borsh_de_or_default(&mut reader)?;
        let max_bin_id: Option<i32> = crate::borsh_de_or_default(&mut reader)?;
        let bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 16] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            min_bin_id,
            max_bin_id,
            bps,
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
pub enum ResizeSide {
    #[default]
    Lower,
    Upper,
}
impl TryFrom<u8> for ResizeSide {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Lower),
            1u8 => Ok(Self::Upper),
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
pub struct RewardInfo {
    pub mint: Pubkey,
    pub vault: Pubkey,
    pub funder: Pubkey,
    pub reward_duration: u64,
    pub reward_duration_end: u64,
    pub reward_rate: u128,
    pub last_update_time: u64,
    pub cumulative_seconds_with_empty_liquidity_reward: u64,
}
impl RewardInfo {
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
pub enum Rounding {
    #[default]
    Up,
    Down,
}
impl TryFrom<u8> for Rounding {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Up),
            1u8 => Ok(Self::Down),
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
    pub function_type: u8,
    pub collect_fee_mode: u8,
    pub padding: [u8; 3],
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
        let function_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let collect_fee_mode: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 3] = crate::borsh_de_or_default(&mut reader)?;
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
            function_type,
            collect_fee_mode,
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
pub struct StrategyParameters {
    pub min_bin_id: i32,
    pub max_bin_id: i32,
    pub strategy_type: StrategyType,
    #[serde(with = "crate::big_array_serde")]
    pub parameteres: [u8; 64],
}
impl StrategyParameters {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let min_bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let max_bin_id: i32 = crate::borsh_de_or_default(&mut reader)?;
        let strategy_type: StrategyType = crate::borsh_de_or_default(&mut reader)?;
        let parameteres = <[u8; 64] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            min_bin_id,
            max_bin_id,
            strategy_type,
            parameteres,
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
    SpotOneSide,
    CurveOneSide,
    BidAskOneSide,
    SpotBalanced,
    CurveBalanced,
    BidAskBalanced,
    SpotImBalanced,
    CurveImBalanced,
    BidAskImBalanced,
}
impl TryFrom<u8> for StrategyType {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::SpotOneSide),
            1u8 => Ok(Self::CurveOneSide),
            2u8 => Ok(Self::BidAskOneSide),
            3u8 => Ok(Self::SpotBalanced),
            4u8 => Ok(Self::CurveBalanced),
            5u8 => Ok(Self::BidAskBalanced),
            6u8 => Ok(Self::SpotImBalanced),
            7u8 => Ok(Self::CurveImBalanced),
            8u8 => Ok(Self::BidAskImBalanced),
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
pub enum TokenProgramFlags {
    #[default]
    TokenProgram,
    TokenProgram2022,
}
impl TryFrom<u8> for TokenProgramFlags {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::TokenProgram),
            1u8 => Ok(Self::TokenProgram2022),
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
pub struct UserRewardInfo {
    pub reward_per_token_completes: [u128; 2],
    pub reward_pendings: [u64; 2],
}
impl UserRewardInfo {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let reward_per_token_completes: [u128; 2] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let reward_pendings: [u64; 2] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            reward_per_token_completes,
            reward_pendings,
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
pub struct VariableParameters {
    pub volatility_accumulator: u32,
    pub volatility_reference: u32,
    pub index_reference: i32,
    pub padding: [u8; 4],
    pub last_update_timestamp: i64,
    pub padding_1: [u8; 8],
}
impl VariableParameters {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let volatility_accumulator: u32 = crate::borsh_de_or_default(&mut reader)?;
        let volatility_reference: u32 = crate::borsh_de_or_default(&mut reader)?;
        let index_reference: i32 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 4] = crate::borsh_de_or_default(&mut reader)?;
        let last_update_timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        let padding_1: [u8; 8] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            volatility_accumulator,
            volatility_reference,
            index_reference,
            padding,
            last_update_timestamp,
            padding_1,
        })
    }
}
