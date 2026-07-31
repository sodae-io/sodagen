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
    pub curve: CurveType,
    pub fee_beneficiaries: Vec<FeeBeneficiary>,
}
impl CreateParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let shift: u128 = crate::borsh_de_or_default(&mut reader)?;
        let initial_token_b_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let curve: CurveType = crate::borsh_de_or_default(&mut reader)?;
        let fee_beneficiaries: Vec<FeeBeneficiary> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            shift,
            initial_token_b_reserves,
            curve,
            fee_beneficiaries,
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
pub enum CurveType {
    #[default]
    ConstantProduct,
    Exponential,
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
pub struct FeeBeneficiary {
    pub wallet: Pubkey,
    pub share_bps: u16,
}
impl FeeBeneficiary {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let wallet: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let share_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { wallet, share_bps })
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
pub struct PlatformConfigView {
    pub authority: Pubkey,
    pub fee_beneficiary: Pubkey,
    pub base_token: Pubkey,
    pub platform_fee_bps: u16,
    pub graduation_threshold: u64,
}
impl PlatformConfigView {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_beneficiary: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_token: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let platform_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let graduation_threshold: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            authority,
            fee_beneficiary,
            base_token,
            platform_fee_bps,
            graduation_threshold,
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
