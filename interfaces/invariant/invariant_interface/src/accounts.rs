use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const FEE_TIER_ACCOUNT_DISCM: [u8; 8] = [56, 75, 159, 76, 142, 68, 190, 105];
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
pub struct FeeTier {
    pub fee: FixedPoint,
    pub tick_spacing: u16,
    pub bump: u8,
}
impl FeeTier {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let fee = if reader.is_empty() {
            Default::default()
        } else {
            <FixedPoint>::deserialize(&mut reader)?
        };
        let tick_spacing: u16 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { fee, tick_spacing, bump })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_spacing, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct FeeTierAccount(pub FeeTier);
impl FeeTierAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FEE_TIER_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(FeeTier::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FEE_TIER_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const ORACLE_ACCOUNT_DISCM: [u8; 8] = [139, 194, 131, 179, 140, 179, 229, 244];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct Oracle {
    #[serde(with = "crate::big_array_serde")]
    pub data: [Record; 256],
    pub head: u16,
    pub amount: u16,
    pub size: u16,
}
impl Oracle {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let data = <[Record; 256] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let head: u16 = crate::borsh_de_or_default(&mut reader)?;
        let amount: u16 = crate::borsh_de_or_default(&mut reader)?;
        let size: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { data, head, amount, size })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.data, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.head, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.size, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct OracleAccount(pub Oracle);
impl OracleAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ORACLE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Oracle::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ORACLE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POOL_ACCOUNT_DISCM: [u8; 8] = [241, 154, 109, 4, 17, 177, 109, 188];
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
pub struct Pool {
    pub token_x: Pubkey,
    pub token_y: Pubkey,
    pub token_x_reserve: Pubkey,
    pub token_y_reserve: Pubkey,
    pub position_iterator: u128,
    pub tick_spacing: u16,
    pub fee: FixedPoint,
    pub protocol_fee: FixedPoint,
    pub liquidity: Liquidity,
    pub sqrt_price: Price,
    pub current_tick_index: i32,
    pub tickmap: Pubkey,
    pub fee_growth_global_x: FeeGrowth,
    pub fee_growth_global_y: FeeGrowth,
    pub fee_protocol_token_x: u64,
    pub fee_protocol_token_y: u64,
    pub seconds_per_liquidity_global: FixedPoint,
    pub start_timestamp: u64,
    pub last_timestamp: u64,
    pub fee_receiver: Pubkey,
    pub oracle_address: Pubkey,
    pub oracle_initialized: bool,
    pub bump: u8,
}
impl Pool {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let token_x: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_y: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_x_reserve: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_y_reserve: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_iterator: u128 = crate::borsh_de_or_default(&mut reader)?;
        let tick_spacing: u16 = crate::borsh_de_or_default(&mut reader)?;
        let fee = if reader.is_empty() {
            Default::default()
        } else {
            <FixedPoint>::deserialize(&mut reader)?
        };
        let protocol_fee = if reader.is_empty() {
            Default::default()
        } else {
            <FixedPoint>::deserialize(&mut reader)?
        };
        let liquidity = if reader.is_empty() {
            Default::default()
        } else {
            <Liquidity>::deserialize(&mut reader)?
        };
        let sqrt_price = if reader.is_empty() {
            Default::default()
        } else {
            <Price>::deserialize(&mut reader)?
        };
        let current_tick_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let tickmap: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_growth_global_x = if reader.is_empty() {
            Default::default()
        } else {
            <FeeGrowth>::deserialize(&mut reader)?
        };
        let fee_growth_global_y = if reader.is_empty() {
            Default::default()
        } else {
            <FeeGrowth>::deserialize(&mut reader)?
        };
        let fee_protocol_token_x: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_protocol_token_y: u64 = crate::borsh_de_or_default(&mut reader)?;
        let seconds_per_liquidity_global = if reader.is_empty() {
            Default::default()
        } else {
            <FixedPoint>::deserialize(&mut reader)?
        };
        let start_timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_receiver: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let oracle_address: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let oracle_initialized: bool = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            token_x,
            token_y,
            token_x_reserve,
            token_y_reserve,
            position_iterator,
            tick_spacing,
            fee,
            protocol_fee,
            liquidity,
            sqrt_price,
            current_tick_index,
            tickmap,
            fee_growth_global_x,
            fee_growth_global_y,
            fee_protocol_token_x,
            fee_protocol_token_y,
            seconds_per_liquidity_global,
            start_timestamp,
            last_timestamp,
            fee_receiver,
            oracle_address,
            oracle_initialized,
            bump,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.token_x, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_y, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_x_reserve, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_y_reserve, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_iterator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_spacing, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sqrt_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.current_tick_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tickmap, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_growth_global_x, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_growth_global_y, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_protocol_token_x, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_protocol_token_y, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.seconds_per_liquidity_global,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.start_timestamp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_timestamp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_receiver, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.oracle_address, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.oracle_initialized, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PoolAccount(pub Pool);
impl PoolAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POOL_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Pool::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POOL_ACCOUNT_DISCM)?;
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
    Default,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct Position {
    pub owner: Pubkey,
    pub pool: Pubkey,
    pub id: u128,
    pub liquidity: Liquidity,
    pub lower_tick_index: i32,
    pub upper_tick_index: i32,
    pub fee_growth_inside_x: FeeGrowth,
    pub fee_growth_inside_y: FeeGrowth,
    pub seconds_per_liquidity_inside: FixedPoint,
    pub last_slot: u64,
    pub tokens_owed_x: FixedPoint,
    pub tokens_owed_y: FixedPoint,
    pub bump: u8,
}
impl Position {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let id: u128 = crate::borsh_de_or_default(&mut reader)?;
        let liquidity = if reader.is_empty() {
            Default::default()
        } else {
            <Liquidity>::deserialize(&mut reader)?
        };
        let lower_tick_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let upper_tick_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let fee_growth_inside_x = if reader.is_empty() {
            Default::default()
        } else {
            <FeeGrowth>::deserialize(&mut reader)?
        };
        let fee_growth_inside_y = if reader.is_empty() {
            Default::default()
        } else {
            <FeeGrowth>::deserialize(&mut reader)?
        };
        let seconds_per_liquidity_inside = if reader.is_empty() {
            Default::default()
        } else {
            <FixedPoint>::deserialize(&mut reader)?
        };
        let last_slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        let tokens_owed_x = if reader.is_empty() {
            Default::default()
        } else {
            <FixedPoint>::deserialize(&mut reader)?
        };
        let tokens_owed_y = if reader.is_empty() {
            Default::default()
        } else {
            <FixedPoint>::deserialize(&mut reader)?
        };
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            owner,
            pool,
            id,
            liquidity,
            lower_tick_index,
            upper_tick_index,
            fee_growth_inside_x,
            fee_growth_inside_y,
            seconds_per_liquidity_inside,
            last_slot,
            tokens_owed_x,
            tokens_owed_y,
            bump,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lower_tick_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.upper_tick_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_growth_inside_x, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_growth_inside_y, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.seconds_per_liquidity_inside,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.last_slot, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tokens_owed_x, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tokens_owed_y, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
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
pub const POSITION_LIST_ACCOUNT_DISCM: [u8; 8] = [32, 7, 119, 109, 46, 230, 105, 205];
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
pub struct PositionList {
    pub head: u32,
    pub bump: u8,
}
impl PositionList {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let head: u32 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { head, bump })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.head, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PositionListAccount(pub PositionList);
impl PositionListAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POSITION_LIST_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(PositionList::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POSITION_LIST_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const STATE_ACCOUNT_DISCM: [u8; 8] = [216, 146, 107, 94, 104, 75, 182, 177];
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
pub struct State {
    pub admin: Pubkey,
    pub nonce: u8,
    pub authority: Pubkey,
    pub bump: u8,
}
impl State {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let nonce: u8 = crate::borsh_de_or_default(&mut reader)?;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            admin,
            nonce,
            authority,
            bump,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.nonce, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct StateAccount(pub State);
impl StateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(State::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const TICK_ACCOUNT_DISCM: [u8; 8] = [176, 94, 67, 247, 133, 173, 7, 115];
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
pub struct Tick {
    pub pool: Pubkey,
    pub index: i32,
    pub sign: bool,
    pub liquidity_change: Liquidity,
    pub liquidity_gross: Liquidity,
    pub sqrt_price: Price,
    pub fee_growth_outside_x: FeeGrowth,
    pub fee_growth_outside_y: FeeGrowth,
    pub seconds_per_liquidity_outside: FixedPoint,
    pub seconds_outside: u64,
    pub bump: u8,
}
impl Tick {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let sign: bool = crate::borsh_de_or_default(&mut reader)?;
        let liquidity_change = if reader.is_empty() {
            Default::default()
        } else {
            <Liquidity>::deserialize(&mut reader)?
        };
        let liquidity_gross = if reader.is_empty() {
            Default::default()
        } else {
            <Liquidity>::deserialize(&mut reader)?
        };
        let sqrt_price = if reader.is_empty() {
            Default::default()
        } else {
            <Price>::deserialize(&mut reader)?
        };
        let fee_growth_outside_x = if reader.is_empty() {
            Default::default()
        } else {
            <FeeGrowth>::deserialize(&mut reader)?
        };
        let fee_growth_outside_y = if reader.is_empty() {
            Default::default()
        } else {
            <FeeGrowth>::deserialize(&mut reader)?
        };
        let seconds_per_liquidity_outside = if reader.is_empty() {
            Default::default()
        } else {
            <FixedPoint>::deserialize(&mut reader)?
        };
        let seconds_outside: u64 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool,
            index,
            sign,
            liquidity_change,
            liquidity_gross,
            sqrt_price,
            fee_growth_outside_x,
            fee_growth_outside_y,
            seconds_per_liquidity_outside,
            seconds_outside,
            bump,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sign, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity_change, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity_gross, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sqrt_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_growth_outside_x, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_growth_outside_y, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.seconds_per_liquidity_outside,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.seconds_outside, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TickAccount(pub Tick);
impl TickAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TICK_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Tick::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TICK_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const TICKMAP_ACCOUNT_DISCM: [u8; 8] = [236, 6, 101, 196, 85, 189, 0, 227];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct Tickmap {
    #[serde(with = "crate::big_array_serde")]
    pub bitmap: [u8; 11091],
}
impl Tickmap {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bitmap = <[u8; 11091] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self { bitmap })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bitmap, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TickmapAccount(pub Tickmap);
impl TickmapAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TICKMAP_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Tickmap::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TICKMAP_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
