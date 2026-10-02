use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const LIQUIDITY_DECREASED_EVENT_DISCM: [u8; 8] = [
    166, 1, 36, 71, 112, 202, 181, 171,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LiquidityDecreased {
    pub whirlpool: Pubkey,
    pub position: Pubkey,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
    pub liquidity: u128,
    pub token_a_amount: u64,
    pub token_b_amount: u64,
    pub token_a_transfer_fee: u64,
    pub token_b_transfer_fee: u64,
}
impl LiquidityDecreased {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let whirlpool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let tick_lower_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let tick_upper_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let token_a_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_b_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_a_transfer_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_b_transfer_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            whirlpool,
            position,
            tick_lower_index,
            tick_upper_index,
            liquidity,
            token_a_amount,
            token_b_amount,
            token_a_transfer_fee,
            token_b_transfer_fee,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.whirlpool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_lower_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_upper_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_a_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_b_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_a_transfer_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_b_transfer_fee, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LiquidityDecreasedEvent(pub LiquidityDecreased);
impl LiquidityDecreasedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LIQUIDITY_DECREASED_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LiquidityDecreased::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LIQUIDITY_DECREASED_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LIQUIDITY_INCREASED_EVENT_DISCM: [u8; 8] = [
    30, 7, 144, 181, 102, 254, 155, 161,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LiquidityIncreased {
    pub whirlpool: Pubkey,
    pub position: Pubkey,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
    pub liquidity: u128,
    pub token_a_amount: u64,
    pub token_b_amount: u64,
    pub token_a_transfer_fee: u64,
    pub token_b_transfer_fee: u64,
}
impl LiquidityIncreased {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let whirlpool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let tick_lower_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let tick_upper_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let token_a_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_b_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_a_transfer_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_b_transfer_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            whirlpool,
            position,
            tick_lower_index,
            tick_upper_index,
            liquidity,
            token_a_amount,
            token_b_amount,
            token_a_transfer_fee,
            token_b_transfer_fee,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.whirlpool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_lower_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_upper_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_a_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_b_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_a_transfer_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_b_transfer_fee, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LiquidityIncreasedEvent(pub LiquidityIncreased);
impl LiquidityIncreasedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LIQUIDITY_INCREASED_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LiquidityIncreased::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LIQUIDITY_INCREASED_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LIQUIDITY_REPOSITIONED_EVENT_DISCM: [u8; 8] = [
    95, 130, 181, 132, 251, 50, 195, 38,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LiquidityRepositioned {
    pub whirlpool: Pubkey,
    pub position: Pubkey,
    pub existing_range_tick_lower_index: i32,
    pub existing_range_tick_upper_index: i32,
    pub new_range_tick_lower_index: i32,
    pub new_range_tick_upper_index: i32,
    pub existing_range_liquidity: u128,
    pub new_range_liquidity: u128,
    pub existing_range_token_a_amount: u64,
    pub existing_range_token_b_amount: u64,
    pub new_range_token_a_amount: u64,
    pub new_range_token_b_amount: u64,
    pub token_a_transfer_amount: u64,
    pub token_a_transfer_fee: u64,
    pub is_token_a_transfer_from_owner: bool,
    pub token_b_transfer_amount: u64,
    pub token_b_transfer_fee: u64,
    pub is_token_b_transfer_from_owner: bool,
}
impl LiquidityRepositioned {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let whirlpool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let existing_range_tick_lower_index: i32 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let existing_range_tick_upper_index: i32 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let new_range_tick_lower_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let new_range_tick_upper_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let existing_range_liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let new_range_liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let existing_range_token_a_amount: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let existing_range_token_b_amount: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let new_range_token_a_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let new_range_token_b_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_a_transfer_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_a_transfer_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let is_token_a_transfer_from_owner: bool = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let token_b_transfer_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_b_transfer_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let is_token_b_transfer_from_owner: bool = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            whirlpool,
            position,
            existing_range_tick_lower_index,
            existing_range_tick_upper_index,
            new_range_tick_lower_index,
            new_range_tick_upper_index,
            existing_range_liquidity,
            new_range_liquidity,
            existing_range_token_a_amount,
            existing_range_token_b_amount,
            new_range_token_a_amount,
            new_range_token_b_amount,
            token_a_transfer_amount,
            token_a_transfer_fee,
            is_token_a_transfer_from_owner,
            token_b_transfer_amount,
            token_b_transfer_fee,
            is_token_b_transfer_from_owner,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.whirlpool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.existing_range_tick_lower_index,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.existing_range_tick_upper_index,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.new_range_tick_lower_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_range_tick_upper_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.existing_range_liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_range_liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.existing_range_token_a_amount,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.existing_range_token_b_amount,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.new_range_token_a_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_range_token_b_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_a_transfer_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_a_transfer_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.is_token_a_transfer_from_owner,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.token_b_transfer_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_b_transfer_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.is_token_b_transfer_from_owner,
            &mut writer,
        )?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LiquidityRepositionedEvent(pub LiquidityRepositioned);
impl LiquidityRepositionedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LIQUIDITY_REPOSITIONED_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LiquidityRepositioned::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LIQUIDITY_REPOSITIONED_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POOL_INITIALIZED_EVENT_DISCM: [u8; 8] = [100, 118, 173, 87, 12, 198, 254, 229];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PoolInitialized {
    pub whirlpool: Pubkey,
    pub whirlpools_config: Pubkey,
    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,
    pub tick_spacing: u16,
    pub token_program_a: Pubkey,
    pub token_program_b: Pubkey,
    pub decimals_a: u8,
    pub decimals_b: u8,
    pub initial_sqrt_price: u128,
}
impl PoolInitialized {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let whirlpool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let whirlpools_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_mint_a: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_mint_b: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let tick_spacing: u16 = crate::borsh_de_or_default(&mut reader)?;
        let token_program_a: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_program_b: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let decimals_a: u8 = crate::borsh_de_or_default(&mut reader)?;
        let decimals_b: u8 = crate::borsh_de_or_default(&mut reader)?;
        let initial_sqrt_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            whirlpool,
            whirlpools_config,
            token_mint_a,
            token_mint_b,
            tick_spacing,
            token_program_a,
            token_program_b,
            decimals_a,
            decimals_b,
            initial_sqrt_price,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.whirlpool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.whirlpools_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_mint_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_mint_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_spacing, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_program_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_program_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.decimals_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.decimals_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.initial_sqrt_price, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PoolInitializedEvent(pub PoolInitialized);
impl PoolInitializedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POOL_INITIALIZED_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = PoolInitialized::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POOL_INITIALIZED_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POSITION_OPENED_EVENT_DISCM: [u8; 8] = [
    237, 175, 243, 230, 147, 117, 101, 121,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PositionOpened {
    pub whirlpool: Pubkey,
    pub position: Pubkey,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
}
impl PositionOpened {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let whirlpool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let tick_lower_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let tick_upper_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            whirlpool,
            position,
            tick_lower_index,
            tick_upper_index,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.whirlpool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_lower_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_upper_index, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PositionOpenedEvent(pub PositionOpened);
impl PositionOpenedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POSITION_OPENED_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = PositionOpened::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POSITION_OPENED_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const TRADED_EVENT_DISCM: [u8; 8] = [225, 202, 73, 175, 147, 43, 160, 150];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Traded {
    pub whirlpool: Pubkey,
    pub a_to_b: bool,
    pub pre_sqrt_price: u128,
    pub post_sqrt_price: u128,
    pub input_amount: u64,
    pub output_amount: u64,
    pub input_transfer_fee: u64,
    pub output_transfer_fee: u64,
    pub lp_fee: u64,
    pub protocol_fee: u64,
}
impl Traded {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let whirlpool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let a_to_b: bool = crate::borsh_de_or_default(&mut reader)?;
        let pre_sqrt_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        let post_sqrt_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        let input_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let output_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let input_transfer_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let output_transfer_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let lp_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            whirlpool,
            a_to_b,
            pre_sqrt_price,
            post_sqrt_price,
            input_amount,
            output_amount,
            input_transfer_fee,
            output_transfer_fee,
            lp_fee,
            protocol_fee,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.whirlpool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.a_to_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pre_sqrt_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.post_sqrt_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.input_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.output_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.input_transfer_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.output_transfer_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fee, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TradedEvent(pub Traded);
impl TradedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TRADED_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = Traded::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TRADED_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
