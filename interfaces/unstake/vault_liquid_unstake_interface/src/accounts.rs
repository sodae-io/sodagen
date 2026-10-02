use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const LST_INFO_ACCOUNT_DISCM: [u8; 8] = [79, 113, 226, 60, 171, 8, 142, 33];
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
pub struct LstInfo {
    pub field_0: Pubkey,
    pub field_1: Pubkey,
    pub field_2: Pubkey,
    pub field_3: Pubkey,
    pub field_4: u64,
}
impl LstInfo {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let field_0: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let field_1: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let field_2: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let field_3: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let field_4: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            field_0,
            field_1,
            field_2,
            field_3,
            field_4,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.field_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.field_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.field_2, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.field_3, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.field_4, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LstInfoAccount(pub LstInfo);
impl LstInfoAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LST_INFO_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(LstInfo::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LST_INFO_ACCOUNT_DISCM)?;
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
    pub field_0: Pubkey,
    pub field_1: Pubkey,
    pub field_2: Pubkey,
    pub field_3: Pubkey,
    pub field_4: u64,
    pub field_5: u64,
    pub field_6: u64,
    pub field_7: u32,
    pub field_8: u32,
    pub field_9: u64,
    pub field_10: u8,
    pub field_11: u8,
    pub field_12: u8,
    pub field_13: u64,
    pub field_14: u64,
    pub field_15: u16,
    pub field_16: u16,
    pub field_17: u32,
    pub field_18: u64,
    pub field_19: u8,
    pub field_20: u64,
    pub field_21: u32,
    pub field_22: [u8; 3],
}
impl Pool {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let field_0: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let field_1: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let field_2: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let field_3: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let field_4: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_5: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_6: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_7: u32 = crate::borsh_de_or_default(&mut reader)?;
        let field_8: u32 = crate::borsh_de_or_default(&mut reader)?;
        let field_9: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_10: u8 = crate::borsh_de_or_default(&mut reader)?;
        let field_11: u8 = crate::borsh_de_or_default(&mut reader)?;
        let field_12: u8 = crate::borsh_de_or_default(&mut reader)?;
        let field_13: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_14: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_15: u16 = crate::borsh_de_or_default(&mut reader)?;
        let field_16: u16 = crate::borsh_de_or_default(&mut reader)?;
        let field_17: u32 = crate::borsh_de_or_default(&mut reader)?;
        let field_18: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_19: u8 = crate::borsh_de_or_default(&mut reader)?;
        let field_20: u64 = crate::borsh_de_or_default(&mut reader)?;
        let field_21: u32 = crate::borsh_de_or_default(&mut reader)?;
        let field_22: [u8; 3] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            field_0,
            field_1,
            field_2,
            field_3,
            field_4,
            field_5,
            field_6,
            field_7,
            field_8,
            field_9,
            field_10,
            field_11,
            field_12,
            field_13,
            field_14,
            field_15,
            field_16,
            field_17,
            field_18,
            field_19,
            field_20,
            field_21,
            field_22,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.field_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.field_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.field_2, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.field_3, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.field_4, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.field_5, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.field_6, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.field_7, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.field_8, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.field_9, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.field_10, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.field_11, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.field_12, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.field_13, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.field_14, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.field_15, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.field_16, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.field_17, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.field_18, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.field_19, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.field_20, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.field_21, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.field_22, &mut writer)?;
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
