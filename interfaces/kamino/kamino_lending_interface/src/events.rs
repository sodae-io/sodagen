use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const BORROW_ORDER_CANCEL_EVENT_EVENT_DISCM: [u8; 8] = [
    88, 228, 231, 230, 234, 248, 69, 144,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BorrowOrderCancelEvent {
    pub before: BorrowOrder,
}
impl BorrowOrderCancelEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let before = if reader.is_empty() {
            Default::default()
        } else {
            <BorrowOrder>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self { before })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.before, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BorrowOrderCancelEventEvent(pub BorrowOrderCancelEvent);
impl BorrowOrderCancelEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BORROW_ORDER_CANCEL_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = BorrowOrderCancelEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BORROW_ORDER_CANCEL_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const BORROW_ORDER_FULL_FILL_EVENT_EVENT_DISCM: [u8; 8] = [
    177, 241, 237, 250, 143, 20, 14, 183,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BorrowOrderFullFillEvent {
    pub before: BorrowOrder,
}
impl BorrowOrderFullFillEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let before = if reader.is_empty() {
            Default::default()
        } else {
            <BorrowOrder>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self { before })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.before, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BorrowOrderFullFillEventEvent(pub BorrowOrderFullFillEvent);
impl BorrowOrderFullFillEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BORROW_ORDER_FULL_FILL_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = BorrowOrderFullFillEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BORROW_ORDER_FULL_FILL_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const BORROW_ORDER_PARTIAL_FILL_EVENT_EVENT_DISCM: [u8; 8] = [
    113, 81, 252, 193, 152, 24, 99, 84,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BorrowOrderPartialFillEvent {
    pub before: BorrowOrder,
    pub after: BorrowOrder,
}
impl BorrowOrderPartialFillEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let before = if reader.is_empty() {
            Default::default()
        } else {
            <BorrowOrder>::deserialize(&mut reader)?
        };
        let after = if reader.is_empty() {
            Default::default()
        } else {
            <BorrowOrder>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self { before, after })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.before, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.after, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BorrowOrderPartialFillEventEvent(pub BorrowOrderPartialFillEvent);
impl BorrowOrderPartialFillEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BORROW_ORDER_PARTIAL_FILL_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = BorrowOrderPartialFillEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BORROW_ORDER_PARTIAL_FILL_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const BORROW_ORDER_PLACE_EVENT_EVENT_DISCM: [u8; 8] = [
    43, 211, 208, 186, 94, 227, 218, 198,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BorrowOrderPlaceEvent {
    pub after: BorrowOrder,
}
impl BorrowOrderPlaceEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let after = if reader.is_empty() {
            Default::default()
        } else {
            <BorrowOrder>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self { after })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.after, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BorrowOrderPlaceEventEvent(pub BorrowOrderPlaceEvent);
impl BorrowOrderPlaceEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BORROW_ORDER_PLACE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = BorrowOrderPlaceEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BORROW_ORDER_PLACE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const BORROW_ORDER_UPDATE_EVENT_EVENT_DISCM: [u8; 8] = [
    21, 33, 67, 131, 48, 184, 90, 64,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BorrowOrderUpdateEvent {
    pub before: BorrowOrder,
    pub after: BorrowOrder,
}
impl BorrowOrderUpdateEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let before = if reader.is_empty() {
            Default::default()
        } else {
            <BorrowOrder>::deserialize(&mut reader)?
        };
        let after = if reader.is_empty() {
            Default::default()
        } else {
            <BorrowOrder>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self { before, after })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.before, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.after, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BorrowOrderUpdateEventEvent(pub BorrowOrderUpdateEvent);
impl BorrowOrderUpdateEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BORROW_ORDER_UPDATE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = BorrowOrderUpdateEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BORROW_ORDER_UPDATE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
