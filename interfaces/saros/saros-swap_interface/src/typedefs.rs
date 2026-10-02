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
pub struct Fees {
    pub trade_fee_numerator: u64,
    pub trade_fee_denominator: u64,
    pub owner_trade_fee_numerator: u64,
    pub owner_trade_fee_denominator: u64,
    pub owner_withdraw_fee_numerator: u64,
    pub owner_withdraw_fee_denominator: u64,
    pub host_fee_numerator: u64,
    pub host_fee_denominator: u64,
}
impl Fees {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let trade_fee_numerator: u64 = crate::borsh_de_or_default(&mut reader)?;
        let trade_fee_denominator: u64 = crate::borsh_de_or_default(&mut reader)?;
        let owner_trade_fee_numerator: u64 = crate::borsh_de_or_default(&mut reader)?;
        let owner_trade_fee_denominator: u64 = crate::borsh_de_or_default(&mut reader)?;
        let owner_withdraw_fee_numerator: u64 = crate::borsh_de_or_default(&mut reader)?;
        let owner_withdraw_fee_denominator: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let host_fee_numerator: u64 = crate::borsh_de_or_default(&mut reader)?;
        let host_fee_denominator: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            trade_fee_numerator,
            trade_fee_denominator,
            owner_trade_fee_numerator,
            owner_trade_fee_denominator,
            owner_withdraw_fee_numerator,
            owner_withdraw_fee_denominator,
            host_fee_numerator,
            host_fee_denominator,
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
pub enum SwapCurve {
    #[default]
    ConstantProduct,
    ConstantPrice,
    Stable,
    Offset,
}
impl TryFrom<u8> for SwapCurve {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::ConstantProduct),
            1u8 => Ok(Self::ConstantPrice),
            2u8 => Ok(Self::Stable),
            3u8 => Ok(Self::Offset),
            _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
        }
    }
}
