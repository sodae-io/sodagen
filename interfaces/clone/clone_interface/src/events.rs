use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const SWAP_EVENT_EVENT_DISCM: [u8; 8] = [64, 198, 205, 232, 38, 8, 113, 226];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SwapEvent {
    pub event_id: u64,
    pub user_address: Pubkey,
    pub pool_index: u8,
    pub input_is_collateral: bool,
    pub input: u64,
    pub output: u64,
    pub trading_fee: u64,
    pub treasury_fee: u64,
}
impl SwapEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let event_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user_address: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let input_is_collateral: bool = crate::borsh_de_or_default(&mut reader)?;
        let input: u64 = crate::borsh_de_or_default(&mut reader)?;
        let output: u64 = crate::borsh_de_or_default(&mut reader)?;
        let trading_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let treasury_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            event_id,
            user_address,
            pool_index,
            input_is_collateral,
            input,
            output,
            trading_fee,
            treasury_fee,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.event_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_address, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.input_is_collateral, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.input, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.output, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.trading_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.treasury_fee, &mut writer)?;
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
pub const LIQUIDITY_DELTA_EVENT_DISCM: [u8; 8] = [16, 36, 128, 146, 175, 158, 76, 185];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LiquidityDelta {
    pub event_id: u64,
    pub user_address: Pubkey,
    pub pool_index: u8,
    pub committed_collateral_delta: i64,
    pub collateral_ild_delta: i64,
    pub onasset_ild_delta: i64,
}
impl LiquidityDelta {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let event_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user_address: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let committed_collateral_delta: i64 = crate::borsh_de_or_default(&mut reader)?;
        let collateral_ild_delta: i64 = crate::borsh_de_or_default(&mut reader)?;
        let onasset_ild_delta: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            event_id,
            user_address,
            pool_index,
            committed_collateral_delta,
            collateral_ild_delta,
            onasset_ild_delta,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.event_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_address, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.committed_collateral_delta, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral_ild_delta, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.onasset_ild_delta, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LiquidityDeltaEvent(pub LiquidityDelta);
impl LiquidityDeltaEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LIQUIDITY_DELTA_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = LiquidityDelta::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LIQUIDITY_DELTA_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POOL_STATE_EVENT_DISCM: [u8; 8] = [179, 75, 157, 220, 238, 184, 176, 136];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PoolState {
    pub event_id: u64,
    pub pool_index: u8,
    pub onasset_ild: i64,
    pub collateral_ild: i64,
    pub committed_collateral_liquidity: u64,
    pub pool_price: u64,
    pub pool_scale: u32,
}
impl PoolState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let event_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pool_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let onasset_ild: i64 = crate::borsh_de_or_default(&mut reader)?;
        let collateral_ild: i64 = crate::borsh_de_or_default(&mut reader)?;
        let committed_collateral_liquidity: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let pool_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pool_scale: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            event_id,
            pool_index,
            onasset_ild,
            collateral_ild,
            committed_collateral_liquidity,
            pool_price,
            pool_scale,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.event_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.onasset_ild, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral_ild, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.committed_collateral_liquidity,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.pool_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_scale, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PoolStateEvent(pub PoolState);
impl PoolStateEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POOL_STATE_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = PoolState::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POOL_STATE_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const BORROW_UPDATE_EVENT_DISCM: [u8; 8] = [63, 37, 227, 177, 225, 75, 174, 11];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BorrowUpdate {
    pub event_id: u64,
    pub pool_index: u8,
    pub user_address: Pubkey,
    pub is_liquidation: bool,
    pub collateral_supplied: u64,
    pub collateral_delta: i64,
    pub borrowed_amount: u64,
    pub borrowed_delta: i64,
}
impl BorrowUpdate {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let event_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pool_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let user_address: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let is_liquidation: bool = crate::borsh_de_or_default(&mut reader)?;
        let collateral_supplied: u64 = crate::borsh_de_or_default(&mut reader)?;
        let collateral_delta: i64 = crate::borsh_de_or_default(&mut reader)?;
        let borrowed_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let borrowed_delta: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            event_id,
            pool_index,
            user_address,
            is_liquidation,
            collateral_supplied,
            collateral_delta,
            borrowed_amount,
            borrowed_delta,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.event_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_address, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.is_liquidation, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral_supplied, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral_delta, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.borrowed_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.borrowed_delta, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BorrowUpdateEvent(pub BorrowUpdate);
impl BorrowUpdateEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BORROW_UPDATE_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = BorrowUpdate::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BORROW_UPDATE_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const COMET_COLLATERAL_UPDATE_EVENT_DISCM: [u8; 8] = [
    190, 151, 89, 61, 222, 27, 98, 109,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CometCollateralUpdate {
    pub event_id: u64,
    pub user_address: Pubkey,
    pub collateral_supplied: u64,
    pub collateral_delta: i64,
}
impl CometCollateralUpdate {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let event_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user_address: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let collateral_supplied: u64 = crate::borsh_de_or_default(&mut reader)?;
        let collateral_delta: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            event_id,
            user_address,
            collateral_supplied,
            collateral_delta,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.event_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_address, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral_supplied, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral_delta, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CometCollateralUpdateEvent(pub CometCollateralUpdate);
impl CometCollateralUpdateEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COMET_COLLATERAL_UPDATE_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = CometCollateralUpdate::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COMET_COLLATERAL_UPDATE_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
