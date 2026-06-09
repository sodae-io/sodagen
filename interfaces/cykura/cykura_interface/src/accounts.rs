use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const FACTORY_STATE_ACCOUNT_DISCM: [u8; 8] = [91, 157, 184, 99, 123, 112, 102, 7];
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
pub struct FactoryState {
    pub bump: u8,
    pub owner: Pubkey,
    pub fee_protocol: u8,
}
impl FactoryState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_protocol: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { bump, owner, fee_protocol })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_protocol, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct FactoryStateAccount(pub FactoryState);
impl FactoryStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FACTORY_STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(FactoryState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FACTORY_STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const FEE_STATE_ACCOUNT_DISCM: [u8; 8] = [63, 224, 16, 85, 193, 36, 235, 220];
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
pub struct FeeState {
    pub bump: u8,
    pub fee: u32,
    pub tick_spacing: u16,
}
impl FeeState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let fee: u32 = crate::borsh_de_or_default(&mut reader)?;
        let tick_spacing: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { bump, fee, tick_spacing })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_spacing, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct FeeStateAccount(pub FeeState);
impl FeeStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FEE_STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(FeeState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FEE_STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const OBSERVATION_STATE_ACCOUNT_DISCM: [u8; 8] = [
    122, 174, 197, 53, 129, 9, 165, 132,
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
pub struct ObservationState {
    pub bump: u8,
    pub index: u16,
    pub block_timestamp: u32,
    pub tick_cumulative: i64,
    pub seconds_per_liquidity_cumulative_x32: u64,
    pub initialized: bool,
}
impl ObservationState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let block_timestamp: u32 = crate::borsh_de_or_default(&mut reader)?;
        let tick_cumulative: i64 = crate::borsh_de_or_default(&mut reader)?;
        let seconds_per_liquidity_cumulative_x32: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let initialized: bool = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            bump,
            index,
            block_timestamp,
            tick_cumulative,
            seconds_per_liquidity_cumulative_x32,
            initialized,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.block_timestamp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_cumulative, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.seconds_per_liquidity_cumulative_x32,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.initialized, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ObservationStateAccount(pub ObservationState);
impl ObservationStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != OBSERVATION_STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(ObservationState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&OBSERVATION_STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POOL_STATE_ACCOUNT_DISCM: [u8; 8] = [247, 237, 227, 245, 215, 195, 222, 70];
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
pub struct PoolState {
    pub bump: u8,
    pub token0: Pubkey,
    pub token1: Pubkey,
    pub fee: u32,
    pub tick_spacing: u16,
    pub liquidity: u64,
    pub sqrt_price_x32: u64,
    pub tick: i32,
    pub observation_index: u16,
    pub observation_cardinality: u16,
    pub observation_cardinality_next: u16,
    pub fee_growth_global0_x32: u64,
    pub fee_growth_global1_x32: u64,
    pub protocol_fees_token0: u64,
    pub protocol_fees_token1: u64,
    pub unlocked: bool,
}
impl PoolState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let token0: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token1: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee: u32 = crate::borsh_de_or_default(&mut reader)?;
        let tick_spacing: u16 = crate::borsh_de_or_default(&mut reader)?;
        let liquidity: u64 = crate::borsh_de_or_default(&mut reader)?;
        let sqrt_price_x32: u64 = crate::borsh_de_or_default(&mut reader)?;
        let tick: i32 = crate::borsh_de_or_default(&mut reader)?;
        let observation_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let observation_cardinality: u16 = crate::borsh_de_or_default(&mut reader)?;
        let observation_cardinality_next: u16 = crate::borsh_de_or_default(&mut reader)?;
        let fee_growth_global0_x32: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_growth_global1_x32: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fees_token0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fees_token1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let unlocked: bool = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            bump,
            token0,
            token1,
            fee,
            tick_spacing,
            liquidity,
            sqrt_price_x32,
            tick,
            observation_index,
            observation_cardinality,
            observation_cardinality_next,
            fee_growth_global0_x32,
            fee_growth_global1_x32,
            protocol_fees_token0,
            protocol_fees_token1,
            unlocked,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_spacing, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sqrt_price_x32, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.observation_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.observation_cardinality, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.observation_cardinality_next,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.fee_growth_global0_x32, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_growth_global1_x32, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fees_token0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fees_token1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.unlocked, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PoolStateAccount(pub PoolState);
impl PoolStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POOL_STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(PoolState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POOL_STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POSITION_STATE_ACCOUNT_DISCM: [u8; 8] = [154, 47, 151, 70, 8, 128, 206, 231];
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
pub struct PositionState {
    pub bump: u8,
    pub liquidity: u64,
    pub fee_growth_inside0_last_x32: u64,
    pub fee_growth_inside1_last_x32: u64,
    pub tokens_owed0: u64,
    pub tokens_owed1: u64,
}
impl PositionState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let liquidity: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_growth_inside0_last_x32: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_growth_inside1_last_x32: u64 = crate::borsh_de_or_default(&mut reader)?;
        let tokens_owed0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let tokens_owed1: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            bump,
            liquidity,
            fee_growth_inside0_last_x32,
            fee_growth_inside1_last_x32,
            tokens_owed0,
            tokens_owed1,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.fee_growth_inside0_last_x32,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.fee_growth_inside1_last_x32,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.tokens_owed0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tokens_owed1, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PositionStateAccount(pub PositionState);
impl PositionStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POSITION_STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(PositionState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POSITION_STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POSITION_MANAGER_STATE_ACCOUNT_DISCM: [u8; 8] = [
    227, 6, 99, 255, 69, 66, 233, 214,
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
pub struct PositionManagerState {
    pub bump: u8,
}
impl PositionManagerState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { bump })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PositionManagerStateAccount(pub PositionManagerState);
impl PositionManagerStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POSITION_MANAGER_STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(PositionManagerState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POSITION_MANAGER_STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SWAP_ROUTER_STATE_ACCOUNT_DISCM: [u8; 8] = [41, 249, 0, 167, 210, 45, 161, 47];
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
pub struct SwapRouterState {
    pub bump: u8,
    pub core: Pubkey,
    pub amount_in_cached: u64,
}
impl SwapRouterState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let core: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount_in_cached: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            bump,
            core,
            amount_in_cached,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.core, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_in_cached, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapRouterStateAccount(pub SwapRouterState);
impl SwapRouterStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_ROUTER_STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(SwapRouterState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_ROUTER_STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const TICK_STATE_ACCOUNT_DISCM: [u8; 8] = [137, 76, 253, 128, 85, 226, 97, 148];
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
pub struct TickState {
    pub bump: u8,
    pub tick: i32,
    pub liquidity_net: i64,
    pub liquidity_gross: u64,
    pub fee_growth_outside0_x32: u64,
    pub fee_growth_outside1_x32: u64,
    pub tick_cumulative_outside: i64,
    pub seconds_per_liquidity_outside_x32: u64,
    pub seconds_outside: u32,
}
impl TickState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let tick: i32 = crate::borsh_de_or_default(&mut reader)?;
        let liquidity_net: i64 = crate::borsh_de_or_default(&mut reader)?;
        let liquidity_gross: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_growth_outside0_x32: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_growth_outside1_x32: u64 = crate::borsh_de_or_default(&mut reader)?;
        let tick_cumulative_outside: i64 = crate::borsh_de_or_default(&mut reader)?;
        let seconds_per_liquidity_outside_x32: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let seconds_outside: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            bump,
            tick,
            liquidity_net,
            liquidity_gross,
            fee_growth_outside0_x32,
            fee_growth_outside1_x32,
            tick_cumulative_outside,
            seconds_per_liquidity_outside_x32,
            seconds_outside,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity_net, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity_gross, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_growth_outside0_x32, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_growth_outside1_x32, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_cumulative_outside, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.seconds_per_liquidity_outside_x32,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.seconds_outside, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TickStateAccount(pub TickState);
impl TickStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TICK_STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(TickState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TICK_STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const TICK_BITMAP_STATE_ACCOUNT_DISCM: [u8; 8] = [
    129, 198, 171, 251, 217, 77, 120, 186,
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
pub struct TickBitmapState {
    pub bump: u8,
    pub word_pos: i16,
    pub word: [u64; 4],
}
impl TickBitmapState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let word_pos: i16 = crate::borsh_de_or_default(&mut reader)?;
        let word: [u64; 4] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { bump, word_pos, word })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.word_pos, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.word, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TickBitmapStateAccount(pub TickBitmapState);
impl TickBitmapStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TICK_BITMAP_STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(TickBitmapState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TICK_BITMAP_STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const TOKENIZED_POSITION_STATE_ACCOUNT_DISCM: [u8; 8] = [
    194, 241, 47, 179, 255, 15, 48, 255,
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
pub struct TokenizedPositionState {
    pub bump: u8,
    pub mint: Pubkey,
    pub pool_id: Pubkey,
    pub tick_lower: i32,
    pub tick_upper: i32,
    pub liquidity: u64,
    pub fee_growth_inside0_last_x32: u64,
    pub fee_growth_inside1_last_x32: u64,
    pub tokens_owed0: u64,
    pub tokens_owed1: u64,
}
impl TokenizedPositionState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let tick_lower: i32 = crate::borsh_de_or_default(&mut reader)?;
        let tick_upper: i32 = crate::borsh_de_or_default(&mut reader)?;
        let liquidity: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_growth_inside0_last_x32: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_growth_inside1_last_x32: u64 = crate::borsh_de_or_default(&mut reader)?;
        let tokens_owed0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let tokens_owed1: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            bump,
            mint,
            pool_id,
            tick_lower,
            tick_upper,
            liquidity,
            fee_growth_inside0_last_x32,
            fee_growth_inside1_last_x32,
            tokens_owed0,
            tokens_owed1,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_lower, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_upper, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.fee_growth_inside0_last_x32,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.fee_growth_inside1_last_x32,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.tokens_owed0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tokens_owed1, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TokenizedPositionStateAccount(pub TokenizedPositionState);
impl TokenizedPositionStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TOKENIZED_POSITION_STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(TokenizedPositionState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TOKENIZED_POSITION_STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
