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
pub struct AddCustodyParams {
    pub is_stable: bool,
    pub oracle: OracleParams,
    pub pricing: PricingParams,
    pub permissions: Permissions,
    pub hourly_funding_dbps: u64,
    pub target_ratio_bps: u64,
    pub increase_position_bps: u64,
    pub decrease_position_bps: u64,
    pub doves_oracle: Pubkey,
    pub max_position_size_usd: u64,
    pub jump_rate: JumpRateState,
    pub price_impact_fee_factor: u64,
    pub price_impact_exponent: f32,
    pub delta_imbalance_threshold_decimal: u64,
    pub max_fee_bps: u64,
    pub doves_ag_oracle: Pubkey,
}
impl AddCustodyParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let is_stable: bool = crate::borsh_de_or_default(&mut reader)?;
        let oracle = if reader.is_empty() {
            Default::default()
        } else {
            <OracleParams>::deserialize(&mut reader)?
        };
        let pricing = if reader.is_empty() {
            Default::default()
        } else {
            <PricingParams>::deserialize(&mut reader)?
        };
        let permissions = if reader.is_empty() {
            Default::default()
        } else {
            <Permissions>::deserialize(&mut reader)?
        };
        let hourly_funding_dbps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let target_ratio_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let increase_position_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let decrease_position_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let doves_oracle: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let max_position_size_usd: u64 = crate::borsh_de_or_default(&mut reader)?;
        let jump_rate = if reader.is_empty() {
            Default::default()
        } else {
            <JumpRateState>::deserialize(&mut reader)?
        };
        let price_impact_fee_factor: u64 = crate::borsh_de_or_default(&mut reader)?;
        let price_impact_exponent: f32 = crate::borsh_de_or_default(&mut reader)?;
        let delta_imbalance_threshold_decimal: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let max_fee_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let doves_ag_oracle: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            is_stable,
            oracle,
            pricing,
            permissions,
            hourly_funding_dbps,
            target_ratio_bps,
            increase_position_bps,
            decrease_position_bps,
            doves_oracle,
            max_position_size_usd,
            jump_rate,
            price_impact_fee_factor,
            price_impact_exponent,
            delta_imbalance_threshold_decimal,
            max_fee_bps,
            doves_ag_oracle,
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
pub struct AddLiquidity2Params {
    pub token_amount_in: u64,
    pub min_lp_amount_out: u64,
    pub token_amount_pre_swap: Option<u64>,
}
impl AddLiquidity2Params {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let token_amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_lp_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_amount_pre_swap: Option<u64> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            token_amount_in,
            min_lp_amount_out,
            token_amount_pre_swap,
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
pub struct AddPoolParams {
    pub name: String,
    pub limit: Limit,
    pub fees: Fees,
    pub max_request_execution_sec: i64,
}
impl AddPoolParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        let limit = if reader.is_empty() {
            Default::default()
        } else {
            <Limit>::deserialize(&mut reader)?
        };
        let fees = if reader.is_empty() {
            Default::default()
        } else {
            <Fees>::deserialize(&mut reader)?
        };
        let max_request_execution_sec: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            name,
            limit,
            fees,
            max_request_execution_sec,
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
pub struct BorrowFromCustodyParams {
    pub amount: u64,
}
impl BorrowFromCustodyParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { amount })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct CloseBorrowPositionParams {}
impl CloseBorrowPositionParams {
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
pub struct CreateAndDelegateStakeAccountParams {
    pub stake_account_index: u64,
    pub stake_amount_lamports: u64,
}
impl CreateAndDelegateStakeAccountParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let stake_account_index: u64 = crate::borsh_de_or_default(&mut reader)?;
        let stake_amount_lamports: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            stake_account_index,
            stake_amount_lamports,
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
pub struct CreateDecreasePositionMarketRequestParams {
    pub collateral_usd_delta: u64,
    pub size_usd_delta: u64,
    pub price_slippage: u64,
    pub jupiter_minimum_out: Option<u64>,
    pub entire_position: Option<bool>,
    pub counter: u64,
}
impl CreateDecreasePositionMarketRequestParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let collateral_usd_delta: u64 = crate::borsh_de_or_default(&mut reader)?;
        let size_usd_delta: u64 = crate::borsh_de_or_default(&mut reader)?;
        let price_slippage: u64 = crate::borsh_de_or_default(&mut reader)?;
        let jupiter_minimum_out: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let entire_position: Option<bool> = crate::borsh_de_or_default(&mut reader)?;
        let counter: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            collateral_usd_delta,
            size_usd_delta,
            price_slippage,
            jupiter_minimum_out,
            entire_position,
            counter,
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
pub struct CreateDecreasePositionRequest2Params {
    pub collateral_usd_delta: u64,
    pub size_usd_delta: u64,
    pub request_type: RequestType,
    pub price_slippage: Option<u64>,
    pub jupiter_minimum_out: Option<u64>,
    pub trigger_price: Option<u64>,
    pub trigger_above_threshold: Option<bool>,
    pub entire_position: Option<bool>,
    pub counter: u64,
}
impl CreateDecreasePositionRequest2Params {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let collateral_usd_delta: u64 = crate::borsh_de_or_default(&mut reader)?;
        let size_usd_delta: u64 = crate::borsh_de_or_default(&mut reader)?;
        let request_type: RequestType = crate::borsh_de_or_default(&mut reader)?;
        let price_slippage: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let jupiter_minimum_out: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let trigger_price: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let trigger_above_threshold: Option<bool> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let entire_position: Option<bool> = crate::borsh_de_or_default(&mut reader)?;
        let counter: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            collateral_usd_delta,
            size_usd_delta,
            request_type,
            price_slippage,
            jupiter_minimum_out,
            trigger_price,
            trigger_above_threshold,
            entire_position,
            counter,
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
pub struct CreateIncreasePositionMarketRequestParams {
    pub size_usd_delta: u64,
    pub collateral_token_delta: u64,
    pub side: Side,
    pub price_slippage: u64,
    pub jupiter_minimum_out: Option<u64>,
    pub counter: u64,
}
impl CreateIncreasePositionMarketRequestParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let size_usd_delta: u64 = crate::borsh_de_or_default(&mut reader)?;
        let collateral_token_delta: u64 = crate::borsh_de_or_default(&mut reader)?;
        let side: Side = crate::borsh_de_or_default(&mut reader)?;
        let price_slippage: u64 = crate::borsh_de_or_default(&mut reader)?;
        let jupiter_minimum_out: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let counter: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            size_usd_delta,
            collateral_token_delta,
            side,
            price_slippage,
            jupiter_minimum_out,
            counter,
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
pub struct CreateTokenMetadataParams {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}
impl CreateTokenMetadataParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        let symbol: String = crate::borsh_de_or_default(&mut reader)?;
        let uri: String = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { name, symbol, uri })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct DecreasePosition4Params {}
impl DecreasePosition4Params {
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
pub struct DecreasePositionWithInternalSwapParams {}
impl DecreasePositionWithInternalSwapParams {
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
pub struct DecreasePositionWithTpslAndInternalSwapParams {}
impl DecreasePositionWithTpslAndInternalSwapParams {
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
pub struct DecreasePositionWithTpslParams {}
impl DecreasePositionWithTpslParams {
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
pub struct DepositParams {
    pub amount: u64,
}
impl DepositParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { amount })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct GetAddLiquidityAmountAndFee2Params {
    pub token_amount_in: u64,
}
impl GetAddLiquidityAmountAndFee2Params {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let token_amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { token_amount_in })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct GetAssetsUnderManagement2Params {
    pub mode: Option<PriceCalcMode>,
}
impl GetAssetsUnderManagement2Params {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mode: Option<PriceCalcMode> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { mode })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct GetRemoveLiquidityAmountAndFee2Params {
    pub lp_amount_in: u64,
}
impl GetRemoveLiquidityAmountAndFee2Params {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let lp_amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { lp_amount_in })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct IncreasePosition4Params {}
impl IncreasePosition4Params {
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
pub struct IncreasePositionPreSwapParams {}
impl IncreasePositionPreSwapParams {
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
pub struct IncreasePositionWithInternalSwapParams {}
impl IncreasePositionWithInternalSwapParams {
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
pub struct InitParams {
    pub allow_swap: bool,
    pub allow_add_liquidity: bool,
    pub allow_remove_liquidity: bool,
    pub allow_increase_position: bool,
    pub allow_decrease_position: bool,
    pub allow_collateral_withdrawal: bool,
    pub allow_liquidate_position: bool,
}
impl InitParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let allow_swap: bool = crate::borsh_de_or_default(&mut reader)?;
        let allow_add_liquidity: bool = crate::borsh_de_or_default(&mut reader)?;
        let allow_remove_liquidity: bool = crate::borsh_de_or_default(&mut reader)?;
        let allow_increase_position: bool = crate::borsh_de_or_default(&mut reader)?;
        let allow_decrease_position: bool = crate::borsh_de_or_default(&mut reader)?;
        let allow_collateral_withdrawal: bool = crate::borsh_de_or_default(&mut reader)?;
        let allow_liquidate_position: bool = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            allow_swap,
            allow_add_liquidity,
            allow_remove_liquidity,
            allow_increase_position,
            allow_decrease_position,
            allow_collateral_withdrawal,
            allow_liquidate_position,
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
pub struct InstantCreateLimitOrderParams {
    pub size_usd_delta: u64,
    pub collateral_token_delta: u64,
    pub side: Side,
    pub trigger_price: u64,
    pub trigger_above_threshold: bool,
    pub counter: u64,
    pub request_time: i64,
}
impl InstantCreateLimitOrderParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let size_usd_delta: u64 = crate::borsh_de_or_default(&mut reader)?;
        let collateral_token_delta: u64 = crate::borsh_de_or_default(&mut reader)?;
        let side: Side = crate::borsh_de_or_default(&mut reader)?;
        let trigger_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let trigger_above_threshold: bool = crate::borsh_de_or_default(&mut reader)?;
        let counter: u64 = crate::borsh_de_or_default(&mut reader)?;
        let request_time: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            size_usd_delta,
            collateral_token_delta,
            side,
            trigger_price,
            trigger_above_threshold,
            counter,
            request_time,
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
pub struct InstantCreateTpslParams {
    pub collateral_usd_delta: u64,
    pub size_usd_delta: u64,
    pub trigger_price: u64,
    pub trigger_above_threshold: bool,
    pub entire_position: bool,
    pub counter: u64,
    pub request_time: i64,
}
impl InstantCreateTpslParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let collateral_usd_delta: u64 = crate::borsh_de_or_default(&mut reader)?;
        let size_usd_delta: u64 = crate::borsh_de_or_default(&mut reader)?;
        let trigger_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let trigger_above_threshold: bool = crate::borsh_de_or_default(&mut reader)?;
        let entire_position: bool = crate::borsh_de_or_default(&mut reader)?;
        let counter: u64 = crate::borsh_de_or_default(&mut reader)?;
        let request_time: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            collateral_usd_delta,
            size_usd_delta,
            trigger_price,
            trigger_above_threshold,
            entire_position,
            counter,
            request_time,
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
pub struct InstantDecreasePosition2Params {
    pub collateral_usd_delta: u64,
    pub size_usd_delta: u64,
    pub price_slippage: u64,
    pub entire_position: Option<bool>,
    pub request_time: i64,
    pub counter: u64,
}
impl InstantDecreasePosition2Params {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let collateral_usd_delta: u64 = crate::borsh_de_or_default(&mut reader)?;
        let size_usd_delta: u64 = crate::borsh_de_or_default(&mut reader)?;
        let price_slippage: u64 = crate::borsh_de_or_default(&mut reader)?;
        let entire_position: Option<bool> = crate::borsh_de_or_default(&mut reader)?;
        let request_time: i64 = crate::borsh_de_or_default(&mut reader)?;
        let counter: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            collateral_usd_delta,
            size_usd_delta,
            price_slippage,
            entire_position,
            request_time,
            counter,
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
pub struct InstantDecreasePositionParams {
    pub collateral_usd_delta: u64,
    pub size_usd_delta: u64,
    pub price_slippage: u64,
    pub entire_position: Option<bool>,
    pub request_time: i64,
}
impl InstantDecreasePositionParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let collateral_usd_delta: u64 = crate::borsh_de_or_default(&mut reader)?;
        let size_usd_delta: u64 = crate::borsh_de_or_default(&mut reader)?;
        let price_slippage: u64 = crate::borsh_de_or_default(&mut reader)?;
        let entire_position: Option<bool> = crate::borsh_de_or_default(&mut reader)?;
        let request_time: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            collateral_usd_delta,
            size_usd_delta,
            price_slippage,
            entire_position,
            request_time,
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
pub struct InstantIncreasePositionPreSwapParams {
    pub amount_in: u64,
    pub min_amount_out: u64,
}
impl InstantIncreasePositionPreSwapParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { amount_in, min_amount_out })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct InstantIncreasePositionParams {
    pub size_usd_delta: u64,
    pub collateral_token_delta: Option<u64>,
    pub side: Side,
    pub price_slippage: u64,
    pub request_time: i64,
}
impl InstantIncreasePositionParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let size_usd_delta: u64 = crate::borsh_de_or_default(&mut reader)?;
        let collateral_token_delta: Option<u64> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let side: Side = crate::borsh_de_or_default(&mut reader)?;
        let price_slippage: u64 = crate::borsh_de_or_default(&mut reader)?;
        let request_time: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            size_usd_delta,
            collateral_token_delta,
            side,
            price_slippage,
            request_time,
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
pub struct InstantUpdateLimitOrderParams {
    pub size_usd_delta: u64,
    pub trigger_price: u64,
    pub request_time: i64,
}
impl InstantUpdateLimitOrderParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let size_usd_delta: u64 = crate::borsh_de_or_default(&mut reader)?;
        let trigger_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let request_time: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            size_usd_delta,
            trigger_price,
            request_time,
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
pub struct InstantUpdateTpslParams {
    pub size_usd_delta: u64,
    pub trigger_price: u64,
    pub request_time: i64,
}
impl InstantUpdateTpslParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let size_usd_delta: u64 = crate::borsh_de_or_default(&mut reader)?;
        let trigger_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let request_time: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            size_usd_delta,
            trigger_price,
            request_time,
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
pub struct LiquidateBorrowPositionParams {}
impl LiquidateBorrowPositionParams {
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
pub struct LiquidateFullPosition4Params {}
impl LiquidateFullPosition4Params {
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
pub struct OperatorSetCustodyConfigParams {
    pub pricing: PricingParams,
    pub hourly_funding_dbps: u64,
    pub target_ratio_bps: u64,
    pub increase_position_bps: u64,
    pub decrease_position_bps: u64,
    pub max_position_size_usd: u64,
    pub jump_rate: JumpRateState,
    pub price_impact_fee_factor: u64,
    pub price_impact_exponent: f32,
    pub delta_imbalance_threshold_decimal: u64,
    pub max_fee_bps: u64,
    pub borrow_lend_parameters: BorrowLendParams,
    pub borrow_hourly_funding_dbps: u64,
    pub borrow_limit_in_token_amount: u64,
    pub min_interest_fee_bps: u64,
    pub min_interest_fee_grace_period_seconds: u64,
    pub max_total_staked_amount_lamports: u64,
    pub external_swap_fee_multiplier_bps: u64,
    pub disable_close_position_request: bool,
    pub withdrawal_limit_token_amount: u64,
    pub withdrawal_limit_interval_seconds: u64,
}
impl OperatorSetCustodyConfigParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pricing = if reader.is_empty() {
            Default::default()
        } else {
            <PricingParams>::deserialize(&mut reader)?
        };
        let hourly_funding_dbps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let target_ratio_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let increase_position_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let decrease_position_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_position_size_usd: u64 = crate::borsh_de_or_default(&mut reader)?;
        let jump_rate = if reader.is_empty() {
            Default::default()
        } else {
            <JumpRateState>::deserialize(&mut reader)?
        };
        let price_impact_fee_factor: u64 = crate::borsh_de_or_default(&mut reader)?;
        let price_impact_exponent: f32 = crate::borsh_de_or_default(&mut reader)?;
        let delta_imbalance_threshold_decimal: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let max_fee_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let borrow_lend_parameters = if reader.is_empty() {
            Default::default()
        } else {
            <BorrowLendParams>::deserialize(&mut reader)?
        };
        let borrow_hourly_funding_dbps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let borrow_limit_in_token_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_interest_fee_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_interest_fee_grace_period_seconds: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let max_total_staked_amount_lamports: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let external_swap_fee_multiplier_bps: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let disable_close_position_request: bool = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let withdrawal_limit_token_amount: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let withdrawal_limit_interval_seconds: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            pricing,
            hourly_funding_dbps,
            target_ratio_bps,
            increase_position_bps,
            decrease_position_bps,
            max_position_size_usd,
            jump_rate,
            price_impact_fee_factor,
            price_impact_exponent,
            delta_imbalance_threshold_decimal,
            max_fee_bps,
            borrow_lend_parameters,
            borrow_hourly_funding_dbps,
            borrow_limit_in_token_amount,
            min_interest_fee_bps,
            min_interest_fee_grace_period_seconds,
            max_total_staked_amount_lamports,
            external_swap_fee_multiplier_bps,
            disable_close_position_request,
            withdrawal_limit_token_amount,
            withdrawal_limit_interval_seconds,
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
pub struct OperatorSetPoolConfigParams {
    pub fees: Fees,
    pub limit: Limit,
    pub max_request_execution_sec: i64,
    pub max_trigger_price_diff_bps: u64,
    pub disable_close_position_request: bool,
    pub max_lp_token_price_change_bps: u64,
}
impl OperatorSetPoolConfigParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let fees = if reader.is_empty() {
            Default::default()
        } else {
            <Fees>::deserialize(&mut reader)?
        };
        let limit = if reader.is_empty() {
            Default::default()
        } else {
            <Limit>::deserialize(&mut reader)?
        };
        let max_request_execution_sec: i64 = crate::borsh_de_or_default(&mut reader)?;
        let max_trigger_price_diff_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let disable_close_position_request: bool = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let max_lp_token_price_change_bps: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            fees,
            limit,
            max_request_execution_sec,
            max_trigger_price_diff_bps,
            disable_close_position_request,
            max_lp_token_price_change_bps,
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
pub struct PartialLiquidateBorrowPositionParams {}
impl PartialLiquidateBorrowPositionParams {
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
pub struct RefreshAssetsUnderManagementParams {}
impl RefreshAssetsUnderManagementParams {
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
pub struct RemoveLiquidity2Params {
    pub lp_amount_in: u64,
    pub min_amount_out: u64,
}
impl RemoveLiquidity2Params {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let lp_amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            lp_amount_in,
            min_amount_out,
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
pub struct RepayToCustodyParams {
    pub amount: u64,
}
impl RepayToCustodyParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { amount })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct SetCustodyConfigParams {
    pub oracle: OracleParams,
    pub pricing: PricingParams,
    pub permissions: Permissions,
    pub hourly_funding_dbps: u64,
    pub target_ratio_bps: u64,
    pub increase_position_bps: u64,
    pub decrease_position_bps: u64,
    pub doves_oracle: Pubkey,
    pub max_position_size_usd: u64,
    pub jump_rate: JumpRateState,
    pub price_impact_fee_factor: u64,
    pub price_impact_exponent: f32,
    pub delta_imbalance_threshold_decimal: u64,
    pub max_fee_bps: u64,
    pub doves_ag_oracle: Pubkey,
    pub borrow_lend_parameters: BorrowLendParams,
    pub borrow_hourly_funding_dbps: u64,
    pub borrow_limit_in_token_amount: u64,
    pub min_interest_fee_bps: u64,
    pub min_interest_fee_grace_period_seconds: u64,
    pub max_total_staked_amount_lamports: u64,
    pub external_swap_fee_multiplier_bps: u64,
    pub disable_close_position_request: bool,
    pub withdrawal_limit_interval_seconds: u64,
    pub withdrawal_limit_token_amount: u64,
}
impl SetCustodyConfigParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let oracle = if reader.is_empty() {
            Default::default()
        } else {
            <OracleParams>::deserialize(&mut reader)?
        };
        let pricing = if reader.is_empty() {
            Default::default()
        } else {
            <PricingParams>::deserialize(&mut reader)?
        };
        let permissions = if reader.is_empty() {
            Default::default()
        } else {
            <Permissions>::deserialize(&mut reader)?
        };
        let hourly_funding_dbps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let target_ratio_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let increase_position_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let decrease_position_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let doves_oracle: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let max_position_size_usd: u64 = crate::borsh_de_or_default(&mut reader)?;
        let jump_rate = if reader.is_empty() {
            Default::default()
        } else {
            <JumpRateState>::deserialize(&mut reader)?
        };
        let price_impact_fee_factor: u64 = crate::borsh_de_or_default(&mut reader)?;
        let price_impact_exponent: f32 = crate::borsh_de_or_default(&mut reader)?;
        let delta_imbalance_threshold_decimal: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let max_fee_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let doves_ag_oracle: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let borrow_lend_parameters = if reader.is_empty() {
            Default::default()
        } else {
            <BorrowLendParams>::deserialize(&mut reader)?
        };
        let borrow_hourly_funding_dbps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let borrow_limit_in_token_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_interest_fee_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_interest_fee_grace_period_seconds: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let max_total_staked_amount_lamports: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let external_swap_fee_multiplier_bps: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let disable_close_position_request: bool = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let withdrawal_limit_interval_seconds: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let withdrawal_limit_token_amount: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            oracle,
            pricing,
            permissions,
            hourly_funding_dbps,
            target_ratio_bps,
            increase_position_bps,
            decrease_position_bps,
            doves_oracle,
            max_position_size_usd,
            jump_rate,
            price_impact_fee_factor,
            price_impact_exponent,
            delta_imbalance_threshold_decimal,
            max_fee_bps,
            doves_ag_oracle,
            borrow_lend_parameters,
            borrow_hourly_funding_dbps,
            borrow_limit_in_token_amount,
            min_interest_fee_bps,
            min_interest_fee_grace_period_seconds,
            max_total_staked_amount_lamports,
            external_swap_fee_multiplier_bps,
            disable_close_position_request,
            withdrawal_limit_interval_seconds,
            withdrawal_limit_token_amount,
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
pub struct SetMaxGlobalSizesParams {
    pub max_global_long_size: u64,
    pub max_global_short_size: u64,
    pub recovery_id: u8,
    #[serde(with = "crate::big_array_serde")]
    pub signature: [u8; 64],
    pub reference_id: [u8; 16],
    pub timestamp: u64,
}
impl SetMaxGlobalSizesParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let max_global_long_size: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_global_short_size: u64 = crate::borsh_de_or_default(&mut reader)?;
        let recovery_id: u8 = crate::borsh_de_or_default(&mut reader)?;
        let signature = <[u8; 64] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let reference_id: [u8; 16] = crate::borsh_de_or_default(&mut reader)?;
        let timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            max_global_long_size,
            max_global_short_size,
            recovery_id,
            signature,
            reference_id,
            timestamp,
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
pub struct SetPerpetualsConfigParams {
    pub permissions: Permissions,
}
impl SetPerpetualsConfigParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let permissions = if reader.is_empty() {
            Default::default()
        } else {
            <Permissions>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self { permissions })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct SetPoolConfigParams {
    pub fees: Fees,
    pub limit: Limit,
    pub max_request_execution_sec: i64,
    pub parameter_update_oracle: Secp256k1Pubkey,
    pub max_trigger_price_diff_bps: u64,
    pub disable_close_position_request: bool,
    pub max_lp_token_price_change_bps: u64,
}
impl SetPoolConfigParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let fees = if reader.is_empty() {
            Default::default()
        } else {
            <Fees>::deserialize(&mut reader)?
        };
        let limit = if reader.is_empty() {
            Default::default()
        } else {
            <Limit>::deserialize(&mut reader)?
        };
        let max_request_execution_sec: i64 = crate::borsh_de_or_default(&mut reader)?;
        let parameter_update_oracle = if reader.is_empty() {
            Default::default()
        } else {
            <Secp256k1Pubkey>::deserialize(&mut reader)?
        };
        let max_trigger_price_diff_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let disable_close_position_request: bool = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let max_lp_token_price_change_bps: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            fees,
            limit,
            max_request_execution_sec,
            parameter_update_oracle,
            max_trigger_price_diff_bps,
            disable_close_position_request,
            max_lp_token_price_change_bps,
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
pub struct SetTestTimeParams {
    pub time: i64,
}
impl SetTestTimeParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let time: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { time })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct Swap2Params {
    pub amount_in: u64,
    pub min_amount_out: u64,
}
impl Swap2Params {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { amount_in, min_amount_out })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct SwapWithTokenLedgerParams {
    pub min_amount_out: u64,
}
impl SwapWithTokenLedgerParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let min_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { min_amount_out })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct TestInitParams {
    pub allow_swap: bool,
    pub allow_add_liquidity: bool,
    pub allow_remove_liquidity: bool,
    pub allow_increase_position: bool,
    pub allow_decrease_position: bool,
    pub allow_collateral_withdrawal: bool,
    pub allow_liquidate_position: bool,
}
impl TestInitParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let allow_swap: bool = crate::borsh_de_or_default(&mut reader)?;
        let allow_add_liquidity: bool = crate::borsh_de_or_default(&mut reader)?;
        let allow_remove_liquidity: bool = crate::borsh_de_or_default(&mut reader)?;
        let allow_increase_position: bool = crate::borsh_de_or_default(&mut reader)?;
        let allow_decrease_position: bool = crate::borsh_de_or_default(&mut reader)?;
        let allow_collateral_withdrawal: bool = crate::borsh_de_or_default(&mut reader)?;
        let allow_liquidate_position: bool = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            allow_swap,
            allow_add_liquidity,
            allow_remove_liquidity,
            allow_increase_position,
            allow_decrease_position,
            allow_collateral_withdrawal,
            allow_liquidate_position,
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
pub struct TransferAdminParams {}
impl TransferAdminParams {
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
pub struct UpdateDecreasePositionRequest2Params {
    pub size_usd_delta: u64,
    pub trigger_price: u64,
}
impl UpdateDecreasePositionRequest2Params {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let size_usd_delta: u64 = crate::borsh_de_or_default(&mut reader)?;
        let trigger_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            size_usd_delta,
            trigger_price,
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
pub struct WithdrawParams {
    pub amount: u64,
}
impl WithdrawParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { amount })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct WithdrawFees2Params {}
impl WithdrawFees2Params {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        *__buf = reader;
        Ok(Self {})
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
pub struct PriceImpactBuffer {
    #[serde(with = "crate::big_array_serde")]
    pub open_interest: [i64; 60],
    pub last_updated: i64,
    pub fee_factor: u64,
    pub exponent: f32,
    pub delta_imbalance_threshold_decimal: u64,
    pub max_fee_bps: u64,
}
impl PriceImpactBuffer {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let open_interest = <[i64; 60] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let last_updated: i64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_factor: u64 = crate::borsh_de_or_default(&mut reader)?;
        let exponent: f32 = crate::borsh_de_or_default(&mut reader)?;
        let delta_imbalance_threshold_decimal: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let max_fee_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            open_interest,
            last_updated,
            fee_factor,
            exponent,
            delta_imbalance_threshold_decimal,
            max_fee_bps,
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
pub struct Assets {
    pub fees_reserves: u64,
    pub owned: u64,
    pub locked: u64,
    pub guaranteed_usd: u64,
    pub global_short_sizes: u64,
    pub global_short_average_prices: u64,
}
impl Assets {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let fees_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let owned: u64 = crate::borsh_de_or_default(&mut reader)?;
        let locked: u64 = crate::borsh_de_or_default(&mut reader)?;
        let guaranteed_usd: u64 = crate::borsh_de_or_default(&mut reader)?;
        let global_short_sizes: u64 = crate::borsh_de_or_default(&mut reader)?;
        let global_short_average_prices: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            fees_reserves,
            owned,
            locked,
            guaranteed_usd,
            global_short_sizes,
            global_short_average_prices,
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
pub struct PricingParams {
    pub trade_impact_fee_scalar: u64,
    pub buffer: u64,
    pub swap_spread: u64,
    pub max_leverage: u64,
    pub max_global_long_sizes: u64,
    pub max_global_short_sizes: u64,
}
impl PricingParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let trade_impact_fee_scalar: u64 = crate::borsh_de_or_default(&mut reader)?;
        let buffer: u64 = crate::borsh_de_or_default(&mut reader)?;
        let swap_spread: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_leverage: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_global_long_sizes: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_global_short_sizes: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            trade_impact_fee_scalar,
            buffer,
            swap_spread,
            max_leverage,
            max_global_long_sizes,
            max_global_short_sizes,
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
pub struct FundingRateState {
    pub cumulative_interest_rate: u128,
    pub last_update: i64,
    pub hourly_funding_dbps: u64,
}
impl FundingRateState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let cumulative_interest_rate: u128 = crate::borsh_de_or_default(&mut reader)?;
        let last_update: i64 = crate::borsh_de_or_default(&mut reader)?;
        let hourly_funding_dbps: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            cumulative_interest_rate,
            last_update,
            hourly_funding_dbps,
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
pub struct JumpRateState {
    pub min_rate_bps: u64,
    pub max_rate_bps: u64,
    pub target_rate_bps: u64,
    pub target_utilization_rate: u64,
}
impl JumpRateState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let min_rate_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_rate_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let target_rate_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let target_utilization_rate: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            min_rate_bps,
            max_rate_bps,
            target_rate_bps,
            target_utilization_rate,
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
pub struct BorrowLendParams {
    pub borrows_limit_in_bps: u64,
    pub maintainance_margin_bps: u64,
    pub protocol_fee_bps: u64,
    pub liquidation_margin: u64,
    pub liquidation_fee_bps: u64,
}
impl BorrowLendParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let borrows_limit_in_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let maintainance_margin_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let liquidation_margin: u64 = crate::borsh_de_or_default(&mut reader)?;
        let liquidation_fee_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            borrows_limit_in_bps,
            maintainance_margin_bps,
            protocol_fee_bps,
            liquidation_margin,
            liquidation_fee_bps,
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
pub struct OraclePrice {
    pub price: u64,
    pub exponent: i32,
}
impl OraclePrice {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let exponent: i32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { price, exponent })
    }
}
#[derive(
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
    pub price: u64,
    pub expo: i32,
    pub publish_time: i64,
}
impl Price {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let expo: i32 = crate::borsh_de_or_default(&mut reader)?;
        let publish_time: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { price, expo, publish_time })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct OracleParams {
    pub oracle_account: Pubkey,
    pub oracle_type: OracleType,
    pub buffer: u64,
    pub max_price_age_sec: u32,
}
impl OracleParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let oracle_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let oracle_type: OracleType = crate::borsh_de_or_default(&mut reader)?;
        let buffer: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_price_age_sec: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            oracle_account,
            oracle_type,
            buffer,
            max_price_age_sec,
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
pub struct AmountAndFee {
    pub amount: u64,
    pub fee: u64,
    pub fee_bps: u64,
}
impl AmountAndFee {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { amount, fee, fee_bps })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct Permissions {
    pub allow_swap: bool,
    pub allow_add_liquidity: bool,
    pub allow_remove_liquidity: bool,
    pub allow_increase_position: bool,
    pub allow_decrease_position: bool,
    pub allow_collateral_withdrawal: bool,
    pub allow_liquidate_position: bool,
}
impl Permissions {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let allow_swap: bool = crate::borsh_de_or_default(&mut reader)?;
        let allow_add_liquidity: bool = crate::borsh_de_or_default(&mut reader)?;
        let allow_remove_liquidity: bool = crate::borsh_de_or_default(&mut reader)?;
        let allow_increase_position: bool = crate::borsh_de_or_default(&mut reader)?;
        let allow_decrease_position: bool = crate::borsh_de_or_default(&mut reader)?;
        let allow_collateral_withdrawal: bool = crate::borsh_de_or_default(&mut reader)?;
        let allow_liquidate_position: bool = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            allow_swap,
            allow_add_liquidity,
            allow_remove_liquidity,
            allow_increase_position,
            allow_decrease_position,
            allow_collateral_withdrawal,
            allow_liquidate_position,
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
pub struct Fees {
    pub swap_multiplier: u64,
    pub stable_swap_multiplier: u64,
    pub add_remove_liquidity_bps: u64,
    pub swap_bps: u64,
    pub tax_bps: u64,
    pub stable_swap_bps: u64,
    pub stable_swap_tax_bps: u64,
    pub liquidation_reward_bps: u64,
    pub protocol_share_bps: u64,
}
impl Fees {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let swap_multiplier: u64 = crate::borsh_de_or_default(&mut reader)?;
        let stable_swap_multiplier: u64 = crate::borsh_de_or_default(&mut reader)?;
        let add_remove_liquidity_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let swap_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let tax_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let stable_swap_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let stable_swap_tax_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let liquidation_reward_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_share_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            swap_multiplier,
            stable_swap_multiplier,
            add_remove_liquidity_bps,
            swap_bps,
            tax_bps,
            stable_swap_bps,
            stable_swap_tax_bps,
            liquidation_reward_bps,
            protocol_share_bps,
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
pub struct PoolApr {
    pub last_updated: i64,
    pub fee_apr_bps: u64,
    pub realized_fee_usd: u64,
}
impl PoolApr {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let last_updated: i64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_apr_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let realized_fee_usd: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            last_updated,
            fee_apr_bps,
            realized_fee_usd,
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
pub struct Limit {
    pub max_aum_usd: u128,
    pub token_weightage_buffer_bps: u128,
    pub buffer: u64,
}
impl Limit {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let max_aum_usd: u128 = crate::borsh_de_or_default(&mut reader)?;
        let token_weightage_buffer_bps: u128 = crate::borsh_de_or_default(&mut reader)?;
        let buffer: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            max_aum_usd,
            token_weightage_buffer_bps,
            buffer,
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
pub struct Secp256k1Pubkey {
    pub prefix: u8,
    pub key: [u8; 32],
}
impl Secp256k1Pubkey {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let prefix: u8 = crate::borsh_de_or_default(&mut reader)?;
        let key: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { prefix, key })
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub enum PriceImpactMechanism {
    #[default]
    TradeSize,
    DeltaImbalance,
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub enum OracleType {
    #[default]
    None,
    Test,
    Pyth,
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub enum PriceCalcMode {
    #[default]
    Min,
    Max,
    Ignore,
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub enum PriceStaleTolerance {
    #[default]
    Strict,
    Loose,
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub enum TradePoolType {
    #[default]
    Increase,
    Decrease,
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub enum RequestType {
    #[default]
    Market,
    Trigger,
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub enum RequestChange {
    #[default]
    None,
    Increase,
    Decrease,
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub enum Side {
    #[default]
    None,
    Long,
    Short,
}
