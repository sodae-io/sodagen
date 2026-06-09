use borsh::{BorshDeserialize, BorshSerialize};
#[allow(unused_imports)]
use crate::*;
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
pub struct WithdrawParams {
    pub withdraw_amount: u64,
    pub withdrawal: Withdrawal,
}
impl WithdrawParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let withdraw_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let withdrawal: Withdrawal = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            withdraw_amount,
            withdrawal,
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
pub enum Withdrawal {
    #[default]
    In,
    Out,
}
