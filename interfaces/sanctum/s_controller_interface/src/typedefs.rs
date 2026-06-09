use borsh::{BorshDeserialize, BorshSerialize};
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
pub struct PoolState {
    pub total_sol_value: u64,
    pub trading_protocol_fee_bps: u16,
    pub lp_protocol_fee_bps: u16,
    pub version: u8,
    pub is_disabled: u8,
    pub is_rebalancing: u8,
    pub padding: [u8; 1],
    pub admin: Pubkey,
    pub rebalance_authority: Pubkey,
    pub protocol_fee_beneficiary: Pubkey,
    pub pricing_program: Pubkey,
    pub lp_token_mint: Pubkey,
}
impl PoolState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let total_sol_value: u64 = crate::borsh_de_or_default(&mut reader)?;
        let trading_protocol_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let lp_protocol_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let version: u8 = crate::borsh_de_or_default(&mut reader)?;
        let is_disabled: u8 = crate::borsh_de_or_default(&mut reader)?;
        let is_rebalancing: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 1] = crate::borsh_de_or_default(&mut reader)?;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let rebalance_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee_beneficiary: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pricing_program: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lp_token_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            total_sol_value,
            trading_protocol_fee_bps,
            lp_protocol_fee_bps,
            version,
            is_disabled,
            is_rebalancing,
            padding,
            admin,
            rebalance_authority,
            protocol_fee_beneficiary,
            pricing_program,
            lp_token_mint,
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
pub struct LstState {
    pub is_input_disabled: u8,
    pub pool_reserves_bump: u8,
    pub protocol_fee_accumulator_bump: u8,
    pub padding: [u8; 5],
    pub sol_value: u64,
    pub mint: Pubkey,
    pub sol_value_calculator: Pubkey,
}
impl LstState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let is_input_disabled: u8 = crate::borsh_de_or_default(&mut reader)?;
        let pool_reserves_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee_accumulator_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 5] = crate::borsh_de_or_default(&mut reader)?;
        let sol_value: u64 = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let sol_value_calculator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            is_input_disabled,
            pool_reserves_bump,
            protocol_fee_accumulator_bump,
            padding,
            sol_value,
            mint,
            sol_value_calculator,
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
pub struct RebalanceRecord {
    pub old_total_sol_value: u64,
    pub padding: [u8; 4],
    pub dst_lst_index: u32,
}
impl RebalanceRecord {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let old_total_sol_value: u64 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 4] = crate::borsh_de_or_default(&mut reader)?;
        let dst_lst_index: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            old_total_sol_value,
            padding,
            dst_lst_index,
        })
    }
}
