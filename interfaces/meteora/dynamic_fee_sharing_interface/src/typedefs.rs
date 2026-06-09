use borsh::{BorshDeserialize, BorshSerialize};
#[allow(unused_imports)]
use crate::*;
use solana_pubkey::Pubkey;
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
pub struct InitializeFeeVaultParameters {
    pub padding: [u64; 8],
    pub users: Vec<UserShare>,
}
impl InitializeFeeVaultParameters {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let padding: [u64; 8] = crate::borsh_de_or_default(&mut reader)?;
        let users: Vec<UserShare> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { padding, users })
    }
}
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
pub struct UserFee {
    pub address: Pubkey,
    pub share: u32,
    pub padding_0: [u8; 4],
    pub fee_claimed: u64,
    pub padding: [u8; 16],
    pub fee_per_share_checkpoint: u128,
}
impl UserFee {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let address: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let share: u32 = crate::borsh_de_or_default(&mut reader)?;
        let padding_0: [u8; 4] = crate::borsh_de_or_default(&mut reader)?;
        let fee_claimed: u64 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 16] = crate::borsh_de_or_default(&mut reader)?;
        let fee_per_share_checkpoint: u128 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            address,
            share,
            padding_0,
            fee_claimed,
            padding,
            fee_per_share_checkpoint,
        })
    }
}
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
pub struct UserShare {
    pub address: Pubkey,
    pub share: u32,
}
impl UserShare {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let address: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let share: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { address, share })
    }
}
