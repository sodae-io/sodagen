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
pub struct FeeConfiguration {
    pub manager_performance_fee: u16,
    pub admin_performance_fee: u16,
    pub manager_management_fee: u16,
    pub admin_management_fee: u16,
    pub redemption_fee: u16,
    pub issuance_fee: u16,
    pub protocol_performance_fee: u16,
    pub protocol_management_fee: u16,
    pub reserved: [u8; 32],
}
impl FeeConfiguration {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let manager_performance_fee: u16 = crate::borsh_de_or_default(&mut reader)?;
        let admin_performance_fee: u16 = crate::borsh_de_or_default(&mut reader)?;
        let manager_management_fee: u16 = crate::borsh_de_or_default(&mut reader)?;
        let admin_management_fee: u16 = crate::borsh_de_or_default(&mut reader)?;
        let redemption_fee: u16 = crate::borsh_de_or_default(&mut reader)?;
        let issuance_fee: u16 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_performance_fee: u16 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_management_fee: u16 = crate::borsh_de_or_default(&mut reader)?;
        let reserved: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            manager_performance_fee,
            admin_performance_fee,
            manager_management_fee,
            admin_management_fee,
            redemption_fee,
            issuance_fee,
            protocol_performance_fee,
            protocol_management_fee,
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
pub struct FeeState {
    pub accumulated_lp_manager_fees: u64,
    pub accumulated_lp_admin_fees: u64,
    pub accumulated_lp_protocol_fees: u64,
    pub reserved: [u8; 16],
}
impl FeeState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let accumulated_lp_manager_fees: u64 = crate::borsh_de_or_default(&mut reader)?;
        let accumulated_lp_admin_fees: u64 = crate::borsh_de_or_default(&mut reader)?;
        let accumulated_lp_protocol_fees: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reserved: [u8; 16] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            accumulated_lp_manager_fees,
            accumulated_lp_admin_fees,
            accumulated_lp_protocol_fees,
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
pub struct FeeUpdate {
    pub last_performance_fee_update_ts: u64,
    pub last_management_fee_update_ts: u64,
}
impl FeeUpdate {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let last_performance_fee_update_ts: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let last_management_fee_update_ts: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            last_performance_fee_update_ts,
            last_management_fee_update_ts,
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
pub struct HighWaterMark {
    pub highest_asset_per_lp_decimal_bits: u128,
    pub last_updated_ts: u64,
    pub reserved: [u8; 8],
}
impl HighWaterMark {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let highest_asset_per_lp_decimal_bits: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let last_updated_ts: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reserved: [u8; 8] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            highest_asset_per_lp_decimal_bits,
            last_updated_ts,
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
pub struct LockedProfitState {
    pub last_updated_locked_profit: u64,
    pub last_report: u64,
}
impl LockedProfitState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let last_updated_locked_profit: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_report: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            last_updated_locked_profit,
            last_report,
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
pub enum ProtocolConfigField {
    #[default]
    OperationalState,
    PendingAdmin,
    Treasury,
}
impl TryFrom<u8> for ProtocolConfigField {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::OperationalState),
            1u8 => Ok(Self::PendingAdmin),
            2u8 => Ok(Self::Treasury),
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
pub enum ProtocolFeeField {
    #[default]
    ProtocolPerformanceFee,
    ProtocolManagementFee,
}
impl TryFrom<u8> for ProtocolFeeField {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::ProtocolPerformanceFee),
            1u8 => Ok(Self::ProtocolManagementFee),
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
pub struct VaultAsset {
    pub mint: Pubkey,
    pub idle_ata: Pubkey,
    pub total_value: u64,
    pub idle_ata_auth_bump: u8,
    #[serde(with = "crate::big_array_serde")]
    pub reserved: [u8; 95],
}
impl VaultAsset {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let idle_ata: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let total_value: u64 = crate::borsh_de_or_default(&mut reader)?;
        let idle_ata_auth_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let reserved = <[u8; 95] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            mint,
            idle_ata,
            total_value,
            idle_ata_auth_bump,
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
pub enum VaultConfigField {
    #[default]
    MaxCap,
    StartAtTs,
    LockedProfitDegradationDuration,
    WithdrawalWaitingPeriod,
    ManagerPerformanceFee,
    AdminPerformanceFee,
    ManagerManagementFee,
    AdminManagementFee,
    RedemptionFee,
    IssuanceFee,
    Manager,
    PendingAdmin,
    DisabledOperations,
}
impl TryFrom<u8> for VaultConfigField {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::MaxCap),
            1u8 => Ok(Self::StartAtTs),
            2u8 => Ok(Self::LockedProfitDegradationDuration),
            3u8 => Ok(Self::WithdrawalWaitingPeriod),
            4u8 => Ok(Self::ManagerPerformanceFee),
            5u8 => Ok(Self::AdminPerformanceFee),
            6u8 => Ok(Self::ManagerManagementFee),
            7u8 => Ok(Self::AdminManagementFee),
            8u8 => Ok(Self::RedemptionFee),
            9u8 => Ok(Self::IssuanceFee),
            10u8 => Ok(Self::Manager),
            11u8 => Ok(Self::PendingAdmin),
            12u8 => Ok(Self::DisabledOperations),
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
pub struct VaultConfiguration {
    pub max_cap: u64,
    pub start_at_ts: u64,
    pub locked_profit_degradation_duration: u64,
    pub withdrawal_waiting_period: u64,
    pub disabled_operations: u16,
    #[serde(with = "crate::big_array_serde")]
    pub reserved: [u8; 46],
}
impl VaultConfiguration {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let max_cap: u64 = crate::borsh_de_or_default(&mut reader)?;
        let start_at_ts: u64 = crate::borsh_de_or_default(&mut reader)?;
        let locked_profit_degradation_duration: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let withdrawal_waiting_period: u64 = crate::borsh_de_or_default(&mut reader)?;
        let disabled_operations: u16 = crate::borsh_de_or_default(&mut reader)?;
        let reserved = <[u8; 46] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            max_cap,
            start_at_ts,
            locked_profit_degradation_duration,
            withdrawal_waiting_period,
            disabled_operations,
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
pub struct VaultInitializationInput {
    pub max_cap: u64,
    pub start_at_ts: u64,
    pub manager_performance_fee: u16,
    pub admin_performance_fee: u16,
    pub manager_management_fee: u16,
    pub admin_management_fee: u16,
    pub locked_profit_degradation_duration: u64,
    pub redemption_fee: u16,
    pub issuance_fee: u16,
    pub withdrawal_waiting_period: u64,
}
impl VaultInitializationInput {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let max_cap: u64 = crate::borsh_de_or_default(&mut reader)?;
        let start_at_ts: u64 = crate::borsh_de_or_default(&mut reader)?;
        let manager_performance_fee: u16 = crate::borsh_de_or_default(&mut reader)?;
        let admin_performance_fee: u16 = crate::borsh_de_or_default(&mut reader)?;
        let manager_management_fee: u16 = crate::borsh_de_or_default(&mut reader)?;
        let admin_management_fee: u16 = crate::borsh_de_or_default(&mut reader)?;
        let locked_profit_degradation_duration: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let redemption_fee: u16 = crate::borsh_de_or_default(&mut reader)?;
        let issuance_fee: u16 = crate::borsh_de_or_default(&mut reader)?;
        let withdrawal_waiting_period: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            max_cap,
            start_at_ts,
            manager_performance_fee,
            admin_performance_fee,
            manager_management_fee,
            admin_management_fee,
            locked_profit_degradation_duration,
            redemption_fee,
            issuance_fee,
            withdrawal_waiting_period,
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
pub struct VaultLp {
    pub mint: Pubkey,
    pub mint_bump: u8,
    pub mint_auth_bump: u8,
    pub reserved: [u8; 30],
}
impl VaultLp {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let mint_auth_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let reserved: [u8; 30] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            mint,
            mint_bump,
            mint_auth_bump,
            reserved,
        })
    }
}
