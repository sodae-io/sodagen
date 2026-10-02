use borsh::{BorshDeserialize, BorshSerialize};
#[allow(unused_imports)]
use crate::*;
use solana_pubkey::Pubkey;
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub enum Curve {
    ExponentialCurveV0 { k: u128 },
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
pub struct InitializeTreasuryManagementArgsV0 {
    pub authority: Pubkey,
    pub curve: Curve,
    pub freeze_unix_time: i64,
    pub window_config: WindowedCircuitBreakerConfigV0,
}
impl InitializeTreasuryManagementArgsV0 {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let curve = <Curve as borsh::BorshDeserialize>::deserialize_reader(&mut reader)?;
        let freeze_unix_time: i64 = crate::borsh_de_or_default(&mut reader)?;
        let window_config = if reader.is_empty() {
            Default::default()
        } else {
            <WindowedCircuitBreakerConfigV0>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            authority,
            curve,
            freeze_unix_time,
            window_config,
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
pub struct RedeemArgsV0 {
    pub amount: u64,
    pub expected_output_amount: u64,
}
impl RedeemArgsV0 {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let expected_output_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            amount,
            expected_output_amount,
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
pub enum ThresholdType {
    #[default]
    Percent,
    Absolute,
}
impl TryFrom<u8> for ThresholdType {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Percent),
            1u8 => Ok(Self::Absolute),
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
pub struct UpdateTreasuryManagementArgsV0 {
    pub authority: Pubkey,
    pub curve: Curve,
    pub freeze_unix_time: i64,
}
impl UpdateTreasuryManagementArgsV0 {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let curve = <Curve as borsh::BorshDeserialize>::deserialize_reader(&mut reader)?;
        let freeze_unix_time: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            authority,
            curve,
            freeze_unix_time,
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
pub struct WindowV0 {
    pub last_aggregated_value: u64,
    pub last_unix_timestamp: i64,
}
impl WindowV0 {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let last_aggregated_value: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_unix_timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            last_aggregated_value,
            last_unix_timestamp,
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
pub struct WindowedCircuitBreakerConfigV0 {
    pub window_size_seconds: u64,
    pub threshold_type: ThresholdType,
    pub threshold: u64,
}
impl WindowedCircuitBreakerConfigV0 {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let window_size_seconds: u64 = crate::borsh_de_or_default(&mut reader)?;
        let threshold_type: ThresholdType = crate::borsh_de_or_default(&mut reader)?;
        let threshold: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            window_size_seconds,
            threshold_type,
            threshold,
        })
    }
}
