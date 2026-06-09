use borsh::{BorshDeserialize, BorshSerialize};
#[allow(unused_imports)]
use crate::*;
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
pub struct GetPriceResult {
    pub price_out: u128,
    pub feasible_out: bool,
}
impl GetPriceResult {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let price_out: u128 = crate::borsh_de_or_default(&mut reader)?;
        let feasible_out: bool = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { price_out, feasible_out })
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
pub struct GetStateResult {
    pub price_out: u128,
    pub spread: u64,
    pub coeff: u64,
    pub feasible_out: bool,
}
impl GetStateResult {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let price_out: u128 = crate::borsh_de_or_default(&mut reader)?;
        let spread: u64 = crate::borsh_de_or_default(&mut reader)?;
        let coeff: u64 = crate::borsh_de_or_default(&mut reader)?;
        let feasible_out: bool = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            price_out,
            spread,
            coeff,
            feasible_out,
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
pub struct QueryResult {
    pub to_amount: u128,
    pub swap_fee: u128,
}
impl QueryResult {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let to_amount: u128 = crate::borsh_de_or_default(&mut reader)?;
        let swap_fee: u128 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { to_amount, swap_fee })
    }
}
