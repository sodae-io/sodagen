use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const DCA_ACCOUNT_DISCM: [u8; 8] = [82, 93, 90, 127, 40, 101, 145, 154];
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
pub struct Dca {
    pub user: Pubkey,
    pub input_mint: Pubkey,
    pub output_mint: Pubkey,
    pub idx: u64,
    pub next_cycle_at: i64,
    pub in_deposited: u64,
    pub in_withdrawn: u64,
    pub out_withdrawn: u64,
    pub in_used: u64,
    pub out_received: u64,
    pub in_amount_per_cycle: u64,
    pub cycle_frequency: i64,
    pub next_cycle_amount_left: u64,
    pub in_account: Pubkey,
    pub out_account: Pubkey,
    pub min_out_amount: u64,
    pub max_out_amount: u64,
    pub keeper_in_balance_before_borrow: u64,
    pub dca_out_balance_before_swap: u64,
    pub created_at: i64,
    pub bump: u8,
}
impl Dca {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let input_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let output_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let idx: u64 = crate::borsh_de_or_default(&mut reader)?;
        let next_cycle_at: i64 = crate::borsh_de_or_default(&mut reader)?;
        let in_deposited: u64 = crate::borsh_de_or_default(&mut reader)?;
        let in_withdrawn: u64 = crate::borsh_de_or_default(&mut reader)?;
        let out_withdrawn: u64 = crate::borsh_de_or_default(&mut reader)?;
        let in_used: u64 = crate::borsh_de_or_default(&mut reader)?;
        let out_received: u64 = crate::borsh_de_or_default(&mut reader)?;
        let in_amount_per_cycle: u64 = crate::borsh_de_or_default(&mut reader)?;
        let cycle_frequency: i64 = crate::borsh_de_or_default(&mut reader)?;
        let next_cycle_amount_left: u64 = crate::borsh_de_or_default(&mut reader)?;
        let in_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let out_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let min_out_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_out_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let keeper_in_balance_before_borrow: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let dca_out_balance_before_swap: u64 = crate::borsh_de_or_default(&mut reader)?;
        let created_at: i64 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            user,
            input_mint,
            output_mint,
            idx,
            next_cycle_at,
            in_deposited,
            in_withdrawn,
            out_withdrawn,
            in_used,
            out_received,
            in_amount_per_cycle,
            cycle_frequency,
            next_cycle_amount_left,
            in_account,
            out_account,
            min_out_amount,
            max_out_amount,
            keeper_in_balance_before_borrow,
            dca_out_balance_before_swap,
            created_at,
            bump,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.input_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.output_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.idx, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.next_cycle_at, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.in_deposited, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.in_withdrawn, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.out_withdrawn, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.in_used, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.out_received, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.in_amount_per_cycle, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.cycle_frequency, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.next_cycle_amount_left, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.in_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.out_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.min_out_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_out_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.keeper_in_balance_before_borrow,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.dca_out_balance_before_swap,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.created_at, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct DcaAccount(pub Dca);
impl DcaAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DCA_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Dca::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DCA_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
