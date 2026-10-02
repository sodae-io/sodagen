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
pub struct InitializeMarketInstruction {
    pub coin_lot_size: u64,
    pub pc_lot_size: u64,
    pub fee_rate_bps: u16,
    pub vault_signer_nonce: u64,
    pub pc_dust_threshold: u64,
}
impl InitializeMarketInstruction {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let coin_lot_size: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pc_lot_size: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_rate_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let vault_signer_nonce: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pc_dust_threshold: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            coin_lot_size,
            pc_lot_size,
            fee_rate_bps,
            vault_signer_nonce,
            pc_dust_threshold,
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
pub struct NewOrderInstructionV1 {
    pub side: Side,
    pub limit_price: u64,
    pub max_qty: u64,
    pub order_type: OrderType,
    pub client_id: u64,
}
impl NewOrderInstructionV1 {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let side: Side = crate::borsh_de_or_default(&mut reader)?;
        let limit_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_qty: u64 = crate::borsh_de_or_default(&mut reader)?;
        let order_type: OrderType = crate::borsh_de_or_default(&mut reader)?;
        let client_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            side,
            limit_price,
            max_qty,
            order_type,
            client_id,
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
pub struct CancelOrderInstructionV2 {
    pub side: Side,
    pub order_id: u128,
}
impl CancelOrderInstructionV2 {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let side: Side = crate::borsh_de_or_default(&mut reader)?;
        let order_id: u128 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { side, order_id })
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
pub struct NewOrderInstructionV2 {
    pub side: Side,
    pub limit_price: u64,
    pub max_qty: u64,
    pub order_type: OrderType,
    pub client_id: u64,
    pub self_trade_behavior: SelfTradeBehavior,
}
impl NewOrderInstructionV2 {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let side: Side = crate::borsh_de_or_default(&mut reader)?;
        let limit_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_qty: u64 = crate::borsh_de_or_default(&mut reader)?;
        let order_type: OrderType = crate::borsh_de_or_default(&mut reader)?;
        let client_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        let self_trade_behavior: SelfTradeBehavior = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            side,
            limit_price,
            max_qty,
            order_type,
            client_id,
            self_trade_behavior,
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
pub struct NewOrderInstructionV3 {
    pub side: Side,
    pub limit_price: u64,
    pub max_coin_qty: u64,
    pub max_native_pc_qty_including_fees: u64,
    pub self_trade_behavior: SelfTradeBehavior,
    pub order_type: OrderType,
    pub client_order_id: u64,
    pub limit: u16,
}
impl NewOrderInstructionV3 {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let side: Side = crate::borsh_de_or_default(&mut reader)?;
        let limit_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_coin_qty: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_native_pc_qty_including_fees: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let self_trade_behavior: SelfTradeBehavior = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let order_type: OrderType = crate::borsh_de_or_default(&mut reader)?;
        let client_order_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        let limit: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            side,
            limit_price,
            max_coin_qty,
            max_native_pc_qty_including_fees,
            self_trade_behavior,
            order_type,
            client_order_id,
            limit,
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
pub struct SendTakeInstruction {
    pub side: Side,
    pub limit_price: u64,
    pub max_coin_qty: u64,
    pub max_native_pc_qty_including_fees: u64,
    pub min_coin_qty: u64,
    pub min_native_pc_qty: u64,
    pub limit: u16,
}
impl SendTakeInstruction {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let side: Side = crate::borsh_de_or_default(&mut reader)?;
        let limit_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_coin_qty: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_native_pc_qty_including_fees: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let min_coin_qty: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_native_pc_qty: u64 = crate::borsh_de_or_default(&mut reader)?;
        let limit: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            side,
            limit_price,
            max_coin_qty,
            max_native_pc_qty_including_fees,
            min_coin_qty,
            min_native_pc_qty,
            limit,
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
pub enum Side {
    #[default]
    Bid,
    Ask,
}
impl TryFrom<u8> for Side {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Bid),
            1u8 => Ok(Self::Ask),
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
    Limit,
    ImmediateOrCancel,
    PostOnly,
}
impl TryFrom<u8> for OrderType {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Limit),
            1u8 => Ok(Self::ImmediateOrCancel),
            2u8 => Ok(Self::PostOnly),
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
pub enum SelfTradeBehavior {
    #[default]
    DecrementTake,
    CancelProvide,
    AbortTransaction,
}
impl TryFrom<u8> for SelfTradeBehavior {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::DecrementTake),
            1u8 => Ok(Self::CancelProvide),
            2u8 => Ok(Self::AbortTransaction),
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
pub enum FeeTier {
    #[default]
    Base,
    Srm2,
    Srm3,
    Srm4,
    Srm5,
    Srm6,
    Msrm,
    Stable,
}
impl TryFrom<u8> for FeeTier {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Base),
            1u8 => Ok(Self::Srm2),
            2u8 => Ok(Self::Srm3),
            3u8 => Ok(Self::Srm4),
            4u8 => Ok(Self::Srm5),
            5u8 => Ok(Self::Srm6),
            6u8 => Ok(Self::Msrm),
            7u8 => Ok(Self::Stable),
            _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
        }
    }
}
