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
pub struct ProgramState {
    pub manager: Pubkey,
    pub lp_withdrawal_fee_bps: u16,
}
impl ProgramState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let manager: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lp_withdrawal_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            manager,
            lp_withdrawal_fee_bps,
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
pub struct FeeAccount {
    pub bump: u8,
    pub padding: u8,
    pub input_fee_bps: i16,
    pub output_fee_bps: i16,
}
impl FeeAccount {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding: u8 = crate::borsh_de_or_default(&mut reader)?;
        let input_fee_bps: i16 = crate::borsh_de_or_default(&mut reader)?;
        let output_fee_bps: i16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            bump,
            padding,
            input_fee_bps,
            output_fee_bps,
        })
    }
}
