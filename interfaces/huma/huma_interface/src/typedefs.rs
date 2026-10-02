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
pub enum AsyncDeploymentStrategyType {
    #[default]
    HumaInstitutional,
}
impl TryFrom<u8> for AsyncDeploymentStrategyType {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::HumaInstitutional),
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
pub enum DeploymentStrategyType {
    #[default]
    Manual,
    JupLend,
    KaminoLend,
}
impl TryFrom<u8> for DeploymentStrategyType {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Manual),
            1u8 => Ok(Self::JupLend),
            2u8 => Ok(Self::KaminoLend),
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
pub struct InstantWithdrawalConfig {
    pub instant_withdrawal_reserve_limit: u64,
    pub instant_withdrawal_fee_configs: Vec<InstantWithdrawalFeeConfig>,
    pub liquidity_source: Option<Pubkey>,
    #[serde(with = "crate::big_array_serde")]
    pub reserved: [u8; 127],
}
impl InstantWithdrawalConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let instant_withdrawal_reserve_limit: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let instant_withdrawal_fee_configs: Vec<InstantWithdrawalFeeConfig> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let liquidity_source: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let reserved = <[u8; 127] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            instant_withdrawal_reserve_limit,
            instant_withdrawal_fee_configs,
            liquidity_source,
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
pub struct InstantWithdrawalFeeConfig {
    pub liquid_asset_ratio_lt_bps: u16,
    pub fee_bps: u16,
    #[serde(with = "crate::big_array_serde")]
    pub reserved: [u8; 160],
}
impl InstantWithdrawalFeeConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let liquid_asset_ratio_lt_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let reserved = <[u8; 160] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            liquid_asset_ratio_lt_bps,
            fee_bps,
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
pub struct InstantWithdrawalFeeConfigInput {
    pub liquid_asset_ratio_lt_bps: u16,
    pub fee_bps: u16,
}
impl InstantWithdrawalFeeConfigInput {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let liquid_asset_ratio_lt_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            liquid_asset_ratio_lt_bps,
            fee_bps,
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
pub struct LPConfig {
    pub liquidity_cap: u128,
    pub min_deposit_amount: u64,
    pub min_redemption_shares: u64,
    pub max_total_redemption_shares_per_window: u128,
    pub max_instant_withdrawal_shares_per_window: u128,
    #[serde(with = "crate::big_array_serde")]
    pub padding: [u8; 144],
}
impl LPConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let liquidity_cap: u128 = crate::borsh_de_or_default(&mut reader)?;
        let min_deposit_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_redemption_shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_total_redemption_shares_per_window: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let max_instant_withdrawal_shares_per_window: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let padding = <[u8; 144] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            liquidity_cap,
            min_deposit_amount,
            min_redemption_shares,
            max_total_redemption_shares_per_window,
            max_instant_withdrawal_shares_per_window,
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
pub struct LenderRedemptionRecord {
    pub num_pending_redemption_requests: u128,
    pub total_amount_processed: u128,
    pub total_amount_withdrawn: u128,
    pub shares_pending_redemption: u128,
    #[serde(with = "crate::big_array_serde")]
    pub padding: [u8; 144],
}
impl LenderRedemptionRecord {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let num_pending_redemption_requests: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let total_amount_processed: u128 = crate::borsh_de_or_default(&mut reader)?;
        let total_amount_withdrawn: u128 = crate::borsh_de_or_default(&mut reader)?;
        let shares_pending_redemption: u128 = crate::borsh_de_or_default(&mut reader)?;
        let padding = <[u8; 144] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            num_pending_redemption_requests,
            total_amount_processed,
            total_amount_withdrawn,
            shares_pending_redemption,
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
pub struct ManageModeTokenMetadataArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}
impl ManageModeTokenMetadataArgs {
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
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct ModeState {
    pub assets: u128,
    pub losses: u128,
    pub cumulative_yields: u128,
    pub assets_refreshed_at: u64,
    #[serde(with = "crate::big_array_serde")]
    pub padding: [u8; 160],
}
impl ModeState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let assets: u128 = crate::borsh_de_or_default(&mut reader)?;
        let losses: u128 = crate::borsh_de_or_default(&mut reader)?;
        let cumulative_yields: u128 = crate::borsh_de_or_default(&mut reader)?;
        let assets_refreshed_at: u64 = crate::borsh_de_or_default(&mut reader)?;
        let padding = <[u8; 160] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            assets,
            losses,
            cumulative_yields,
            assets_refreshed_at,
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
pub struct PoolStats {
    pub cumulative_amount_deployed: u128,
    pub cumulative_amount_paid_back: u128,
    #[serde(with = "crate::big_array_serde")]
    pub padding: [u8; 160],
}
impl PoolStats {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let cumulative_amount_deployed: u128 = crate::borsh_de_or_default(&mut reader)?;
        let cumulative_amount_paid_back: u128 = crate::borsh_de_or_default(&mut reader)?;
        let padding = <[u8; 160] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            cumulative_amount_deployed,
            cumulative_amount_paid_back,
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
pub enum PoolStatus {
    #[default]
    Off,
    On,
    PreClosure,
    Closed,
}
impl TryFrom<u8> for PoolStatus {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Off),
            1u8 => Ok(Self::On),
            2u8 => Ok(Self::PreClosure),
            3u8 => Ok(Self::Closed),
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
pub struct Redemption {
    pub next_request_id: u128,
    pub last_request_id: u128,
    pub global_redemption_gating: RedemptionGating,
    pub instant_withdrawal_gating: RedemptionGating,
    #[serde(with = "crate::big_array_serde")]
    pub padding: [u8; 136],
}
impl Redemption {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let next_request_id: u128 = crate::borsh_de_or_default(&mut reader)?;
        let last_request_id: u128 = crate::borsh_de_or_default(&mut reader)?;
        let global_redemption_gating = <RedemptionGating>::deserialize(&mut reader)?;
        let instant_withdrawal_gating = <RedemptionGating>::deserialize(&mut reader)?;
        let padding = <[u8; 136] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            next_request_id,
            last_request_id,
            global_redemption_gating,
            instant_withdrawal_gating,
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
pub struct RedemptionGating {
    pub total_shares_requested: u128,
    pub window_starts_at: u64,
    #[serde(with = "crate::big_array_serde")]
    pub padding: [u8; 80],
}
impl RedemptionGating {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let total_shares_requested: u128 = crate::borsh_de_or_default(&mut reader)?;
        let window_starts_at: u64 = crate::borsh_de_or_default(&mut reader)?;
        let padding = <[u8; 80] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            total_shares_requested,
            window_starts_at,
            padding,
        })
    }
}
