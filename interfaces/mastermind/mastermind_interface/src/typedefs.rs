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
pub struct CurveDataV1 {
    pub completed: i64,
    pub real_token_reserve: u64,
    pub real_sol_reserve: u64,
    pub virtual_token_reserve: u64,
    pub virtual_sol_reserve: u64,
    pub migration_kind: MigrationKind,
}
impl CurveDataV1 {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let completed: i64 = crate::borsh_de_or_default(&mut reader)?;
        let real_token_reserve: u64 = crate::borsh_de_or_default(&mut reader)?;
        let real_sol_reserve: u64 = crate::borsh_de_or_default(&mut reader)?;
        let virtual_token_reserve: u64 = crate::borsh_de_or_default(&mut reader)?;
        let virtual_sol_reserve: u64 = crate::borsh_de_or_default(&mut reader)?;
        let migration_kind: MigrationKind = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            completed,
            real_token_reserve,
            real_sol_reserve,
            virtual_token_reserve,
            virtual_sol_reserve,
            migration_kind,
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
pub struct InstructionBumps {
    pub bonding_curve: u8,
    pub bonding_curve_sol_associated_account: u8,
}
impl InstructionBumps {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bonding_curve: u8 = crate::borsh_de_or_default(&mut reader)?;
        let bonding_curve_sol_associated_account: u8 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            bonding_curve,
            bonding_curve_sol_associated_account,
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
pub enum MigrationKind {
    #[default]
    PSol,
    WSol,
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
pub struct StateDataV1 {
    pub initialized: bool,
    pub authority: Pubkey,
    pub pending_authority: Pubkey,
    pub wrapper_mint: Pubkey,
    pub fee_receiver: Pubkey,
    pub fee_bps: u64,
    pub initial_virtual_token_reserve: u64,
    pub initial_virtual_sol_reserve: u64,
    pub coin_lot_size: u64,
    pub pc_lot_size: u64,
    pub solvent_wrapper: u64,
    pub migrator: Pubkey,
    pub migration_count: u64,
    pub migration_refund: u64,
}
impl StateDataV1 {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let initialized: bool = crate::borsh_de_or_default(&mut reader)?;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pending_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let wrapper_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_receiver: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let initial_virtual_token_reserve: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let initial_virtual_sol_reserve: u64 = crate::borsh_de_or_default(&mut reader)?;
        let coin_lot_size: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pc_lot_size: u64 = crate::borsh_de_or_default(&mut reader)?;
        let solvent_wrapper: u64 = crate::borsh_de_or_default(&mut reader)?;
        let migrator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let migration_count: u64 = crate::borsh_de_or_default(&mut reader)?;
        let migration_refund: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            initialized,
            authority,
            pending_authority,
            wrapper_mint,
            fee_receiver,
            fee_bps,
            initial_virtual_token_reserve,
            initial_virtual_sol_reserve,
            coin_lot_size,
            pc_lot_size,
            solvent_wrapper,
            migrator,
            migration_count,
            migration_refund,
        })
    }
}
