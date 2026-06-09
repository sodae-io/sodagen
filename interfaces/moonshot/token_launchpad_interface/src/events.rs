use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const TRADE_EVENT_EVENT_DISCM: [u8; 8] = [189, 219, 127, 211, 78, 230, 97, 238];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TradeEvent {
    pub amount: u64,
    pub collateral_amount: u64,
    pub dex_fee: u64,
    pub helio_fee: u64,
    pub allocation: u64,
    pub curve: Pubkey,
    pub cost_token: Pubkey,
    pub sender: Pubkey,
    pub r#type: TradeType,
    pub label: String,
}
impl TradeEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let collateral_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let dex_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let helio_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let allocation: u64 = crate::borsh_de_or_default(&mut reader)?;
        let curve: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let cost_token: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let sender: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let r#type: TradeType = crate::borsh_de_or_default(&mut reader)?;
        let label: String = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            amount,
            collateral_amount,
            dex_fee,
            helio_fee,
            allocation,
            curve,
            cost_token,
            sender,
            r#type,
            label,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.dex_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.helio_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.allocation, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.curve, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.cost_token, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sender, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.r#type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.label, &mut writer)?;
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
pub const MIGRATION_EVENT_EVENT_DISCM: [u8; 8] = [255, 202, 76, 147, 91, 231, 73, 22];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MigrationEvent {
    pub tokens_migrated: u64,
    pub tokens_burned: u64,
    pub collateral_migrated: u64,
    pub fee: u64,
    pub label: String,
}
impl MigrationEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let tokens_migrated: u64 = crate::borsh_de_or_default(&mut reader)?;
        let tokens_burned: u64 = crate::borsh_de_or_default(&mut reader)?;
        let collateral_migrated: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let label: String = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            tokens_migrated,
            tokens_burned,
            collateral_migrated,
            fee,
            label,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.tokens_migrated, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tokens_burned, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral_migrated, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.label, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MigrationEventEvent(pub MigrationEvent);
impl MigrationEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MIGRATION_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = MigrationEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MIGRATION_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
