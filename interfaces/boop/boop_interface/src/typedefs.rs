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
pub enum BondingCurveStatus {
    #[default]
    Trading,
    Graduated,
    PoolPriceCorrected,
    LiquidityProvisioned,
    LiquidityLocked,
}
impl TryFrom<u8> for BondingCurveStatus {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Trading),
            1u8 => Ok(Self::Graduated),
            2u8 => Ok(Self::PoolPriceCorrected),
            3u8 => Ok(Self::LiquidityProvisioned),
            4u8 => Ok(Self::LiquidityLocked),
            _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
        }
    }
}
