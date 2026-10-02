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
pub enum OrderStatus {
    #[default]
    Active,
    Filled,
    Cancelled,
}
impl TryFrom<u8> for OrderStatus {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Active),
            1u8 => Ok(Self::Filled),
            2u8 => Ok(Self::Cancelled),
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
    Vanilla,
}
impl TryFrom<u8> for OrderType {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Vanilla),
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
pub enum UpdateGlobalConfigMode {
    #[default]
    UpdateEmergencyMode,
    UpdateFlashTakeOrderBlocked,
    UpdateBlockNewOrders,
    UpdateBlockOrderTaking,
    UpdateHostFeeBps,
    UpdateAdminAuthorityCached,
    UpdateOrderTakingPermissionless,
    UpdateOrderCloseDelaySeconds,
    UpdateTxnFeeCost,
    UpdateAtaCreationCost,
}
impl TryFrom<u8> for UpdateGlobalConfigMode {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::UpdateEmergencyMode),
            1u8 => Ok(Self::UpdateFlashTakeOrderBlocked),
            2u8 => Ok(Self::UpdateBlockNewOrders),
            3u8 => Ok(Self::UpdateBlockOrderTaking),
            4u8 => Ok(Self::UpdateHostFeeBps),
            5u8 => Ok(Self::UpdateAdminAuthorityCached),
            6u8 => Ok(Self::UpdateOrderTakingPermissionless),
            7u8 => Ok(Self::UpdateOrderCloseDelaySeconds),
            8u8 => Ok(Self::UpdateTxnFeeCost),
            9u8 => Ok(Self::UpdateAtaCreationCost),
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
pub enum UpdateGlobalConfigValue {
    Bool(bool),
    U16(u16),
    U64(u64),
    Pubkey(Pubkey),
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
pub enum UpdateOrderMode {
    #[default]
    UpdatePermissionless,
    UpdateCounterparty,
}
impl TryFrom<u8> for UpdateOrderMode {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::UpdatePermissionless),
            1u8 => Ok(Self::UpdateCounterparty),
            _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
        }
    }
}
