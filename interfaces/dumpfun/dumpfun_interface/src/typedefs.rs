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
pub struct RampingLimit {
    pub start_sec: i64,
    pub end_sec: i64,
    pub limit_bps: u16,
}
impl RampingLimit {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let start_sec: i64 = crate::borsh_de_or_default(&mut reader)?;
        let end_sec: i64 = crate::borsh_de_or_default(&mut reader)?;
        let limit_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            start_sec,
            end_sec,
            limit_bps,
        })
    }
}
