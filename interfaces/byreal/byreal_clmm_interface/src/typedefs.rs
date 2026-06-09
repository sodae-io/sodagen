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
pub struct CreatePoolDecayFeeParams {
    pub sqrt_price_x64: u128,
    pub open_time: Option<u64>,
    pub use_decay_fee: bool,
    pub decay_fee_on_sell_mint0: bool,
    pub decay_fee_on_sell_mint1: bool,
    pub init_decay_fee_rate: u8,
    pub decay_fee_decrease_rate: u8,
    pub decay_fee_decrease_interval: u8,
}
impl CreatePoolDecayFeeParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let sqrt_price_x64: u128 = crate::borsh_de_or_default(&mut reader)?;
        let open_time: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let use_decay_fee: bool = crate::borsh_de_or_default(&mut reader)?;
        let decay_fee_on_sell_mint0: bool = crate::borsh_de_or_default(&mut reader)?;
        let decay_fee_on_sell_mint1: bool = crate::borsh_de_or_default(&mut reader)?;
        let init_decay_fee_rate: u8 = crate::borsh_de_or_default(&mut reader)?;
        let decay_fee_decrease_rate: u8 = crate::borsh_de_or_default(&mut reader)?;
        let decay_fee_decrease_interval: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            sqrt_price_x64,
            open_time,
            use_decay_fee,
            decay_fee_on_sell_mint0,
            decay_fee_on_sell_mint1,
            init_decay_fee_rate,
            decay_fee_decrease_rate,
            decay_fee_decrease_interval,
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
pub struct InitAdminGroupParams {
    pub fee_keeper: Pubkey,
    pub reward_config_manager: Pubkey,
    pub reward_claim_manager: Pubkey,
    pub pool_manager: Pubkey,
    pub emergency_manager: Pubkey,
    pub normal_manager: Pubkey,
}
impl InitAdminGroupParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let fee_keeper: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let reward_config_manager: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let reward_claim_manager: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_manager: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let emergency_manager: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let normal_manager: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            fee_keeper,
            reward_config_manager,
            reward_claim_manager,
            pool_manager,
            emergency_manager,
            normal_manager,
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
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct SetSwapDynamicFeeParamsInput {
    pub enabled: bool,
    pub arbitrage_fee_buffer_ppm: Option<u16>,
    pub trade_slippage_fee_base: Option<u8>,
    pub trade_slippage_fee_trade_size_threshold: Option<u8>,
    pub imbalance_fee_base: Option<u8>,
    pub imbalance_fee_x: Option<u8>,
    pub token0_pyth_feed_id: Option<[u8; 32]>,
    pub token1_pyth_feed_id: Option<[u8; 32]>,
}
impl SetSwapDynamicFeeParamsInput {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let enabled: bool = crate::borsh_de_or_default(&mut reader)?;
        let arbitrage_fee_buffer_ppm: Option<u16> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let trade_slippage_fee_base: Option<u8> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let trade_slippage_fee_trade_size_threshold: Option<u8> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let imbalance_fee_base: Option<u8> = crate::borsh_de_or_default(&mut reader)?;
        let imbalance_fee_x: Option<u8> = crate::borsh_de_or_default(&mut reader)?;
        let token0_pyth_feed_id: Option<[u8; 32]> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let token1_pyth_feed_id: Option<[u8; 32]> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            enabled,
            arbitrage_fee_buffer_ppm,
            trade_slippage_fee_base,
            trade_slippage_fee_trade_size_threshold,
            imbalance_fee_base,
            imbalance_fee_x,
            token0_pyth_feed_id,
            token1_pyth_feed_id,
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
pub struct UpdateAdminGroupParams {
    pub fee_keeper: Option<Pubkey>,
    pub reward_config_manager: Option<Pubkey>,
    pub reward_claim_manager: Option<Pubkey>,
    pub pool_manager: Option<Pubkey>,
    pub emergency_manager: Option<Pubkey>,
    pub normal_manager: Option<Pubkey>,
}
impl UpdateAdminGroupParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let fee_keeper: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let reward_config_manager: Option<Pubkey> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let reward_claim_manager: Option<Pubkey> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let pool_manager: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let emergency_manager: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let normal_manager: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            fee_keeper,
            reward_config_manager,
            reward_claim_manager,
            pool_manager,
            emergency_manager,
            normal_manager,
        })
    }
}
