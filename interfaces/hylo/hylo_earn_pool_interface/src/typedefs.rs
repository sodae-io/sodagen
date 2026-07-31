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
pub struct DepositLimiter {
    pub limit: UFixValue64,
}
impl DepositLimiter {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let limit = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self { limit })
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
pub struct FeePair {
    pub mint: UFixValue64,
    pub redeem: UFixValue64,
}
impl FeePair {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mint = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let redeem = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self { mint, redeem })
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
pub struct HarvestCache {
    pub epoch: u64,
    pub stability_pool_cap: UFixValue64,
    pub stablecoin_to_pool: UFixValue64,
}
impl HarvestCache {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let epoch: u64 = crate::borsh_de_or_default(&mut reader)?;
        let stability_pool_cap = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let stablecoin_to_pool = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            epoch,
            stability_pool_cap,
            stablecoin_to_pool,
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
pub struct LevercoinFees {
    pub normal: FeePair,
    pub sell_zone_1: FeePair,
    pub sell_zone_2: FeePair,
}
impl LevercoinFees {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let normal = if reader.is_empty() {
            Default::default()
        } else {
            <FeePair>::deserialize(&mut reader)?
        };
        let sell_zone_1 = if reader.is_empty() {
            Default::default()
        } else {
            <FeePair>::deserialize(&mut reader)?
        };
        let sell_zone_2 = if reader.is_empty() {
            Default::default()
        } else {
            <FeePair>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            normal,
            sell_zone_1,
            sell_zone_2,
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
pub struct PoolDrawdown {
    pub ledger: VirtualStablecoin,
}
impl PoolDrawdown {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let ledger = if reader.is_empty() {
            Default::default()
        } else {
            <VirtualStablecoin>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self { ledger })
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
pub struct RebalanceCurveConfig {
    pub floor_pct: UFixValue64,
    pub ceil_pct: UFixValue64,
}
impl RebalanceCurveConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let floor_pct = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let ceil_pct = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self { floor_pct, ceil_pct })
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
pub struct SlippageConfig {
    pub expected_token_out: UFixValue64,
    pub slippage_tolerance: UFixValue64,
}
impl SlippageConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let expected_token_out = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let slippage_tolerance = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            expected_token_out,
            slippage_tolerance,
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
pub struct StablecoinFees {
    pub normal: FeePair,
    pub mode_1: FeePair,
}
impl StablecoinFees {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let normal = if reader.is_empty() {
            Default::default()
        } else {
            <FeePair>::deserialize(&mut reader)?
        };
        let mode_1 = if reader.is_empty() {
            Default::default()
        } else {
            <FeePair>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self { normal, mode_1 })
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
pub struct TokenMetadata {
    pub symbol: String,
    pub uri: String,
}
impl TokenMetadata {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let symbol: String = crate::borsh_de_or_default(&mut reader)?;
        let uri: String = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { symbol, uri })
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
pub struct TotalSolCache {
    pub current_update_epoch: u64,
    pub total_sol: UFixValue64,
}
impl TotalSolCache {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let current_update_epoch: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_sol = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            current_update_epoch,
            total_sol,
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
pub struct UFixValue64 {
    pub bits: u64,
    pub exp: i8,
}
impl UFixValue64 {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bits: u64 = crate::borsh_de_or_default(&mut reader)?;
        let exp: i8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { bits, exp })
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
pub struct VirtualStablecoin {
    pub supply: UFixValue64,
}
impl VirtualStablecoin {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let supply = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self { supply })
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
pub struct WithdrawalLimiter {
    pub limit: UFixValue64,
    pub withdrawal_ledger: VirtualStablecoin,
    pub epoch: u64,
}
impl WithdrawalLimiter {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let limit = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let withdrawal_ledger = if reader.is_empty() {
            Default::default()
        } else {
            <VirtualStablecoin>::deserialize(&mut reader)?
        };
        let epoch: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            limit,
            withdrawal_ledger,
            epoch,
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
pub struct YieldHarvestConfig {
    pub allocation: UFixValue64,
    pub fee: UFixValue64,
}
impl YieldHarvestConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let allocation = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let fee = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self { allocation, fee })
    }
}
