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
pub struct AddressBool {
    pub addr: Pubkey,
    pub value: bool,
}
impl AddressBool {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let addr: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let value: bool = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { addr, value })
    }
}
