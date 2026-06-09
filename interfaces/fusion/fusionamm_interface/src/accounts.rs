use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const FUSION_POOL_ACCOUNT_DISCM: [u8; 8] = [254, 204, 207, 98, 25, 181, 29, 67];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct FusionPool {
    pub bump: [u8; 1],
    pub version: u16,
    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,
    pub token_vault_a: Pubkey,
    pub token_vault_b: Pubkey,
    pub tick_spacing: u16,
    pub tick_spacing_seed: [u8; 2],
    pub fee_rate: u16,
    pub protocol_fee_rate: u16,
    pub unused_0: u32,
    pub liquidity: u128,
    pub sqrt_price: u128,
    pub tick_current_index: i32,
    pub protocol_fee_owed_a: u64,
    pub protocol_fee_owed_b: u64,
    pub fee_growth_global_a: u128,
    pub fee_growth_global_b: u128,
    pub orders_total_amount_a: u64,
    pub orders_total_amount_b: u64,
    pub orders_filled_amount_a: u64,
    pub orders_filled_amount_b: u64,
    pub olp_fee_owed_a: u64,
    pub olp_fee_owed_b: u64,
    pub ma_sqrt_price: u128,
    pub last_swap_timestamp: u64,
    #[serde(with = "crate::big_array_serde")]
    pub reserved: [u8; 116],
}
impl FusionPool {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: [u8; 1] = crate::borsh_de_or_default(&mut reader)?;
        let version: u16 = crate::borsh_de_or_default(&mut reader)?;
        let token_mint_a: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_mint_b: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_vault_a: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_vault_b: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let tick_spacing: u16 = crate::borsh_de_or_default(&mut reader)?;
        let tick_spacing_seed: [u8; 2] = crate::borsh_de_or_default(&mut reader)?;
        let fee_rate: u16 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee_rate: u16 = crate::borsh_de_or_default(&mut reader)?;
        let unused_0: u32 = crate::borsh_de_or_default(&mut reader)?;
        let liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let sqrt_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        let tick_current_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee_owed_a: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee_owed_b: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_growth_global_a: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fee_growth_global_b: u128 = crate::borsh_de_or_default(&mut reader)?;
        let orders_total_amount_a: u64 = crate::borsh_de_or_default(&mut reader)?;
        let orders_total_amount_b: u64 = crate::borsh_de_or_default(&mut reader)?;
        let orders_filled_amount_a: u64 = crate::borsh_de_or_default(&mut reader)?;
        let orders_filled_amount_b: u64 = crate::borsh_de_or_default(&mut reader)?;
        let olp_fee_owed_a: u64 = crate::borsh_de_or_default(&mut reader)?;
        let olp_fee_owed_b: u64 = crate::borsh_de_or_default(&mut reader)?;
        let ma_sqrt_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        let last_swap_timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reserved = <[u8; 116] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            bump,
            version,
            token_mint_a,
            token_mint_b,
            token_vault_a,
            token_vault_b,
            tick_spacing,
            tick_spacing_seed,
            fee_rate,
            protocol_fee_rate,
            unused_0,
            liquidity,
            sqrt_price,
            tick_current_index,
            protocol_fee_owed_a,
            protocol_fee_owed_b,
            fee_growth_global_a,
            fee_growth_global_b,
            orders_total_amount_a,
            orders_total_amount_b,
            orders_filled_amount_a,
            orders_filled_amount_b,
            olp_fee_owed_a,
            olp_fee_owed_b,
            ma_sqrt_price,
            last_swap_timestamp,
            reserved,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.version, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_mint_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_mint_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_vault_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_vault_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_spacing, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_spacing_seed, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fee_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.unused_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sqrt_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_current_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fee_owed_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fee_owed_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_growth_global_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_growth_global_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.orders_total_amount_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.orders_total_amount_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.orders_filled_amount_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.orders_filled_amount_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.olp_fee_owed_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.olp_fee_owed_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.ma_sqrt_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_swap_timestamp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserved, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct FusionPoolAccount(pub FusionPool);
impl FusionPoolAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FUSION_POOL_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(FusionPool::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FUSION_POOL_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const FUSION_POOLS_CONFIG_ACCOUNT_DISCM: [u8; 8] = [
    191, 199, 19, 11, 75, 86, 239, 169,
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
pub struct FusionPoolsConfig {
    pub version: u16,
    pub fee_authority: Pubkey,
    pub collect_protocol_fees_authority: Pubkey,
    pub token_badge_authority: Pubkey,
    pub default_protocol_fee_rate: u16,
    pub unused_0: u16,
    pub unused_1: u16,
    #[serde(with = "crate::big_array_serde")]
    pub reserved: [u8; 170],
}
impl FusionPoolsConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let version: u16 = crate::borsh_de_or_default(&mut reader)?;
        let fee_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let collect_protocol_fees_authority: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let token_badge_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let default_protocol_fee_rate: u16 = crate::borsh_de_or_default(&mut reader)?;
        let unused_0: u16 = crate::borsh_de_or_default(&mut reader)?;
        let unused_1: u16 = crate::borsh_de_or_default(&mut reader)?;
        let reserved = <[u8; 170] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            version,
            fee_authority,
            collect_protocol_fees_authority,
            token_badge_authority,
            default_protocol_fee_rate,
            unused_0,
            unused_1,
            reserved,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.version, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.collect_protocol_fees_authority,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.token_badge_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.default_protocol_fee_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.unused_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.unused_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserved, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct FusionPoolsConfigAccount(pub FusionPoolsConfig);
impl FusionPoolsConfigAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FUSION_POOLS_CONFIG_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(FusionPoolsConfig::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FUSION_POOLS_CONFIG_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LIMIT_ORDER_ACCOUNT_DISCM: [u8; 8] = [137, 183, 212, 91, 115, 29, 141, 227];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct LimitOrder {
    pub version: u16,
    pub fusion_pool: Pubkey,
    pub limit_order_mint: Pubkey,
    pub tick_index: i32,
    pub amount: u64,
    pub a_to_b: bool,
    pub age: u64,
    #[serde(with = "crate::big_array_serde")]
    pub reserved: [u8; 128],
}
impl LimitOrder {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let version: u16 = crate::borsh_de_or_default(&mut reader)?;
        let fusion_pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let limit_order_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let tick_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let a_to_b: bool = crate::borsh_de_or_default(&mut reader)?;
        let age: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reserved = <[u8; 128] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            version,
            fusion_pool,
            limit_order_mint,
            tick_index,
            amount,
            a_to_b,
            age,
            reserved,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.version, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fusion_pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.limit_order_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.a_to_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.age, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserved, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LimitOrderAccount(pub LimitOrder);
impl LimitOrderAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LIMIT_ORDER_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(LimitOrder::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LIMIT_ORDER_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POSITION_ACCOUNT_DISCM: [u8; 8] = [170, 188, 143, 228, 122, 64, 247, 208];
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
    pub version: u16,
    pub fusion_pool: Pubkey,
    pub position_mint: Pubkey,
    pub liquidity: u128,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
    pub fee_growth_checkpoint_a: u128,
    pub fee_owed_a: u64,
    pub fee_growth_checkpoint_b: u128,
    pub fee_owed_b: u64,
    #[serde(with = "crate::big_array_serde")]
    pub reserved: [u8; 128],
}
impl Position {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let version: u16 = crate::borsh_de_or_default(&mut reader)?;
        let fusion_pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let tick_lower_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let tick_upper_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let fee_growth_checkpoint_a: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fee_owed_a: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_growth_checkpoint_b: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fee_owed_b: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reserved = <[u8; 128] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            version,
            fusion_pool,
            position_mint,
            liquidity,
            tick_lower_index,
            tick_upper_index,
            fee_growth_checkpoint_a,
            fee_owed_a,
            fee_growth_checkpoint_b,
            fee_owed_b,
            reserved,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.version, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fusion_pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_lower_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_upper_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_growth_checkpoint_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_owed_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_growth_checkpoint_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_owed_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserved, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PositionAccount(pub Position);
impl PositionAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POSITION_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Position::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POSITION_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POSITION_BUNDLE_ACCOUNT_DISCM: [u8; 8] = [129, 169, 175, 65, 185, 95, 32, 100];
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
pub struct PositionBundle {
    pub position_bundle_mint: Pubkey,
    pub position_bitmap: [u8; 32],
}
impl PositionBundle {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let position_bundle_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_bitmap: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            position_bundle_mint,
            position_bitmap,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.position_bundle_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_bitmap, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PositionBundleAccount(pub PositionBundle);
impl PositionBundleAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POSITION_BUNDLE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(PositionBundle::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POSITION_BUNDLE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POSITION_LOCK_ACCOUNT_DISCM: [u8; 8] = [163, 137, 166, 27, 112, 14, 172, 118];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct PositionLock {
    pub position_mint: Pubkey,
    pub position_owner: Pubkey,
    pub fusion_pool: Pubkey,
    pub locked_timestamp: u64,
    pub lock_type: PositionLockType,
    #[serde(with = "crate::big_array_serde")]
    pub reserved: [u8; 128],
}
impl PositionLock {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let position_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fusion_pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let locked_timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        let lock_type: PositionLockType = crate::borsh_de_or_default(&mut reader)?;
        let reserved = <[u8; 128] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            position_mint,
            position_owner,
            fusion_pool,
            locked_timestamp,
            lock_type,
            reserved,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.position_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fusion_pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.locked_timestamp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lock_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserved, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PositionLockAccount(pub PositionLock);
impl PositionLockAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POSITION_LOCK_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(PositionLock::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POSITION_LOCK_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const TICK_ARRAY_ACCOUNT_DISCM: [u8; 8] = [69, 97, 189, 190, 110, 7, 66, 187];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct TickArray {
    pub start_tick_index: i32,
    #[serde(with = "crate::big_array_serde")]
    pub ticks: [Tick; 88],
    pub fusion_pool: Pubkey,
}
impl TickArray {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let start_tick_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let ticks = <[Tick; 88] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let fusion_pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            start_tick_index,
            ticks,
            fusion_pool,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.start_tick_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.ticks, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fusion_pool, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TickArrayAccount(pub TickArray);
impl TickArrayAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TICK_ARRAY_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(TickArray::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TICK_ARRAY_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const TOKEN_BADGE_ACCOUNT_DISCM: [u8; 8] = [116, 219, 204, 229, 249, 116, 255, 150];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct TokenBadge {
    pub token_mint: Pubkey,
    #[serde(with = "crate::big_array_serde")]
    pub reserved: [u8; 128],
}
impl TokenBadge {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let token_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let reserved = <[u8; 128] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self { token_mint, reserved })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.token_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserved, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TokenBadgeAccount(pub TokenBadge);
impl TokenBadgeAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TOKEN_BADGE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(TokenBadge::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TOKEN_BADGE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
