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
pub struct LpChangeEvent {
    pub pool_id: Pubkey,
    pub lp_amount_before: u64,
    pub token_0_vault_before: u64,
    pub token_1_vault_before: u64,
    pub token_0_amount: u64,
    pub token_1_amount: u64,
    pub token_0_transfer_fee: u64,
    pub token_1_transfer_fee: u64,
    pub change_type: u8,
}
impl LpChangeEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lp_amount_before: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_0_vault_before: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_1_vault_before: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_0_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_1_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_0_transfer_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_1_transfer_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let change_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool_id,
            lp_amount_before,
            token_0_vault_before,
            token_1_vault_before,
            token_0_amount,
            token_1_amount,
            token_0_transfer_fee,
            token_1_transfer_fee,
            change_type,
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
pub struct MigrationEvent {
    pub from_pool: Pubkey,
    pub to_pool: Pubkey,
    pub token_0_amount_withdrawn: u64,
    pub token_1_amount_withdrawn: u64,
    pub lp_tokens_migrated: u128,
}
impl MigrationEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let from_pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let to_pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_0_amount_withdrawn: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_1_amount_withdrawn: u64 = crate::borsh_de_or_default(&mut reader)?;
        let lp_tokens_migrated: u128 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            from_pool,
            to_pool,
            token_0_amount_withdrawn,
            token_1_amount_withdrawn,
            lp_tokens_migrated,
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
pub struct Observation {
    pub block_timestamp: u64,
    pub cumulative_token_0_price_x32: u128,
    pub cumulative_token_1_price_x32: u128,
}
impl Observation {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let block_timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        let cumulative_token_0_price_x32: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let cumulative_token_1_price_x32: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            block_timestamp,
            cumulative_token_0_price_x32,
            cumulative_token_1_price_x32,
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
pub struct PartnerInfo {
    pub partner: Pubkey,
    pub lp_token_linked_with_partner: u64,
    pub total_claimed_fee_amount_token_0: u64,
    pub total_claimed_fee_amount_token_1: u64,
    pub total_earned_fee_amount_token_0: u64,
    pub total_earned_fee_amount_token_1: u64,
}
impl PartnerInfo {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let partner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lp_token_linked_with_partner: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_claimed_fee_amount_token_0: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let total_claimed_fee_amount_token_1: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let total_earned_fee_amount_token_0: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let total_earned_fee_amount_token_1: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            partner,
            lp_token_linked_with_partner,
            total_claimed_fee_amount_token_0,
            total_claimed_fee_amount_token_1,
            total_earned_fee_amount_token_0,
            total_earned_fee_amount_token_1,
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
pub struct SwapEvent {
    pub pool_id: Pubkey,
    pub input_vault_before: u64,
    pub output_vault_before: u64,
    pub input_amount: u64,
    pub output_amount: u64,
    pub input_mint: Pubkey,
    pub output_mint: Pubkey,
    pub input_transfer_fee: u64,
    pub output_transfer_fee: u64,
    pub base_input: bool,
    pub dynamic_fee: u128,
}
impl SwapEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let input_vault_before: u64 = crate::borsh_de_or_default(&mut reader)?;
        let output_vault_before: u64 = crate::borsh_de_or_default(&mut reader)?;
        let input_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let output_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let input_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let output_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let input_transfer_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let output_transfer_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let base_input: bool = crate::borsh_de_or_default(&mut reader)?;
        let dynamic_fee: u128 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool_id,
            input_vault_before,
            output_vault_before,
            input_amount,
            output_amount,
            input_mint,
            output_mint,
            input_transfer_fee,
            output_transfer_fee,
            base_input,
            dynamic_fee,
        })
    }
}
