use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const CONFIG_ACCOUNT_DISCM: [u8; 8] = [155, 12, 170, 224, 30, 250, 204, 130];
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
pub struct Config {
    pub authority: Pubkey,
    pub padding: [u64; 16],
}
impl Config {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u64; 16] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { authority, padding })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConfigAccount(pub Config);
impl ConfigAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CONFIG_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Config::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CONFIG_ACCOUNT_DISCM)?;
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
    pub creator: Pubkey,
    pub base_mint: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub base_reserve: u64,
    pub quote_reserve: u64,
    pub virtual_base_reserve: u64,
    pub virtual_quote_reserve: u64,
    pub creator_fee: u64,
    pub protocol_fee: u64,
    pub is_migrated: u8,
    pub _padding0: [u8; 7],
    pub padding: [u64; 15],
}
impl Pool {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let quote_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_reserve: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_reserve: u64 = crate::borsh_de_or_default(&mut reader)?;
        let virtual_base_reserve: u64 = crate::borsh_de_or_default(&mut reader)?;
        let virtual_quote_reserve: u64 = crate::borsh_de_or_default(&mut reader)?;
        let creator_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let is_migrated: u8 = crate::borsh_de_or_default(&mut reader)?;
        let _padding0: [u8; 7] = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u64; 15] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            creator,
            base_mint,
            base_vault,
            quote_vault,
            base_reserve,
            quote_reserve,
            virtual_base_reserve,
            virtual_quote_reserve,
            creator_fee,
            protocol_fee,
            is_migrated,
            _padding0,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.creator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_reserve, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_reserve, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.virtual_base_reserve, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.virtual_quote_reserve, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.is_migrated, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._padding0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
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
