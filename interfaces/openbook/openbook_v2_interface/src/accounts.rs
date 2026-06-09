use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const MARKET_ACCOUNT_DISCM: [u8; 8] = [219, 190, 213, 55, 0, 227, 198, 154];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct Market {
    pub bump: u8,
    pub base_decimals: u8,
    pub quote_decimals: u8,
    pub padding1: [u8; 5],
    pub market_authority: Pubkey,
    pub time_expiry: i64,
    pub collect_fee_admin: Pubkey,
    pub open_orders_admin: NonZeroPubkeyOption,
    pub consume_events_admin: NonZeroPubkeyOption,
    pub close_market_admin: NonZeroPubkeyOption,
    pub name: [u8; 16],
    pub bids: Pubkey,
    pub asks: Pubkey,
    pub event_heap: Pubkey,
    pub oracle_a: NonZeroPubkeyOption,
    pub oracle_b: NonZeroPubkeyOption,
    pub oracle_config: OracleConfig,
    pub quote_lot_size: i64,
    pub base_lot_size: i64,
    pub seq_num: u64,
    pub registration_time: i64,
    pub maker_fee: i64,
    pub taker_fee: i64,
    pub fees_accrued: u128,
    pub fees_to_referrers: u128,
    pub referrer_rebates_accrued: u64,
    pub fees_available: u64,
    pub maker_volume: u128,
    pub taker_volume_wo_oo: u128,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub market_base_vault: Pubkey,
    pub base_deposit_total: u64,
    pub market_quote_vault: Pubkey,
    pub quote_deposit_total: u64,
    #[serde(with = "crate::big_array_serde")]
    pub reserved: [u8; 128],
}
impl Market {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let base_decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let quote_decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding1: [u8; 5] = crate::borsh_de_or_default(&mut reader)?;
        let market_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let time_expiry: i64 = crate::borsh_de_or_default(&mut reader)?;
        let collect_fee_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let open_orders_admin = if reader.is_empty() {
            Default::default()
        } else {
            <NonZeroPubkeyOption>::deserialize(&mut reader)?
        };
        let consume_events_admin = if reader.is_empty() {
            Default::default()
        } else {
            <NonZeroPubkeyOption>::deserialize(&mut reader)?
        };
        let close_market_admin = if reader.is_empty() {
            Default::default()
        } else {
            <NonZeroPubkeyOption>::deserialize(&mut reader)?
        };
        let name: [u8; 16] = crate::borsh_de_or_default(&mut reader)?;
        let bids: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let asks: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let event_heap: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let oracle_a = if reader.is_empty() {
            Default::default()
        } else {
            <NonZeroPubkeyOption>::deserialize(&mut reader)?
        };
        let oracle_b = if reader.is_empty() {
            Default::default()
        } else {
            <NonZeroPubkeyOption>::deserialize(&mut reader)?
        };
        let oracle_config = <OracleConfig>::deserialize(&mut reader)?;
        let quote_lot_size: i64 = crate::borsh_de_or_default(&mut reader)?;
        let base_lot_size: i64 = crate::borsh_de_or_default(&mut reader)?;
        let seq_num: u64 = crate::borsh_de_or_default(&mut reader)?;
        let registration_time: i64 = crate::borsh_de_or_default(&mut reader)?;
        let maker_fee: i64 = crate::borsh_de_or_default(&mut reader)?;
        let taker_fee: i64 = crate::borsh_de_or_default(&mut reader)?;
        let fees_accrued: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fees_to_referrers: u128 = crate::borsh_de_or_default(&mut reader)?;
        let referrer_rebates_accrued: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fees_available: u64 = crate::borsh_de_or_default(&mut reader)?;
        let maker_volume: u128 = crate::borsh_de_or_default(&mut reader)?;
        let taker_volume_wo_oo: u128 = crate::borsh_de_or_default(&mut reader)?;
        let base_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let quote_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let market_base_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_deposit_total: u64 = crate::borsh_de_or_default(&mut reader)?;
        let market_quote_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let quote_deposit_total: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reserved = <[u8; 128] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            bump,
            base_decimals,
            quote_decimals,
            padding1,
            market_authority,
            time_expiry,
            collect_fee_admin,
            open_orders_admin,
            consume_events_admin,
            close_market_admin,
            name,
            bids,
            asks,
            event_heap,
            oracle_a,
            oracle_b,
            oracle_config,
            quote_lot_size,
            base_lot_size,
            seq_num,
            registration_time,
            maker_fee,
            taker_fee,
            fees_accrued,
            fees_to_referrers,
            referrer_rebates_accrued,
            fees_available,
            maker_volume,
            taker_volume_wo_oo,
            base_mint,
            quote_mint,
            market_base_vault,
            base_deposit_total,
            market_quote_vault,
            quote_deposit_total,
            reserved,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_decimals, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_decimals, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.market_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.time_expiry, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collect_fee_admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.open_orders_admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.consume_events_admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.close_market_admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.name, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bids, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.asks, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.event_heap, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.oracle_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.oracle_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.oracle_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_lot_size, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_lot_size, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.seq_num, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.registration_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.maker_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.taker_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fees_accrued, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fees_to_referrers, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.referrer_rebates_accrued, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fees_available, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.maker_volume, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.taker_volume_wo_oo, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.market_base_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_deposit_total, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.market_quote_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_deposit_total, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserved, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MarketAccount(pub Market);
impl MarketAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MARKET_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Market::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MARKET_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const OPEN_ORDERS_ACCOUNT_ACCOUNT_DISCM: [u8; 8] = [
    255, 194, 78, 123, 16, 105, 208, 165,
];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct OpenOrdersAccount {
    pub owner: Pubkey,
    pub market: Pubkey,
    pub name: [u8; 32],
    pub delegate: NonZeroPubkeyOption,
    pub account_num: u32,
    pub bump: u8,
    pub version: u8,
    pub padding: [u8; 2],
    pub position: Position,
    pub open_orders: [OpenOrder; 24],
}
impl OpenOrdersAccount {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let market: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let name: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        let delegate = if reader.is_empty() {
            Default::default()
        } else {
            <NonZeroPubkeyOption>::deserialize(&mut reader)?
        };
        let account_num: u32 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let version: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 2] = crate::borsh_de_or_default(&mut reader)?;
        let position = <Position>::deserialize(&mut reader)?;
        let open_orders: [OpenOrder; 24] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            owner,
            market,
            name,
            delegate,
            account_num,
            bump,
            version,
            padding,
            position,
            open_orders,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.market, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.name, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.delegate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.account_num, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.version, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.open_orders, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct OpenOrdersAccountAccount(pub OpenOrdersAccount);
impl OpenOrdersAccountAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != OPEN_ORDERS_ACCOUNT_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(OpenOrdersAccount::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&OPEN_ORDERS_ACCOUNT_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const OPEN_ORDERS_INDEXER_ACCOUNT_DISCM: [u8; 8] = [
    195, 83, 128, 213, 204, 91, 19, 150,
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
pub struct OpenOrdersIndexer {
    pub bump: u8,
    pub created_counter: u32,
    pub addresses: Vec<Pubkey>,
}
impl OpenOrdersIndexer {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let created_counter: u32 = crate::borsh_de_or_default(&mut reader)?;
        let addresses: Vec<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            bump,
            created_counter,
            addresses,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.created_counter, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.addresses, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct OpenOrdersIndexerAccount(pub OpenOrdersIndexer);
impl OpenOrdersIndexerAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != OPEN_ORDERS_INDEXER_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(OpenOrdersIndexer::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&OPEN_ORDERS_INDEXER_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const STUB_ORACLE_ACCOUNT_DISCM: [u8; 8] = [224, 251, 254, 99, 177, 174, 137, 4];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct StubOracle {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub price: f64,
    pub last_update_ts: i64,
    pub last_update_slot: u64,
    pub deviation: f64,
    #[serde(with = "crate::big_array_serde")]
    pub reserved: [u8; 104],
}
impl StubOracle {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let price: f64 = crate::borsh_de_or_default(&mut reader)?;
        let last_update_ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let last_update_slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        let deviation: f64 = crate::borsh_de_or_default(&mut reader)?;
        let reserved = <[u8; 104] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            owner,
            mint,
            price,
            last_update_ts,
            last_update_slot,
            deviation,
            reserved,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_update_ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_update_slot, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.deviation, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserved, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct StubOracleAccount(pub StubOracle);
impl StubOracleAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != STUB_ORACLE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(StubOracle::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&STUB_ORACLE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const BOOK_SIDE_ACCOUNT_DISCM: [u8; 8] = [72, 44, 225, 141, 178, 130, 97, 57];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct BookSide {
    pub roots: [OrderTreeRoot; 2],
    pub reserved_roots: [OrderTreeRoot; 4],
    #[serde(with = "crate::big_array_serde")]
    pub reserved: [u8; 256],
    pub nodes: OrderTreeNodes,
}
impl BookSide {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let roots: [OrderTreeRoot; 2] = crate::borsh_de_or_default(&mut reader)?;
        let reserved_roots: [OrderTreeRoot; 4] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let reserved = <[u8; 256] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let nodes = <OrderTreeNodes>::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            roots,
            reserved_roots,
            reserved,
            nodes,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.roots, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserved_roots, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserved, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.nodes, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BookSideAccount(pub BookSide);
impl BookSideAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BOOK_SIDE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(BookSide::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BOOK_SIDE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EVENT_HEAP_ACCOUNT_DISCM: [u8; 8] = [119, 59, 61, 19, 165, 84, 57, 175];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct EventHeap {
    pub header: EventHeapHeader,
    #[serde(with = "crate::big_array_serde")]
    pub nodes: [EventNode; 600],
    #[serde(with = "crate::big_array_serde")]
    pub reserved: [u8; 64],
}
impl EventHeap {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let header = if reader.is_empty() {
            Default::default()
        } else {
            <EventHeapHeader>::deserialize(&mut reader)?
        };
        let nodes = <[EventNode; 600] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let reserved = <[u8; 64] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self { header, nodes, reserved })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.header, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.nodes, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserved, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EventHeapAccount(pub EventHeap);
impl EventHeapAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVENT_HEAP_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(EventHeap::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVENT_HEAP_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
