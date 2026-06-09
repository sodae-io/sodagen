use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const ENDO_AVS_ACCOUNT_DISCM: [u8; 8] = [169, 223, 251, 169, 163, 99, 77, 37];
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
pub struct EndoAVS {
    pub bump: u8,
    pub authority: Pubkey,
    pub avs_token_mint: Pubkey,
    pub delegated_token_mint: Pubkey,
    pub delegated_token_vault: Pubkey,
    pub name: String,
    pub url: String,
}
impl EndoAVS {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let avs_token_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let delegated_token_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let delegated_token_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        let url: String = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            bump,
            authority,
            avs_token_mint,
            delegated_token_mint,
            delegated_token_vault,
            name,
            url,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.avs_token_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.delegated_token_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.delegated_token_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.name, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.url, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct EndoAVSAccount(pub EndoAVS);
impl EndoAVSAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ENDO_AVS_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(EndoAVS::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ENDO_AVS_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
