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
pub enum PoolState {
    #[default]
    Initialized,
    Active,
    Graduated,
    Migrated,
}
impl TryFrom<u8> for PoolState {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Initialized),
            1u8 => Ok(Self::Active),
            2u8 => Ok(Self::Graduated),
            3u8 => Ok(Self::Migrated),
            _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
        }
    }
}
