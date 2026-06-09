use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const POOL_BALANCE_UPDATED_EVENT_EVENT_DISCM: [u8; 8] = [
    172, 82, 114, 207, 27, 103, 211, 4,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PoolBalanceUpdatedEvent {
    pub pubkey: Pubkey,
    pub data: PoolBalanceUpdatedData,
}
impl PoolBalanceUpdatedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pubkey: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let data = if reader.is_empty() {
            Default::default()
        } else {
            <PoolBalanceUpdatedData>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self { pubkey, data })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pubkey, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.data, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PoolBalanceUpdatedEventEvent(pub PoolBalanceUpdatedEvent);
impl PoolBalanceUpdatedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POOL_BALANCE_UPDATED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = PoolBalanceUpdatedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POOL_BALANCE_UPDATED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POOL_UPDATED_EVENT_EVENT_DISCM: [u8; 8] = [
    128, 39, 94, 221, 230, 222, 127, 141,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PoolUpdatedEvent {
    pub pubkey: Pubkey,
    pub data: PoolUpdatedData,
}
impl PoolUpdatedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pubkey: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let data = if reader.is_empty() {
            Default::default()
        } else {
            <PoolUpdatedData>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self { pubkey, data })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pubkey, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.data, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PoolUpdatedEventEvent(pub PoolUpdatedEvent);
impl PoolUpdatedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POOL_UPDATED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = PoolUpdatedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POOL_UPDATED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
