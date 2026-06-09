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
pub struct CreateParams {
    pub shift: u128,
    pub initial_token_b_reserves: u64,
    pub fee_params: FeeParams,
}
impl CreateParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let shift: u128 = crate::borsh_de_or_default(&mut reader)?;
        let initial_token_b_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_params = if reader.is_empty() {
            Default::default()
        } else {
            <FeeParams>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            shift,
            initial_token_b_reserves,
            fee_params,
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
pub struct FeeParams {
    pub normalization_period: u64,
    pub decay: f64,
    pub reference: u64,
    pub royalties_bps: u16,
    pub privileged_swapper: Option<Pubkey>,
}
impl FeeParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let normalization_period: u64 = crate::borsh_de_or_default(&mut reader)?;
        let decay: f64 = crate::borsh_de_or_default(&mut reader)?;
        let reference: u64 = crate::borsh_de_or_default(&mut reader)?;
        let royalties_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let privileged_swapper: Option<Pubkey> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            normalization_period,
            decay,
            reference,
            royalties_bps,
            privileged_swapper,
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
pub struct SwapParams {
    pub amount: u64,
    pub limit: u64,
}
impl SwapParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let limit: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { amount, limit })
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
pub struct SwapResult {
    pub new_reserves_a: u128,
    pub new_reserves_b: u128,
    pub amount_a: u64,
    pub amount_b: u64,
    pub fee_a: u64,
}
impl SwapResult {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let new_reserves_a: u128 = crate::borsh_de_or_default(&mut reader)?;
        let new_reserves_b: u128 = crate::borsh_de_or_default(&mut reader)?;
        let amount_a: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_b: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_a: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            new_reserves_a,
            new_reserves_b,
            amount_a,
            amount_b,
            fee_a,
        })
    }
}
