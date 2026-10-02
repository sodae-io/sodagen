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
pub struct ConfigPairParams {
    pub mint_one_normal_route_fee_rate: Option<u16>,
    pub mint_two_normal_route_fee_rate: Option<u16>,
    pub mint_one_preferred_route_fee_rate: Option<u16>,
    pub mint_two_preferred_route_fee_rate: Option<u16>,
}
impl ConfigPairParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mint_one_normal_route_fee_rate: Option<u16> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let mint_two_normal_route_fee_rate: Option<u16> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let mint_one_preferred_route_fee_rate: Option<u16> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let mint_two_preferred_route_fee_rate: Option<u16> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            mint_one_normal_route_fee_rate,
            mint_two_normal_route_fee_rate,
            mint_one_preferred_route_fee_rate,
            mint_two_preferred_route_fee_rate,
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
pub struct SSLMathConfig {
    pub mean_window: Option<u8>,
    pub std_window: Option<u8>,
    pub fixed_price_distance: Option<u16>,
    pub minimum_price_distance: Option<u16>,
    pub std_weight: Option<u32>,
    pub latest_price_weight: Option<u16>,
    pub check_price_gap_count: Option<u16>,
}
impl SSLMathConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mean_window: Option<u8> = crate::borsh_de_or_default(&mut reader)?;
        let std_window: Option<u8> = crate::borsh_de_or_default(&mut reader)?;
        let fixed_price_distance: Option<u16> = crate::borsh_de_or_default(&mut reader)?;
        let minimum_price_distance: Option<u16> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let std_weight: Option<u32> = crate::borsh_de_or_default(&mut reader)?;
        let latest_price_weight: Option<u16> = crate::borsh_de_or_default(&mut reader)?;
        let check_price_gap_count: Option<u16> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            mean_window,
            std_window,
            fixed_price_distance,
            minimum_price_distance,
            std_weight,
            latest_price_weight,
            check_price_gap_count,
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
pub struct SSLMathParams {
    pub mean_window: u8,
    pub std_window: u8,
    pub fixed_price_distance: u16,
    pub minimum_price_distance: u16,
    pub deprecated0: u16,
    pub latest_price_weight: u16,
    pub check_price_gap_count: u16,
    pub pad0: [u8; 4],
    pub std_weight: u32,
    pub pad1: [u8; 4],
    pub space: [u8; 32],
}
impl SSLMathParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mean_window: u8 = crate::borsh_de_or_default(&mut reader)?;
        let std_window: u8 = crate::borsh_de_or_default(&mut reader)?;
        let fixed_price_distance: u16 = crate::borsh_de_or_default(&mut reader)?;
        let minimum_price_distance: u16 = crate::borsh_de_or_default(&mut reader)?;
        let deprecated0: u16 = crate::borsh_de_or_default(&mut reader)?;
        let latest_price_weight: u16 = crate::borsh_de_or_default(&mut reader)?;
        let check_price_gap_count: u16 = crate::borsh_de_or_default(&mut reader)?;
        let pad0: [u8; 4] = crate::borsh_de_or_default(&mut reader)?;
        let std_weight: u32 = crate::borsh_de_or_default(&mut reader)?;
        let pad1: [u8; 4] = crate::borsh_de_or_default(&mut reader)?;
        let space: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            mean_window,
            std_window,
            fixed_price_distance,
            minimum_price_distance,
            deprecated0,
            latest_price_weight,
            check_price_gap_count,
            pad0,
            std_weight,
            pad1,
            space,
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
pub struct MaxPoolTokenRatio {
    pub input_token: AssetType,
    pub output_token: AssetType,
    pub pool_token_ratio: u16,
}
impl MaxPoolTokenRatio {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let input_token: AssetType = crate::borsh_de_or_default(&mut reader)?;
        let output_token: AssetType = crate::borsh_de_or_default(&mut reader)?;
        let pool_token_ratio: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            input_token,
            output_token,
            pool_token_ratio,
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
pub struct PoolRegistryConfig {
    pub new_admin: Option<Pubkey>,
    pub new_suspend_admin: Option<Pubkey>,
    pub max_pool_token_ratios: Vec<MaxPoolTokenRatio>,
    pub pricing_algo: Option<PricingAlgo>,
}
impl PoolRegistryConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let new_admin: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let new_suspend_admin: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let max_pool_token_ratios: Vec<MaxPoolTokenRatio> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let pricing_algo: Option<PricingAlgo> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            new_admin,
            new_suspend_admin,
            max_pool_token_ratios,
            pricing_algo,
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
pub enum OracleType {
    #[default]
    Uninitialized,
    Pyth,
    Switchboardv2,
    Invalid,
}
impl TryFrom<u8> for OracleType {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Uninitialized),
            1u8 => Ok(Self::Pyth),
            2u8 => Ok(Self::Switchboardv2),
            3u8 => Ok(Self::Invalid),
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
    Uninitialized,
    BlueChip,
    Volatile,
    Stable,
    Invalid,
}
impl TryFrom<u8> for AssetType {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Uninitialized),
            1u8 => Ok(Self::BlueChip),
            2u8 => Ok(Self::Volatile),
            3u8 => Ok(Self::Stable),
            4u8 => Ok(Self::Invalid),
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
pub enum ConfigSSLPoolStatus {
    #[default]
    Active,
    Suspended,
    Locked,
}
impl TryFrom<u8> for ConfigSSLPoolStatus {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Active),
            1u8 => Ok(Self::Suspended),
            2u8 => Ok(Self::Locked),
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
pub enum PricingAlgo {
    #[default]
    Version1,
    Version2,
    Version3,
    Invalid,
}
impl TryFrom<u8> for PricingAlgo {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Version1),
            1u8 => Ok(Self::Version2),
            2u8 => Ok(Self::Version3),
            3u8 => Ok(Self::Invalid),
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
pub enum SSLPoolStatus {
    #[default]
    Uninitialized,
    Active,
    Suspended,
    Locked,
    Invalid,
}
impl TryFrom<u8> for SSLPoolStatus {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Uninitialized),
            1u8 => Ok(Self::Active),
            2u8 => Ok(Self::Suspended),
            3u8 => Ok(Self::Locked),
            4u8 => Ok(Self::Invalid),
            _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
        }
    }
}
