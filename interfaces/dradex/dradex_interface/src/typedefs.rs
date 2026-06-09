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
pub struct FeeRates {
    pub maker: u64,
    pub taker: u64,
}
impl FeeRates {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let maker: u64 = crate::borsh_de_or_default(&mut reader)?;
        let taker: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { maker, taker })
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
pub struct MarketConfig {
    pub t0_lot_size: u64,
    pub t1_lot_size: u64,
    pub fee_rates: FeeRates,
}
impl MarketConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let t0_lot_size: u64 = crate::borsh_de_or_default(&mut reader)?;
        let t1_lot_size: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_rates = if reader.is_empty() {
            Default::default()
        } else {
            <FeeRates>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            t0_lot_size,
            t1_lot_size,
            fee_rates,
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
pub struct MarketInput {
    pub t0_lot_size: u64,
    pub t1_lot_size: u64,
    pub fee_tier: u8,
}
impl MarketInput {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let t0_lot_size: u64 = crate::borsh_de_or_default(&mut reader)?;
        let t1_lot_size: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_tier: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            t0_lot_size,
            t1_lot_size,
            fee_tier,
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
pub struct OrderInput {
    pub side: u8,
    pub limit_price: u64,
    pub amount: u64,
    pub client_order_id: u64,
    pub order_type: u8,
    pub limit_total: Option<u64>,
    pub min_amount_out: Option<u64>,
}
impl OrderInput {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let side: u8 = crate::borsh_de_or_default(&mut reader)?;
        let limit_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let client_order_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        let order_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let limit_total: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let min_amount_out: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            side,
            limit_price,
            amount,
            client_order_id,
            order_type,
            limit_total,
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
pub struct OrderBook {
    pub bids: Pubkey,
    pub asks: Pubkey,
    pub next_seq_num: u64,
}
impl OrderBook {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bids: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let asks: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let next_seq_num: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { bids, asks, next_seq_num })
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
pub struct CancelOrderInput {
    pub side: u8,
    pub order_id: u128,
}
impl CancelOrderInput {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let side: u8 = crate::borsh_de_or_default(&mut reader)?;
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
pub struct EventItem {
    pub flag: u8,
    pub fee_tier: UserFeeTier,
    pub padding: [u8; 6],
    pub key: u128,
    pub owner: [u64; 4],
    pub quantity: u64,
    pub total: u64,
}
impl EventItem {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let flag: u8 = crate::borsh_de_or_default(&mut reader)?;
        let fee_tier: UserFeeTier = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 6] = crate::borsh_de_or_default(&mut reader)?;
        let key: u128 = crate::borsh_de_or_default(&mut reader)?;
        let owner: [u64; 4] = crate::borsh_de_or_default(&mut reader)?;
        let quantity: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            flag,
            fee_tier,
            padding,
            key,
            owner,
            quantity,
            total,
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
pub struct DexConfigInput {
    pub crank_penalty: u16,
}
impl DexConfigInput {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let crank_penalty: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { crank_penalty })
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
pub struct DexUserFeeTierInput {
    pub fee_tier: u8,
}
impl DexUserFeeTierInput {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let fee_tier: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { fee_tier })
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
pub struct ConsumeEventsInput {
    pub limit: Option<u32>,
}
impl ConsumeEventsInput {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let limit: Option<u32> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { limit })
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
    Default,
    ImmediateOrCancel,
    PostOnly,
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
pub enum MarketFeeTier {
    #[default]
    Default,
    None,
    Stable,
    Classic,
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
pub enum UserFeeTier {
    #[default]
    Default,
    None,
    Partner,
    Referrer,
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
pub enum FeeRole {
    #[default]
    Taker,
    Maker,
    Both,
}
