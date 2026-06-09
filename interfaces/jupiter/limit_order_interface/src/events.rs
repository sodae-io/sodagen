use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const TRADE_EVENT_EVENT_DISCM: [u8; 8] = [189, 219, 127, 211, 78, 230, 97, 238];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TradeEvent {
    pub order_key: Pubkey,
    pub taker: Pubkey,
    pub remaining_in_amount: u64,
    pub remaining_out_amount: u64,
    pub in_amount: u64,
    pub out_amount: u64,
}
impl TradeEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let order_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let taker: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let remaining_in_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let remaining_out_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let in_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let out_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            order_key,
            taker,
            remaining_in_amount,
            remaining_out_amount,
            in_amount,
            out_amount,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.order_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.taker, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.remaining_in_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.remaining_out_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.in_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.out_amount, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TradeEventEvent(pub TradeEvent);
impl TradeEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TRADE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = TradeEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TRADE_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CANCEL_ORDER_EVENT_EVENT_DISCM: [u8; 8] = [174, 66, 141, 17, 4, 224, 162, 77];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CancelOrderEvent {
    pub order_key: Pubkey,
}
impl CancelOrderEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let order_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { order_key })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.order_key, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CancelOrderEventEvent(pub CancelOrderEvent);
impl CancelOrderEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CANCEL_ORDER_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = CancelOrderEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CANCEL_ORDER_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CREATE_ORDER_EVENT_EVENT_DISCM: [u8; 8] = [49, 142, 72, 166, 230, 29, 84, 84];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateOrderEvent {
    pub order_key: Pubkey,
    pub maker: Pubkey,
    pub input_mint: Pubkey,
    pub output_mint: Pubkey,
    pub in_amount: u64,
    pub out_amount: u64,
    pub expired_at: Option<i64>,
}
impl CreateOrderEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let order_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let maker: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let input_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let output_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let in_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let out_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let expired_at: Option<i64> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            order_key,
            maker,
            input_mint,
            output_mint,
            in_amount,
            out_amount,
            expired_at,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.order_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.maker, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.input_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.output_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.in_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.out_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.expired_at, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateOrderEventEvent(pub CreateOrderEvent);
impl CreateOrderEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_ORDER_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = CreateOrderEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_ORDER_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
