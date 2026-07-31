use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const BUY_EVENT_EVENT_DISCM: [u8; 8] = [103, 244, 82, 31, 44, 245, 119, 119];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BuyEvent {
    pub pair: Pubkey,
    pub user: Pubkey,
    pub amount_a: u64,
    pub amount_b: u64,
    pub new_reserves_a: u128,
    pub new_reserves_b: u128,
    pub fee_a: u64,
}
impl BuyEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pair: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount_a: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_b: u64 = crate::borsh_de_or_default(&mut reader)?;
        let new_reserves_a: u128 = crate::borsh_de_or_default(&mut reader)?;
        let new_reserves_b: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fee_a: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pair,
            user,
            amount_a,
            amount_b,
            new_reserves_a,
            new_reserves_b,
            fee_a,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pair, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_reserves_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_reserves_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_a, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BuyEventEvent(pub BuyEvent);
impl BuyEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BUY_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = BuyEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BUY_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const PAIR_CREATED_EVENT_DISCM: [u8; 8] = [173, 73, 77, 43, 235, 157, 56, 30];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PairCreated {
    pub pair: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub initial_token_reserves: u64,
    pub shift: u128,
    pub curve: CurveType,
}
impl PairCreated {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pair: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint_a: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint_b: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let initial_token_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let shift: u128 = crate::borsh_de_or_default(&mut reader)?;
        let curve: CurveType = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pair,
            mint_a,
            mint_b,
            initial_token_reserves,
            shift,
            curve,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pair, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.initial_token_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.shift, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.curve, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PairCreatedEvent(pub PairCreated);
impl PairCreatedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PAIR_CREATED_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = PairCreated::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PAIR_CREATED_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const PAIR_GRADUATED_EVENT_DISCM: [u8; 8] = [204, 30, 104, 104, 11, 1, 171, 176];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PairGraduated {
    pub pair: Pubkey,
    pub amm_pool: Pubkey,
    pub token_a_reserves: u128,
    pub token_b_reserves: u128,
}
impl PairGraduated {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pair: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amm_pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_a_reserves: u128 = crate::borsh_de_or_default(&mut reader)?;
        let token_b_reserves: u128 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pair,
            amm_pool,
            token_a_reserves,
            token_b_reserves,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pair, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amm_pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_a_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_b_reserves, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PairGraduatedEvent(pub PairGraduated);
impl PairGraduatedEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PAIR_GRADUATED_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = PairGraduated::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PAIR_GRADUATED_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SELL_EVENT_EVENT_DISCM: [u8; 8] = [62, 47, 55, 10, 165, 3, 220, 42];
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SellEvent {
    pub pair: Pubkey,
    pub user: Pubkey,
    pub amount_a: u64,
    pub amount_b: u64,
    pub new_reserves_a: u128,
    pub new_reserves_b: u128,
    pub fee_a: u64,
}
impl SellEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pair: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount_a: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_b: u64 = crate::borsh_de_or_default(&mut reader)?;
        let new_reserves_a: u128 = crate::borsh_de_or_default(&mut reader)?;
        let new_reserves_b: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fee_a: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pair,
            user,
            amount_a,
            amount_b,
            new_reserves_a,
            new_reserves_b,
            fee_a,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pair, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_reserves_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.new_reserves_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_a, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SellEventEvent(pub SellEvent);
impl SellEventEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader: &[u8] = *__buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SELL_EVENT_EVENT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        let inner = SellEvent::deserialize(&mut reader)?;
        *__buf = reader;
        Ok(Self(inner))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SELL_EVENT_EVENT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
