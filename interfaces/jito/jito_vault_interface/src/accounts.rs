use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct Config {
    pub admin: Pubkey,
    pub restaking_program: Pubkey,
    pub epoch_length: PodU64,
    pub num_vaults: PodU64,
    pub deposit_withdrawal_fee_cap_bps: PodU16,
    pub fee_rate_of_change_bps: PodU16,
    pub fee_bump_bps: PodU16,
    pub program_fee_bps: PodU16,
    pub program_fee_wallet: Pubkey,
    pub fee_admin: Pubkey,
    pub bump: u8,
    #[serde(with = "crate::big_array_serde")]
    pub reserved: [u8; 229],
}
impl Config {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let restaking_program: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let epoch_length = if reader.is_empty() {
            Default::default()
        } else {
            <PodU64>::deserialize(&mut reader)?
        };
        let num_vaults = if reader.is_empty() {
            Default::default()
        } else {
            <PodU64>::deserialize(&mut reader)?
        };
        let deposit_withdrawal_fee_cap_bps = if reader.is_empty() {
            Default::default()
        } else {
            <PodU16>::deserialize(&mut reader)?
        };
        let fee_rate_of_change_bps = if reader.is_empty() {
            Default::default()
        } else {
            <PodU16>::deserialize(&mut reader)?
        };
        let fee_bump_bps = if reader.is_empty() {
            Default::default()
        } else {
            <PodU16>::deserialize(&mut reader)?
        };
        let program_fee_bps = if reader.is_empty() {
            Default::default()
        } else {
            <PodU16>::deserialize(&mut reader)?
        };
        let program_fee_wallet: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let reserved = <[u8; 229] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            admin,
            restaking_program,
            epoch_length,
            num_vaults,
            deposit_withdrawal_fee_cap_bps,
            fee_rate_of_change_bps,
            fee_bump_bps,
            program_fee_bps,
            program_fee_wallet,
            fee_admin,
            bump,
            reserved,
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
pub struct Vault {
    pub base: Pubkey,
    pub vrt_mint: Pubkey,
    pub supported_mint: Pubkey,
    pub vrt_supply: PodU64,
    pub tokens_deposited: PodU64,
    pub deposit_capacity: PodU64,
    pub delegation_state: DelegationState,
    pub additional_assets_need_unstaking: PodU64,
    pub vrt_enqueued_for_cooldown_amount: PodU64,
    pub vrt_cooling_down_amount: PodU64,
    pub vrt_ready_to_claim_amount: PodU64,
    pub admin: Pubkey,
    pub delegation_admin: Pubkey,
    pub operator_admin: Pubkey,
    pub ncn_admin: Pubkey,
    pub slasher_admin: Pubkey,
    pub capacity_admin: Pubkey,
    pub fee_admin: Pubkey,
    pub delegate_asset_admin: Pubkey,
    pub fee_wallet: Pubkey,
    pub mint_burn_admin: Pubkey,
    pub metadata_admin: Pubkey,
    pub vault_index: PodU64,
    pub ncn_count: PodU64,
    pub operator_count: PodU64,
    pub slasher_count: PodU64,
    pub last_fee_change_slot: PodU64,
    pub last_full_state_update_slot: PodU64,
    pub deposit_fee_bps: PodU16,
    pub withdrawal_fee_bps: PodU16,
    pub next_withdrawal_fee_bps: PodU16,
    pub reward_fee_bps: PodU16,
    pub program_fee_bps: PodU16,
    pub bump: u8,
    pub is_paused: PodBool,
    pub last_start_state_update_slot: PodU64,
    #[serde(with = "crate::big_array_serde")]
    pub reserved: [u8; 251],
}
impl Vault {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let base: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vrt_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let supported_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vrt_supply = if reader.is_empty() {
            Default::default()
        } else {
            <PodU64>::deserialize(&mut reader)?
        };
        let tokens_deposited = if reader.is_empty() {
            Default::default()
        } else {
            <PodU64>::deserialize(&mut reader)?
        };
        let deposit_capacity = if reader.is_empty() {
            Default::default()
        } else {
            <PodU64>::deserialize(&mut reader)?
        };
        let delegation_state = <DelegationState>::deserialize(&mut reader)?;
        let additional_assets_need_unstaking = if reader.is_empty() {
            Default::default()
        } else {
            <PodU64>::deserialize(&mut reader)?
        };
        let vrt_enqueued_for_cooldown_amount = if reader.is_empty() {
            Default::default()
        } else {
            <PodU64>::deserialize(&mut reader)?
        };
        let vrt_cooling_down_amount = if reader.is_empty() {
            Default::default()
        } else {
            <PodU64>::deserialize(&mut reader)?
        };
        let vrt_ready_to_claim_amount = if reader.is_empty() {
            Default::default()
        } else {
            <PodU64>::deserialize(&mut reader)?
        };
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let delegation_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let operator_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let ncn_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let slasher_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let capacity_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let delegate_asset_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_wallet: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint_burn_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let metadata_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault_index = if reader.is_empty() {
            Default::default()
        } else {
            <PodU64>::deserialize(&mut reader)?
        };
        let ncn_count = if reader.is_empty() {
            Default::default()
        } else {
            <PodU64>::deserialize(&mut reader)?
        };
        let operator_count = if reader.is_empty() {
            Default::default()
        } else {
            <PodU64>::deserialize(&mut reader)?
        };
        let slasher_count = if reader.is_empty() {
            Default::default()
        } else {
            <PodU64>::deserialize(&mut reader)?
        };
        let last_fee_change_slot = if reader.is_empty() {
            Default::default()
        } else {
            <PodU64>::deserialize(&mut reader)?
        };
        let last_full_state_update_slot = if reader.is_empty() {
            Default::default()
        } else {
            <PodU64>::deserialize(&mut reader)?
        };
        let deposit_fee_bps = if reader.is_empty() {
            Default::default()
        } else {
            <PodU16>::deserialize(&mut reader)?
        };
        let withdrawal_fee_bps = if reader.is_empty() {
            Default::default()
        } else {
            <PodU16>::deserialize(&mut reader)?
        };
        let next_withdrawal_fee_bps = if reader.is_empty() {
            Default::default()
        } else {
            <PodU16>::deserialize(&mut reader)?
        };
        let reward_fee_bps = if reader.is_empty() {
            Default::default()
        } else {
            <PodU16>::deserialize(&mut reader)?
        };
        let program_fee_bps = if reader.is_empty() {
            Default::default()
        } else {
            <PodU16>::deserialize(&mut reader)?
        };
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let is_paused = if reader.is_empty() {
            Default::default()
        } else {
            <PodBool>::deserialize(&mut reader)?
        };
        let last_start_state_update_slot = if reader.is_empty() {
            Default::default()
        } else {
            <PodU64>::deserialize(&mut reader)?
        };
        let reserved = <[u8; 251] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            base,
            vrt_mint,
            supported_mint,
            vrt_supply,
            tokens_deposited,
            deposit_capacity,
            delegation_state,
            additional_assets_need_unstaking,
            vrt_enqueued_for_cooldown_amount,
            vrt_cooling_down_amount,
            vrt_ready_to_claim_amount,
            admin,
            delegation_admin,
            operator_admin,
            ncn_admin,
            slasher_admin,
            capacity_admin,
            fee_admin,
            delegate_asset_admin,
            fee_wallet,
            mint_burn_admin,
            metadata_admin,
            vault_index,
            ncn_count,
            operator_count,
            slasher_count,
            last_fee_change_slot,
            last_full_state_update_slot,
            deposit_fee_bps,
            withdrawal_fee_bps,
            next_withdrawal_fee_bps,
            reward_fee_bps,
            program_fee_bps,
            bump,
            is_paused,
            last_start_state_update_slot,
            reserved,
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
pub struct VaultNcnSlasherOperatorTicket {
    pub vault: Pubkey,
    pub ncn: Pubkey,
    pub slasher: Pubkey,
    pub operator: Pubkey,
    pub epoch: PodU64,
    pub slashed: PodU64,
    pub bump: u8,
    #[serde(with = "crate::big_array_serde")]
    pub reserved: [u8; 263],
}
impl VaultNcnSlasherOperatorTicket {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let ncn: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let slasher: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let operator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let epoch = if reader.is_empty() {
            Default::default()
        } else {
            <PodU64>::deserialize(&mut reader)?
        };
        let slashed = if reader.is_empty() {
            Default::default()
        } else {
            <PodU64>::deserialize(&mut reader)?
        };
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let reserved = <[u8; 263] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            vault,
            ncn,
            slasher,
            operator,
            epoch,
            slashed,
            bump,
            reserved,
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
pub struct VaultNcnSlasherTicket {
    pub vault: Pubkey,
    pub ncn: Pubkey,
    pub slasher: Pubkey,
    pub max_slashable_per_epoch: PodU64,
    pub index: PodU64,
    pub state: SlotToggle,
    pub bump: u8,
    #[serde(with = "crate::big_array_serde")]
    pub reserved: [u8; 263],
}
impl VaultNcnSlasherTicket {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let ncn: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let slasher: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let max_slashable_per_epoch = if reader.is_empty() {
            Default::default()
        } else {
            <PodU64>::deserialize(&mut reader)?
        };
        let index = if reader.is_empty() {
            Default::default()
        } else {
            <PodU64>::deserialize(&mut reader)?
        };
        let state = if reader.is_empty() {
            Default::default()
        } else {
            <SlotToggle>::deserialize(&mut reader)?
        };
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let reserved = <[u8; 263] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            vault,
            ncn,
            slasher,
            max_slashable_per_epoch,
            index,
            state,
            bump,
            reserved,
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
pub struct VaultNcnTicket {
    pub vault: Pubkey,
    pub ncn: Pubkey,
    pub index: PodU64,
    pub state: SlotToggle,
    pub bump: u8,
    #[serde(with = "crate::big_array_serde")]
    pub reserved: [u8; 263],
}
impl VaultNcnTicket {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let ncn: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let index = if reader.is_empty() {
            Default::default()
        } else {
            <PodU64>::deserialize(&mut reader)?
        };
        let state = if reader.is_empty() {
            Default::default()
        } else {
            <SlotToggle>::deserialize(&mut reader)?
        };
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let reserved = <[u8; 263] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            vault,
            ncn,
            index,
            state,
            bump,
            reserved,
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
pub struct VaultOperatorDelegation {
    pub vault: Pubkey,
    pub operator: Pubkey,
    pub delegation_state: DelegationState,
    pub last_update_slot: PodU64,
    pub index: PodU64,
    pub bump: u8,
    #[serde(with = "crate::big_array_serde")]
    pub reserved: [u8; 263],
}
impl VaultOperatorDelegation {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let operator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let delegation_state = <DelegationState>::deserialize(&mut reader)?;
        let last_update_slot = if reader.is_empty() {
            Default::default()
        } else {
            <PodU64>::deserialize(&mut reader)?
        };
        let index = if reader.is_empty() {
            Default::default()
        } else {
            <PodU64>::deserialize(&mut reader)?
        };
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let reserved = <[u8; 263] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            vault,
            operator,
            delegation_state,
            last_update_slot,
            index,
            bump,
            reserved,
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
pub struct VaultStakerWithdrawalTicket {
    pub vault: Pubkey,
    pub staker: Pubkey,
    pub base: Pubkey,
    pub vrt_amount: PodU64,
    pub slot_unstaked: PodU64,
    pub bump: u8,
    #[serde(with = "crate::big_array_serde")]
    pub reserved: [u8; 263],
}
impl VaultStakerWithdrawalTicket {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let staker: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vrt_amount = if reader.is_empty() {
            Default::default()
        } else {
            <PodU64>::deserialize(&mut reader)?
        };
        let slot_unstaked = if reader.is_empty() {
            Default::default()
        } else {
            <PodU64>::deserialize(&mut reader)?
        };
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let reserved = <[u8; 263] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            vault,
            staker,
            base,
            vrt_amount,
            slot_unstaked,
            bump,
            reserved,
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
pub struct VaultUpdateStateTracker {
    pub vault: Pubkey,
    pub ncn_epoch: PodU64,
    pub last_updated_index: PodU64,
    pub delegation_state: DelegationState,
    pub withdrawal_allocation_method: u8,
    #[serde(with = "crate::big_array_serde")]
    pub reserved: [u8; 263],
}
impl VaultUpdateStateTracker {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let ncn_epoch = if reader.is_empty() {
            Default::default()
        } else {
            <PodU64>::deserialize(&mut reader)?
        };
        let last_updated_index = if reader.is_empty() {
            Default::default()
        } else {
            <PodU64>::deserialize(&mut reader)?
        };
        let delegation_state = <DelegationState>::deserialize(&mut reader)?;
        let withdrawal_allocation_method: u8 = crate::borsh_de_or_default(&mut reader)?;
        let reserved = <[u8; 263] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            vault,
            ncn_epoch,
            last_updated_index,
            delegation_state,
            withdrawal_allocation_method,
            reserved,
        })
    }
}
