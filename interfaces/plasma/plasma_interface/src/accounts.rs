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
pub struct PoolAccount {
    pub pool_header: PoolHeader,
    pub amm: Amm,
}
impl PoolAccount {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_header = if reader.is_empty() {
            Default::default()
        } else {
            <PoolHeader>::deserialize(&mut reader)?
        };
        let amm = if reader.is_empty() {
            Default::default()
        } else {
            <Amm>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self { pool_header, amm })
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
pub struct LpPositionAccount {
    pub authority: Pubkey,
    pub pool: Pubkey,
    pub status: u64,
    pub lp_position: LpPosition,
}
impl LpPositionAccount {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let status: u64 = crate::borsh_de_or_default(&mut reader)?;
        let lp_position = if reader.is_empty() {
            Default::default()
        } else {
            <LpPosition>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            authority,
            pool,
            status,
            lp_position,
        })
    }
}
