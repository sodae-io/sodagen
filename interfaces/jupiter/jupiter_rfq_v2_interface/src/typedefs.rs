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
pub struct FillExactInParams {
    pub expire_at: u64,
    pub tick_size_qpb: u64,
    pub lot_size_base: u64,
    pub levels: Vec<Level>,
}
impl FillExactInParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let expire_at: u64 = crate::borsh_de_or_default(&mut reader)?;
        let tick_size_qpb: u64 = crate::borsh_de_or_default(&mut reader)?;
        let lot_size_base: u64 = crate::borsh_de_or_default(&mut reader)?;
        let levels: Vec<Level> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            expire_at,
            tick_size_qpb,
            lot_size_base,
            levels,
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
pub struct Level {
    pub px_ticks: u64,
    pub qty_lots: u64,
}
impl Level {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let px_ticks: u64 = crate::borsh_de_or_default(&mut reader)?;
        let qty_lots: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { px_ticks, qty_lots })
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
pub enum Side {
    #[default]
    Bid,
    Ask,
}
