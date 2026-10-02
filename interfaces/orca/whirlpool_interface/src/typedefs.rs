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
    TransferHookA,
    TransferHookB,
    TransferHookReward,
    TransferHookInput,
    TransferHookIntermediate,
    TransferHookOutput,
    SupplementalTickArrays,
    SupplementalTickArraysOne,
    SupplementalTickArraysTwo,
    TransferHookDepositA,
    TransferHookDepositB,
    TransferHookWithdrawalA,
    TransferHookWithdrawalB,
}
impl TryFrom<u8> for AccountsType {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::TransferHookA),
            1u8 => Ok(Self::TransferHookB),
            2u8 => Ok(Self::TransferHookReward),
            3u8 => Ok(Self::TransferHookInput),
            4u8 => Ok(Self::TransferHookIntermediate),
            5u8 => Ok(Self::TransferHookOutput),
            6u8 => Ok(Self::SupplementalTickArrays),
            7u8 => Ok(Self::SupplementalTickArraysOne),
            8u8 => Ok(Self::SupplementalTickArraysTwo),
            9u8 => Ok(Self::TransferHookDepositA),
            10u8 => Ok(Self::TransferHookDepositB),
            11u8 => Ok(Self::TransferHookWithdrawalA),
            12u8 => Ok(Self::TransferHookWithdrawalB),
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
pub struct AdaptiveFeeConstants {
    pub filter_period: u16,
    pub decay_period: u16,
    pub reduction_factor: u16,
    pub adaptive_fee_control_factor: u32,
    pub max_volatility_accumulator: u32,
    pub tick_group_size: u16,
    pub major_swap_threshold_ticks: u16,
    pub reserved: [u8; 16],
}
impl AdaptiveFeeConstants {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let filter_period: u16 = crate::borsh_de_or_default(&mut reader)?;
        let decay_period: u16 = crate::borsh_de_or_default(&mut reader)?;
        let reduction_factor: u16 = crate::borsh_de_or_default(&mut reader)?;
        let adaptive_fee_control_factor: u32 = crate::borsh_de_or_default(&mut reader)?;
        let max_volatility_accumulator: u32 = crate::borsh_de_or_default(&mut reader)?;
        let tick_group_size: u16 = crate::borsh_de_or_default(&mut reader)?;
        let major_swap_threshold_ticks: u16 = crate::borsh_de_or_default(&mut reader)?;
        let reserved: [u8; 16] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            filter_period,
            decay_period,
            reduction_factor,
            adaptive_fee_control_factor,
            max_volatility_accumulator,
            tick_group_size,
            major_swap_threshold_ticks,
            reserved,
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
pub struct AdaptiveFeeVariables {
    pub last_reference_update_timestamp: u64,
    pub last_major_swap_timestamp: u64,
    pub volatility_reference: u32,
    pub tick_group_index_reference: i32,
    pub volatility_accumulator: u32,
    pub reserved: [u8; 16],
}
impl AdaptiveFeeVariables {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let last_reference_update_timestamp: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let last_major_swap_timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        let volatility_reference: u32 = crate::borsh_de_or_default(&mut reader)?;
        let tick_group_index_reference: i32 = crate::borsh_de_or_default(&mut reader)?;
        let volatility_accumulator: u32 = crate::borsh_de_or_default(&mut reader)?;
        let reserved: [u8; 16] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            last_reference_update_timestamp,
            last_major_swap_timestamp,
            volatility_reference,
            tick_group_index_reference,
            volatility_accumulator,
            reserved,
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
pub enum ConfigFeatureFlag {
    TokenBadge(bool),
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
pub enum DynamicTick {
    #[default]
    Uninitialized,
    Initialized(DynamicTickData),
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
pub struct DynamicTickData {
    pub liquidity_net: i128,
    pub liquidity_gross: u128,
    pub fee_growth_outside_a: u128,
    pub fee_growth_outside_b: u128,
    pub reward_growths_outside: [u128; 3],
}
impl DynamicTickData {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let liquidity_net: i128 = crate::borsh_de_or_default(&mut reader)?;
        let liquidity_gross: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fee_growth_outside_a: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fee_growth_outside_b: u128 = crate::borsh_de_or_default(&mut reader)?;
        let reward_growths_outside: [u128; 3] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            liquidity_net,
            liquidity_gross,
            fee_growth_outside_a,
            fee_growth_outside_b,
            reward_growths_outside,
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
pub enum IncreaseLiquidityMethod {
    ByTokenAmounts {
        token_max_a: u64,
        token_max_b: u64,
        min_sqrt_price: u128,
        max_sqrt_price: u128,
    },
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
pub enum LockType {
    #[default]
    Permanent,
}
impl TryFrom<u8> for LockType {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Permanent),
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
pub enum LockTypeLabel {
    #[default]
    Permanent,
}
impl TryFrom<u8> for LockTypeLabel {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Permanent),
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
pub struct OpenPositionBumps {
    pub position_bump: u8,
}
impl OpenPositionBumps {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let position_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { position_bump })
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
pub struct OpenPositionWithMetadataBumps {
    pub position_bump: u8,
    pub metadata_bump: u8,
}
impl OpenPositionWithMetadataBumps {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let position_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let metadata_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            position_bump,
            metadata_bump,
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
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub enum RepositionLiquidityMethod {
    ByLiquidity {
        new_liquidity_amount: u128,
        existing_range_token_min_a: u64,
        existing_range_token_min_b: u64,
        new_range_token_max_a: u64,
        new_range_token_max_b: u64,
    },
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
pub struct Tick {
    pub initialized: bool,
    pub liquidity_net: i128,
    pub liquidity_gross: u128,
    pub fee_growth_outside_a: u128,
    pub fee_growth_outside_b: u128,
    pub reward_growths_outside: [u128; 3],
}
impl Tick {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let initialized: bool = crate::borsh_de_or_default(&mut reader)?;
        let liquidity_net: i128 = crate::borsh_de_or_default(&mut reader)?;
        let liquidity_gross: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fee_growth_outside_a: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fee_growth_outside_b: u128 = crate::borsh_de_or_default(&mut reader)?;
        let reward_growths_outside: [u128; 3] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            initialized,
            liquidity_net,
            liquidity_gross,
            fee_growth_outside_a,
            fee_growth_outside_b,
            reward_growths_outside,
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
pub enum TokenBadgeAttribute {
    RequireNonTransferablePosition(bool),
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
pub struct WhirlpoolBumps {
    pub whirlpool_bump: u8,
}
impl WhirlpoolBumps {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let whirlpool_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { whirlpool_bump })
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
    pub extension: [u8; 32],
    pub emissions_per_second_x64: u128,
    pub growth_global_x64: u128,
}
impl WhirlpoolRewardInfo {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let extension: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        let emissions_per_second_x64: u128 = crate::borsh_de_or_default(&mut reader)?;
        let growth_global_x64: u128 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            mint,
            vault,
            extension,
            emissions_per_second_x64,
            growth_global_x64,
        })
    }
}
