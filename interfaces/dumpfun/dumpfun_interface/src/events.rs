use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const BUY_TOKEN_EVENT_EVENT_DISCM: [u8; 8] = [90, 138, 104, 84, 222, 143, 82, 123];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BuyTokenEvent {
    pub user: Pubkey,
    pub mint: Pubkey,
    pub sol_in: u64,
    pub token_out: u64,
    pub buy_time: i64,
}
impl BuyTokenEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let sol_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        let buy_time: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            user,
            mint,
            sol_in,
            token_out,
            buy_time,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sol_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_out, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.buy_time, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BuyTokenEventEvent(pub BuyTokenEvent);
impl BuyTokenEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BUY_TOKEN_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = BuyTokenEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BUY_TOKEN_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const DRAIN_POOL_EVENT_EVENT_DISCM: [u8; 8] = [116, 220, 198, 208, 98, 61, 234, 65];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DrainPoolEvent {
    pub pool: Pubkey,
    pub mint: Pubkey,
    pub creator_wallet: Pubkey,
    pub company_wallet: Pubkey,
    pub creator_amount: u64,
    pub company_amount: u64,
    pub total_drained: u64,
    pub drain_time: i64,
}
impl DrainPoolEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let creator_wallet: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let company_wallet: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let creator_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let company_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_drained: u64 = crate::borsh_de_or_default(&mut reader)?;
        let drain_time: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool,
            mint,
            creator_wallet,
            company_wallet,
            creator_amount,
            company_amount,
            total_drained,
            drain_time,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator_wallet, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.company_wallet, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.company_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_drained, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.drain_time, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct DrainPoolEventEvent(pub DrainPoolEvent);
impl DrainPoolEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DRAIN_POOL_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = DrainPoolEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DRAIN_POOL_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SELL_TOKEN_EVENT_EVENT_DISCM: [u8; 8] = [
    148, 185, 126, 171, 239, 120, 196, 178,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SellTokenEvent {
    pub user: Pubkey,
    pub mint: Pubkey,
    pub token_in: u64,
    pub sol_out: u64,
    pub sell_time: i64,
}
impl SellTokenEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let sol_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        let sell_time: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            user,
            mint,
            token_in,
            sol_out,
            sell_time,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_in, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sol_out, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sell_time, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SellTokenEventEvent(pub SellTokenEvent);
impl SellTokenEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SELL_TOKEN_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = SellTokenEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SELL_TOKEN_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const TOKEN_CREATED_EVENT_EVENT_DISCM: [u8; 8] = [
    96, 122, 113, 138, 50, 227, 149, 57,
];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TokenCreatedEvent {
    pub creator: Pubkey,
    pub mint: Pubkey,
    pub create_time: i64,
    pub sell_lock_period: i64,
}
impl TokenCreatedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let create_time: i64 = crate::borsh_de_or_default(&mut reader)?;
        let sell_lock_period: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            creator,
            mint,
            create_time,
            sell_lock_period,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.creator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.create_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sell_lock_period, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TokenCreatedEventEvent(pub TokenCreatedEvent);
impl TokenCreatedEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TOKEN_CREATED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = TokenCreatedEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TOKEN_CREATED_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
