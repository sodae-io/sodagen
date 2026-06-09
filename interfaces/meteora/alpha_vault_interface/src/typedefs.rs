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
pub enum AccountsType {
    #[default]
    TransferHookX,
    TransferHookY,
    TransferHookReward,
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
pub struct CreateMerkleRootConfigParams {
    pub root: [u8; 32],
    pub version: u64,
}
impl CreateMerkleRootConfigParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let root: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        let version: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { root, version })
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
pub struct FcfsConfigParameters {
    pub max_depositing_cap: u64,
    pub start_vesting_duration: u64,
    pub end_vesting_duration: u64,
    pub depositing_duration_until_last_join_point: u64,
    pub individual_depositing_cap: u64,
    pub escrow_fee: u64,
    pub activation_type: u8,
    pub index: u64,
}
impl FcfsConfigParameters {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let max_depositing_cap: u64 = crate::borsh_de_or_default(&mut reader)?;
        let start_vesting_duration: u64 = crate::borsh_de_or_default(&mut reader)?;
        let end_vesting_duration: u64 = crate::borsh_de_or_default(&mut reader)?;
        let depositing_duration_until_last_join_point: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let individual_depositing_cap: u64 = crate::borsh_de_or_default(&mut reader)?;
        let escrow_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let activation_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let index: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            max_depositing_cap,
            start_vesting_duration,
            end_vesting_duration,
            depositing_duration_until_last_join_point,
            individual_depositing_cap,
            escrow_fee,
            activation_type,
            index,
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
pub struct InitializeFcfsVaultParams {
    pub pool_type: u8,
    pub quote_mint: Pubkey,
    pub base_mint: Pubkey,
    pub depositing_point: u64,
    pub start_vesting_point: u64,
    pub end_vesting_point: u64,
    pub max_depositing_cap: u64,
    pub individual_depositing_cap: u64,
    pub escrow_fee: u64,
    pub whitelist_mode: u8,
}
impl InitializeFcfsVaultParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let quote_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let depositing_point: u64 = crate::borsh_de_or_default(&mut reader)?;
        let start_vesting_point: u64 = crate::borsh_de_or_default(&mut reader)?;
        let end_vesting_point: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_depositing_cap: u64 = crate::borsh_de_or_default(&mut reader)?;
        let individual_depositing_cap: u64 = crate::borsh_de_or_default(&mut reader)?;
        let escrow_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let whitelist_mode: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool_type,
            quote_mint,
            base_mint,
            depositing_point,
            start_vesting_point,
            end_vesting_point,
            max_depositing_cap,
            individual_depositing_cap,
            escrow_fee,
            whitelist_mode,
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
pub struct InitializeProrataVaultParams {
    pub pool_type: u8,
    pub quote_mint: Pubkey,
    pub base_mint: Pubkey,
    pub depositing_point: u64,
    pub start_vesting_point: u64,
    pub end_vesting_point: u64,
    pub max_buying_cap: u64,
    pub escrow_fee: u64,
    pub whitelist_mode: u8,
}
impl InitializeProrataVaultParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let quote_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let depositing_point: u64 = crate::borsh_de_or_default(&mut reader)?;
        let start_vesting_point: u64 = crate::borsh_de_or_default(&mut reader)?;
        let end_vesting_point: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_buying_cap: u64 = crate::borsh_de_or_default(&mut reader)?;
        let escrow_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let whitelist_mode: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool_type,
            quote_mint,
            base_mint,
            depositing_point,
            start_vesting_point,
            end_vesting_point,
            max_buying_cap,
            escrow_fee,
            whitelist_mode,
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
pub struct InitializeVaultWithConfigParams {
    pub pool_type: u8,
    pub quote_mint: Pubkey,
    pub base_mint: Pubkey,
    pub whitelist_mode: u8,
}
impl InitializeVaultWithConfigParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let quote_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let whitelist_mode: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool_type,
            quote_mint,
            base_mint,
            whitelist_mode,
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
pub struct ProrataConfigParameters {
    pub max_buying_cap: u64,
    pub start_vesting_duration: u64,
    pub end_vesting_duration: u64,
    pub escrow_fee: u64,
    pub activation_type: u8,
    pub index: u64,
}
impl ProrataConfigParameters {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let max_buying_cap: u64 = crate::borsh_de_or_default(&mut reader)?;
        let start_vesting_duration: u64 = crate::borsh_de_or_default(&mut reader)?;
        let end_vesting_duration: u64 = crate::borsh_de_or_default(&mut reader)?;
        let escrow_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let activation_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let index: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            max_buying_cap,
            start_vesting_duration,
            end_vesting_duration,
            escrow_fee,
            activation_type,
            index,
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
pub struct RemainingAccountsInfo {
    pub slices: Vec<RemainingAccountsSlice>,
}
impl RemainingAccountsInfo {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let slices: Vec<RemainingAccountsSlice> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self { slices })
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
pub struct RemainingAccountsSlice {
    pub accounts_type: AccountsType,
    pub length: u8,
}
impl RemainingAccountsSlice {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let accounts_type: AccountsType = crate::borsh_de_or_default(&mut reader)?;
        let length: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { accounts_type, length })
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
pub struct UpdateFcfsVaultParams {
    pub max_depositing_cap: u64,
    pub depositing_point: u64,
    pub individual_depositing_cap: u64,
    pub start_vesting_point: u64,
    pub end_vesting_point: u64,
}
impl UpdateFcfsVaultParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let max_depositing_cap: u64 = crate::borsh_de_or_default(&mut reader)?;
        let depositing_point: u64 = crate::borsh_de_or_default(&mut reader)?;
        let individual_depositing_cap: u64 = crate::borsh_de_or_default(&mut reader)?;
        let start_vesting_point: u64 = crate::borsh_de_or_default(&mut reader)?;
        let end_vesting_point: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            max_depositing_cap,
            depositing_point,
            individual_depositing_cap,
            start_vesting_point,
            end_vesting_point,
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
pub struct UpdateProrataVaultParams {
    pub max_buying_cap: u64,
    pub start_vesting_point: u64,
    pub end_vesting_point: u64,
}
impl UpdateProrataVaultParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let max_buying_cap: u64 = crate::borsh_de_or_default(&mut reader)?;
        let start_vesting_point: u64 = crate::borsh_de_or_default(&mut reader)?;
        let end_vesting_point: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            max_buying_cap,
            start_vesting_point,
            end_vesting_point,
        })
    }
}
