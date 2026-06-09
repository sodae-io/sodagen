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
