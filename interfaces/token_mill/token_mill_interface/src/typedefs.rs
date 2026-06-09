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
pub struct MarketSettings {
    pub max_supply: u64,
    pub sqrt_price_a_x96: u128,
    pub sqrt_price_b_x96: u128,
    pub liquidity_a: u128,
    pub liquidity_b: u128,
    pub fee: u32,
}
impl MarketSettings {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let max_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let sqrt_price_a_x96: u128 = crate::borsh_de_or_default(&mut reader)?;
        let sqrt_price_b_x96: u128 = crate::borsh_de_or_default(&mut reader)?;
        let liquidity_a: u128 = crate::borsh_de_or_default(&mut reader)?;
        let liquidity_b: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fee: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            max_supply,
            sqrt_price_a_x96,
            sqrt_price_b_x96,
            liquidity_a,
            liquidity_b,
            fee,
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
pub struct MarketSettingsInput {
    pub max_supply: u64,
    pub supply_at_graduation: u64,
    pub sqrt_price_a_x96: u128,
    pub sqrt_price_b_x96: u128,
    pub fee: u32,
}
impl MarketSettingsInput {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let max_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let supply_at_graduation: u64 = crate::borsh_de_or_default(&mut reader)?;
        let sqrt_price_a_x96: u128 = crate::borsh_de_or_default(&mut reader)?;
        let sqrt_price_b_x96: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fee: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            max_supply,
            supply_at_graduation,
            sqrt_price_a_x96,
            sqrt_price_b_x96,
            fee,
        })
    }
}
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub enum SwapParameters {
    BuyExactIn(u64, u64),
    BuyExactOut(u64, u64),
    SellExactIn(u64, u64),
    SellExactOut(u64, u64),
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
pub struct SwapResult {
    pub amount_in: u64,
    pub amount_out: u64,
    pub fee_amount_token_in: u64,
    pub fee_amount_token_1: u64,
    pub next_sqrt_price: u128,
}
impl SwapResult {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_amount_token_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_amount_token_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let next_sqrt_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            amount_in,
            amount_out,
            fee_amount_token_in,
            fee_amount_token_1,
            next_sqrt_price,
        })
    }
}
