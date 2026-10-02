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
pub struct UpdatePerpMarketSummaryStatsParams {
    pub quote_asset_amount_with_unsettled_lp: Option<i64>,
    pub net_unsettled_funding_pnl: Option<i64>,
    pub update_amm_summary_stats: Option<bool>,
    pub exclude_total_liq_fee: Option<bool>,
}
impl UpdatePerpMarketSummaryStatsParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let quote_asset_amount_with_unsettled_lp: Option<i64> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let net_unsettled_funding_pnl: Option<i64> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let update_amm_summary_stats: Option<bool> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let exclude_total_liq_fee: Option<bool> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            quote_asset_amount_with_unsettled_lp,
            net_unsettled_funding_pnl,
            update_amm_summary_stats,
            exclude_total_liq_fee,
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
pub struct ConstituentParams {
    pub max_weight_deviation: Option<i64>,
    pub swap_fee_min: Option<i64>,
    pub swap_fee_max: Option<i64>,
    pub max_borrow_token_amount: Option<u64>,
    pub oracle_staleness_threshold: Option<u64>,
    pub cost_to_trade_bps: Option<i32>,
    pub constituent_derivative_index: Option<i16>,
    pub derivative_weight: Option<u64>,
    pub volatility: Option<u64>,
    pub gamma_execution: Option<u8>,
    pub gamma_inventory: Option<u8>,
    pub xi: Option<u8>,
}
impl ConstituentParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let max_weight_deviation: Option<i64> = crate::borsh_de_or_default(&mut reader)?;
        let swap_fee_min: Option<i64> = crate::borsh_de_or_default(&mut reader)?;
        let swap_fee_max: Option<i64> = crate::borsh_de_or_default(&mut reader)?;
        let max_borrow_token_amount: Option<u64> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let oracle_staleness_threshold: Option<u64> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let cost_to_trade_bps: Option<i32> = crate::borsh_de_or_default(&mut reader)?;
        let constituent_derivative_index: Option<i16> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let derivative_weight: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let volatility: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let gamma_execution: Option<u8> = crate::borsh_de_or_default(&mut reader)?;
        let gamma_inventory: Option<u8> = crate::borsh_de_or_default(&mut reader)?;
        let xi: Option<u8> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            max_weight_deviation,
            swap_fee_min,
            swap_fee_max,
            max_borrow_token_amount,
            oracle_staleness_threshold,
            cost_to_trade_bps,
            constituent_derivative_index,
            derivative_weight,
            volatility,
            gamma_execution,
            gamma_inventory,
            xi,
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
pub struct LpPoolParams {
    pub max_settle_quote_amount: Option<u64>,
    pub volatility: Option<u64>,
    pub gamma_execution: Option<u8>,
    pub xi: Option<u8>,
    pub max_aum: Option<u128>,
    pub whitelist_mint: Option<Pubkey>,
}
impl LpPoolParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let max_settle_quote_amount: Option<u64> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let volatility: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let gamma_execution: Option<u8> = crate::borsh_de_or_default(&mut reader)?;
        let xi: Option<u8> = crate::borsh_de_or_default(&mut reader)?;
        let max_aum: Option<u128> = crate::borsh_de_or_default(&mut reader)?;
        let whitelist_mint: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            max_settle_quote_amount,
            volatility,
            gamma_execution,
            xi,
            max_aum,
            whitelist_mint,
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
pub struct OverrideAmmCacheParams {
    pub quote_owed_from_lp_pool: Option<i64>,
    pub last_settle_slot: Option<u64>,
    pub last_fee_pool_token_amount: Option<u128>,
    pub last_net_pnl_pool_token_amount: Option<i128>,
    pub amm_position_scalar: Option<u8>,
    pub amm_inventory_limit: Option<i64>,
}
impl OverrideAmmCacheParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let quote_owed_from_lp_pool: Option<i64> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let last_settle_slot: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let last_fee_pool_token_amount: Option<u128> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let last_net_pnl_pool_token_amount: Option<i128> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let amm_position_scalar: Option<u8> = crate::borsh_de_or_default(&mut reader)?;
        let amm_inventory_limit: Option<i64> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            quote_owed_from_lp_pool,
            last_settle_slot,
            last_fee_pool_token_amount,
            last_net_pnl_pool_token_amount,
            amm_position_scalar,
            amm_inventory_limit,
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
pub struct AddAmmConstituentMappingDatum {
    pub constituent_index: u16,
    pub perp_market_index: u16,
    pub weight: i64,
}
impl AddAmmConstituentMappingDatum {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let constituent_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let perp_market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let weight: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            constituent_index,
            perp_market_index,
            weight,
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
pub struct CacheInfo {
    pub oracle: Pubkey,
    pub last_fee_pool_token_amount: u128,
    pub last_net_pnl_pool_token_amount: i128,
    pub last_exchange_fees: u128,
    pub last_settle_amm_ex_fees: u128,
    pub last_settle_amm_pnl: i128,
    pub position: i64,
    pub slot: u64,
    pub last_settle_amount: u64,
    pub last_settle_slot: u64,
    pub last_settle_ts: i64,
    pub quote_owed_from_lp_pool: i64,
    pub amm_inventory_limit: i64,
    pub oracle_price: i64,
    pub oracle_slot: u64,
    pub market_index: u16,
    pub oracle_source: u8,
    pub oracle_validity: u8,
    pub lp_status_for_perp_market: u8,
    pub amm_position_scalar: u8,
    #[serde(with = "crate::big_array_serde")]
    pub padding: [u8; 34],
}
impl CacheInfo {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let oracle: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let last_fee_pool_token_amount: u128 = crate::borsh_de_or_default(&mut reader)?;
        let last_net_pnl_pool_token_amount: i128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let last_exchange_fees: u128 = crate::borsh_de_or_default(&mut reader)?;
        let last_settle_amm_ex_fees: u128 = crate::borsh_de_or_default(&mut reader)?;
        let last_settle_amm_pnl: i128 = crate::borsh_de_or_default(&mut reader)?;
        let position: i64 = crate::borsh_de_or_default(&mut reader)?;
        let slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_settle_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_settle_slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_settle_ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_owed_from_lp_pool: i64 = crate::borsh_de_or_default(&mut reader)?;
        let amm_inventory_limit: i64 = crate::borsh_de_or_default(&mut reader)?;
        let oracle_price: i64 = crate::borsh_de_or_default(&mut reader)?;
        let oracle_slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        let market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let oracle_source: u8 = crate::borsh_de_or_default(&mut reader)?;
        let oracle_validity: u8 = crate::borsh_de_or_default(&mut reader)?;
        let lp_status_for_perp_market: u8 = crate::borsh_de_or_default(&mut reader)?;
        let amm_position_scalar: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding = <[u8; 34] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            oracle,
            last_fee_pool_token_amount,
            last_net_pnl_pool_token_amount,
            last_exchange_fees,
            last_settle_amm_ex_fees,
            last_settle_amm_pnl,
            position,
            slot,
            last_settle_amount,
            last_settle_slot,
            last_settle_ts,
            quote_owed_from_lp_pool,
            amm_inventory_limit,
            oracle_price,
            oracle_slot,
            market_index,
            oracle_source,
            oracle_validity,
            lp_status_for_perp_market,
            amm_position_scalar,
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
pub struct AmmCacheFixed {
    pub bump: u8,
    pub pad: [u8; 3],
    pub len: u32,
}
impl AmmCacheFixed {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let pad: [u8; 3] = crate::borsh_de_or_default(&mut reader)?;
        let len: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { bump, pad, len })
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
pub struct LiquidatePerpRecord {
    pub market_index: u16,
    pub oracle_price: i64,
    pub base_asset_amount: i64,
    pub quote_asset_amount: i64,
    pub lp_shares: u64,
    pub fill_record_id: u64,
    pub user_order_id: u32,
    pub liquidator_order_id: u32,
    pub liquidator_fee: u64,
    pub if_fee: u64,
}
impl LiquidatePerpRecord {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let oracle_price: i64 = crate::borsh_de_or_default(&mut reader)?;
        let base_asset_amount: i64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_asset_amount: i64 = crate::borsh_de_or_default(&mut reader)?;
        let lp_shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fill_record_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user_order_id: u32 = crate::borsh_de_or_default(&mut reader)?;
        let liquidator_order_id: u32 = crate::borsh_de_or_default(&mut reader)?;
        let liquidator_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let if_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            market_index,
            oracle_price,
            base_asset_amount,
            quote_asset_amount,
            lp_shares,
            fill_record_id,
            user_order_id,
            liquidator_order_id,
            liquidator_fee,
            if_fee,
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
pub struct LiquidateSpotRecord {
    pub asset_market_index: u16,
    pub asset_price: i64,
    pub asset_transfer: u128,
    pub liability_market_index: u16,
    pub liability_price: i64,
    pub liability_transfer: u128,
    pub if_fee: u64,
}
impl LiquidateSpotRecord {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let asset_market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let asset_price: i64 = crate::borsh_de_or_default(&mut reader)?;
        let asset_transfer: u128 = crate::borsh_de_or_default(&mut reader)?;
        let liability_market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let liability_price: i64 = crate::borsh_de_or_default(&mut reader)?;
        let liability_transfer: u128 = crate::borsh_de_or_default(&mut reader)?;
        let if_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            asset_market_index,
            asset_price,
            asset_transfer,
            liability_market_index,
            liability_price,
            liability_transfer,
            if_fee,
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
pub struct LiquidateBorrowForPerpPnlRecord {
    pub perp_market_index: u16,
    pub market_oracle_price: i64,
    pub pnl_transfer: u128,
    pub liability_market_index: u16,
    pub liability_price: i64,
    pub liability_transfer: u128,
}
impl LiquidateBorrowForPerpPnlRecord {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let perp_market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let market_oracle_price: i64 = crate::borsh_de_or_default(&mut reader)?;
        let pnl_transfer: u128 = crate::borsh_de_or_default(&mut reader)?;
        let liability_market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let liability_price: i64 = crate::borsh_de_or_default(&mut reader)?;
        let liability_transfer: u128 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            perp_market_index,
            market_oracle_price,
            pnl_transfer,
            liability_market_index,
            liability_price,
            liability_transfer,
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
pub struct LiquidatePerpPnlForDepositRecord {
    pub perp_market_index: u16,
    pub market_oracle_price: i64,
    pub pnl_transfer: u128,
    pub asset_market_index: u16,
    pub asset_price: i64,
    pub asset_transfer: u128,
}
impl LiquidatePerpPnlForDepositRecord {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let perp_market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let market_oracle_price: i64 = crate::borsh_de_or_default(&mut reader)?;
        let pnl_transfer: u128 = crate::borsh_de_or_default(&mut reader)?;
        let asset_market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let asset_price: i64 = crate::borsh_de_or_default(&mut reader)?;
        let asset_transfer: u128 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            perp_market_index,
            market_oracle_price,
            pnl_transfer,
            asset_market_index,
            asset_price,
            asset_transfer,
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
pub struct PerpBankruptcyRecord {
    pub market_index: u16,
    pub pnl: i128,
    pub if_payment: u128,
    pub clawback_user: Option<Pubkey>,
    pub clawback_user_payment: Option<u128>,
    pub cumulative_funding_rate_delta: i128,
}
impl PerpBankruptcyRecord {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let pnl: i128 = crate::borsh_de_or_default(&mut reader)?;
        let if_payment: u128 = crate::borsh_de_or_default(&mut reader)?;
        let clawback_user: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let clawback_user_payment: Option<u128> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let cumulative_funding_rate_delta: i128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            market_index,
            pnl,
            if_payment,
            clawback_user,
            clawback_user_payment,
            cumulative_funding_rate_delta,
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
pub struct SpotBankruptcyRecord {
    pub market_index: u16,
    pub borrow_amount: u128,
    pub if_payment: u128,
    pub cumulative_deposit_interest_delta: u128,
}
impl SpotBankruptcyRecord {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let borrow_amount: u128 = crate::borsh_de_or_default(&mut reader)?;
        let if_payment: u128 = crate::borsh_de_or_default(&mut reader)?;
        let cumulative_deposit_interest_delta: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            market_index,
            borrow_amount,
            if_payment,
            cumulative_deposit_interest_delta,
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
pub struct IfRebalanceConfigParams {
    pub total_in_amount: u64,
    pub epoch_max_in_amount: u64,
    pub epoch_duration: i64,
    pub out_market_index: u16,
    pub in_market_index: u16,
    pub max_slippage_bps: u16,
    pub swap_mode: u8,
    pub status: u8,
}
impl IfRebalanceConfigParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let total_in_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let epoch_max_in_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let epoch_duration: i64 = crate::borsh_de_or_default(&mut reader)?;
        let out_market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let in_market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let max_slippage_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let swap_mode: u8 = crate::borsh_de_or_default(&mut reader)?;
        let status: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            total_in_amount,
            epoch_max_in_amount,
            epoch_duration,
            out_market_index,
            in_market_index,
            max_slippage_bps,
            swap_mode,
            status,
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
pub struct ConstituentSpotBalance {
    pub scaled_balance: u128,
    pub cumulative_deposits: i64,
    pub market_index: u16,
    pub balance_type: SpotBalanceType,
    pub padding: [u8; 5],
}
impl ConstituentSpotBalance {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let scaled_balance: u128 = crate::borsh_de_or_default(&mut reader)?;
        let cumulative_deposits: i64 = crate::borsh_de_or_default(&mut reader)?;
        let market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let balance_type: SpotBalanceType = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 5] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            scaled_balance,
            cumulative_deposits,
            market_index,
            balance_type,
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
pub struct AmmConstituentDatum {
    pub perp_market_index: u16,
    pub constituent_index: u16,
    pub padding: [u8; 4],
    pub last_slot: u64,
    pub weight: i64,
}
impl AmmConstituentDatum {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let perp_market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let constituent_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 4] = crate::borsh_de_or_default(&mut reader)?;
        let last_slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        let weight: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            perp_market_index,
            constituent_index,
            padding,
            last_slot,
            weight,
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
pub struct AmmConstituentMappingFixed {
    pub lp_pool: Pubkey,
    pub bump: u8,
    pub pad: [u8; 3],
    pub len: u32,
}
impl AmmConstituentMappingFixed {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let lp_pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let pad: [u8; 3] = crate::borsh_de_or_default(&mut reader)?;
        let len: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { lp_pool, bump, pad, len })
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
pub struct TargetsDatum {
    pub cost_to_trade_bps: i32,
    pub padding: [u8; 4],
    pub target_base: i64,
    pub last_oracle_slot: u64,
    pub last_position_slot: u64,
}
impl TargetsDatum {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let cost_to_trade_bps: i32 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 4] = crate::borsh_de_or_default(&mut reader)?;
        let target_base: i64 = crate::borsh_de_or_default(&mut reader)?;
        let last_oracle_slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_position_slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            cost_to_trade_bps,
            padding,
            target_base,
            last_oracle_slot,
            last_position_slot,
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
pub struct ConstituentTargetBaseFixed {
    pub lp_pool: Pubkey,
    pub bump: u8,
    pub pad: [u8; 3],
    pub len: u32,
}
impl ConstituentTargetBaseFixed {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let lp_pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let pad: [u8; 3] = crate::borsh_de_or_default(&mut reader)?;
        let len: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { lp_pool, bump, pad, len })
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
pub struct ConstituentCorrelationsFixed {
    pub lp_pool: Pubkey,
    pub bump: u8,
    pub pad: [u8; 3],
    pub len: u32,
}
impl ConstituentCorrelationsFixed {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let lp_pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let pad: [u8; 3] = crate::borsh_de_or_default(&mut reader)?;
        let len: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { lp_pool, bump, pad, len })
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
pub struct MarketIdentifier {
    pub market_type: MarketType,
    pub market_index: u16,
}
impl MarketIdentifier {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let market_type: MarketType = crate::borsh_de_or_default(&mut reader)?;
        let market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { market_type, market_index })
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
pub struct HistoricalOracleData {
    pub last_oracle_price: i64,
    pub last_oracle_conf: u64,
    pub last_oracle_delay: i64,
    pub last_oracle_price_twap: i64,
    pub last_oracle_price_twap5min: i64,
    pub last_oracle_price_twap_ts: i64,
}
impl HistoricalOracleData {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let last_oracle_price: i64 = crate::borsh_de_or_default(&mut reader)?;
        let last_oracle_conf: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_oracle_delay: i64 = crate::borsh_de_or_default(&mut reader)?;
        let last_oracle_price_twap: i64 = crate::borsh_de_or_default(&mut reader)?;
        let last_oracle_price_twap5min: i64 = crate::borsh_de_or_default(&mut reader)?;
        let last_oracle_price_twap_ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            last_oracle_price,
            last_oracle_conf,
            last_oracle_delay,
            last_oracle_price_twap,
            last_oracle_price_twap5min,
            last_oracle_price_twap_ts,
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
pub struct HistoricalIndexData {
    pub last_index_bid_price: u64,
    pub last_index_ask_price: u64,
    pub last_index_price_twap: u64,
    pub last_index_price_twap5min: u64,
    pub last_index_price_twap_ts: i64,
}
impl HistoricalIndexData {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let last_index_bid_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_index_ask_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_index_price_twap: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_index_price_twap5min: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_index_price_twap_ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            last_index_bid_price,
            last_index_ask_price,
            last_index_price_twap,
            last_index_price_twap5min,
            last_index_price_twap_ts,
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
pub struct PrelaunchOracleParams {
    pub perp_market_index: u16,
    pub price: Option<i64>,
    pub max_price: Option<i64>,
}
impl PrelaunchOracleParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let perp_market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let price: Option<i64> = crate::borsh_de_or_default(&mut reader)?;
        let max_price: Option<i64> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            perp_market_index,
            price,
            max_price,
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
pub struct OrderParams {
    pub order_type: OrderType,
    pub market_type: MarketType,
    pub direction: PositionDirection,
    pub user_order_id: u8,
    pub base_asset_amount: u64,
    pub price: u64,
    pub market_index: u16,
    pub reduce_only: bool,
    pub post_only: PostOnlyParam,
    pub bit_flags: u8,
    pub max_ts: Option<i64>,
    pub trigger_price: Option<u64>,
    pub trigger_condition: OrderTriggerCondition,
    pub oracle_price_offset: Option<i32>,
    pub auction_duration: Option<u8>,
    pub auction_start_price: Option<i64>,
    pub auction_end_price: Option<i64>,
}
impl OrderParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let order_type: OrderType = crate::borsh_de_or_default(&mut reader)?;
        let market_type: MarketType = crate::borsh_de_or_default(&mut reader)?;
        let direction: PositionDirection = crate::borsh_de_or_default(&mut reader)?;
        let user_order_id: u8 = crate::borsh_de_or_default(&mut reader)?;
        let base_asset_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let reduce_only: bool = crate::borsh_de_or_default(&mut reader)?;
        let post_only: PostOnlyParam = crate::borsh_de_or_default(&mut reader)?;
        let bit_flags: u8 = crate::borsh_de_or_default(&mut reader)?;
        let max_ts: Option<i64> = crate::borsh_de_or_default(&mut reader)?;
        let trigger_price: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let trigger_condition: OrderTriggerCondition = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let oracle_price_offset: Option<i32> = crate::borsh_de_or_default(&mut reader)?;
        let auction_duration: Option<u8> = crate::borsh_de_or_default(&mut reader)?;
        let auction_start_price: Option<i64> = crate::borsh_de_or_default(&mut reader)?;
        let auction_end_price: Option<i64> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            order_type,
            market_type,
            direction,
            user_order_id,
            base_asset_amount,
            price,
            market_index,
            reduce_only,
            post_only,
            bit_flags,
            max_ts,
            trigger_price,
            trigger_condition,
            oracle_price_offset,
            auction_duration,
            auction_start_price,
            auction_end_price,
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
pub struct SignedMsgOrderParamsMessage {
    pub signed_msg_order_params: OrderParams,
    pub sub_account_id: u16,
    pub slot: u64,
    pub uuid: [u8; 8],
    pub take_profit_order_params: Option<SignedMsgTriggerOrderParams>,
    pub stop_loss_order_params: Option<SignedMsgTriggerOrderParams>,
    pub max_margin_ratio: Option<u16>,
    pub builder_idx: Option<u8>,
    pub builder_fee_tenth_bps: Option<u16>,
    pub isolated_position_deposit: Option<u64>,
}
impl SignedMsgOrderParamsMessage {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let signed_msg_order_params = if reader.is_empty() {
            Default::default()
        } else {
            <OrderParams>::deserialize(&mut reader)?
        };
        let sub_account_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        let uuid: [u8; 8] = crate::borsh_de_or_default(&mut reader)?;
        let take_profit_order_params: Option<SignedMsgTriggerOrderParams> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let stop_loss_order_params: Option<SignedMsgTriggerOrderParams> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let max_margin_ratio: Option<u16> = crate::borsh_de_or_default(&mut reader)?;
        let builder_idx: Option<u8> = crate::borsh_de_or_default(&mut reader)?;
        let builder_fee_tenth_bps: Option<u16> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let isolated_position_deposit: Option<u64> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            signed_msg_order_params,
            sub_account_id,
            slot,
            uuid,
            take_profit_order_params,
            stop_loss_order_params,
            max_margin_ratio,
            builder_idx,
            builder_fee_tenth_bps,
            isolated_position_deposit,
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
pub struct SignedMsgOrderParamsDelegateMessage {
    pub signed_msg_order_params: OrderParams,
    pub taker_pubkey: Pubkey,
    pub slot: u64,
    pub uuid: [u8; 8],
    pub take_profit_order_params: Option<SignedMsgTriggerOrderParams>,
    pub stop_loss_order_params: Option<SignedMsgTriggerOrderParams>,
    pub max_margin_ratio: Option<u16>,
    pub builder_idx: Option<u8>,
    pub builder_fee_tenth_bps: Option<u16>,
    pub isolated_position_deposit: Option<u64>,
}
impl SignedMsgOrderParamsDelegateMessage {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let signed_msg_order_params = if reader.is_empty() {
            Default::default()
        } else {
            <OrderParams>::deserialize(&mut reader)?
        };
        let taker_pubkey: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        let uuid: [u8; 8] = crate::borsh_de_or_default(&mut reader)?;
        let take_profit_order_params: Option<SignedMsgTriggerOrderParams> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let stop_loss_order_params: Option<SignedMsgTriggerOrderParams> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let max_margin_ratio: Option<u16> = crate::borsh_de_or_default(&mut reader)?;
        let builder_idx: Option<u8> = crate::borsh_de_or_default(&mut reader)?;
        let builder_fee_tenth_bps: Option<u16> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let isolated_position_deposit: Option<u64> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            signed_msg_order_params,
            taker_pubkey,
            slot,
            uuid,
            take_profit_order_params,
            stop_loss_order_params,
            max_margin_ratio,
            builder_idx,
            builder_fee_tenth_bps,
            isolated_position_deposit,
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
pub struct SignedMsgTriggerOrderParams {
    pub trigger_price: u64,
    pub base_asset_amount: u64,
}
impl SignedMsgTriggerOrderParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let trigger_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let base_asset_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            trigger_price,
            base_asset_amount,
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
pub struct ModifyOrderParams {
    pub direction: Option<PositionDirection>,
    pub base_asset_amount: Option<u64>,
    pub price: Option<u64>,
    pub reduce_only: Option<bool>,
    pub post_only: Option<PostOnlyParam>,
    pub bit_flags: Option<u8>,
    pub max_ts: Option<i64>,
    pub trigger_price: Option<u64>,
    pub trigger_condition: Option<OrderTriggerCondition>,
    pub oracle_price_offset: Option<i32>,
    pub auction_duration: Option<u8>,
    pub auction_start_price: Option<i64>,
    pub auction_end_price: Option<i64>,
    pub policy: Option<u8>,
}
impl ModifyOrderParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let direction: Option<PositionDirection> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let base_asset_amount: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let price: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let reduce_only: Option<bool> = crate::borsh_de_or_default(&mut reader)?;
        let post_only: Option<PostOnlyParam> = crate::borsh_de_or_default(&mut reader)?;
        let bit_flags: Option<u8> = crate::borsh_de_or_default(&mut reader)?;
        let max_ts: Option<i64> = crate::borsh_de_or_default(&mut reader)?;
        let trigger_price: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let trigger_condition: Option<OrderTriggerCondition> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let oracle_price_offset: Option<i32> = crate::borsh_de_or_default(&mut reader)?;
        let auction_duration: Option<u8> = crate::borsh_de_or_default(&mut reader)?;
        let auction_start_price: Option<i64> = crate::borsh_de_or_default(&mut reader)?;
        let auction_end_price: Option<i64> = crate::borsh_de_or_default(&mut reader)?;
        let policy: Option<u8> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            direction,
            base_asset_amount,
            price,
            reduce_only,
            post_only,
            bit_flags,
            max_ts,
            trigger_price,
            trigger_condition,
            oracle_price_offset,
            auction_duration,
            auction_start_price,
            auction_end_price,
            policy,
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
pub struct InsuranceClaim {
    pub revenue_withdraw_since_last_settle: i64,
    pub max_revenue_withdraw_per_period: u64,
    pub quote_max_insurance: u64,
    pub quote_settled_insurance: u64,
    pub last_revenue_withdraw_ts: i64,
}
impl InsuranceClaim {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let revenue_withdraw_since_last_settle: i64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let max_revenue_withdraw_per_period: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let quote_max_insurance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_settled_insurance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_revenue_withdraw_ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            revenue_withdraw_since_last_settle,
            max_revenue_withdraw_per_period,
            quote_max_insurance,
            quote_settled_insurance,
            last_revenue_withdraw_ts,
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
pub struct PoolBalance {
    pub scaled_balance: u128,
    pub market_index: u16,
    pub padding: [u8; 6],
}
impl PoolBalance {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let scaled_balance: u128 = crate::borsh_de_or_default(&mut reader)?;
        let market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 6] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            scaled_balance,
            market_index,
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
pub struct AMM {
    pub oracle: Pubkey,
    pub historical_oracle_data: HistoricalOracleData,
    pub base_asset_amount_per_lp: i128,
    pub quote_asset_amount_per_lp: i128,
    pub fee_pool: PoolBalance,
    pub base_asset_reserve: u128,
    pub quote_asset_reserve: u128,
    pub concentration_coef: u128,
    pub min_base_asset_reserve: u128,
    pub max_base_asset_reserve: u128,
    pub sqrt_k: u128,
    pub peg_multiplier: u128,
    pub terminal_quote_asset_reserve: u128,
    pub base_asset_amount_long: i128,
    pub base_asset_amount_short: i128,
    pub base_asset_amount_with_amm: i128,
    pub base_asset_amount_with_unsettled_lp: i128,
    pub max_open_interest: u128,
    pub quote_asset_amount: i128,
    pub quote_entry_amount_long: i128,
    pub quote_entry_amount_short: i128,
    pub quote_break_even_amount_long: i128,
    pub quote_break_even_amount_short: i128,
    pub user_lp_shares: u128,
    pub last_funding_rate: i64,
    pub last_funding_rate_long: i64,
    pub last_funding_rate_short: i64,
    pub last24h_avg_funding_rate: i64,
    pub total_fee: i128,
    pub total_mm_fee: i128,
    pub total_exchange_fee: u128,
    pub total_fee_minus_distributions: i128,
    pub total_fee_withdrawn: u128,
    pub total_liquidation_fee: u128,
    pub cumulative_funding_rate_long: i128,
    pub cumulative_funding_rate_short: i128,
    pub total_social_loss: u128,
    pub ask_base_asset_reserve: u128,
    pub ask_quote_asset_reserve: u128,
    pub bid_base_asset_reserve: u128,
    pub bid_quote_asset_reserve: u128,
    pub last_oracle_normalised_price: i64,
    pub last_oracle_reserve_price_spread_pct: i64,
    pub last_bid_price_twap: u64,
    pub last_ask_price_twap: u64,
    pub last_mark_price_twap: u64,
    pub last_mark_price_twap5min: u64,
    pub last_update_slot: u64,
    pub last_oracle_conf_pct: u64,
    pub net_revenue_since_last_funding: i64,
    pub last_funding_rate_ts: i64,
    pub funding_period: i64,
    pub order_step_size: u64,
    pub order_tick_size: u64,
    pub min_order_size: u64,
    pub mm_oracle_slot: u64,
    pub volume24h: u64,
    pub long_intensity_volume: u64,
    pub short_intensity_volume: u64,
    pub last_trade_ts: i64,
    pub mark_std: u64,
    pub oracle_std: u64,
    pub last_mark_price_twap_ts: i64,
    pub base_spread: u32,
    pub max_spread: u32,
    pub long_spread: u32,
    pub short_spread: u32,
    pub mm_oracle_price: i64,
    pub max_fill_reserve_fraction: u16,
    pub max_slippage_ratio: u16,
    pub curve_update_intensity: u8,
    pub amm_jit_intensity: u8,
    pub oracle_source: OracleSource,
    pub last_oracle_valid: bool,
    pub target_base_asset_amount_per_lp: i32,
    pub per_lp_base: i8,
    pub oracle_low_risk_slot_delay_override: i8,
    pub amm_spread_adjustment: i8,
    pub oracle_slot_delay_override: i8,
    pub mm_oracle_sequence_id: u64,
    pub net_unsettled_funding_pnl: i64,
    pub quote_asset_amount_with_unsettled_lp: i64,
    pub reference_price_offset: i32,
    pub amm_inventory_spread_adjustment: i8,
    pub reference_price_offset_deadband_pct: u8,
    pub padding: [u8; 2],
    pub last_funding_oracle_twap: i64,
}
impl AMM {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let oracle: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let historical_oracle_data = if reader.is_empty() {
            Default::default()
        } else {
            <HistoricalOracleData>::deserialize(&mut reader)?
        };
        let base_asset_amount_per_lp: i128 = crate::borsh_de_or_default(&mut reader)?;
        let quote_asset_amount_per_lp: i128 = crate::borsh_de_or_default(&mut reader)?;
        let fee_pool = if reader.is_empty() {
            Default::default()
        } else {
            <PoolBalance>::deserialize(&mut reader)?
        };
        let base_asset_reserve: u128 = crate::borsh_de_or_default(&mut reader)?;
        let quote_asset_reserve: u128 = crate::borsh_de_or_default(&mut reader)?;
        let concentration_coef: u128 = crate::borsh_de_or_default(&mut reader)?;
        let min_base_asset_reserve: u128 = crate::borsh_de_or_default(&mut reader)?;
        let max_base_asset_reserve: u128 = crate::borsh_de_or_default(&mut reader)?;
        let sqrt_k: u128 = crate::borsh_de_or_default(&mut reader)?;
        let peg_multiplier: u128 = crate::borsh_de_or_default(&mut reader)?;
        let terminal_quote_asset_reserve: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let base_asset_amount_long: i128 = crate::borsh_de_or_default(&mut reader)?;
        let base_asset_amount_short: i128 = crate::borsh_de_or_default(&mut reader)?;
        let base_asset_amount_with_amm: i128 = crate::borsh_de_or_default(&mut reader)?;
        let base_asset_amount_with_unsettled_lp: i128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let max_open_interest: u128 = crate::borsh_de_or_default(&mut reader)?;
        let quote_asset_amount: i128 = crate::borsh_de_or_default(&mut reader)?;
        let quote_entry_amount_long: i128 = crate::borsh_de_or_default(&mut reader)?;
        let quote_entry_amount_short: i128 = crate::borsh_de_or_default(&mut reader)?;
        let quote_break_even_amount_long: i128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let quote_break_even_amount_short: i128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let user_lp_shares: u128 = crate::borsh_de_or_default(&mut reader)?;
        let last_funding_rate: i64 = crate::borsh_de_or_default(&mut reader)?;
        let last_funding_rate_long: i64 = crate::borsh_de_or_default(&mut reader)?;
        let last_funding_rate_short: i64 = crate::borsh_de_or_default(&mut reader)?;
        let last24h_avg_funding_rate: i64 = crate::borsh_de_or_default(&mut reader)?;
        let total_fee: i128 = crate::borsh_de_or_default(&mut reader)?;
        let total_mm_fee: i128 = crate::borsh_de_or_default(&mut reader)?;
        let total_exchange_fee: u128 = crate::borsh_de_or_default(&mut reader)?;
        let total_fee_minus_distributions: i128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let total_fee_withdrawn: u128 = crate::borsh_de_or_default(&mut reader)?;
        let total_liquidation_fee: u128 = crate::borsh_de_or_default(&mut reader)?;
        let cumulative_funding_rate_long: i128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let cumulative_funding_rate_short: i128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let total_social_loss: u128 = crate::borsh_de_or_default(&mut reader)?;
        let ask_base_asset_reserve: u128 = crate::borsh_de_or_default(&mut reader)?;
        let ask_quote_asset_reserve: u128 = crate::borsh_de_or_default(&mut reader)?;
        let bid_base_asset_reserve: u128 = crate::borsh_de_or_default(&mut reader)?;
        let bid_quote_asset_reserve: u128 = crate::borsh_de_or_default(&mut reader)?;
        let last_oracle_normalised_price: i64 = crate::borsh_de_or_default(&mut reader)?;
        let last_oracle_reserve_price_spread_pct: i64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let last_bid_price_twap: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_ask_price_twap: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_mark_price_twap: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_mark_price_twap5min: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_update_slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_oracle_conf_pct: u64 = crate::borsh_de_or_default(&mut reader)?;
        let net_revenue_since_last_funding: i64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let last_funding_rate_ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let funding_period: i64 = crate::borsh_de_or_default(&mut reader)?;
        let order_step_size: u64 = crate::borsh_de_or_default(&mut reader)?;
        let order_tick_size: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_order_size: u64 = crate::borsh_de_or_default(&mut reader)?;
        let mm_oracle_slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        let volume24h: u64 = crate::borsh_de_or_default(&mut reader)?;
        let long_intensity_volume: u64 = crate::borsh_de_or_default(&mut reader)?;
        let short_intensity_volume: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_trade_ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let mark_std: u64 = crate::borsh_de_or_default(&mut reader)?;
        let oracle_std: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_mark_price_twap_ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let base_spread: u32 = crate::borsh_de_or_default(&mut reader)?;
        let max_spread: u32 = crate::borsh_de_or_default(&mut reader)?;
        let long_spread: u32 = crate::borsh_de_or_default(&mut reader)?;
        let short_spread: u32 = crate::borsh_de_or_default(&mut reader)?;
        let mm_oracle_price: i64 = crate::borsh_de_or_default(&mut reader)?;
        let max_fill_reserve_fraction: u16 = crate::borsh_de_or_default(&mut reader)?;
        let max_slippage_ratio: u16 = crate::borsh_de_or_default(&mut reader)?;
        let curve_update_intensity: u8 = crate::borsh_de_or_default(&mut reader)?;
        let amm_jit_intensity: u8 = crate::borsh_de_or_default(&mut reader)?;
        let oracle_source: OracleSource = crate::borsh_de_or_default(&mut reader)?;
        let last_oracle_valid: bool = crate::borsh_de_or_default(&mut reader)?;
        let target_base_asset_amount_per_lp: i32 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let per_lp_base: i8 = crate::borsh_de_or_default(&mut reader)?;
        let oracle_low_risk_slot_delay_override: i8 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let amm_spread_adjustment: i8 = crate::borsh_de_or_default(&mut reader)?;
        let oracle_slot_delay_override: i8 = crate::borsh_de_or_default(&mut reader)?;
        let mm_oracle_sequence_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        let net_unsettled_funding_pnl: i64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_asset_amount_with_unsettled_lp: i64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let reference_price_offset: i32 = crate::borsh_de_or_default(&mut reader)?;
        let amm_inventory_spread_adjustment: i8 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let reference_price_offset_deadband_pct: u8 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let padding: [u8; 2] = crate::borsh_de_or_default(&mut reader)?;
        let last_funding_oracle_twap: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            oracle,
            historical_oracle_data,
            base_asset_amount_per_lp,
            quote_asset_amount_per_lp,
            fee_pool,
            base_asset_reserve,
            quote_asset_reserve,
            concentration_coef,
            min_base_asset_reserve,
            max_base_asset_reserve,
            sqrt_k,
            peg_multiplier,
            terminal_quote_asset_reserve,
            base_asset_amount_long,
            base_asset_amount_short,
            base_asset_amount_with_amm,
            base_asset_amount_with_unsettled_lp,
            max_open_interest,
            quote_asset_amount,
            quote_entry_amount_long,
            quote_entry_amount_short,
            quote_break_even_amount_long,
            quote_break_even_amount_short,
            user_lp_shares,
            last_funding_rate,
            last_funding_rate_long,
            last_funding_rate_short,
            last24h_avg_funding_rate,
            total_fee,
            total_mm_fee,
            total_exchange_fee,
            total_fee_minus_distributions,
            total_fee_withdrawn,
            total_liquidation_fee,
            cumulative_funding_rate_long,
            cumulative_funding_rate_short,
            total_social_loss,
            ask_base_asset_reserve,
            ask_quote_asset_reserve,
            bid_base_asset_reserve,
            bid_quote_asset_reserve,
            last_oracle_normalised_price,
            last_oracle_reserve_price_spread_pct,
            last_bid_price_twap,
            last_ask_price_twap,
            last_mark_price_twap,
            last_mark_price_twap5min,
            last_update_slot,
            last_oracle_conf_pct,
            net_revenue_since_last_funding,
            last_funding_rate_ts,
            funding_period,
            order_step_size,
            order_tick_size,
            min_order_size,
            mm_oracle_slot,
            volume24h,
            long_intensity_volume,
            short_intensity_volume,
            last_trade_ts,
            mark_std,
            oracle_std,
            last_mark_price_twap_ts,
            base_spread,
            max_spread,
            long_spread,
            short_spread,
            mm_oracle_price,
            max_fill_reserve_fraction,
            max_slippage_ratio,
            curve_update_intensity,
            amm_jit_intensity,
            oracle_source,
            last_oracle_valid,
            target_base_asset_amount_per_lp,
            per_lp_base,
            oracle_low_risk_slot_delay_override,
            amm_spread_adjustment,
            oracle_slot_delay_override,
            mm_oracle_sequence_id,
            net_unsettled_funding_pnl,
            quote_asset_amount_with_unsettled_lp,
            reference_price_offset,
            amm_inventory_spread_adjustment,
            reference_price_offset_deadband_pct,
            padding,
            last_funding_oracle_twap,
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
pub struct RevenueShareOrder {
    pub fees_accrued: u64,
    pub order_id: u32,
    pub fee_tenth_bps: u16,
    pub market_index: u16,
    pub sub_account_id: u16,
    pub builder_idx: u8,
    pub bit_flags: u8,
    pub user_order_index: u8,
    pub market_type: MarketType,
    pub padding: [u8; 10],
}
impl RevenueShareOrder {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let fees_accrued: u64 = crate::borsh_de_or_default(&mut reader)?;
        let order_id: u32 = crate::borsh_de_or_default(&mut reader)?;
        let fee_tenth_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let sub_account_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let builder_idx: u8 = crate::borsh_de_or_default(&mut reader)?;
        let bit_flags: u8 = crate::borsh_de_or_default(&mut reader)?;
        let user_order_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let market_type: MarketType = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 10] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            fees_accrued,
            order_id,
            fee_tenth_bps,
            market_index,
            sub_account_id,
            builder_idx,
            bit_flags,
            user_order_index,
            market_type,
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
pub struct BuilderInfo {
    pub authority: Pubkey,
    pub max_fee_tenth_bps: u16,
    pub padding: [u8; 6],
}
impl BuilderInfo {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let max_fee_tenth_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 6] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            authority,
            max_fee_tenth_bps,
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
pub struct RevenueShareEscrowFixed {
    pub authority: Pubkey,
    pub referrer: Pubkey,
    pub referrer_boost_expire_ts: u32,
    pub referrer_reward_offset: i8,
    pub referee_fee_numerator_offset: i8,
    pub referrer_boost_numerator: i8,
    pub reserved_fixed: [u8; 17],
}
impl RevenueShareEscrowFixed {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let referrer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let referrer_boost_expire_ts: u32 = crate::borsh_de_or_default(&mut reader)?;
        let referrer_reward_offset: i8 = crate::borsh_de_or_default(&mut reader)?;
        let referee_fee_numerator_offset: i8 = crate::borsh_de_or_default(&mut reader)?;
        let referrer_boost_numerator: i8 = crate::borsh_de_or_default(&mut reader)?;
        let reserved_fixed: [u8; 17] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            authority,
            referrer,
            referrer_boost_expire_ts,
            referrer_reward_offset,
            referee_fee_numerator_offset,
            referrer_boost_numerator,
            reserved_fixed,
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
pub struct ScaleOrderParams {
    pub market_type: MarketType,
    pub direction: PositionDirection,
    pub market_index: u16,
    pub total_base_asset_amount: u64,
    pub start_price: u64,
    pub end_price: u64,
    pub order_count: u8,
    pub size_distribution: SizeDistribution,
    pub reduce_only: bool,
    pub post_only: PostOnlyParam,
    pub bit_flags: u8,
    pub max_ts: Option<i64>,
}
impl ScaleOrderParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let market_type: MarketType = crate::borsh_de_or_default(&mut reader)?;
        let direction: PositionDirection = crate::borsh_de_or_default(&mut reader)?;
        let market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let total_base_asset_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let start_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let end_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let order_count: u8 = crate::borsh_de_or_default(&mut reader)?;
        let size_distribution: SizeDistribution = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let reduce_only: bool = crate::borsh_de_or_default(&mut reader)?;
        let post_only: PostOnlyParam = crate::borsh_de_or_default(&mut reader)?;
        let bit_flags: u8 = crate::borsh_de_or_default(&mut reader)?;
        let max_ts: Option<i64> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            market_type,
            direction,
            market_index,
            total_base_asset_amount,
            start_price,
            end_price,
            order_count,
            size_distribution,
            reduce_only,
            post_only,
            bit_flags,
            max_ts,
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
pub struct SignedMsgOrderId {
    pub uuid: [u8; 8],
    pub max_slot: u64,
    pub order_id: u32,
    pub padding: u32,
}
impl SignedMsgOrderId {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let uuid: [u8; 8] = crate::borsh_de_or_default(&mut reader)?;
        let max_slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        let order_id: u32 = crate::borsh_de_or_default(&mut reader)?;
        let padding: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            uuid,
            max_slot,
            order_id,
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
pub struct SignedMsgUserOrdersFixed {
    pub user_pubkey: Pubkey,
    pub padding: u32,
    pub len: u32,
}
impl SignedMsgUserOrdersFixed {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let user_pubkey: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let padding: u32 = crate::borsh_de_or_default(&mut reader)?;
        let len: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { user_pubkey, padding, len })
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
pub struct InsuranceFund {
    pub vault: Pubkey,
    pub total_shares: u128,
    pub user_shares: u128,
    pub shares_base: u128,
    pub unstaking_period: i64,
    pub last_revenue_settle_ts: i64,
    pub revenue_settle_period: i64,
    pub total_factor: u32,
    pub user_factor: u32,
}
impl InsuranceFund {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let total_shares: u128 = crate::borsh_de_or_default(&mut reader)?;
        let user_shares: u128 = crate::borsh_de_or_default(&mut reader)?;
        let shares_base: u128 = crate::borsh_de_or_default(&mut reader)?;
        let unstaking_period: i64 = crate::borsh_de_or_default(&mut reader)?;
        let last_revenue_settle_ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let revenue_settle_period: i64 = crate::borsh_de_or_default(&mut reader)?;
        let total_factor: u32 = crate::borsh_de_or_default(&mut reader)?;
        let user_factor: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            vault,
            total_shares,
            user_shares,
            shares_base,
            unstaking_period,
            last_revenue_settle_ts,
            revenue_settle_period,
            total_factor,
            user_factor,
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
pub struct OracleGuardRails {
    pub price_divergence: PriceDivergenceGuardRails,
    pub validity: ValidityGuardRails,
}
impl OracleGuardRails {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let price_divergence = if reader.is_empty() {
            Default::default()
        } else {
            <PriceDivergenceGuardRails>::deserialize(&mut reader)?
        };
        let validity = if reader.is_empty() {
            Default::default()
        } else {
            <ValidityGuardRails>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self { price_divergence, validity })
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
pub struct PriceDivergenceGuardRails {
    pub mark_oracle_percent_divergence: u64,
    pub oracle_twap5min_percent_divergence: u64,
}
impl PriceDivergenceGuardRails {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mark_oracle_percent_divergence: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let oracle_twap5min_percent_divergence: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            mark_oracle_percent_divergence,
            oracle_twap5min_percent_divergence,
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
pub struct ValidityGuardRails {
    pub slots_before_stale_for_amm: i64,
    pub slots_before_stale_for_margin: i64,
    pub confidence_interval_max_size: u64,
    pub too_volatile_ratio: i64,
}
impl ValidityGuardRails {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let slots_before_stale_for_amm: i64 = crate::borsh_de_or_default(&mut reader)?;
        let slots_before_stale_for_margin: i64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let confidence_interval_max_size: u64 = crate::borsh_de_or_default(&mut reader)?;
        let too_volatile_ratio: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            slots_before_stale_for_amm,
            slots_before_stale_for_margin,
            confidence_interval_max_size,
            too_volatile_ratio,
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
pub struct FeeStructure {
    pub fee_tiers: [FeeTier; 10],
    pub filler_reward_structure: OrderFillerRewardStructure,
    pub referrer_reward_epoch_upper_bound: u64,
    pub flat_filler_fee: u64,
}
impl FeeStructure {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let fee_tiers: [FeeTier; 10] = crate::borsh_de_or_default(&mut reader)?;
        let filler_reward_structure = if reader.is_empty() {
            Default::default()
        } else {
            <OrderFillerRewardStructure>::deserialize(&mut reader)?
        };
        let referrer_reward_epoch_upper_bound: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let flat_filler_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            fee_tiers,
            filler_reward_structure,
            referrer_reward_epoch_upper_bound,
            flat_filler_fee,
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
pub struct FeeTier {
    pub fee_numerator: u32,
    pub fee_denominator: u32,
    pub maker_rebate_numerator: u32,
    pub maker_rebate_denominator: u32,
    pub referrer_reward_numerator: u32,
    pub referrer_reward_denominator: u32,
    pub referee_fee_numerator: u32,
    pub referee_fee_denominator: u32,
}
impl FeeTier {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let fee_numerator: u32 = crate::borsh_de_or_default(&mut reader)?;
        let fee_denominator: u32 = crate::borsh_de_or_default(&mut reader)?;
        let maker_rebate_numerator: u32 = crate::borsh_de_or_default(&mut reader)?;
        let maker_rebate_denominator: u32 = crate::borsh_de_or_default(&mut reader)?;
        let referrer_reward_numerator: u32 = crate::borsh_de_or_default(&mut reader)?;
        let referrer_reward_denominator: u32 = crate::borsh_de_or_default(&mut reader)?;
        let referee_fee_numerator: u32 = crate::borsh_de_or_default(&mut reader)?;
        let referee_fee_denominator: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            fee_numerator,
            fee_denominator,
            maker_rebate_numerator,
            maker_rebate_denominator,
            referrer_reward_numerator,
            referrer_reward_denominator,
            referee_fee_numerator,
            referee_fee_denominator,
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
pub struct OrderFillerRewardStructure {
    pub reward_numerator: u32,
    pub reward_denominator: u32,
    pub time_based_reward_lower_bound: u128,
}
impl OrderFillerRewardStructure {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let reward_numerator: u32 = crate::borsh_de_or_default(&mut reader)?;
        let reward_denominator: u32 = crate::borsh_de_or_default(&mut reader)?;
        let time_based_reward_lower_bound: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            reward_numerator,
            reward_denominator,
            time_based_reward_lower_bound,
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
pub struct UserFees {
    pub total_fee_paid: u64,
    pub total_fee_rebate: u64,
    pub total_token_discount: u64,
    pub total_referee_discount: u64,
    pub total_referrer_reward: u64,
    pub current_epoch_referrer_reward: u64,
}
impl UserFees {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let total_fee_paid: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_fee_rebate: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_token_discount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_referee_discount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_referrer_reward: u64 = crate::borsh_de_or_default(&mut reader)?;
        let current_epoch_referrer_reward: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            total_fee_paid,
            total_fee_rebate,
            total_token_discount,
            total_referee_discount,
            total_referrer_reward,
            current_epoch_referrer_reward,
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
pub struct SpotPosition {
    pub scaled_balance: u64,
    pub open_bids: i64,
    pub open_asks: i64,
    pub cumulative_deposits: i64,
    pub market_index: u16,
    pub balance_type: SpotBalanceType,
    pub open_orders: u8,
    pub padding: [u8; 4],
}
impl SpotPosition {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let scaled_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let open_bids: i64 = crate::borsh_de_or_default(&mut reader)?;
        let open_asks: i64 = crate::borsh_de_or_default(&mut reader)?;
        let cumulative_deposits: i64 = crate::borsh_de_or_default(&mut reader)?;
        let market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let balance_type: SpotBalanceType = crate::borsh_de_or_default(&mut reader)?;
        let open_orders: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 4] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            scaled_balance,
            open_bids,
            open_asks,
            cumulative_deposits,
            market_index,
            balance_type,
            open_orders,
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
pub struct PerpPosition {
    pub last_cumulative_funding_rate: i64,
    pub base_asset_amount: i64,
    pub quote_asset_amount: i64,
    pub quote_break_even_amount: i64,
    pub quote_entry_amount: i64,
    pub open_bids: i64,
    pub open_asks: i64,
    pub settled_pnl: i64,
    pub lp_shares: u64,
    pub isolated_position_scaled_balance: u64,
    pub last_quote_asset_amount_per_lp: i64,
    pub padding: [u8; 2],
    pub max_margin_ratio: u16,
    pub market_index: u16,
    pub open_orders: u8,
    pub position_flag: u8,
}
impl PerpPosition {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let last_cumulative_funding_rate: i64 = crate::borsh_de_or_default(&mut reader)?;
        let base_asset_amount: i64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_asset_amount: i64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_break_even_amount: i64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_entry_amount: i64 = crate::borsh_de_or_default(&mut reader)?;
        let open_bids: i64 = crate::borsh_de_or_default(&mut reader)?;
        let open_asks: i64 = crate::borsh_de_or_default(&mut reader)?;
        let settled_pnl: i64 = crate::borsh_de_or_default(&mut reader)?;
        let lp_shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        let isolated_position_scaled_balance: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let last_quote_asset_amount_per_lp: i64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let padding: [u8; 2] = crate::borsh_de_or_default(&mut reader)?;
        let max_margin_ratio: u16 = crate::borsh_de_or_default(&mut reader)?;
        let market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let open_orders: u8 = crate::borsh_de_or_default(&mut reader)?;
        let position_flag: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            last_cumulative_funding_rate,
            base_asset_amount,
            quote_asset_amount,
            quote_break_even_amount,
            quote_entry_amount,
            open_bids,
            open_asks,
            settled_pnl,
            lp_shares,
            isolated_position_scaled_balance,
            last_quote_asset_amount_per_lp,
            padding,
            max_margin_ratio,
            market_index,
            open_orders,
            position_flag,
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
pub struct Order {
    pub slot: u64,
    pub price: u64,
    pub base_asset_amount: u64,
    pub base_asset_amount_filled: u64,
    pub quote_asset_amount_filled: u64,
    pub trigger_price: u64,
    pub auction_start_price: i64,
    pub auction_end_price: i64,
    pub max_ts: i64,
    pub oracle_price_offset: i32,
    pub order_id: u32,
    pub market_index: u16,
    pub status: OrderStatus,
    pub order_type: OrderType,
    pub market_type: MarketType,
    pub user_order_id: u8,
    pub existing_position_direction: PositionDirection,
    pub direction: PositionDirection,
    pub reduce_only: bool,
    pub post_only: bool,
    pub immediate_or_cancel: bool,
    pub trigger_condition: OrderTriggerCondition,
    pub auction_duration: u8,
    pub posted_slot_tail: u8,
    pub bit_flags: u8,
    pub padding: [u8; 1],
}
impl Order {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        let price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let base_asset_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let base_asset_amount_filled: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_asset_amount_filled: u64 = crate::borsh_de_or_default(&mut reader)?;
        let trigger_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let auction_start_price: i64 = crate::borsh_de_or_default(&mut reader)?;
        let auction_end_price: i64 = crate::borsh_de_or_default(&mut reader)?;
        let max_ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let oracle_price_offset: i32 = crate::borsh_de_or_default(&mut reader)?;
        let order_id: u32 = crate::borsh_de_or_default(&mut reader)?;
        let market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let status: OrderStatus = crate::borsh_de_or_default(&mut reader)?;
        let order_type: OrderType = crate::borsh_de_or_default(&mut reader)?;
        let market_type: MarketType = crate::borsh_de_or_default(&mut reader)?;
        let user_order_id: u8 = crate::borsh_de_or_default(&mut reader)?;
        let existing_position_direction: PositionDirection = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let direction: PositionDirection = crate::borsh_de_or_default(&mut reader)?;
        let reduce_only: bool = crate::borsh_de_or_default(&mut reader)?;
        let post_only: bool = crate::borsh_de_or_default(&mut reader)?;
        let immediate_or_cancel: bool = crate::borsh_de_or_default(&mut reader)?;
        let trigger_condition: OrderTriggerCondition = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let auction_duration: u8 = crate::borsh_de_or_default(&mut reader)?;
        let posted_slot_tail: u8 = crate::borsh_de_or_default(&mut reader)?;
        let bit_flags: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 1] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            slot,
            price,
            base_asset_amount,
            base_asset_amount_filled,
            quote_asset_amount_filled,
            trigger_price,
            auction_start_price,
            auction_end_price,
            max_ts,
            oracle_price_offset,
            order_id,
            market_index,
            status,
            order_type,
            market_type,
            user_order_id,
            existing_position_direction,
            direction,
            reduce_only,
            post_only,
            immediate_or_cancel,
            trigger_condition,
            auction_duration,
            posted_slot_tail,
            bit_flags,
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
pub enum SwapDirection {
    #[default]
    Add,
    Remove,
}
impl TryFrom<u8> for SwapDirection {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Add),
            1u8 => Ok(Self::Remove),
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
pub enum ModifyOrderId {
    UserOrderId(u8),
    OrderId(u32),
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
pub enum PositionDirection {
    #[default]
    Long,
    Short,
}
impl TryFrom<u8> for PositionDirection {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Long),
            1u8 => Ok(Self::Short),
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
pub enum SpotFulfillmentType {
    #[default]
    SerumV3,
    Match,
    PhoenixV1,
    OpenbookV2,
}
impl TryFrom<u8> for SpotFulfillmentType {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::SerumV3),
            1u8 => Ok(Self::Match),
            2u8 => Ok(Self::PhoenixV1),
            3u8 => Ok(Self::OpenbookV2),
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
pub enum SwapReduceOnly {
    #[default]
    In,
    Out,
}
impl TryFrom<u8> for SwapReduceOnly {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::In),
            1u8 => Ok(Self::Out),
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
pub enum TwapPeriod {
    #[default]
    FundingPeriod,
    FiveMin,
}
impl TryFrom<u8> for TwapPeriod {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::FundingPeriod),
            1u8 => Ok(Self::FiveMin),
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
pub enum LiquidationMultiplierType {
    #[default]
    Discount,
    Premium,
}
impl TryFrom<u8> for LiquidationMultiplierType {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Discount),
            1u8 => Ok(Self::Premium),
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
pub enum SettlementDirection {
    #[default]
    ToLpPool,
    FromLpPool,
    None,
}
impl TryFrom<u8> for SettlementDirection {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::ToLpPool),
            1u8 => Ok(Self::FromLpPool),
            2u8 => Ok(Self::None),
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
pub enum MarginRequirementType {
    #[default]
    Initial,
    Fill,
    Maintenance,
}
impl TryFrom<u8> for MarginRequirementType {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Initial),
            1u8 => Ok(Self::Fill),
            2u8 => Ok(Self::Maintenance),
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
pub enum OracleValidity {
    #[default]
    NonPositive,
    TooVolatile,
    TooUncertain,
    StaleForMargin,
    InsufficientDataPoints,
    StaleForAmm { immediate: bool, low_risk: bool },
    Valid,
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
pub enum DriftAction {
    #[default]
    UpdateFunding,
    SettlePnl,
    TriggerOrder,
    FillOrderMatch,
    FillOrderAmmLowRisk,
    FillOrderAmmImmediate,
    Liquidate,
    MarginCalc,
    UpdateTwap,
    UpdateAmmCurve,
    OracleOrderPrice,
    UseMmOraclePrice,
    UpdateAmmCache,
    UpdateLpPoolAum,
    LpPoolSwap,
}
impl TryFrom<u8> for DriftAction {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::UpdateFunding),
            1u8 => Ok(Self::SettlePnl),
            2u8 => Ok(Self::TriggerOrder),
            3u8 => Ok(Self::FillOrderMatch),
            4u8 => Ok(Self::FillOrderAmmLowRisk),
            5u8 => Ok(Self::FillOrderAmmImmediate),
            6u8 => Ok(Self::Liquidate),
            7u8 => Ok(Self::MarginCalc),
            8u8 => Ok(Self::UpdateTwap),
            9u8 => Ok(Self::UpdateAmmCurve),
            10u8 => Ok(Self::OracleOrderPrice),
            11u8 => Ok(Self::UseMmOraclePrice),
            12u8 => Ok(Self::UpdateAmmCache),
            13u8 => Ok(Self::UpdateLpPoolAum),
            14u8 => Ok(Self::LpPoolSwap),
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
pub enum LogMode {
    #[default]
    None,
    ExchangeOracle,
    MmOracle,
    SafeMmOracle,
    Margin,
}
impl TryFrom<u8> for LogMode {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::None),
            1u8 => Ok(Self::ExchangeOracle),
            2u8 => Ok(Self::MmOracle),
            3u8 => Ok(Self::SafeMmOracle),
            4u8 => Ok(Self::Margin),
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
pub enum PositionUpdateType {
    #[default]
    Open,
    Increase,
    Reduce,
    Close,
    Flip,
}
impl TryFrom<u8> for PositionUpdateType {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Open),
            1u8 => Ok(Self::Increase),
            2u8 => Ok(Self::Reduce),
            3u8 => Ok(Self::Close),
            4u8 => Ok(Self::Flip),
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
pub enum DepositExplanation {
    #[default]
    None,
    Transfer,
    Borrow,
    RepayBorrow,
    Reward,
}
impl TryFrom<u8> for DepositExplanation {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::None),
            1u8 => Ok(Self::Transfer),
            2u8 => Ok(Self::Borrow),
            3u8 => Ok(Self::RepayBorrow),
            4u8 => Ok(Self::Reward),
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
pub enum DepositDirection {
    #[default]
    Deposit,
    Withdraw,
}
impl TryFrom<u8> for DepositDirection {
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
pub enum OrderAction {
    #[default]
    Place,
    Cancel,
    Fill,
    Trigger,
    Expire,
}
impl TryFrom<u8> for OrderAction {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Place),
            1u8 => Ok(Self::Cancel),
            2u8 => Ok(Self::Fill),
            3u8 => Ok(Self::Trigger),
            4u8 => Ok(Self::Expire),
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
pub enum OrderActionExplanation {
    #[default]
    None,
    InsufficientFreeCollateral,
    OraclePriceBreachedLimitPrice,
    MarketOrderFilledToLimitPrice,
    OrderExpired,
    Liquidation,
    OrderFilledWithAmm,
    OrderFilledWithAmmJit,
    OrderFilledWithMatch,
    OrderFilledWithMatchJit,
    MarketExpired,
    RiskingIncreasingOrder,
    ReduceOnlyOrderIncreasedPosition,
    OrderFillWithSerum,
    NoBorrowLiquidity,
    OrderFillWithPhoenix,
    OrderFilledWithAmmJitLpSplit,
    OrderFilledWithLpJit,
    DeriskLp,
    OrderFilledWithOpenbookV2,
    TransferPerpPosition,
}
impl TryFrom<u8> for OrderActionExplanation {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::None),
            1u8 => Ok(Self::InsufficientFreeCollateral),
            2u8 => Ok(Self::OraclePriceBreachedLimitPrice),
            3u8 => Ok(Self::MarketOrderFilledToLimitPrice),
            4u8 => Ok(Self::OrderExpired),
            5u8 => Ok(Self::Liquidation),
            6u8 => Ok(Self::OrderFilledWithAmm),
            7u8 => Ok(Self::OrderFilledWithAmmJit),
            8u8 => Ok(Self::OrderFilledWithMatch),
            9u8 => Ok(Self::OrderFilledWithMatchJit),
            10u8 => Ok(Self::MarketExpired),
            11u8 => Ok(Self::RiskingIncreasingOrder),
            12u8 => Ok(Self::ReduceOnlyOrderIncreasedPosition),
            13u8 => Ok(Self::OrderFillWithSerum),
            14u8 => Ok(Self::NoBorrowLiquidity),
            15u8 => Ok(Self::OrderFillWithPhoenix),
            16u8 => Ok(Self::OrderFilledWithAmmJitLpSplit),
            17u8 => Ok(Self::OrderFilledWithLpJit),
            18u8 => Ok(Self::DeriskLp),
            19u8 => Ok(Self::OrderFilledWithOpenbookV2),
            20u8 => Ok(Self::TransferPerpPosition),
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
pub enum LPAction {
    #[default]
    AddLiquidity,
    RemoveLiquidity,
    SettleLiquidity,
    RemoveLiquidityDerisk,
}
impl TryFrom<u8> for LPAction {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::AddLiquidity),
            1u8 => Ok(Self::RemoveLiquidity),
            2u8 => Ok(Self::SettleLiquidity),
            3u8 => Ok(Self::RemoveLiquidityDerisk),
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
pub enum LiquidationType {
    #[default]
    LiquidatePerp,
    LiquidateSpot,
    LiquidateBorrowForPerpPnl,
    LiquidatePerpPnlForDeposit,
    PerpBankruptcy,
    SpotBankruptcy,
}
impl TryFrom<u8> for LiquidationType {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::LiquidatePerp),
            1u8 => Ok(Self::LiquidateSpot),
            2u8 => Ok(Self::LiquidateBorrowForPerpPnl),
            3u8 => Ok(Self::LiquidatePerpPnlForDeposit),
            4u8 => Ok(Self::PerpBankruptcy),
            5u8 => Ok(Self::SpotBankruptcy),
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
pub enum LiquidationBitFlag {
    #[default]
    IsolatedPosition,
}
impl TryFrom<u8> for LiquidationBitFlag {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::IsolatedPosition),
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
pub enum SettlePnlExplanation {
    #[default]
    None,
    ExpiredPosition,
}
impl TryFrom<u8> for SettlePnlExplanation {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::None),
            1u8 => Ok(Self::ExpiredPosition),
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
pub enum StakeAction {
    #[default]
    Stake,
    UnstakeRequest,
    UnstakeCancelRequest,
    Unstake,
    UnstakeTransfer,
    StakeTransfer,
    AdminDeposit,
}
impl TryFrom<u8> for StakeAction {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Stake),
            1u8 => Ok(Self::UnstakeRequest),
            2u8 => Ok(Self::UnstakeCancelRequest),
            3u8 => Ok(Self::Unstake),
            4u8 => Ok(Self::UnstakeTransfer),
            5u8 => Ok(Self::StakeTransfer),
            6u8 => Ok(Self::AdminDeposit),
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
pub enum FillMode {
    #[default]
    Fill,
    PlaceAndMake,
    PlaceAndTake(bool, u8),
    Liquidation,
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
pub enum PerpFulfillmentMethod {
    Amm(Option<u64>),
    Match(Pubkey, u16, u64),
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
pub enum SpotFulfillmentMethod {
    #[default]
    ExternalMarket,
    Match(Pubkey, u16),
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
pub enum ConstituentStatus {
    #[default]
    ReduceOnly,
    Decommissioned,
}
impl TryFrom<u8> for ConstituentStatus {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::ReduceOnly),
            1u8 => Ok(Self::Decommissioned),
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
pub enum MarginCalculationMode {
    #[default]
    Standard,
    Liquidation { market_to_track_margin_requirement: Option<MarketIdentifier> },
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
pub enum MarginTypeConfig {
    Default(MarginRequirementType),
    IsolatedPositionOverride {
        market_index: u16,
        margin_requirement_type: MarginRequirementType,
        default_isolated_margin_requirement_type: MarginRequirementType,
        cross_margin_requirement_type: MarginRequirementType,
    },
    CrossMarginOverride {
        margin_requirement_type: MarginRequirementType,
        default_margin_requirement_type: MarginRequirementType,
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
pub enum OracleSource {
    #[default]
    Pyth,
    Switchboard,
    QuoteAsset,
    Pyth1K,
    Pyth1M,
    PythStableCoin,
    Prelaunch,
    PythPull,
    Pyth1KPull,
    Pyth1MPull,
    PythStableCoinPull,
    SwitchboardOnDemand,
    PythLazer,
    PythLazer1K,
    PythLazer1M,
    PythLazerStableCoin,
}
impl TryFrom<u8> for OracleSource {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Pyth),
            1u8 => Ok(Self::Switchboard),
            2u8 => Ok(Self::QuoteAsset),
            3u8 => Ok(Self::Pyth1K),
            4u8 => Ok(Self::Pyth1M),
            5u8 => Ok(Self::PythStableCoin),
            6u8 => Ok(Self::Prelaunch),
            7u8 => Ok(Self::PythPull),
            8u8 => Ok(Self::Pyth1KPull),
            9u8 => Ok(Self::Pyth1MPull),
            10u8 => Ok(Self::PythStableCoinPull),
            11u8 => Ok(Self::SwitchboardOnDemand),
            12u8 => Ok(Self::PythLazer),
            13u8 => Ok(Self::PythLazer1K),
            14u8 => Ok(Self::PythLazer1M),
            15u8 => Ok(Self::PythLazerStableCoin),
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
pub enum OrderParamsBitFlag {
    #[default]
    ImmediateOrCancel,
    UpdateHighLeverageMode,
}
impl TryFrom<u8> for OrderParamsBitFlag {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::ImmediateOrCancel),
            1u8 => Ok(Self::UpdateHighLeverageMode),
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
pub enum PostOnlyParam {
    #[default]
    None,
    MustPostOnly,
    TryPostOnly,
    Slide,
}
impl TryFrom<u8> for PostOnlyParam {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::None),
            1u8 => Ok(Self::MustPostOnly),
            2u8 => Ok(Self::TryPostOnly),
            3u8 => Ok(Self::Slide),
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
pub enum ModifyOrderPolicy {
    #[default]
    MustModify,
    ExcludePreviousFill,
}
impl TryFrom<u8> for ModifyOrderPolicy {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::MustModify),
            1u8 => Ok(Self::ExcludePreviousFill),
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
pub enum PlaceAndTakeOrderSuccessCondition {
    #[default]
    PartialFill,
    FullFill,
}
impl TryFrom<u8> for PlaceAndTakeOrderSuccessCondition {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::PartialFill),
            1u8 => Ok(Self::FullFill),
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
pub enum PerpOperation {
    #[default]
    UpdateFunding,
    AmmFill,
    Fill,
    SettlePnl,
    SettlePnlWithPosition,
    Liquidation,
    AmmImmediateFill,
    SettleRevPool,
}
impl TryFrom<u8> for PerpOperation {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::UpdateFunding),
            1u8 => Ok(Self::AmmFill),
            2u8 => Ok(Self::Fill),
            3u8 => Ok(Self::SettlePnl),
            4u8 => Ok(Self::SettlePnlWithPosition),
            5u8 => Ok(Self::Liquidation),
            6u8 => Ok(Self::AmmImmediateFill),
            7u8 => Ok(Self::SettleRevPool),
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
pub enum SpotOperation {
    #[default]
    UpdateCumulativeInterest,
    Fill,
    Deposit,
    Withdraw,
    Liquidation,
}
impl TryFrom<u8> for SpotOperation {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::UpdateCumulativeInterest),
            1u8 => Ok(Self::Fill),
            2u8 => Ok(Self::Deposit),
            3u8 => Ok(Self::Withdraw),
            4u8 => Ok(Self::Liquidation),
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
pub enum InsuranceFundOperation {
    #[default]
    Init,
    Add,
    RequestRemove,
    Remove,
}
impl TryFrom<u8> for InsuranceFundOperation {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Init),
            1u8 => Ok(Self::Add),
            2u8 => Ok(Self::RequestRemove),
            3u8 => Ok(Self::Remove),
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
pub enum PerpLpOperation {
    #[default]
    TrackAmmRevenue,
    SettleQuoteOwed,
}
impl TryFrom<u8> for PerpLpOperation {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::TrackAmmRevenue),
            1u8 => Ok(Self::SettleQuoteOwed),
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
pub enum ConstituentLpOperation {
    #[default]
    Swap,
    Deposit,
    Withdraw,
}
impl TryFrom<u8> for ConstituentLpOperation {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Swap),
            1u8 => Ok(Self::Deposit),
            2u8 => Ok(Self::Withdraw),
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
pub enum MarketStatus {
    #[default]
    Initialized,
    Active,
    FundingPaused,
    AmmPaused,
    FillPaused,
    WithdrawPaused,
    ReduceOnly,
    Settlement,
    Delisted,
}
impl TryFrom<u8> for MarketStatus {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Initialized),
            1u8 => Ok(Self::Active),
            2u8 => Ok(Self::FundingPaused),
            3u8 => Ok(Self::AmmPaused),
            4u8 => Ok(Self::FillPaused),
            5u8 => Ok(Self::WithdrawPaused),
            6u8 => Ok(Self::ReduceOnly),
            7u8 => Ok(Self::Settlement),
            8u8 => Ok(Self::Delisted),
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
pub enum LpStatus {
    #[default]
    Uncollateralized,
    Active,
    Decommissioning,
}
impl TryFrom<u8> for LpStatus {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Uncollateralized),
            1u8 => Ok(Self::Active),
            2u8 => Ok(Self::Decommissioning),
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
pub enum ContractType {
    #[default]
    Perpetual,
    Future,
    Prediction,
}
impl TryFrom<u8> for ContractType {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Perpetual),
            1u8 => Ok(Self::Future),
            2u8 => Ok(Self::Prediction),
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
pub enum ContractTier {
    #[default]
    A,
    B,
    C,
    Speculative,
    HighlySpeculative,
    Isolated,
}
impl TryFrom<u8> for ContractTier {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::A),
            1u8 => Ok(Self::B),
            2u8 => Ok(Self::C),
            3u8 => Ok(Self::Speculative),
            4u8 => Ok(Self::HighlySpeculative),
            5u8 => Ok(Self::Isolated),
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
pub enum MarketConfigFlag {
    #[default]
    DisableFormulaicKUpdate,
}
impl TryFrom<u8> for MarketConfigFlag {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::DisableFormulaicKUpdate),
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
pub enum RevenueShareOrderBitFlag {
    #[default]
    Init,
    Open,
    Completed,
    Referral,
}
impl TryFrom<u8> for RevenueShareOrderBitFlag {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Init),
            1u8 => Ok(Self::Open),
            2u8 => Ok(Self::Completed),
            3u8 => Ok(Self::Referral),
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
pub enum SizeDistribution {
    #[default]
    Flat,
    Ascending,
    Descending,
}
impl TryFrom<u8> for SizeDistribution {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Flat),
            1u8 => Ok(Self::Ascending),
            2u8 => Ok(Self::Descending),
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
pub enum SettlePnlMode {
    #[default]
    MustSettle,
    TrySettle,
}
impl TryFrom<u8> for SettlePnlMode {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::MustSettle),
            1u8 => Ok(Self::TrySettle),
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
pub enum SpotBalanceType {
    #[default]
    Deposit,
    Borrow,
}
impl TryFrom<u8> for SpotBalanceType {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Deposit),
            1u8 => Ok(Self::Borrow),
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
pub enum SpotFulfillmentConfigStatus {
    #[default]
    Enabled,
    Disabled,
}
impl TryFrom<u8> for SpotFulfillmentConfigStatus {
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
pub enum AssetTier {
    #[default]
    Collateral,
    Protected,
    Cross,
    Isolated,
    Unlisted,
}
impl TryFrom<u8> for AssetTier {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Collateral),
            1u8 => Ok(Self::Protected),
            2u8 => Ok(Self::Cross),
            3u8 => Ok(Self::Isolated),
            4u8 => Ok(Self::Unlisted),
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
pub enum TokenProgramFlag {
    #[default]
    Token2022,
    TransferHook,
}
impl TryFrom<u8> for TokenProgramFlag {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Token2022),
            1u8 => Ok(Self::TransferHook),
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
pub enum ExchangeStatus {
    #[default]
    DepositPaused,
    WithdrawPaused,
    AmmPaused,
    FillPaused,
    LiqPaused,
    FundingPaused,
    SettlePnlPaused,
    AmmImmediateFillPaused,
}
impl TryFrom<u8> for ExchangeStatus {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::DepositPaused),
            1u8 => Ok(Self::WithdrawPaused),
            2u8 => Ok(Self::AmmPaused),
            3u8 => Ok(Self::FillPaused),
            4u8 => Ok(Self::LiqPaused),
            5u8 => Ok(Self::FundingPaused),
            6u8 => Ok(Self::SettlePnlPaused),
            7u8 => Ok(Self::AmmImmediateFillPaused),
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
pub enum FeatureBitFlags {
    #[default]
    MmOracleUpdate,
    MedianTriggerPrice,
    BuilderCodes,
    BuilderReferral,
}
impl TryFrom<u8> for FeatureBitFlags {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::MmOracleUpdate),
            1u8 => Ok(Self::MedianTriggerPrice),
            2u8 => Ok(Self::BuilderCodes),
            3u8 => Ok(Self::BuilderReferral),
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
pub enum LpPoolFeatureBitFlags {
    #[default]
    SettleLpPool,
    SwapLpPool,
    MintRedeemLpPool,
}
impl TryFrom<u8> for LpPoolFeatureBitFlags {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::SettleLpPool),
            1u8 => Ok(Self::SwapLpPool),
            2u8 => Ok(Self::MintRedeemLpPool),
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
pub enum UserStatus {
    #[default]
    BeingLiquidated,
    Bankrupt,
    ReduceOnly,
    AdvancedLp,
    ProtectedMakerOrders,
}
impl TryFrom<u8> for UserStatus {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::BeingLiquidated),
            1u8 => Ok(Self::Bankrupt),
            2u8 => Ok(Self::ReduceOnly),
            3u8 => Ok(Self::AdvancedLp),
            4u8 => Ok(Self::ProtectedMakerOrders),
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
pub enum AssetType {
    #[default]
    Base,
    Quote,
}
impl TryFrom<u8> for AssetType {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Base),
            1u8 => Ok(Self::Quote),
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
pub enum OrderStatus {
    #[default]
    Init,
    Open,
    Filled,
    Canceled,
}
impl TryFrom<u8> for OrderStatus {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Init),
            1u8 => Ok(Self::Open),
            2u8 => Ok(Self::Filled),
            3u8 => Ok(Self::Canceled),
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
pub enum OrderType {
    #[default]
    Market,
    Limit,
    TriggerMarket,
    TriggerLimit,
    Oracle,
}
impl TryFrom<u8> for OrderType {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Market),
            1u8 => Ok(Self::Limit),
            2u8 => Ok(Self::TriggerMarket),
            3u8 => Ok(Self::TriggerLimit),
            4u8 => Ok(Self::Oracle),
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
pub enum OrderTriggerCondition {
    #[default]
    Above,
    Below,
    TriggeredAbove,
    TriggeredBelow,
}
impl TryFrom<u8> for OrderTriggerCondition {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Above),
            1u8 => Ok(Self::Below),
            2u8 => Ok(Self::TriggeredAbove),
            3u8 => Ok(Self::TriggeredBelow),
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
pub enum MarketType {
    #[default]
    Spot,
    Perp,
}
impl TryFrom<u8> for MarketType {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Spot),
            1u8 => Ok(Self::Perp),
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
pub enum OrderBitFlag {
    #[default]
    SignedMessage,
    OracleTriggerMarket,
    SafeTriggerOrder,
    NewTriggerReduceOnly,
    HasBuilder,
    IsIsolatedPosition,
}
impl TryFrom<u8> for OrderBitFlag {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::SignedMessage),
            1u8 => Ok(Self::OracleTriggerMarket),
            2u8 => Ok(Self::SafeTriggerOrder),
            3u8 => Ok(Self::NewTriggerReduceOnly),
            4u8 => Ok(Self::HasBuilder),
            5u8 => Ok(Self::IsIsolatedPosition),
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
pub enum PositionFlag {
    #[default]
    IsolatedPosition,
    BeingLiquidated,
    Bankrupt,
}
impl TryFrom<u8> for PositionFlag {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::IsolatedPosition),
            1u8 => Ok(Self::BeingLiquidated),
            2u8 => Ok(Self::Bankrupt),
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
pub enum ReferrerStatus {
    #[default]
    IsReferrer,
    IsReferred,
    BuilderReferral,
}
impl TryFrom<u8> for ReferrerStatus {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::IsReferrer),
            1u8 => Ok(Self::IsReferred),
            2u8 => Ok(Self::BuilderReferral),
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
pub enum MarginMode {
    #[default]
    Default,
    HighLeverage,
    HighLeverageMaintenance,
}
impl TryFrom<u8> for MarginMode {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Default),
            1u8 => Ok(Self::HighLeverage),
            2u8 => Ok(Self::HighLeverageMaintenance),
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
pub enum FuelOverflowStatus {
    #[default]
    Exists,
}
impl TryFrom<u8> for FuelOverflowStatus {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Exists),
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
pub enum UserStatsPausedOperations {
    #[default]
    UpdateBidAskTwap,
    AmmAtomicFill,
    AmmAtomicRiskIncreasingFill,
}
impl TryFrom<u8> for UserStatsPausedOperations {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::UpdateBidAskTwap),
            1u8 => Ok(Self::AmmAtomicFill),
            2u8 => Ok(Self::AmmAtomicRiskIncreasingFill),
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
pub enum SignatureVerificationError {
    #[default]
    InvalidEd25519InstructionProgramId,
    InvalidEd25519InstructionDataLength,
    InvalidSignatureIndex,
    InvalidSignatureOffset,
    InvalidPublicKeyOffset,
    InvalidMessageOffset,
    InvalidMessageDataSize,
    InvalidInstructionIndex,
    MessageOffsetOverflow,
    InvalidMessageHex,
    InvalidMessageData,
    LoadInstructionAtFailed,
}
impl TryFrom<u8> for SignatureVerificationError {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::InvalidEd25519InstructionProgramId),
            1u8 => Ok(Self::InvalidEd25519InstructionDataLength),
            2u8 => Ok(Self::InvalidSignatureIndex),
            3u8 => Ok(Self::InvalidSignatureOffset),
            4u8 => Ok(Self::InvalidPublicKeyOffset),
            5u8 => Ok(Self::InvalidMessageOffset),
            6u8 => Ok(Self::InvalidMessageDataSize),
            7u8 => Ok(Self::InvalidInstructionIndex),
            8u8 => Ok(Self::MessageOffsetOverflow),
            9u8 => Ok(Self::InvalidMessageHex),
            10u8 => Ok(Self::InvalidMessageData),
            11u8 => Ok(Self::LoadInstructionAtFailed),
            _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
        }
    }
}
