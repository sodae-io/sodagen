use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const MARKET_STATE_V2_ACCOUNT_DISCM: [u8; 8] = [181, 11, 35, 91, 85, 209, 1, 51];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct MarketStateV2 {
    pub inner: MarketState,
    pub open_orders_authority: Pubkey,
    pub prune_authority: Pubkey,
    pub consume_events_authority: Pubkey,
    #[serde(with = "crate::big_array_serde")]
    pub padding: [u8; 992],
}
impl MarketStateV2 {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let inner = if reader.is_empty() {
            Default::default()
        } else {
            <MarketState>::deserialize(&mut reader)?
        };
        let open_orders_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let prune_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let consume_events_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let padding = <[u8; 992] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            inner,
            open_orders_authority,
            prune_authority,
            consume_events_authority,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.inner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.open_orders_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.prune_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.consume_events_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MarketStateV2Account(pub MarketStateV2);
impl MarketStateV2Account {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MARKET_STATE_V2_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(MarketStateV2::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MARKET_STATE_V2_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MARKET_STATE_ACCOUNT_DISCM: [u8; 8] = [0, 125, 123, 215, 95, 96, 164, 194];
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
pub struct MarketState {
    pub account_flags: u64,
    pub own_address: [u64; 4],
    pub vault_signer_nonce: u64,
    pub coin_mint: [u64; 4],
    pub pc_mint: [u64; 4],
    pub coin_vault: [u64; 4],
    pub coin_deposits_total: u64,
    pub coin_fees_accrued: u64,
    pub pc_vault: [u64; 4],
    pub pc_deposits_total: u64,
    pub pc_fees_accrued: u64,
    pub pc_dust_threshold: u64,
    pub req_q: [u64; 4],
    pub event_q: [u64; 4],
    pub bids: [u64; 4],
    pub asks: [u64; 4],
    pub coin_lot_size: u64,
    pub pc_lot_size: u64,
    pub fee_rate_bps: u64,
    pub referrer_rebates_accrued: u64,
}
impl MarketState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let account_flags: u64 = crate::borsh_de_or_default(&mut reader)?;
        let own_address: [u64; 4] = crate::borsh_de_or_default(&mut reader)?;
        let vault_signer_nonce: u64 = crate::borsh_de_or_default(&mut reader)?;
        let coin_mint: [u64; 4] = crate::borsh_de_or_default(&mut reader)?;
        let pc_mint: [u64; 4] = crate::borsh_de_or_default(&mut reader)?;
        let coin_vault: [u64; 4] = crate::borsh_de_or_default(&mut reader)?;
        let coin_deposits_total: u64 = crate::borsh_de_or_default(&mut reader)?;
        let coin_fees_accrued: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pc_vault: [u64; 4] = crate::borsh_de_or_default(&mut reader)?;
        let pc_deposits_total: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pc_fees_accrued: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pc_dust_threshold: u64 = crate::borsh_de_or_default(&mut reader)?;
        let req_q: [u64; 4] = crate::borsh_de_or_default(&mut reader)?;
        let event_q: [u64; 4] = crate::borsh_de_or_default(&mut reader)?;
        let bids: [u64; 4] = crate::borsh_de_or_default(&mut reader)?;
        let asks: [u64; 4] = crate::borsh_de_or_default(&mut reader)?;
        let coin_lot_size: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pc_lot_size: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_rate_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let referrer_rebates_accrued: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            account_flags,
            own_address,
            vault_signer_nonce,
            coin_mint,
            pc_mint,
            coin_vault,
            coin_deposits_total,
            coin_fees_accrued,
            pc_vault,
            pc_deposits_total,
            pc_fees_accrued,
            pc_dust_threshold,
            req_q,
            event_q,
            bids,
            asks,
            coin_lot_size,
            pc_lot_size,
            fee_rate_bps,
            referrer_rebates_accrued,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.account_flags, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.own_address, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault_signer_nonce, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.coin_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pc_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.coin_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.coin_deposits_total, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.coin_fees_accrued, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pc_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pc_deposits_total, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pc_fees_accrued, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pc_dust_threshold, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.req_q, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.event_q, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bids, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.asks, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.coin_lot_size, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pc_lot_size, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_rate_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.referrer_rebates_accrued, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MarketStateAccount(pub MarketState);
impl MarketStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MARKET_STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(MarketState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MARKET_STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const OPEN_ORDERS_ACCOUNT_DISCM: [u8; 8] = [139, 166, 123, 206, 111, 2, 116, 33];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct OpenOrders {
    pub account_flags: u64,
    pub market: [u64; 4],
    pub owner: [u64; 4],
    pub native_coin_free: u64,
    pub native_coin_total: u64,
    pub native_pc_free: u64,
    pub native_pc_total: u64,
    pub free_slot_bits: u128,
    pub is_bid_bits: u128,
    #[serde(with = "crate::big_array_serde")]
    pub orders: [u128; 128],
    #[serde(with = "crate::big_array_serde")]
    pub client_order_ids: [u64; 128],
    pub referrer_rebates_accrued: u64,
}
impl OpenOrders {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let account_flags: u64 = crate::borsh_de_or_default(&mut reader)?;
        let market: [u64; 4] = crate::borsh_de_or_default(&mut reader)?;
        let owner: [u64; 4] = crate::borsh_de_or_default(&mut reader)?;
        let native_coin_free: u64 = crate::borsh_de_or_default(&mut reader)?;
        let native_coin_total: u64 = crate::borsh_de_or_default(&mut reader)?;
        let native_pc_free: u64 = crate::borsh_de_or_default(&mut reader)?;
        let native_pc_total: u64 = crate::borsh_de_or_default(&mut reader)?;
        let free_slot_bits: u128 = crate::borsh_de_or_default(&mut reader)?;
        let is_bid_bits: u128 = crate::borsh_de_or_default(&mut reader)?;
        let orders = <[u128; 128] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let client_order_ids = <[u64; 128] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let referrer_rebates_accrued: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            account_flags,
            market,
            owner,
            native_coin_free,
            native_coin_total,
            native_pc_free,
            native_pc_total,
            free_slot_bits,
            is_bid_bits,
            orders,
            client_order_ids,
            referrer_rebates_accrued,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.account_flags, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.market, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.native_coin_free, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.native_coin_total, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.native_pc_free, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.native_pc_total, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.free_slot_bits, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.is_bid_bits, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.orders, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.client_order_ids, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.referrer_rebates_accrued, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct OpenOrdersAccount(pub OpenOrders);
impl OpenOrdersAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != OPEN_ORDERS_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(OpenOrders::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&OPEN_ORDERS_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REQUEST_QUEUE_HEADER_ACCOUNT_DISCM: [u8; 8] = [
    28, 165, 120, 78, 225, 191, 157, 182,
];
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
pub struct RequestQueueHeader {
    pub account_flags: u64,
    pub head: u64,
    pub count: u64,
    pub next_seq_num: u64,
}
impl RequestQueueHeader {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let account_flags: u64 = crate::borsh_de_or_default(&mut reader)?;
        let head: u64 = crate::borsh_de_or_default(&mut reader)?;
        let count: u64 = crate::borsh_de_or_default(&mut reader)?;
        let next_seq_num: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            account_flags,
            head,
            count,
            next_seq_num,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.account_flags, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.head, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.count, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.next_seq_num, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RequestQueueHeaderAccount(pub RequestQueueHeader);
impl RequestQueueHeaderAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REQUEST_QUEUE_HEADER_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(RequestQueueHeader::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REQUEST_QUEUE_HEADER_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REQUEST_ACCOUNT_DISCM: [u8; 8] = [125, 172, 150, 161, 162, 115, 39, 71];
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
pub struct Request {
    pub request_flags: u8,
    pub owner_slot: u8,
    pub fee_tier: u8,
    pub self_trade_behavior: u8,
    pub padding: [u8; 4],
    pub max_coin_qty_or_cancel_id: u64,
    pub native_pc_qty_locked: u64,
    pub order_id: u128,
    pub owner: [u64; 4],
    pub client_order_id: u64,
}
impl Request {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let request_flags: u8 = crate::borsh_de_or_default(&mut reader)?;
        let owner_slot: u8 = crate::borsh_de_or_default(&mut reader)?;
        let fee_tier: u8 = crate::borsh_de_or_default(&mut reader)?;
        let self_trade_behavior: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 4] = crate::borsh_de_or_default(&mut reader)?;
        let max_coin_qty_or_cancel_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        let native_pc_qty_locked: u64 = crate::borsh_de_or_default(&mut reader)?;
        let order_id: u128 = crate::borsh_de_or_default(&mut reader)?;
        let owner: [u64; 4] = crate::borsh_de_or_default(&mut reader)?;
        let client_order_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            request_flags,
            owner_slot,
            fee_tier,
            self_trade_behavior,
            padding,
            max_coin_qty_or_cancel_id,
            native_pc_qty_locked,
            order_id,
            owner,
            client_order_id,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.request_flags, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner_slot, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_tier, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.self_trade_behavior, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_coin_qty_or_cancel_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.native_pc_qty_locked, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.order_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.client_order_id, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RequestAccount(pub Request);
impl RequestAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REQUEST_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Request::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REQUEST_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REQUEST_VIEW_ACCOUNT_DISCM: [u8; 8] = [206, 80, 233, 109, 31, 54, 182, 125];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub enum RequestView {
    NewOrder {
        side: Side,
        order_type: OrderType,
        owner_slot: u8,
        fee_tier: FeeTier,
        order_id: u128,
        max_coin_qty: u64,
        native_pc_qty_locked: Option<u64>,
        owner: [u64; 4],
        client_order_id: Option<u64>,
        self_trade_behavior: SelfTradeBehavior,
    },
    CancelOrder {
        side: Side,
        order_id: u128,
        cancel_id: u64,
        expected_owner_slot: u8,
        expected_owner: [u64; 4],
        client_order_id: Option<u64>,
    },
}
#[derive(Clone, Debug, PartialEq)]
pub struct RequestViewAccount(pub RequestView);
impl RequestViewAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REQUEST_VIEW_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(RequestView::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REQUEST_VIEW_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EVENT_QUEUE_HEADER_ACCOUNT_DISCM: [u8; 8] = [
    63, 228, 117, 106, 79, 9, 172, 71,
];
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
pub struct EventQueueHeader {
    pub account_flags: u64,
    pub head: u64,
    pub count: u64,
    pub seq_num: u64,
}
impl EventQueueHeader {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let account_flags: u64 = crate::borsh_de_or_default(&mut reader)?;
        let head: u64 = crate::borsh_de_or_default(&mut reader)?;
        let count: u64 = crate::borsh_de_or_default(&mut reader)?;
        let seq_num: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            account_flags,
            head,
            count,
            seq_num,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.account_flags, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.head, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.count, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.seq_num, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EventQueueHeaderAccount(pub EventQueueHeader);
impl EventQueueHeaderAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVENT_QUEUE_HEADER_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(EventQueueHeader::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVENT_QUEUE_HEADER_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EVENT_ACCOUNT_DISCM: [u8; 8] = [125, 192, 125, 158, 9, 115, 152, 233];
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
pub struct Event {
    pub event_flags: u8,
    pub owner_slot: u8,
    pub fee_tier: u8,
    pub _padding: [u8; 5],
    pub native_qty_released: u64,
    pub native_qty_paid: u64,
    pub native_fee_or_rebate: u64,
    pub order_id: u128,
    pub owner: [u64; 4],
    pub client_order_id: u64,
}
impl Event {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let event_flags: u8 = crate::borsh_de_or_default(&mut reader)?;
        let owner_slot: u8 = crate::borsh_de_or_default(&mut reader)?;
        let fee_tier: u8 = crate::borsh_de_or_default(&mut reader)?;
        let _padding: [u8; 5] = crate::borsh_de_or_default(&mut reader)?;
        let native_qty_released: u64 = crate::borsh_de_or_default(&mut reader)?;
        let native_qty_paid: u64 = crate::borsh_de_or_default(&mut reader)?;
        let native_fee_or_rebate: u64 = crate::borsh_de_or_default(&mut reader)?;
        let order_id: u128 = crate::borsh_de_or_default(&mut reader)?;
        let owner: [u64; 4] = crate::borsh_de_or_default(&mut reader)?;
        let client_order_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            event_flags,
            owner_slot,
            fee_tier,
            _padding,
            native_qty_released,
            native_qty_paid,
            native_fee_or_rebate,
            order_id,
            owner,
            client_order_id,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.event_flags, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner_slot, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_tier, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._padding, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.native_qty_released, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.native_qty_paid, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.native_fee_or_rebate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.order_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.client_order_id, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EventAccount(pub Event);
impl EventAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVENT_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Event::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVENT_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EVENT_VIEW_ACCOUNT_DISCM: [u8; 8] = [207, 149, 151, 29, 47, 44, 215, 175];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub enum EventView {
    Fill {
        side: Side,
        maker: bool,
        native_qty_paid: u64,
        native_qty_received: u64,
        native_fee_or_rebate: u64,
        order_id: u128,
        owner: [u64; 4],
        owner_slot: u8,
        fee_tier: FeeTier,
        client_order_id: Option<u64>,
    },
    Out {
        side: Side,
        release_funds: bool,
        native_qty_unlocked: u64,
        native_qty_still_locked: u64,
        order_id: u128,
        owner: [u64; 4],
        owner_slot: u8,
        client_order_id: Option<u64>,
    },
}
#[derive(Clone, Debug, PartialEq)]
pub struct EventViewAccount(pub EventView);
impl EventViewAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVENT_VIEW_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(EventView::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVENT_VIEW_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
