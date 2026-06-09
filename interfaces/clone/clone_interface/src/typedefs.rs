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
pub struct MetadataArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}
impl MetadataArgs {
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
pub struct AssetInfo {
    pub onasset_mint: Pubkey,
    pub oracle_info_index: u8,
    pub il_health_score_coefficient: u16,
    pub position_health_score_coefficient: u16,
    pub min_overcollateral_ratio: u16,
    pub max_liquidation_overcollateral_ratio: u16,
}
impl AssetInfo {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let onasset_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let oracle_info_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let il_health_score_coefficient: u16 = crate::borsh_de_or_default(&mut reader)?;
        let position_health_score_coefficient: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let min_overcollateral_ratio: u16 = crate::borsh_de_or_default(&mut reader)?;
        let max_liquidation_overcollateral_ratio: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            onasset_mint,
            oracle_info_index,
            il_health_score_coefficient,
            position_health_score_coefficient,
            min_overcollateral_ratio,
            max_liquidation_overcollateral_ratio,
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
pub struct OracleInfo {
    pub source: OracleSource,
    pub address: Pubkey,
    pub price: i64,
    pub expo: u8,
    pub status: Status,
    pub last_update_slot: u64,
    pub rescale_factor: u8,
}
impl OracleInfo {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let source: OracleSource = crate::borsh_de_or_default(&mut reader)?;
        let address: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let price: i64 = crate::borsh_de_or_default(&mut reader)?;
        let expo: u8 = crate::borsh_de_or_default(&mut reader)?;
        let status: Status = crate::borsh_de_or_default(&mut reader)?;
        let last_update_slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        let rescale_factor: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            source,
            address,
            price,
            expo,
            status,
            last_update_slot,
            rescale_factor,
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
pub struct Pool {
    pub underlying_asset_token_account: Pubkey,
    pub committed_collateral_liquidity: u64,
    pub collateral_ild: i64,
    pub onasset_ild: i64,
    pub treasury_trading_fee_bps: u16,
    pub liquidity_trading_fee_bps: u16,
    pub asset_info: AssetInfo,
    pub status: Status,
}
impl Pool {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let underlying_asset_token_account: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let committed_collateral_liquidity: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let collateral_ild: i64 = crate::borsh_de_or_default(&mut reader)?;
        let onasset_ild: i64 = crate::borsh_de_or_default(&mut reader)?;
        let treasury_trading_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let liquidity_trading_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let asset_info = if reader.is_empty() {
            Default::default()
        } else {
            <AssetInfo>::deserialize(&mut reader)?
        };
        let status: Status = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            underlying_asset_token_account,
            committed_collateral_liquidity,
            collateral_ild,
            onasset_ild,
            treasury_trading_fee_bps,
            liquidity_trading_fee_bps,
            asset_info,
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
pub struct Collateral {
    pub oracle_info_index: u8,
    pub mint: Pubkey,
    pub vault: Pubkey,
    pub collateralization_ratio: u8,
    pub scale: u8,
}
impl Collateral {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let oracle_info_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let collateralization_ratio: u8 = crate::borsh_de_or_default(&mut reader)?;
        let scale: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            oracle_info_index,
            mint,
            vault,
            collateralization_ratio,
            scale,
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
pub struct Comet {
    pub collateral_amount: u64,
    pub positions: Vec<LiquidityPosition>,
}
impl Comet {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let collateral_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let positions: Vec<LiquidityPosition> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            collateral_amount,
            positions,
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
pub struct LiquidityPosition {
    pub pool_index: u8,
    pub committed_collateral_liquidity: u64,
    pub collateral_ild_rebate: i64,
    pub onasset_ild_rebate: i64,
}
impl LiquidityPosition {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let committed_collateral_liquidity: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let collateral_ild_rebate: i64 = crate::borsh_de_or_default(&mut reader)?;
        let onasset_ild_rebate: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool_index,
            committed_collateral_liquidity,
            collateral_ild_rebate,
            onasset_ild_rebate,
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
pub struct Borrow {
    pub pool_index: u8,
    pub borrowed_onasset: u64,
    pub collateral_amount: u64,
}
impl Borrow {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let borrowed_onasset: u64 = crate::borsh_de_or_default(&mut reader)?;
        let collateral_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool_index,
            borrowed_onasset,
            collateral_amount,
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
pub enum PaymentType {
    #[default]
    Onasset,
    Collateral,
    CollateralFromWallet,
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
pub enum CloneParameters {
    AddAuth { address: Pubkey },
    RemoveAuth { address: Pubkey },
    CometCollateralLiquidationFee { value: u16 },
    CometOnassetLiquidationFee { value: u16 },
    BorrowLiquidationFee { value: u16 },
    TreasuryAddress { address: Pubkey },
    CollateralizationRatio { value: u8 },
    NonAuthLiquidationsEnabled { value: bool },
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
pub enum UpdateOracleParameters {
    Add { address: Pubkey, source: OracleSource, rescale_factor: Option<u8> },
    Remove { index: u8 },
    Modify {
        index: u8,
        address: Option<Pubkey>,
        source: Option<OracleSource>,
        status: Option<Status>,
    },
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
pub enum PoolParameters {
    Status { value: Status },
    TreasuryTradingFee { value: u16 },
    LiquidityTradingFee { value: u16 },
    OracleInfoIndex { value: u8 },
    MinOvercollateralRatio { value: u16 },
    MaxLiquidationOvercollateralRatio { value: u16 },
    IlHealthScoreCoefficient { value: u16 },
    PositionHealthScoreCoefficient { value: u16 },
}
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub enum Status {
    #[default]
    Active,
    Frozen,
    Extraction,
    Liquidation,
    Deprecation,
}
#[derive(
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
}
