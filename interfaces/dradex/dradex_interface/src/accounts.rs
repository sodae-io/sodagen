use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const DEX_STATE_ACCOUNT_DISCM: [u8; 8] = [181, 89, 162, 169, 251, 158, 9, 60];
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
pub struct DexState {
    pub crank_penalty: u16,
    pub reserved: [u8; 7],
}
impl DexState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let crank_penalty: u16 = crate::borsh_de_or_default(&mut reader)?;
        let reserved: [u8; 7] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { crank_penalty, reserved })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.crank_penalty, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserved, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct DexStateAccount(pub DexState);
impl DexStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEX_STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(DexState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEX_STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const DAO_CONFIG_ACCOUNT_DISCM: [u8; 8] = [55, 209, 87, 224, 30, 202, 192, 246];
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
pub struct DaoConfig {
    pub fund_manager: Pubkey,
    pub reserved: [u64; 32],
}
impl DaoConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let fund_manager: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let reserved: [u64; 32] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { fund_manager, reserved })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.fund_manager, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserved, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct DaoConfigAccount(pub DaoConfig);
impl DaoConfigAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DAO_CONFIG_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(DaoConfig::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DAO_CONFIG_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const PAIR_ACCOUNT_DISCM: [u8; 8] = [85, 72, 49, 176, 182, 228, 141, 82];
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
pub struct Pair {}
impl Pair {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        *__buf = reader;
        Ok(Self {})
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PairAccount(pub Pair);
impl PairAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PAIR_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Pair::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PAIR_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MARKET_ACCOUNT_DISCM: [u8; 8] = [219, 190, 213, 55, 0, 227, 198, 154];
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
pub struct Market {
    pub pair: Pubkey,
    pub t0: Pubkey,
    pub t1: Pubkey,
    pub t0_vault: Pubkey,
    pub t1_vault: Pubkey,
    pub lp_token: Pubkey,
    pub authority: Pubkey,
    pub config: MarketConfig,
    pub order_book: OrderBook,
    pub next_user_id: u64,
    pub token_decimals: [u8; 2],
    pub pool: [u64; 2],
    pub dao_revenue: [u64; 2],
    pub event_queue: Pubkey,
    pub reserved: [u64; 20],
}
impl Market {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pair: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let t0: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let t1: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let t0_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let t1_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lp_token: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let config = if reader.is_empty() {
            Default::default()
        } else {
            <MarketConfig>::deserialize(&mut reader)?
        };
        let order_book = if reader.is_empty() {
            Default::default()
        } else {
            <OrderBook>::deserialize(&mut reader)?
        };
        let next_user_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_decimals: [u8; 2] = crate::borsh_de_or_default(&mut reader)?;
        let pool: [u64; 2] = crate::borsh_de_or_default(&mut reader)?;
        let dao_revenue: [u64; 2] = crate::borsh_de_or_default(&mut reader)?;
        let event_queue: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let reserved: [u64; 20] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pair,
            t0,
            t1,
            t0_vault,
            t1_vault,
            lp_token,
            authority,
            config,
            order_book,
            next_user_id,
            token_decimals,
            pool,
            dao_revenue,
            event_queue,
            reserved,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pair, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.t0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.t1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.t0_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.t1_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_token, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.order_book, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.next_user_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_decimals, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.dao_revenue, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.event_queue, &mut writer)?;
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
pub const MARKET_USER_ACCOUNT_DISCM: [u8; 8] = [236, 236, 149, 82, 151, 154, 15, 23];
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
pub struct MarketUser {
    pub id: u64,
    pub t0_pending: u64,
    pub t1_pending: u64,
    pub t0_unlocked: u64,
    pub t1_unlocked: u64,
}
impl MarketUser {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let id: u64 = crate::borsh_de_or_default(&mut reader)?;
        let t0_pending: u64 = crate::borsh_de_or_default(&mut reader)?;
        let t1_pending: u64 = crate::borsh_de_or_default(&mut reader)?;
        let t0_unlocked: u64 = crate::borsh_de_or_default(&mut reader)?;
        let t1_unlocked: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            id,
            t0_pending,
            t1_pending,
            t0_unlocked,
            t1_unlocked,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.t0_pending, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.t1_pending, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.t0_unlocked, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.t1_unlocked, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MarketUserAccount(pub MarketUser);
impl MarketUserAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MARKET_USER_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(MarketUser::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MARKET_USER_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const DEX_USER_ACCOUNT_DISCM: [u8; 8] = [97, 238, 123, 182, 146, 251, 87, 192];
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
pub struct DexUser {
    pub fee_tier: UserFeeTier,
    pub staked_amount: u64,
}
impl DexUser {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let fee_tier: UserFeeTier = crate::borsh_de_or_default(&mut reader)?;
        let staked_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { fee_tier, staked_amount })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.fee_tier, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.staked_amount, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct DexUserAccount(pub DexUser);
impl DexUserAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEX_USER_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(DexUser::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEX_USER_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const EVENT_QUEUE_ACCOUNT_DISCM: [u8; 8] = [41, 208, 116, 209, 173, 116, 141, 68];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct EventQueue {
    pub len: u32,
    pub padding: [u8; 4],
    #[serde(with = "crate::big_array_serde")]
    pub items: [EventItem; 128],
}
impl EventQueue {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let len: u32 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 4] = crate::borsh_de_or_default(&mut reader)?;
        let items = <[EventItem; 128] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self { len, padding, items })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.len, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.items, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EventQueueAccount(pub EventQueue);
impl EventQueueAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVENT_QUEUE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(EventQueue::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVENT_QUEUE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
