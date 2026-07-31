use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const SWAP_GLOBAL_ACCOUNT_DISCM: [u8; 8] = [15, 184, 147, 129, 183, 219, 223, 163];
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
pub struct SwapGlobal {
    pub bump: u8,
    pub admin: Pubkey,
    pub whitelisted_unwrappers: Vec<Pubkey>,
    pub whitelisted_extensions: Vec<WhitelistedExtension>,
}
impl SwapGlobal {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let whitelisted_unwrappers: Vec<Pubkey> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let whitelisted_extensions: Vec<WhitelistedExtension> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            bump,
            admin,
            whitelisted_unwrappers,
            whitelisted_extensions,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.whitelisted_unwrappers, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.whitelisted_extensions, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapGlobalAccount(pub SwapGlobal);
impl SwapGlobalAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SWAP_GLOBAL_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(SwapGlobal::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SWAP_GLOBAL_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
