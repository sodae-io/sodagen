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
    pub vault_program: Pubkey,
    pub ncn_count: PodU64,
    pub operator_count: PodU64,
    pub epoch_length: PodU64,
    pub bump: u8,
    #[serde(with = "crate::big_array_serde")]
    pub reserved: [u8; 263],
}
impl Config {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault_program: Pubkey = crate::borsh_de_or_default(&mut reader)?;
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
        let epoch_length = if reader.is_empty() {
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
            admin,
            vault_program,
            ncn_count,
            operator_count,
            epoch_length,
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
pub struct Ncn {
    pub base: Pubkey,
    pub admin: Pubkey,
    pub operator_admin: Pubkey,
    pub vault_admin: Pubkey,
    pub slasher_admin: Pubkey,
    pub delegate_admin: Pubkey,
    pub metadata_admin: Pubkey,
    pub weight_table_admin: Pubkey,
    pub ncn_program_admin: Pubkey,
    pub index: PodU64,
    pub operator_count: PodU64,
    pub vault_count: PodU64,
    pub slasher_count: PodU64,
    pub bump: u8,
    #[serde(with = "crate::big_array_serde")]
    pub reserved: [u8; 263],
}
impl Ncn {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let base: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let operator_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let slasher_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let delegate_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let metadata_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let weight_table_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let ncn_program_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let index = if reader.is_empty() {
            Default::default()
        } else {
            <PodU64>::deserialize(&mut reader)?
        };
        let operator_count = if reader.is_empty() {
            Default::default()
        } else {
            <PodU64>::deserialize(&mut reader)?
        };
        let vault_count = if reader.is_empty() {
            Default::default()
        } else {
            <PodU64>::deserialize(&mut reader)?
        };
        let slasher_count = if reader.is_empty() {
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
            base,
            admin,
            operator_admin,
            vault_admin,
            slasher_admin,
            delegate_admin,
            metadata_admin,
            weight_table_admin,
            ncn_program_admin,
            index,
            operator_count,
            vault_count,
            slasher_count,
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
pub struct NcnOperatorState {
    pub ncn: Pubkey,
    pub operator: Pubkey,
    pub index: PodU64,
    pub ncn_opt_in_state: SlotToggle,
    pub operator_opt_in_state: SlotToggle,
    pub bump: u8,
    #[serde(with = "crate::big_array_serde")]
    pub reserved: [u8; 263],
}
impl NcnOperatorState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let ncn: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let operator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let index = if reader.is_empty() {
            Default::default()
        } else {
            <PodU64>::deserialize(&mut reader)?
        };
        let ncn_opt_in_state = if reader.is_empty() {
            Default::default()
        } else {
            <SlotToggle>::deserialize(&mut reader)?
        };
        let operator_opt_in_state = if reader.is_empty() {
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
            ncn,
            operator,
            index,
            ncn_opt_in_state,
            operator_opt_in_state,
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
pub struct NcnVaultSlasherTicket {
    pub ncn: Pubkey,
    pub vault: Pubkey,
    pub slasher: Pubkey,
    pub max_slashable_per_epoch: PodU64,
    pub index: PodU64,
    pub state: SlotToggle,
    pub bump: u8,
    #[serde(with = "crate::big_array_serde")]
    pub reserved: [u8; 263],
}
impl NcnVaultSlasherTicket {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let ncn: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
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
            ncn,
            vault,
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
pub struct NcnVaultTicket {
    pub ncn: Pubkey,
    pub vault: Pubkey,
    pub index: PodU64,
    pub state: SlotToggle,
    pub bump: u8,
    #[serde(with = "crate::big_array_serde")]
    pub reserved: [u8; 263],
}
impl NcnVaultTicket {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let ncn: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
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
            ncn,
            vault,
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
pub struct Operator {
    pub base: Pubkey,
    pub admin: Pubkey,
    pub ncn_admin: Pubkey,
    pub vault_admin: Pubkey,
    pub delegate_admin: Pubkey,
    pub metadata_admin: Pubkey,
    pub voter: Pubkey,
    pub index: PodU64,
    pub ncn_count: PodU64,
    pub vault_count: PodU64,
    pub operator_fee_bps: PodU16,
    pub bump: u8,
    #[serde(with = "crate::big_array_serde")]
    pub reserved_space: [u8; 261],
}
impl Operator {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let base: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let ncn_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let delegate_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let metadata_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let voter: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let index = if reader.is_empty() {
            Default::default()
        } else {
            <PodU64>::deserialize(&mut reader)?
        };
        let ncn_count = if reader.is_empty() {
            Default::default()
        } else {
            <PodU64>::deserialize(&mut reader)?
        };
        let vault_count = if reader.is_empty() {
            Default::default()
        } else {
            <PodU64>::deserialize(&mut reader)?
        };
        let operator_fee_bps = if reader.is_empty() {
            Default::default()
        } else {
            <PodU16>::deserialize(&mut reader)?
        };
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let reserved_space = <[u8; 261] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            base,
            admin,
            ncn_admin,
            vault_admin,
            delegate_admin,
            metadata_admin,
            voter,
            index,
            ncn_count,
            vault_count,
            operator_fee_bps,
            bump,
            reserved_space,
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
pub struct OperatorVaultTicket {
    pub operator: Pubkey,
    pub vault: Pubkey,
    pub index: PodU64,
    pub state: SlotToggle,
    pub bump: u8,
    #[serde(with = "crate::big_array_serde")]
    pub reserved: [u8; 263],
}
impl OperatorVaultTicket {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let operator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
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
            operator,
            vault,
            index,
            state,
            bump,
            reserved,
        })
    }
}
