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
pub struct TokenMultiplier {
    pub token_a_multiplier: u64,
    pub token_b_multiplier: u64,
    pub precision_factor: u8,
}
impl TokenMultiplier {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let token_a_multiplier: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_b_multiplier: u64 = crate::borsh_de_or_default(&mut reader)?;
        let precision_factor: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            token_a_multiplier,
            token_b_multiplier,
            precision_factor,
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
pub struct PoolFees {
    pub trade_fee_numerator: u64,
    pub trade_fee_denominator: u64,
    pub protocol_trade_fee_numerator: u64,
    pub protocol_trade_fee_denominator: u64,
}
impl PoolFees {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let trade_fee_numerator: u64 = crate::borsh_de_or_default(&mut reader)?;
        let trade_fee_denominator: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_trade_fee_numerator: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_trade_fee_denominator: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            trade_fee_numerator,
            trade_fee_denominator,
            protocol_trade_fee_numerator,
            protocol_trade_fee_denominator,
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
pub struct Depeg {
    pub base_virtual_price: u64,
    pub base_cache_updated: u64,
    pub depeg_type: DepegType,
}
impl Depeg {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let base_virtual_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let base_cache_updated: u64 = crate::borsh_de_or_default(&mut reader)?;
        let depeg_type: DepegType = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            base_virtual_price,
            base_cache_updated,
            depeg_type,
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
pub struct ConfigParameters {
    pub trade_fee_numerator: u64,
    pub protocol_trade_fee_numerator: u64,
    pub activation_duration: u64,
    pub vault_config_key: Pubkey,
    pub pool_creator_authority: Pubkey,
    pub activation_type: u8,
    pub index: u64,
    pub partner_fee_numerator: u64,
    pub fee_curve: FeeCurveInfoFromDuration,
}
impl ConfigParameters {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let trade_fee_numerator: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_trade_fee_numerator: u64 = crate::borsh_de_or_default(&mut reader)?;
        let activation_duration: u64 = crate::borsh_de_or_default(&mut reader)?;
        let vault_config_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_creator_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let activation_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let index: u64 = crate::borsh_de_or_default(&mut reader)?;
        let partner_fee_numerator: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_curve = if reader.is_empty() {
            Default::default()
        } else {
            <FeeCurveInfoFromDuration>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            trade_fee_numerator,
            protocol_trade_fee_numerator,
            activation_duration,
            vault_config_key,
            pool_creator_authority,
            activation_type,
            index,
            partner_fee_numerator,
            fee_curve,
        })
    }
}
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct CustomizableParams {
    pub trade_fee_numerator: u32,
    pub activation_point: Option<u64>,
    pub has_alpha_vault: bool,
    pub activation_type: u8,
    pub fee_curve: FeeCurveInfoFromDuration,
    #[serde(with = "crate::big_array_serde")]
    pub padding: [u8; 53],
}
impl CustomizableParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let trade_fee_numerator: u32 = crate::borsh_de_or_default(&mut reader)?;
        let activation_point: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let has_alpha_vault: bool = crate::borsh_de_or_default(&mut reader)?;
        let activation_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let fee_curve = if reader.is_empty() {
            Default::default()
        } else {
            <FeeCurveInfoFromDuration>::deserialize(&mut reader)?
        };
        let padding = <[u8; 53] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            trade_fee_numerator,
            activation_point,
            has_alpha_vault,
            activation_type,
            fee_curve,
            padding,
        })
    }
}
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct UpdateConfigParams {
    pub trade_fee_numerator: u64,
    pub fee_curve: FeeCurveInfoFromDuration,
    #[serde(with = "crate::big_array_serde")]
    pub padding: [u8; 50],
}
impl UpdateConfigParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let trade_fee_numerator: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_curve = if reader.is_empty() {
            Default::default()
        } else {
            <FeeCurveInfoFromDuration>::deserialize(&mut reader)?
        };
        let padding = <[u8; 50] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            trade_fee_numerator,
            fee_curve,
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
pub struct FeeCurveInfoFromDuration {
    pub fee_curve_type: FeeCurveType,
    pub points: [FeeBpsFromDuration; 6],
}
impl FeeCurveInfoFromDuration {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let fee_curve_type: FeeCurveType = crate::borsh_de_or_default(&mut reader)?;
        let points: [FeeBpsFromDuration; 6] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { fee_curve_type, points })
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
pub struct FeeBpsFromDuration {
    pub fee_bps: u16,
    pub activated_duration: u32,
}
impl FeeBpsFromDuration {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let activated_duration: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            fee_bps,
            activated_duration,
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
pub struct Padding {
    pub padding0: [u8; 6],
    pub padding1: [u64; 12],
    pub padding2: [u64; 21],
}
impl Padding {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let padding0: [u8; 6] = crate::borsh_de_or_default(&mut reader)?;
        let padding1: [u64; 12] = crate::borsh_de_or_default(&mut reader)?;
        let padding2: [u64; 21] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            padding0,
            padding1,
            padding2,
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
pub struct PartnerInfo {
    pub fee_numerator: u64,
    pub partner_authority: Pubkey,
    pub pending_fee_a: u64,
    pub pending_fee_b: u64,
}
impl PartnerInfo {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let fee_numerator: u64 = crate::borsh_de_or_default(&mut reader)?;
        let partner_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pending_fee_a: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pending_fee_b: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            fee_numerator,
            partner_authority,
            pending_fee_a,
            pending_fee_b,
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
pub struct FeeCurveInfo {
    pub fee_curve_type: FeeCurveType,
    pub points: [FeeBpsFromActivatedPoint; 7],
}
impl FeeCurveInfo {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let fee_curve_type: FeeCurveType = crate::borsh_de_or_default(&mut reader)?;
        let points: [FeeBpsFromActivatedPoint; 7] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self { fee_curve_type, points })
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
pub struct FeeBpsFromActivatedPoint {
    pub fee_bps: u16,
    pub activated_point: u64,
}
impl FeeBpsFromActivatedPoint {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let activated_point: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { fee_bps, activated_point })
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
pub struct Bootstrapping {
    pub activation_point: u64,
    pub whitelisted_vault: Pubkey,
    pub pool_creator: Pubkey,
    pub activation_type: u8,
}
impl Bootstrapping {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let activation_point: u64 = crate::borsh_de_or_default(&mut reader)?;
        let whitelisted_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let activation_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            activation_point,
            whitelisted_vault,
            pool_creator,
            activation_type,
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
pub enum ActivationType {
    #[default]
    Slot,
    Timestamp,
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
pub enum RoundDirection {
    #[default]
    Floor,
    Ceiling,
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
pub enum TradeDirection {
    #[default]
    AtoB,
    BtoA,
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
pub enum NewCurveType {
    #[default]
    ConstantProduct,
    Stable {
        amp: u64,
        token_multiplier: TokenMultiplier,
        depeg: Depeg,
        last_amp_updated_timestamp: u64,
    },
    NewCurve { field_one: u64, field_two: u64 },
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
pub enum CurveType {
    #[default]
    ConstantProduct,
    Stable {
        amp: u64,
        token_multiplier: TokenMultiplier,
        depeg: Depeg,
        last_amp_updated_timestamp: u64,
    },
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
pub enum DepegType {
    #[default]
    None,
    Marinade,
    Lido,
    SplStake,
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
pub enum Rounding {
    #[default]
    Up,
    Down,
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
pub enum PoolType {
    #[default]
    Permissioned,
    Permissionless,
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
pub enum FeeCurveType {
    #[default]
    None,
    Flat,
    Linear,
}
