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
pub enum CreatorFeeOn {
    #[default]
    BothToken,
    OnlyToken0,
    OnlyToken1,
}
impl TryFrom<u8> for CreatorFeeOn {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::BothToken),
            1u8 => Ok(Self::OnlyToken0),
            2u8 => Ok(Self::OnlyToken1),
            _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
        }
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
pub struct Observation {
    pub block_timestamp: u64,
    pub cumulative_token_0_price_x32: u128,
    pub cumulative_token_1_price_x32: u128,
}
impl Observation {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let block_timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        let cumulative_token_0_price_x32: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let cumulative_token_1_price_x32: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            block_timestamp,
            cumulative_token_0_price_x32,
            cumulative_token_1_price_x32,
        })
    }
}
