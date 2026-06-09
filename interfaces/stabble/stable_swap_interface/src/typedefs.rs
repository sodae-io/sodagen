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
pub struct PoolBalanceUpdatedData {
    pub balances: Vec<u64>,
}
impl PoolBalanceUpdatedData {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let balances: Vec<u64> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { balances })
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
pub struct PoolToken {
    pub mint: Pubkey,
    pub decimals: u8,
    pub scaling_up: bool,
    pub scaling_factor: u64,
    pub balance: u64,
}
impl PoolToken {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let scaling_up: bool = crate::borsh_de_or_default(&mut reader)?;
        let scaling_factor: u64 = crate::borsh_de_or_default(&mut reader)?;
        let balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            mint,
            decimals,
            scaling_up,
            scaling_factor,
            balance,
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
pub struct PoolUpdatedData {
    pub is_active: bool,
    pub amp_initial_factor: u16,
    pub amp_target_factor: u16,
    pub ramp_start_ts: i64,
    pub ramp_stop_ts: i64,
    pub swap_fee: u64,
    pub max_supply: u64,
}
impl PoolUpdatedData {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let is_active: bool = crate::borsh_de_or_default(&mut reader)?;
        let amp_initial_factor: u16 = crate::borsh_de_or_default(&mut reader)?;
        let amp_target_factor: u16 = crate::borsh_de_or_default(&mut reader)?;
        let ramp_start_ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let ramp_stop_ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let swap_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            is_active,
            amp_initial_factor,
            amp_target_factor,
            ramp_start_ts,
            ramp_stop_ts,
            swap_fee,
            max_supply,
        })
    }
}
