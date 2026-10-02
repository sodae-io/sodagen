use borsh::{BorshDeserialize, BorshSerialize};
#[allow(unused_imports)]
use crate::*;
#[derive(
    Clone,
    Debug,
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct LockedProfitTracker {
    pub last_updated_locked_profit: u64,
    pub last_report: u64,
    pub locked_profit_degradation: u64,
}
impl LockedProfitTracker {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let last_updated_locked_profit: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_report: u64 = crate::borsh_de_or_default(&mut reader)?;
        let locked_profit_degradation: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            last_updated_locked_profit,
            last_report,
            locked_profit_degradation,
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
pub struct VaultBumps {
    pub vault_bump: u8,
    pub token_vault_bump: u8,
}
impl VaultBumps {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let vault_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let token_vault_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            vault_bump,
            token_vault_bump,
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
pub struct StrategyBumps {
    pub strategy_index: u8,
    pub other_bumps: [u8; 10],
}
impl StrategyBumps {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let strategy_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let other_bumps: [u8; 10] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            strategy_index,
            other_bumps,
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
    PortFinanceWithoutLm,
    PortFinanceWithLm,
    SolendWithoutLm,
    Mango,
    SolendWithLm,
    ApricotWithoutLm,
    Francium,
    Tulip,
    Vault,
    Drift,
    Frakt,
    Marginfi,
    Kamino,
}
impl TryFrom<u8> for StrategyType {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::PortFinanceWithoutLm),
            1u8 => Ok(Self::PortFinanceWithLm),
            2u8 => Ok(Self::SolendWithoutLm),
            3u8 => Ok(Self::Mango),
            4u8 => Ok(Self::SolendWithLm),
            5u8 => Ok(Self::ApricotWithoutLm),
            6u8 => Ok(Self::Francium),
            7u8 => Ok(Self::Tulip),
            8u8 => Ok(Self::Vault),
            9u8 => Ok(Self::Drift),
            10u8 => Ok(Self::Frakt),
            11u8 => Ok(Self::Marginfi),
            12u8 => Ok(Self::Kamino),
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
pub enum VaultType {
    #[default]
    IdleVault,
    RebalanceVault,
}
impl TryFrom<u8> for VaultType {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::IdleVault),
            1u8 => Ok(Self::RebalanceVault),
            _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
        }
    }
}
