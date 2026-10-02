use borsh::{BorshDeserialize, BorshSerialize};
#[allow(unused_imports)]
use crate::*;
use solana_pubkey::Pubkey;
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
pub enum FarmConfigOption {
    #[default]
    UpdateRewardRps,
    UpdateRewardMinClaimDuration,
    WithdrawAuthority,
    DepositWarmupPeriod,
    WithdrawCooldownPeriod,
    RewardType,
    RpsDecimals,
    LockingMode,
    LockingStartTimestamp,
    LockingDuration,
    LockingEarlyWithdrawalPenaltyBps,
    DepositCapAmount,
    SlashedAmountSpillAddress,
    ScopePricesAccount,
    ScopeOraclePriceId,
    ScopeOracleMaxAge,
    UpdateRewardScheduleCurvePoints,
    UpdatePendingFarmAdmin,
    UpdateStrategyId,
    UpdateDelegatedRpsAdmin,
    UpdateVaultId,
    UpdateExtraDelegatedAuthority,
    UpdateIsRewardUserOnceEnabled,
    UpdateDelegatedAuthority,
    UpdateIsHarvestingPermissionless,
}
impl TryFrom<u8> for FarmConfigOption {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::UpdateRewardRps),
            1u8 => Ok(Self::UpdateRewardMinClaimDuration),
            2u8 => Ok(Self::WithdrawAuthority),
            3u8 => Ok(Self::DepositWarmupPeriod),
            4u8 => Ok(Self::WithdrawCooldownPeriod),
            5u8 => Ok(Self::RewardType),
            6u8 => Ok(Self::RpsDecimals),
            7u8 => Ok(Self::LockingMode),
            8u8 => Ok(Self::LockingStartTimestamp),
            9u8 => Ok(Self::LockingDuration),
            10u8 => Ok(Self::LockingEarlyWithdrawalPenaltyBps),
            11u8 => Ok(Self::DepositCapAmount),
            12u8 => Ok(Self::SlashedAmountSpillAddress),
            13u8 => Ok(Self::ScopePricesAccount),
            14u8 => Ok(Self::ScopeOraclePriceId),
            15u8 => Ok(Self::ScopeOracleMaxAge),
            16u8 => Ok(Self::UpdateRewardScheduleCurvePoints),
            17u8 => Ok(Self::UpdatePendingFarmAdmin),
            18u8 => Ok(Self::UpdateStrategyId),
            19u8 => Ok(Self::UpdateDelegatedRpsAdmin),
            20u8 => Ok(Self::UpdateVaultId),
            21u8 => Ok(Self::UpdateExtraDelegatedAuthority),
            22u8 => Ok(Self::UpdateIsRewardUserOnceEnabled),
            23u8 => Ok(Self::UpdateDelegatedAuthority),
            24u8 => Ok(Self::UpdateIsHarvestingPermissionless),
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
pub enum GlobalConfigOption {
    #[default]
    SetPendingGlobalAdmin,
    SetTreasuryFeeBps,
}
impl TryFrom<u8> for GlobalConfigOption {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::SetPendingGlobalAdmin),
            1u8 => Ok(Self::SetTreasuryFeeBps),
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
pub enum LockingMode {
    #[default]
    None,
    Continuous,
    WithExpiry,
}
impl TryFrom<u8> for LockingMode {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::None),
            1u8 => Ok(Self::Continuous),
            2u8 => Ok(Self::WithExpiry),
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
pub struct RewardInfo {
    pub token: TokenInfo,
    pub rewards_vault: Pubkey,
    pub rewards_available: u64,
    pub reward_schedule_curve: RewardScheduleCurve,
    pub min_claim_duration_seconds: u64,
    pub last_issuance_ts: u64,
    pub rewards_issued_unclaimed: u64,
    pub rewards_issued_cumulative: u64,
    pub reward_per_share_scaled: u128,
    pub placeholder0: u64,
    pub reward_type: u8,
    pub rewards_per_second_decimals: u8,
    pub padding0: [u8; 6],
    pub padding1: [u64; 20],
}
impl RewardInfo {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let token = if reader.is_empty() {
            Default::default()
        } else {
            <TokenInfo>::deserialize(&mut reader)?
        };
        let rewards_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let rewards_available: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reward_schedule_curve = if reader.is_empty() {
            Default::default()
        } else {
            <RewardScheduleCurve>::deserialize(&mut reader)?
        };
        let min_claim_duration_seconds: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_issuance_ts: u64 = crate::borsh_de_or_default(&mut reader)?;
        let rewards_issued_unclaimed: u64 = crate::borsh_de_or_default(&mut reader)?;
        let rewards_issued_cumulative: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reward_per_share_scaled: u128 = crate::borsh_de_or_default(&mut reader)?;
        let placeholder0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reward_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let rewards_per_second_decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding0: [u8; 6] = crate::borsh_de_or_default(&mut reader)?;
        let padding1: [u64; 20] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            token,
            rewards_vault,
            rewards_available,
            reward_schedule_curve,
            min_claim_duration_seconds,
            last_issuance_ts,
            rewards_issued_unclaimed,
            rewards_issued_cumulative,
            reward_per_share_scaled,
            placeholder0,
            reward_type,
            rewards_per_second_decimals,
            padding0,
            padding1,
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
pub struct RewardPerTimeUnitPoint {
    pub ts_start: u64,
    pub reward_per_time_unit: u64,
}
impl RewardPerTimeUnitPoint {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let ts_start: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reward_per_time_unit: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            ts_start,
            reward_per_time_unit,
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
pub struct RewardScheduleCurve {
    pub points: [RewardPerTimeUnitPoint; 20],
}
impl RewardScheduleCurve {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let points: [RewardPerTimeUnitPoint; 20] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self { points })
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
pub enum RewardType {
    #[default]
    Proportional,
    Constant,
}
impl TryFrom<u8> for RewardType {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Proportional),
            1u8 => Ok(Self::Constant),
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
pub enum TimeUnit {
    #[default]
    Seconds,
    Slots,
}
impl TryFrom<u8> for TimeUnit {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Seconds),
            1u8 => Ok(Self::Slots),
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
pub struct TokenInfo {
    pub mint: Pubkey,
    pub decimals: u64,
    pub token_program: Pubkey,
    pub padding: [u64; 6],
}
impl TokenInfo {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let decimals: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_program: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u64; 6] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            mint,
            decimals,
            token_program,
            padding,
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
pub struct DatedPrice {
    pub price: Price,
    pub last_updated_slot: u64,
    pub unix_timestamp: u64,
    pub reserved: [u64; 2],
    pub reserved2: [u16; 3],
    pub index: u16,
}
impl DatedPrice {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let price = if reader.is_empty() {
            Default::default()
        } else {
            <Price>::deserialize(&mut reader)?
        };
        let last_updated_slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        let unix_timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reserved: [u64; 2] = crate::borsh_de_or_default(&mut reader)?;
        let reserved2: [u16; 3] = crate::borsh_de_or_default(&mut reader)?;
        let index: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            price,
            last_updated_slot,
            unix_timestamp,
            reserved,
            reserved2,
            index,
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
pub struct Price {
    pub value: u64,
    pub exp: u64,
}
impl Price {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let value: u64 = crate::borsh_de_or_default(&mut reader)?;
        let exp: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { value, exp })
    }
}
