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
pub enum CurveType {
    #[default]
    ConstantProduct,
    ConstantPrice,
    Offset,
}
impl TryFrom<u8> for CurveType {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::ConstantProduct),
            1u8 => Ok(Self::ConstantPrice),
            2u8 => Ok(Self::Offset),
            _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
        }
    }
}
