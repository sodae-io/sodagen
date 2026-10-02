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
pub struct InitDexParams {
    pub center_price: u64,
    pub fee: u32,
    pub revenue_cut: u32,
    pub upper_percent: u32,
    pub lower_percent: u32,
    pub upper_shift_threshold: u32,
    pub lower_shift_threshold: u32,
    pub threshold_shift_time: u32,
    pub max_center_price: u64,
    pub min_center_price: u64,
}
impl InitDexParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let center_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee: u32 = crate::borsh_de_or_default(&mut reader)?;
        let revenue_cut: u32 = crate::borsh_de_or_default(&mut reader)?;
        let upper_percent: u32 = crate::borsh_de_or_default(&mut reader)?;
        let lower_percent: u32 = crate::borsh_de_or_default(&mut reader)?;
        let upper_shift_threshold: u32 = crate::borsh_de_or_default(&mut reader)?;
        let lower_shift_threshold: u32 = crate::borsh_de_or_default(&mut reader)?;
        let threshold_shift_time: u32 = crate::borsh_de_or_default(&mut reader)?;
        let max_center_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_center_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            center_price,
            fee,
            revenue_cut,
            upper_percent,
            lower_percent,
            upper_shift_threshold,
            lower_shift_threshold,
            threshold_shift_time,
            max_center_price,
            min_center_price,
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
pub struct UserBorrowConfigParams {
    pub expand_percent: u16,
    pub expand_duration: u32,
    pub base_debt_ceiling: u64,
    pub max_debt_ceiling: u64,
}
impl UserBorrowConfigParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let expand_percent: u16 = crate::borsh_de_or_default(&mut reader)?;
        let expand_duration: u32 = crate::borsh_de_or_default(&mut reader)?;
        let base_debt_ceiling: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_debt_ceiling: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            expand_percent,
            expand_duration,
            base_debt_ceiling,
            max_debt_ceiling,
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
pub struct UserSupplyConfigParams {
    pub expand_percent: u16,
    pub expand_duration: u64,
    pub base_withdrawal_limit: u64,
}
impl UserSupplyConfigParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let expand_percent: u16 = crate::borsh_de_or_default(&mut reader)?;
        let expand_duration: u64 = crate::borsh_de_or_default(&mut reader)?;
        let base_withdrawal_limit: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            expand_percent,
            expand_duration,
            base_withdrawal_limit,
        })
    }
}
