use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const EVENT_EMITTER_ACCOUNT_DISCM: [u8; 8] = [215, 12, 224, 60, 205, 124, 188, 73];
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
pub struct EventEmitter {
    pub event_id: i64,
}
impl EventEmitter {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let event_id: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { event_id })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.event_id, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EventEmitterAccount(pub EventEmitter);
impl EventEmitterAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EVENT_EMITTER_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(EventEmitter::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EVENT_EMITTER_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LIQUIDITY_ACCOUNT_ACCOUNT_DISCM: [u8; 8] = [
    190, 123, 167, 176, 248, 51, 215, 26,
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
pub struct LiquidityAccount {
    pub pool_registry: Pubkey,
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub amount_deposited: u64,
    pub last_observed_tap: u64,
    pub last_claimed: i64,
    pub total_earned: u64,
    pub created_at: i64,
    pub last_deposit_at: u64,
    pub last_obs_rewards_per_share: [u8; 16],
    #[serde(with = "crate::big_array_serde")]
    pub space: [u8; 104],
}
impl LiquidityAccount {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_registry: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount_deposited: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_observed_tap: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_claimed: i64 = crate::borsh_de_or_default(&mut reader)?;
        let total_earned: u64 = crate::borsh_de_or_default(&mut reader)?;
        let created_at: i64 = crate::borsh_de_or_default(&mut reader)?;
        let last_deposit_at: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_obs_rewards_per_share: [u8; 16] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let space = <[u8; 104] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            pool_registry,
            mint,
            owner,
            amount_deposited,
            last_observed_tap,
            last_claimed,
            total_earned,
            created_at,
            last_deposit_at,
            last_obs_rewards_per_share,
            space,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_registry, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_deposited, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_observed_tap, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_claimed, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_earned, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.created_at, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_deposit_at, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_obs_rewards_per_share, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.space, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LiquidityAccountAccount(pub LiquidityAccount);
impl LiquidityAccountAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LIQUIDITY_ACCOUNT_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(LiquidityAccount::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LIQUIDITY_ACCOUNT_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const HISTORICAL_DECIMAL_ACCOUNT_DISCM: [u8; 8] = [
    144, 127, 32, 118, 167, 32, 56, 53,
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
pub struct HistoricalDecimal {
    pub num: i64,
    pub scale: u32,
    pub pad0: [u8; 4],
}
impl HistoricalDecimal {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let num: i64 = crate::borsh_de_or_default(&mut reader)?;
        let scale: u32 = crate::borsh_de_or_default(&mut reader)?;
        let pad0: [u8; 4] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { num, scale, pad0 })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.num, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.scale, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pad0, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct HistoricalDecimalAccount(pub HistoricalDecimal);
impl HistoricalDecimalAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != HISTORICAL_DECIMAL_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(HistoricalDecimal::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&HISTORICAL_DECIMAL_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const HISTORICAL_PRICE_ACCOUNT_DISCM: [u8; 8] = [
    54, 85, 122, 59, 135, 122, 158, 140,
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
pub struct HistoricalPrice {
    pub price: HistoricalDecimal,
    pub slot: u64,
}
impl HistoricalPrice {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let price = if reader.is_empty() {
            Default::default()
        } else {
            <HistoricalDecimal>::deserialize(&mut reader)?
        };
        let slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { price, slot })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.slot, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct HistoricalPriceAccount(pub HistoricalPrice);
impl HistoricalPriceAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != HISTORICAL_PRICE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(HistoricalPrice::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&HISTORICAL_PRICE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const ORACLE_PRICE_HISTORY_ACCOUNT_DISCM: [u8; 8] = [
    180, 252, 163, 223, 218, 49, 27, 242,
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
pub struct OraclePriceHistory {
    pub oracle_type: u8,
    pub minimum_elapsed_slots: u8,
    pub max_slot_price_staleness: u8,
    pub backup_oracle_type: u8,
    pub backup_oracle2_type: u8,
    pub pad0: [u8; 3],
    pub pool_registry: Pubkey,
    pub oracle_address: Pubkey,
    pub mint: Pubkey,
    pub num_updates: u64,
    pub backup_oracle: Pubkey,
    pub latest_backup_oracle_price: HistoricalPrice,
    pub backup_oracle2: Pubkey,
    pub latest_backup_oracle2_price: HistoricalPrice,
    pub space: [u8; 16],
    #[serde(with = "crate::big_array_serde")]
    pub price_history: [HistoricalPrice; 256],
}
impl OraclePriceHistory {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let oracle_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let minimum_elapsed_slots: u8 = crate::borsh_de_or_default(&mut reader)?;
        let max_slot_price_staleness: u8 = crate::borsh_de_or_default(&mut reader)?;
        let backup_oracle_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let backup_oracle2_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let pad0: [u8; 3] = crate::borsh_de_or_default(&mut reader)?;
        let pool_registry: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let oracle_address: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let num_updates: u64 = crate::borsh_de_or_default(&mut reader)?;
        let backup_oracle: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let latest_backup_oracle_price = if reader.is_empty() {
            Default::default()
        } else {
            <HistoricalPrice>::deserialize(&mut reader)?
        };
        let backup_oracle2: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let latest_backup_oracle2_price = if reader.is_empty() {
            Default::default()
        } else {
            <HistoricalPrice>::deserialize(&mut reader)?
        };
        let space: [u8; 16] = crate::borsh_de_or_default(&mut reader)?;
        let price_history = <[HistoricalPrice; 256] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            oracle_type,
            minimum_elapsed_slots,
            max_slot_price_staleness,
            backup_oracle_type,
            backup_oracle2_type,
            pad0,
            pool_registry,
            oracle_address,
            mint,
            num_updates,
            backup_oracle,
            latest_backup_oracle_price,
            backup_oracle2,
            latest_backup_oracle2_price,
            space,
            price_history,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.oracle_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.minimum_elapsed_slots, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_slot_price_staleness, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.backup_oracle_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.backup_oracle2_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pad0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_registry, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.oracle_address, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.num_updates, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.backup_oracle, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.latest_backup_oracle_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.backup_oracle2, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.latest_backup_oracle2_price,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.space, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.price_history, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct OraclePriceHistoryAccount(pub OraclePriceHistory);
impl OraclePriceHistoryAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ORACLE_PRICE_HISTORY_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(OraclePriceHistory::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ORACLE_PRICE_HISTORY_ACCOUNT_DISCM)?;
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
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct Pair {
    pub pool_registry: Pubkey,
    pub mints: (Pubkey, Pubkey),
    pub fee_collector: (Pubkey, Pubkey),
    pub normal_route_fee_rates: (u16, u16),
    pub total_fees_generated_native: ([u8; 16], [u8; 16]),
    pub total_historical_volume: [u8; 16],
    pub total_internally_swapped: ([u8; 16], [u8; 16]),
    pub preferred_route_fee_rates: (u16, u16),
    #[serde(with = "crate::big_array_serde")]
    pub space: [u8; 124],
}
impl Pair {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_registry: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mints: (Pubkey, Pubkey) = crate::borsh_de_or_default(&mut reader)?;
        let fee_collector: (Pubkey, Pubkey) = crate::borsh_de_or_default(&mut reader)?;
        let normal_route_fee_rates: (u16, u16) = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let total_fees_generated_native: ([u8; 16], [u8; 16]) = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let total_historical_volume: [u8; 16] = crate::borsh_de_or_default(&mut reader)?;
        let total_internally_swapped: ([u8; 16], [u8; 16]) = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let preferred_route_fee_rates: (u16, u16) = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let space = <[u8; 124] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            pool_registry,
            mints,
            fee_collector,
            normal_route_fee_rates,
            total_fees_generated_native,
            total_historical_volume,
            total_internally_swapped,
            preferred_route_fee_rates,
            space,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_registry, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mints, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_collector, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.normal_route_fee_rates, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.total_fees_generated_native,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.total_historical_volume, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_internally_swapped, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.preferred_route_fee_rates, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.space, &mut writer)?;
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
pub const SSL_POOL_ACCOUNT_DISCM: [u8; 8] = [206, 97, 114, 137, 251, 86, 247, 135];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct SSLPool {
    pub status: u8,
    pub asset_type: u8,
    pub pad0: [u8; 6],
    pub mint: Pubkey,
    pub mint_decimals: u8,
    pub bump: u8,
    pub pad1: [u8; 6],
    pub total_accumulated_lp_reward: u64,
    pub total_liquidity_deposits: u64,
    pub oracle_price_histories: [Pubkey; 3],
    pub math_params: SSLMathParams,
    pub rewards_per_share: [u8; 16],
    #[serde(with = "crate::big_array_serde")]
    pub space: [u8; 48],
}
impl SSLPool {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let status: u8 = crate::borsh_de_or_default(&mut reader)?;
        let asset_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let pad0: [u8; 6] = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint_decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let pad1: [u8; 6] = crate::borsh_de_or_default(&mut reader)?;
        let total_accumulated_lp_reward: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_liquidity_deposits: u64 = crate::borsh_de_or_default(&mut reader)?;
        let oracle_price_histories: [Pubkey; 3] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let math_params = if reader.is_empty() {
            Default::default()
        } else {
            <SSLMathParams>::deserialize(&mut reader)?
        };
        let rewards_per_share: [u8; 16] = crate::borsh_de_or_default(&mut reader)?;
        let space = <[u8; 48] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            status,
            asset_type,
            pad0,
            mint,
            mint_decimals,
            bump,
            pad1,
            total_accumulated_lp_reward,
            total_liquidity_deposits,
            oracle_price_histories,
            math_params,
            rewards_per_share,
            space,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.status, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.asset_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pad0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint_decimals, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pad1, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.total_accumulated_lp_reward,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.total_liquidity_deposits, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.oracle_price_histories, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.math_params, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.rewards_per_share, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.space, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SSLPoolAccount(pub SSLPool);
impl SSLPoolAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SSL_POOL_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(SSLPool::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SSL_POOL_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POOL_REGISTRY_ACCOUNT_DISCM: [u8; 8] = [113, 149, 124, 60, 130, 240, 64, 157];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct PoolRegistry {
    pub admin: Pubkey,
    pub seed: Pubkey,
    pub suspend_admin: Pubkey,
    pub bump: u8,
    pub pricing_algo: u8,
    pub pad0: [u8; 6],
    pub num_entries: u32,
    pub pad1: [u8; 4],
    pub categorical_pool_token_ratios: [u16; 16],
    #[serde(with = "crate::big_array_serde")]
    pub space: [u8; 96],
    pub entries: [SSLPool; 32],
}
impl PoolRegistry {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let seed: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let suspend_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let pricing_algo: u8 = crate::borsh_de_or_default(&mut reader)?;
        let pad0: [u8; 6] = crate::borsh_de_or_default(&mut reader)?;
        let num_entries: u32 = crate::borsh_de_or_default(&mut reader)?;
        let pad1: [u8; 4] = crate::borsh_de_or_default(&mut reader)?;
        let categorical_pool_token_ratios: [u16; 16] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let space = <[u8; 96] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let entries = <[SSLPool; 32] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            admin,
            seed,
            suspend_admin,
            bump,
            pricing_algo,
            pad0,
            num_entries,
            pad1,
            categorical_pool_token_ratios,
            space,
            entries,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.seed, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.suspend_admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pricing_algo, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pad0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.num_entries, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pad1, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.categorical_pool_token_ratios,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.space, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.entries, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PoolRegistryAccount(pub PoolRegistry);
impl PoolRegistryAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POOL_REGISTRY_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(PoolRegistry::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POOL_REGISTRY_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
