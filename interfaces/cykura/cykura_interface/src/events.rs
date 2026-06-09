use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const OWNER_CHANGED_EVENT_DISCM: [u8; 8] = [34, 223, 103, 225, 239, 231, 51, 53];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OwnerChanged {
    pub old_owner: Pubkey,
    pub new_owner: Pubkey,
}
impl OwnerChanged {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let old_owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let new_owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { old_owner, new_owner })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.old_owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_owner, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct OwnerChangedEvent(pub OwnerChanged);
impl OwnerChangedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != OWNER_CHANGED_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = OwnerChanged::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&OWNER_CHANGED_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SET_FEE_PROTOCOL_EVENT_EVENT_DISCM: [u8; 8] = [
    46, 164, 117, 117, 242, 203, 38, 44,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetFeeProtocolEvent {
    pub fee_protocol_old: u8,
    pub fee_protocol: u8,
}
impl SetFeeProtocolEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let fee_protocol_old: u8 = crate::borsh_de_or_default(&mut reader)?;
        let fee_protocol: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            fee_protocol_old,
            fee_protocol,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.fee_protocol_old, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_protocol, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetFeeProtocolEventEvent(pub SetFeeProtocolEvent);
impl SetFeeProtocolEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_FEE_PROTOCOL_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = SetFeeProtocolEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_FEE_PROTOCOL_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const FEE_AMOUNT_ENABLED_EVENT_DISCM: [u8; 8] = [71, 250, 148, 93, 152, 157, 78, 60];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FeeAmountEnabled {
    pub fee: u32,
    pub tick_spacing: u16,
}
impl FeeAmountEnabled {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let fee: u32 = crate::borsh_de_or_default(&mut reader)?;
        let tick_spacing: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { fee, tick_spacing })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_spacing, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct FeeAmountEnabledEvent(pub FeeAmountEnabled);
impl FeeAmountEnabledEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FEE_AMOUNT_ENABLED_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = FeeAmountEnabled::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FEE_AMOUNT_ENABLED_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const INCREASE_OBSERVATION_CARDINALITY_NEXT_EVENT_DISCM: [u8; 8] = [
    192, 235, 169, 141, 52, 156, 10, 95,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IncreaseObservationCardinalityNext {
    pub observation_cardinality_next_old: u16,
    pub observation_cardinality_next_new: u16,
}
impl IncreaseObservationCardinalityNext {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let observation_cardinality_next_old: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let observation_cardinality_next_new: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            observation_cardinality_next_old,
            observation_cardinality_next_new,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(
            &self.observation_cardinality_next_old,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.observation_cardinality_next_new,
            &mut writer,
        )?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct IncreaseObservationCardinalityNextEvent(
    pub IncreaseObservationCardinalityNext,
);
impl IncreaseObservationCardinalityNextEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INCREASE_OBSERVATION_CARDINALITY_NEXT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = IncreaseObservationCardinalityNext::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INCREASE_OBSERVATION_CARDINALITY_NEXT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POOL_CREATED_AND_INITIALIZED_EVENT_DISCM: [u8; 8] = [
    130, 87, 219, 179, 253, 91, 63, 229,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PoolCreatedAndInitialized {
    pub token0: Pubkey,
    pub token1: Pubkey,
    pub fee: u32,
    pub tick_spacing: u16,
    pub pool_state: Pubkey,
    pub sqrt_price_x32: u64,
    pub tick: i32,
}
impl PoolCreatedAndInitialized {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let token0: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token1: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee: u32 = crate::borsh_de_or_default(&mut reader)?;
        let tick_spacing: u16 = crate::borsh_de_or_default(&mut reader)?;
        let pool_state: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let sqrt_price_x32: u64 = crate::borsh_de_or_default(&mut reader)?;
        let tick: i32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            token0,
            token1,
            fee,
            tick_spacing,
            pool_state,
            sqrt_price_x32,
            tick,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.token0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_spacing, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sqrt_price_x32, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PoolCreatedAndInitializedEvent(pub PoolCreatedAndInitialized);
impl PoolCreatedAndInitializedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POOL_CREATED_AND_INITIALIZED_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = PoolCreatedAndInitialized::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POOL_CREATED_AND_INITIALIZED_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const COLLECT_PROTOCOL_EVENT_EVENT_DISCM: [u8; 8] = [
    125, 36, 34, 209, 1, 47, 57, 110,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CollectProtocolEvent {
    pub pool_state: Pubkey,
    pub sender: Pubkey,
    pub recipient_wallet0: Pubkey,
    pub recipient_wallet1: Pubkey,
    pub amount0: u64,
    pub amount1: u64,
}
impl CollectProtocolEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_state: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let sender: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let recipient_wallet0: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let recipient_wallet1: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount1: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool_state,
            sender,
            recipient_wallet0,
            recipient_wallet1,
            amount0,
            amount1,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sender, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.recipient_wallet0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.recipient_wallet1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount1, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CollectProtocolEventEvent(pub CollectProtocolEvent);
impl CollectProtocolEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COLLECT_PROTOCOL_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = CollectProtocolEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COLLECT_PROTOCOL_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SWAP_EVENT_EVENT_DISCM: [u8; 8] = [64, 198, 205, 232, 38, 8, 113, 226];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapEvent {
    pub pool_state: Pubkey,
    pub sender: Pubkey,
    pub token_account0: Pubkey,
    pub token_account1: Pubkey,
    pub amount0: i64,
    pub amount1: i64,
    pub sqrt_price_x32: u64,
    pub liquidity: u64,
    pub tick: i32,
}
impl SwapEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_state: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let sender: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_account0: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_account1: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount0: i64 = crate::borsh_de_or_default(&mut reader)?;
        let amount1: i64 = crate::borsh_de_or_default(&mut reader)?;
        let sqrt_price_x32: u64 = crate::borsh_de_or_default(&mut reader)?;
        let liquidity: u64 = crate::borsh_de_or_default(&mut reader)?;
        let tick: i32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool_state,
            sender,
            token_account0,
            token_account1,
            amount0,
            amount1,
            sqrt_price_x32,
            liquidity,
            tick,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sender, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_account0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_account1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sqrt_price_x32, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapEventEvent(pub SwapEvent);
impl SwapEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = SwapEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const MINT_EVENT_EVENT_DISCM: [u8; 8] = [197, 144, 146, 149, 66, 164, 95, 16];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MintEvent {
    pub pool_state: Pubkey,
    pub sender: Pubkey,
    pub owner: Pubkey,
    pub tick_lower: i32,
    pub tick_upper: i32,
    pub amount: u64,
    pub amount0: u64,
    pub amount1: u64,
}
impl MintEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_state: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let sender: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let tick_lower: i32 = crate::borsh_de_or_default(&mut reader)?;
        let tick_upper: i32 = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount1: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool_state,
            sender,
            owner,
            tick_lower,
            tick_upper,
            amount,
            amount0,
            amount1,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sender, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_lower, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_upper, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount1, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MintEventEvent(pub MintEvent);
impl MintEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MINT_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = MintEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MINT_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const BURN_EVENT_EVENT_DISCM: [u8; 8] = [33, 89, 47, 117, 82, 124, 238, 250];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BurnEvent {
    pub pool_state: Pubkey,
    pub owner: Pubkey,
    pub tick_lower: i32,
    pub tick_upper: i32,
    pub amount: u64,
    pub amount0: u64,
    pub amount1: u64,
}
impl BurnEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_state: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let tick_lower: i32 = crate::borsh_de_or_default(&mut reader)?;
        let tick_upper: i32 = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount1: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool_state,
            owner,
            tick_lower,
            tick_upper,
            amount,
            amount0,
            amount1,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_lower, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_upper, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount1, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BurnEventEvent(pub BurnEvent);
impl BurnEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BURN_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = BurnEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BURN_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const COLLECT_EVENT_EVENT_DISCM: [u8; 8] = [138, 16, 76, 55, 167, 75, 242, 47];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CollectEvent {
    pub pool_state: Pubkey,
    pub owner: Pubkey,
    pub tick_lower: i32,
    pub tick_upper: i32,
    pub amount0: u64,
    pub amount1: u64,
}
impl CollectEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_state: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let tick_lower: i32 = crate::borsh_de_or_default(&mut reader)?;
        let tick_upper: i32 = crate::borsh_de_or_default(&mut reader)?;
        let amount0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount1: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool_state,
            owner,
            tick_lower,
            tick_upper,
            amount0,
            amount1,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_lower, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_upper, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount1, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CollectEventEvent(pub CollectEvent);
impl CollectEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COLLECT_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = CollectEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COLLECT_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const INCREASE_LIQUIDITY_EVENT_EVENT_DISCM: [u8; 8] = [
    49, 79, 105, 212, 32, 34, 30, 84,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IncreaseLiquidityEvent {
    pub token_id: Pubkey,
    pub liquidity: u64,
    pub amount0: u64,
    pub amount1: u64,
}
impl IncreaseLiquidityEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let token_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let liquidity: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount1: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            token_id,
            liquidity,
            amount0,
            amount1,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.token_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount1, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct IncreaseLiquidityEventEvent(pub IncreaseLiquidityEvent);
impl IncreaseLiquidityEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INCREASE_LIQUIDITY_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = IncreaseLiquidityEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INCREASE_LIQUIDITY_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const DECREASE_LIQUIDITY_EVENT_EVENT_DISCM: [u8; 8] = [
    58, 222, 86, 58, 68, 50, 85, 56,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DecreaseLiquidityEvent {
    pub token_id: Pubkey,
    pub liquidity: u64,
    pub amount0: u64,
    pub amount1: u64,
}
impl DecreaseLiquidityEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let token_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let liquidity: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount1: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            token_id,
            liquidity,
            amount0,
            amount1,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.token_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount1, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct DecreaseLiquidityEventEvent(pub DecreaseLiquidityEvent);
impl DecreaseLiquidityEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DECREASE_LIQUIDITY_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = DecreaseLiquidityEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DECREASE_LIQUIDITY_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const COLLECT_TOKENIZED_EVENT_EVENT_DISCM: [u8; 8] = [
    172, 148, 235, 162, 57, 96, 157, 223,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CollectTokenizedEvent {
    pub token_id: Pubkey,
    pub recipient_wallet0: Pubkey,
    pub recipient_wallet1: Pubkey,
    pub amount0: u64,
    pub amount1: u64,
}
impl CollectTokenizedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let token_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let recipient_wallet0: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let recipient_wallet1: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount1: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            token_id,
            recipient_wallet0,
            recipient_wallet1,
            amount0,
            amount1,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.token_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.recipient_wallet0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.recipient_wallet1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount1, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CollectTokenizedEventEvent(pub CollectTokenizedEvent);
impl CollectTokenizedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COLLECT_TOKENIZED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = CollectTokenizedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COLLECT_TOKENIZED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
