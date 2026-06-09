use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const WRAPPED_TOKEN_ACCOUNT_DISCM: [u8; 8] = [28, 41, 198, 163, 189, 149, 175, 142];
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
pub struct WrappedToken {
    pub decimals: u8,
    pub multiplier: u64,
    pub wrapper_underlying_mint: Pubkey,
    pub wrapper_underlying_tokens: Pubkey,
    pub wrapper_mint: Pubkey,
}
impl WrappedToken {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let multiplier: u64 = crate::borsh_de_or_default(&mut reader)?;
        let wrapper_underlying_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let wrapper_underlying_tokens: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let wrapper_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            decimals,
            multiplier,
            wrapper_underlying_mint,
            wrapper_underlying_tokens,
            wrapper_mint,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.decimals, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.multiplier, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.wrapper_underlying_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.wrapper_underlying_tokens, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.wrapper_mint, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct WrappedTokenAccount(pub WrappedToken);
impl WrappedTokenAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WRAPPED_TOKEN_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(WrappedToken::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WRAPPED_TOKEN_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
