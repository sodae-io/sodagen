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
pub struct NonZeroPubkeyOption {
    pub key: Pubkey,
}
impl NonZeroPubkeyOption {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { key })
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
pub struct Position {
    pub bids_base_lots: i64,
    pub asks_base_lots: i64,
    pub base_free_native: u64,
    pub quote_free_native: u64,
    pub locked_maker_fees: u64,
    pub referrer_rebates_available: u64,
    pub penalty_heap_count: u64,
    pub maker_volume: u128,
    pub taker_volume: u128,
    pub bids_quote_lots: i64,
    #[serde(with = "crate::big_array_serde")]
    pub reserved: [u8; 64],
}
impl Position {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bids_base_lots: i64 = crate::borsh_de_or_default(&mut reader)?;
        let asks_base_lots: i64 = crate::borsh_de_or_default(&mut reader)?;
        let base_free_native: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_free_native: u64 = crate::borsh_de_or_default(&mut reader)?;
        let locked_maker_fees: u64 = crate::borsh_de_or_default(&mut reader)?;
        let referrer_rebates_available: u64 = crate::borsh_de_or_default(&mut reader)?;
        let penalty_heap_count: u64 = crate::borsh_de_or_default(&mut reader)?;
        let maker_volume: u128 = crate::borsh_de_or_default(&mut reader)?;
        let taker_volume: u128 = crate::borsh_de_or_default(&mut reader)?;
        let bids_quote_lots: i64 = crate::borsh_de_or_default(&mut reader)?;
        let reserved = <[u8; 64] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            bids_base_lots,
            asks_base_lots,
            base_free_native,
            quote_free_native,
            locked_maker_fees,
            referrer_rebates_available,
            penalty_heap_count,
            maker_volume,
            taker_volume,
            bids_quote_lots,
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
pub struct OpenOrder {
    pub id: u128,
    pub client_id: u64,
    pub locked_price: i64,
    pub is_free: u8,
    pub side_and_tree: u8,
    pub padding: [u8; 6],
}
impl OpenOrder {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let id: u128 = crate::borsh_de_or_default(&mut reader)?;
        let client_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        let locked_price: i64 = crate::borsh_de_or_default(&mut reader)?;
        let is_free: u8 = crate::borsh_de_or_default(&mut reader)?;
        let side_and_tree: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 6] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            id,
            client_id,
            locked_price,
            is_free,
            side_and_tree,
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
pub struct OracleConfig {
    pub conf_filter: f64,
    pub max_staleness_slots: i64,
    #[serde(with = "crate::big_array_serde")]
    pub reserved: [u8; 72],
}
impl OracleConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let conf_filter: f64 = crate::borsh_de_or_default(&mut reader)?;
        let max_staleness_slots: i64 = crate::borsh_de_or_default(&mut reader)?;
        let reserved = <[u8; 72] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            conf_filter,
            max_staleness_slots,
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
pub struct OracleConfigParams {
    pub conf_filter: f32,
    pub max_staleness_slots: Option<u32>,
}
impl OracleConfigParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let conf_filter: f32 = crate::borsh_de_or_default(&mut reader)?;
        let max_staleness_slots: Option<u32> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            conf_filter,
            max_staleness_slots,
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
pub struct EventHeapHeader {
    pub free_head: u16,
    pub used_head: u16,
    pub count: u16,
    pub padd: u16,
    pub seq_num: u64,
}
impl EventHeapHeader {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let free_head: u16 = crate::borsh_de_or_default(&mut reader)?;
        let used_head: u16 = crate::borsh_de_or_default(&mut reader)?;
        let count: u16 = crate::borsh_de_or_default(&mut reader)?;
        let padd: u16 = crate::borsh_de_or_default(&mut reader)?;
        let seq_num: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            free_head,
            used_head,
            count,
            padd,
            seq_num,
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
pub struct EventNode {
    pub next: u16,
    pub prev: u16,
    pub pad: [u8; 4],
    pub event: AnyEvent,
}
impl EventNode {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let next: u16 = crate::borsh_de_or_default(&mut reader)?;
        let prev: u16 = crate::borsh_de_or_default(&mut reader)?;
        let pad: [u8; 4] = crate::borsh_de_or_default(&mut reader)?;
        let event = <AnyEvent>::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self { next, prev, pad, event })
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
pub struct AnyEvent {
    pub event_type: u8,
    #[serde(with = "crate::big_array_serde")]
    pub padding: [u8; 143],
}
impl AnyEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let event_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding = <[u8; 143] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self { event_type, padding })
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
pub struct FillEvent {
    pub event_type: u8,
    pub taker_side: u8,
    pub maker_out: u8,
    pub maker_slot: u8,
    pub padding: [u8; 4],
    pub timestamp: u64,
    pub seq_num: u64,
    pub maker: Pubkey,
    pub maker_timestamp: u64,
    pub taker: Pubkey,
    pub taker_client_order_id: u64,
    pub price: i64,
    pub peg_limit: i64,
    pub quantity: i64,
    pub maker_client_order_id: u64,
    pub reserved: [u8; 8],
}
impl FillEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let event_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let taker_side: u8 = crate::borsh_de_or_default(&mut reader)?;
        let maker_out: u8 = crate::borsh_de_or_default(&mut reader)?;
        let maker_slot: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 4] = crate::borsh_de_or_default(&mut reader)?;
        let timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        let seq_num: u64 = crate::borsh_de_or_default(&mut reader)?;
        let maker: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let maker_timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        let taker: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let taker_client_order_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        let price: i64 = crate::borsh_de_or_default(&mut reader)?;
        let peg_limit: i64 = crate::borsh_de_or_default(&mut reader)?;
        let quantity: i64 = crate::borsh_de_or_default(&mut reader)?;
        let maker_client_order_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reserved: [u8; 8] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            event_type,
            taker_side,
            maker_out,
            maker_slot,
            padding,
            timestamp,
            seq_num,
            maker,
            maker_timestamp,
            taker,
            taker_client_order_id,
            price,
            peg_limit,
            quantity,
            maker_client_order_id,
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
pub struct OutEvent {
    pub event_type: u8,
    pub side: u8,
    pub owner_slot: u8,
    pub padding0: [u8; 5],
    pub timestamp: u64,
    pub seq_num: u64,
    pub owner: Pubkey,
    pub quantity: i64,
    #[serde(with = "crate::big_array_serde")]
    pub padding1: [u8; 80],
}
impl OutEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let event_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let side: u8 = crate::borsh_de_or_default(&mut reader)?;
        let owner_slot: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding0: [u8; 5] = crate::borsh_de_or_default(&mut reader)?;
        let timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        let seq_num: u64 = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let quantity: i64 = crate::borsh_de_or_default(&mut reader)?;
        let padding1 = <[u8; 80] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            event_type,
            side,
            owner_slot,
            padding0,
            timestamp,
            seq_num,
            owner,
            quantity,
            padding1,
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
pub struct InnerNode {
    pub tag: u8,
    pub padding: [u8; 3],
    pub prefix_len: u32,
    pub key: u128,
    pub children: [u32; 2],
    pub child_earliest_expiry: [u64; 2],
    #[serde(with = "crate::big_array_serde")]
    pub reserved: [u8; 40],
}
impl InnerNode {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let tag: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 3] = crate::borsh_de_or_default(&mut reader)?;
        let prefix_len: u32 = crate::borsh_de_or_default(&mut reader)?;
        let key: u128 = crate::borsh_de_or_default(&mut reader)?;
        let children: [u32; 2] = crate::borsh_de_or_default(&mut reader)?;
        let child_earliest_expiry: [u64; 2] = crate::borsh_de_or_default(&mut reader)?;
        let reserved = <[u8; 40] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            tag,
            padding,
            prefix_len,
            key,
            children,
            child_earliest_expiry,
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
pub struct LeafNode {
    pub tag: u8,
    pub owner_slot: u8,
    pub time_in_force: u16,
    pub padding: [u8; 4],
    pub key: u128,
    pub owner: Pubkey,
    pub quantity: i64,
    pub timestamp: u64,
    pub peg_limit: i64,
    pub client_order_id: u64,
}
impl LeafNode {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let tag: u8 = crate::borsh_de_or_default(&mut reader)?;
        let owner_slot: u8 = crate::borsh_de_or_default(&mut reader)?;
        let time_in_force: u16 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 4] = crate::borsh_de_or_default(&mut reader)?;
        let key: u128 = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let quantity: i64 = crate::borsh_de_or_default(&mut reader)?;
        let timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        let peg_limit: i64 = crate::borsh_de_or_default(&mut reader)?;
        let client_order_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            tag,
            owner_slot,
            time_in_force,
            padding,
            key,
            owner,
            quantity,
            timestamp,
            peg_limit,
            client_order_id,
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
pub struct AnyNode {
    pub tag: u8,
    #[serde(with = "crate::big_array_serde")]
    pub data: [u8; 87],
}
impl AnyNode {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let tag: u8 = crate::borsh_de_or_default(&mut reader)?;
        let data = <[u8; 87] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self { tag, data })
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
pub struct OrderTreeRoot {
    pub maybe_node: u32,
    pub leaf_count: u32,
}
impl OrderTreeRoot {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let maybe_node: u32 = crate::borsh_de_or_default(&mut reader)?;
        let leaf_count: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { maybe_node, leaf_count })
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
pub struct OrderTreeNodes {
    pub order_tree_type: u8,
    pub padding: [u8; 3],
    pub bump_index: u32,
    pub free_list_len: u32,
    pub free_list_head: u32,
    #[serde(with = "crate::big_array_serde")]
    pub reserved: [u8; 512],
    #[serde(with = "crate::big_array_serde")]
    pub nodes: [AnyNode; 1024],
}
impl OrderTreeNodes {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let order_tree_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 3] = crate::borsh_de_or_default(&mut reader)?;
        let bump_index: u32 = crate::borsh_de_or_default(&mut reader)?;
        let free_list_len: u32 = crate::borsh_de_or_default(&mut reader)?;
        let free_list_head: u32 = crate::borsh_de_or_default(&mut reader)?;
        let reserved = <[u8; 512] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let nodes = <[AnyNode; 1024] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            order_tree_type,
            padding,
            bump_index,
            free_list_len,
            free_list_head,
            reserved,
            nodes,
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
pub struct I80F48 {
    pub val: i128,
}
impl I80F48 {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let val: i128 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { val })
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
pub struct PlaceOrderArgs {
    pub side: Side,
    pub price_lots: i64,
    pub max_base_lots: i64,
    pub max_quote_lots_including_fees: i64,
    pub client_order_id: u64,
    pub order_type: PlaceOrderType,
    pub expiry_timestamp: u64,
    pub self_trade_behavior: SelfTradeBehavior,
    pub limit: u8,
}
impl PlaceOrderArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let side: Side = crate::borsh_de_or_default(&mut reader)?;
        let price_lots: i64 = crate::borsh_de_or_default(&mut reader)?;
        let max_base_lots: i64 = crate::borsh_de_or_default(&mut reader)?;
        let max_quote_lots_including_fees: i64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let client_order_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        let order_type: PlaceOrderType = crate::borsh_de_or_default(&mut reader)?;
        let expiry_timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        let self_trade_behavior: SelfTradeBehavior = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let limit: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            side,
            price_lots,
            max_base_lots,
            max_quote_lots_including_fees,
            client_order_id,
            order_type,
            expiry_timestamp,
            self_trade_behavior,
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
pub struct PlaceMultipleOrdersArgs {
    pub price_lots: i64,
    pub max_quote_lots_including_fees: i64,
    pub expiry_timestamp: u64,
}
impl PlaceMultipleOrdersArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let price_lots: i64 = crate::borsh_de_or_default(&mut reader)?;
        let max_quote_lots_including_fees: i64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let expiry_timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            price_lots,
            max_quote_lots_including_fees,
            expiry_timestamp,
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
pub struct PlaceOrderPeggedArgs {
    pub side: Side,
    pub price_offset_lots: i64,
    pub peg_limit: i64,
    pub max_base_lots: i64,
    pub max_quote_lots_including_fees: i64,
    pub client_order_id: u64,
    pub order_type: PlaceOrderType,
    pub expiry_timestamp: u64,
    pub self_trade_behavior: SelfTradeBehavior,
    pub limit: u8,
}
impl PlaceOrderPeggedArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let side: Side = crate::borsh_de_or_default(&mut reader)?;
        let price_offset_lots: i64 = crate::borsh_de_or_default(&mut reader)?;
        let peg_limit: i64 = crate::borsh_de_or_default(&mut reader)?;
        let max_base_lots: i64 = crate::borsh_de_or_default(&mut reader)?;
        let max_quote_lots_including_fees: i64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let client_order_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        let order_type: PlaceOrderType = crate::borsh_de_or_default(&mut reader)?;
        let expiry_timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        let self_trade_behavior: SelfTradeBehavior = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let limit: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            side,
            price_offset_lots,
            peg_limit,
            max_base_lots,
            max_quote_lots_including_fees,
            client_order_id,
            order_type,
            expiry_timestamp,
            self_trade_behavior,
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
pub struct PlaceTakeOrderArgs {
    pub side: Side,
    pub price_lots: i64,
    pub max_base_lots: i64,
    pub max_quote_lots_including_fees: i64,
    pub order_type: PlaceOrderType,
    pub limit: u8,
}
impl PlaceTakeOrderArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let side: Side = crate::borsh_de_or_default(&mut reader)?;
        let price_lots: i64 = crate::borsh_de_or_default(&mut reader)?;
        let max_base_lots: i64 = crate::borsh_de_or_default(&mut reader)?;
        let max_quote_lots_including_fees: i64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let order_type: PlaceOrderType = crate::borsh_de_or_default(&mut reader)?;
        let limit: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            side,
            price_lots,
            max_base_lots,
            max_quote_lots_including_fees,
            order_type,
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
pub enum OracleType {
    #[default]
    Pyth,
    Stub,
    SwitchboardV1,
    SwitchboardV2,
    RaydiumClmm,
}
impl TryFrom<u8> for OracleType {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Pyth),
            1u8 => Ok(Self::Stub),
            2u8 => Ok(Self::SwitchboardV1),
            3u8 => Ok(Self::SwitchboardV2),
            4u8 => Ok(Self::RaydiumClmm),
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
pub enum OrderState {
    #[default]
    Valid,
    Invalid,
    Skipped,
}
impl TryFrom<u8> for OrderState {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Valid),
            1u8 => Ok(Self::Invalid),
            2u8 => Ok(Self::Skipped),
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
pub enum BookSideOrderTree {
    #[default]
    Fixed,
    OraclePegged,
}
impl TryFrom<u8> for BookSideOrderTree {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Fixed),
            1u8 => Ok(Self::OraclePegged),
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
pub enum EventType {
    #[default]
    Fill,
    Out,
}
impl TryFrom<u8> for EventType {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Fill),
            1u8 => Ok(Self::Out),
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
pub enum NodeTag {
    #[default]
    Uninitialized,
    InnerNode,
    LeafNode,
    FreeNode,
    LastFreeNode,
}
impl TryFrom<u8> for NodeTag {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Uninitialized),
            1u8 => Ok(Self::InnerNode),
            2u8 => Ok(Self::LeafNode),
            3u8 => Ok(Self::FreeNode),
            4u8 => Ok(Self::LastFreeNode),
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
pub enum PlaceOrderType {
    #[default]
    Limit,
    ImmediateOrCancel,
    PostOnly,
    Market,
    PostOnlySlide,
    FillOrKill,
}
impl TryFrom<u8> for PlaceOrderType {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Limit),
            1u8 => Ok(Self::ImmediateOrCancel),
            2u8 => Ok(Self::PostOnly),
            3u8 => Ok(Self::Market),
            4u8 => Ok(Self::PostOnlySlide),
            5u8 => Ok(Self::FillOrKill),
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
pub enum PostOrderType {
    #[default]
    Limit,
    PostOnly,
    PostOnlySlide,
}
impl TryFrom<u8> for PostOrderType {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Limit),
            1u8 => Ok(Self::PostOnly),
            2u8 => Ok(Self::PostOnlySlide),
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
pub enum SideAndOrderTree {
    #[default]
    BidFixed,
    AskFixed,
    BidOraclePegged,
    AskOraclePegged,
}
impl TryFrom<u8> for SideAndOrderTree {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::BidFixed),
            1u8 => Ok(Self::AskFixed),
            2u8 => Ok(Self::BidOraclePegged),
            3u8 => Ok(Self::AskOraclePegged),
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
pub enum OrderParams {
    #[default]
    Market,
    ImmediateOrCancel { price_lots: i64 },
    Fixed { price_lots: i64, order_type: PostOrderType },
    OraclePegged { price_offset_lots: i64, order_type: PostOrderType, peg_limit: i64 },
    FillOrKill { price_lots: i64 },
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
pub enum OrderTreeType {
    #[default]
    Bids,
    Asks,
}
impl TryFrom<u8> for OrderTreeType {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Bids),
            1u8 => Ok(Self::Asks),
            _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
        }
    }
}
