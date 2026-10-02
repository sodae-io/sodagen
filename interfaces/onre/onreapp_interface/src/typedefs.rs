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
pub struct ApprovalMessage {
    pub program_id: Pubkey,
    pub user_pubkey: Pubkey,
    pub expiry_unix: u64,
}
impl ApprovalMessage {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let program_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let user_pubkey: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let expiry_unix: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            program_id,
            user_pubkey,
            expiry_unix,
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
pub enum ConfigurableVaultKind {
    #[default]
    OfferFee,
    ManagementFee,
    PerformanceFee,
    PropAmmBuyFee,
    OfferProceeds,
    PropAmmProceeds,
    PermissionlessOfferFee,
    RedemptionFee,
    PropAmmSellFee,
}
impl TryFrom<u8> for ConfigurableVaultKind {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::OfferFee),
            1u8 => Ok(Self::ManagementFee),
            2u8 => Ok(Self::PerformanceFee),
            3u8 => Ok(Self::PropAmmBuyFee),
            4u8 => Ok(Self::OfferProceeds),
            5u8 => Ok(Self::PropAmmProceeds),
            6u8 => Ok(Self::PermissionlessOfferFee),
            7u8 => Ok(Self::RedemptionFee),
            8u8 => Ok(Self::PropAmmSellFee),
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
pub struct OfferVector {
    pub start_time: u64,
    pub base_time: u64,
    pub base_price: u64,
    pub apr: u64,
    pub price_fix_duration: u64,
}
impl OfferVector {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let start_time: u64 = crate::borsh_de_or_default(&mut reader)?;
        let base_time: u64 = crate::borsh_de_or_default(&mut reader)?;
        let base_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let apr: u64 = crate::borsh_de_or_default(&mut reader)?;
        let price_fix_duration: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            start_time,
            base_time,
            base_price,
            apr,
            price_fix_duration,
        })
    }
}
