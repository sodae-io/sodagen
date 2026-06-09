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
pub struct WithdrawDestToken {
    pub withdraw_amount: u64,
    pub coin_amount: u64,
    pub pc_amount: u64,
    pub dest_token_coin: Pubkey,
    pub dest_token_pc: Pubkey,
}
impl WithdrawDestToken {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let withdraw_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let coin_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pc_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let dest_token_coin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let dest_token_pc: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            withdraw_amount,
            coin_amount,
            pc_amount,
            dest_token_coin,
            dest_token_pc,
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
pub struct WithdrawQueue {
    pub owner: [u64; 4],
    pub head: u64,
    pub count: u64,
    #[serde(with = "crate::big_array_serde")]
    pub buf: [WithdrawDestToken; 64],
}
impl WithdrawQueue {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let owner: [u64; 4] = crate::borsh_de_or_default(&mut reader)?;
        let head: u64 = crate::borsh_de_or_default(&mut reader)?;
        let count: u64 = crate::borsh_de_or_default(&mut reader)?;
        let buf = <[WithdrawDestToken; 64] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self { owner, head, count, buf })
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
pub struct TargetOrder {
    pub price: u64,
    pub vol: u64,
}
impl TargetOrder {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let vol: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { price, vol })
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
pub struct OutPutData {
    pub need_take_pnl_coin: u64,
    pub need_take_pnl_pc: u64,
    pub total_pnl_pc: u64,
    pub total_pnl_coin: u64,
    pub pool_open_time: u64,
    pub punish_pc_amount: u64,
    pub punish_coin_amount: u64,
    pub orderbook_to_init_time: u64,
    pub swap_coin_in_amount: u128,
    pub swap_pc_out_amount: u128,
    pub swap_take_pc_fee: u64,
    pub swap_pc_in_amount: u128,
    pub swap_coin_out_amount: u128,
    pub swap_take_coin_fee: u64,
}
impl OutPutData {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let need_take_pnl_coin: u64 = crate::borsh_de_or_default(&mut reader)?;
        let need_take_pnl_pc: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_pnl_pc: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_pnl_coin: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pool_open_time: u64 = crate::borsh_de_or_default(&mut reader)?;
        let punish_pc_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let punish_coin_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let orderbook_to_init_time: u64 = crate::borsh_de_or_default(&mut reader)?;
        let swap_coin_in_amount: u128 = crate::borsh_de_or_default(&mut reader)?;
        let swap_pc_out_amount: u128 = crate::borsh_de_or_default(&mut reader)?;
        let swap_take_pc_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let swap_pc_in_amount: u128 = crate::borsh_de_or_default(&mut reader)?;
        let swap_coin_out_amount: u128 = crate::borsh_de_or_default(&mut reader)?;
        let swap_take_coin_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            need_take_pnl_coin,
            need_take_pnl_pc,
            total_pnl_pc,
            total_pnl_coin,
            pool_open_time,
            punish_pc_amount,
            punish_coin_amount,
            orderbook_to_init_time,
            swap_coin_in_amount,
            swap_pc_out_amount,
            swap_take_pc_fee,
            swap_pc_in_amount,
            swap_coin_out_amount,
            swap_take_coin_fee,
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
pub struct AmmConfig {
    pub pnl_owner: Pubkey,
    pub cancel_owner: Pubkey,
    pub pending1: [u64; 28],
    pub pending2: [u64; 32],
}
impl AmmConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pnl_owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let cancel_owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pending1: [u64; 28] = crate::borsh_de_or_default(&mut reader)?;
        let pending2: [u64; 32] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pnl_owner,
            cancel_owner,
            pending1,
            pending2,
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
pub struct LastOrderDistance {
    pub last_order_numerator: u64,
    pub last_order_denominator: u64,
}
impl LastOrderDistance {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let last_order_numerator: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_order_denominator: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            last_order_numerator,
            last_order_denominator,
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
pub struct NeedTake {
    pub need_take_pc: u64,
    pub need_take_coin: u64,
}
impl NeedTake {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let need_take_pc: u64 = crate::borsh_de_or_default(&mut reader)?;
        let need_take_coin: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            need_take_pc,
            need_take_coin,
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
pub struct SwapInstructionBaseIn {
    pub amount_in: u64,
    pub minimum_amount_out: u64,
}
impl SwapInstructionBaseIn {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let minimum_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            amount_in,
            minimum_amount_out,
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
pub struct SwapInstructionBaseOut {
    pub max_amount_in: u64,
    pub amount_out: u64,
}
impl SwapInstructionBaseOut {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let max_amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { max_amount_in, amount_out })
    }
}
