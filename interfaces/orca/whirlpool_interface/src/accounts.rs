use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const ADAPTIVE_FEE_TIER_ACCOUNT_DISCM: [u8; 8] = [
    147, 16, 144, 116, 47, 146, 149, 46,
];
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
pub struct AdaptiveFeeTier {
    pub whirlpools_config: Pubkey,
    pub fee_tier_index: u16,
    pub tick_spacing: u16,
    pub initialize_pool_authority: Pubkey,
    pub delegated_fee_authority: Pubkey,
    pub default_base_fee_rate: u16,
    pub filter_period: u16,
    pub decay_period: u16,
    pub reduction_factor: u16,
    pub adaptive_fee_control_factor: u32,
    pub max_volatility_accumulator: u32,
    pub tick_group_size: u16,
    pub major_swap_threshold_ticks: u16,
}
impl AdaptiveFeeTier {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let whirlpools_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_tier_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let tick_spacing: u16 = crate::borsh_de_or_default(&mut reader)?;
        let initialize_pool_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let delegated_fee_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let default_base_fee_rate: u16 = crate::borsh_de_or_default(&mut reader)?;
        let filter_period: u16 = crate::borsh_de_or_default(&mut reader)?;
        let decay_period: u16 = crate::borsh_de_or_default(&mut reader)?;
        let reduction_factor: u16 = crate::borsh_de_or_default(&mut reader)?;
        let adaptive_fee_control_factor: u32 = crate::borsh_de_or_default(&mut reader)?;
        let max_volatility_accumulator: u32 = crate::borsh_de_or_default(&mut reader)?;
        let tick_group_size: u16 = crate::borsh_de_or_default(&mut reader)?;
        let major_swap_threshold_ticks: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            whirlpools_config,
            fee_tier_index,
            tick_spacing,
            initialize_pool_authority,
            delegated_fee_authority,
            default_base_fee_rate,
            filter_period,
            decay_period,
            reduction_factor,
            adaptive_fee_control_factor,
            max_volatility_accumulator,
            tick_group_size,
            major_swap_threshold_ticks,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.whirlpools_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_tier_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_spacing, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.initialize_pool_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.delegated_fee_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.default_base_fee_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.filter_period, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.decay_period, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reduction_factor, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.adaptive_fee_control_factor,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.max_volatility_accumulator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_group_size, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.major_swap_threshold_ticks, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct AdaptiveFeeTierAccount(pub AdaptiveFeeTier);
impl AdaptiveFeeTierAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADAPTIVE_FEE_TIER_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(AdaptiveFeeTier::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADAPTIVE_FEE_TIER_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const DYNAMIC_TICK_ARRAY_ACCOUNT_DISCM: [u8; 8] = [
    17, 216, 246, 142, 225, 199, 218, 56,
];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct DynamicTickArray {
    pub start_tick_index: i32,
    pub whirlpool: Pubkey,
    pub tick_bitmap: u128,
    #[serde(with = "crate::big_array_serde")]
    pub ticks: [DynamicTick; 88],
}
impl DynamicTickArray {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let start_tick_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let whirlpool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let tick_bitmap: u128 = crate::borsh_de_or_default(&mut reader)?;
        let ticks = <[DynamicTick; 88] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            start_tick_index,
            whirlpool,
            tick_bitmap,
            ticks,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.start_tick_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.whirlpool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_bitmap, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.ticks, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct DynamicTickArrayAccount(pub DynamicTickArray);
impl DynamicTickArrayAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DYNAMIC_TICK_ARRAY_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(DynamicTickArray::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DYNAMIC_TICK_ARRAY_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const FEE_TIER_ACCOUNT_DISCM: [u8; 8] = [56, 75, 159, 76, 142, 68, 190, 105];
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
pub struct FeeTier {
    pub whirlpools_config: Pubkey,
    pub tick_spacing: u16,
    pub default_fee_rate: u16,
}
impl FeeTier {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let whirlpools_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let tick_spacing: u16 = crate::borsh_de_or_default(&mut reader)?;
        let default_fee_rate: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            whirlpools_config,
            tick_spacing,
            default_fee_rate,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.whirlpools_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_spacing, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.default_fee_rate, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct FeeTierAccount(pub FeeTier);
impl FeeTierAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FEE_TIER_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(FeeTier::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FEE_TIER_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LOCK_CONFIG_ACCOUNT_DISCM: [u8; 8] = [106, 47, 238, 159, 124, 12, 160, 192];
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
pub struct LockConfig {
    pub position: Pubkey,
    pub position_owner: Pubkey,
    pub whirlpool: Pubkey,
    pub locked_timestamp: u64,
    pub lock_type: LockTypeLabel,
}
impl LockConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let position: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let whirlpool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let locked_timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        let lock_type: LockTypeLabel = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            position,
            position_owner,
            whirlpool,
            locked_timestamp,
            lock_type,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.position, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.whirlpool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.locked_timestamp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lock_type, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LockConfigAccount(pub LockConfig);
impl LockConfigAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOCK_CONFIG_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(LockConfig::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOCK_CONFIG_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const ORACLE_ACCOUNT_DISCM: [u8; 8] = [139, 194, 131, 179, 140, 179, 229, 244];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct Oracle {
    pub whirlpool: Pubkey,
    pub trade_enable_timestamp: u64,
    pub adaptive_fee_constants: AdaptiveFeeConstants,
    pub adaptive_fee_variables: AdaptiveFeeVariables,
    #[serde(with = "crate::big_array_serde")]
    pub reserved: [u8; 128],
}
impl Oracle {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let whirlpool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let trade_enable_timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        let adaptive_fee_constants = if reader.is_empty() {
            Default::default()
        } else {
            <AdaptiveFeeConstants>::deserialize(&mut reader)?
        };
        let adaptive_fee_variables = if reader.is_empty() {
            Default::default()
        } else {
            <AdaptiveFeeVariables>::deserialize(&mut reader)?
        };
        let reserved = <[u8; 128] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            whirlpool,
            trade_enable_timestamp,
            adaptive_fee_constants,
            adaptive_fee_variables,
            reserved,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.whirlpool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.trade_enable_timestamp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.adaptive_fee_constants, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.adaptive_fee_variables, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserved, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct OracleAccount(pub Oracle);
impl OracleAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ORACLE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Oracle::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ORACLE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POSITION_ACCOUNT_DISCM: [u8; 8] = [170, 188, 143, 228, 122, 64, 247, 208];
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
pub struct Position {
    pub whirlpool: Pubkey,
    pub position_mint: Pubkey,
    pub liquidity: u128,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
    pub fee_growth_checkpoint_a: u128,
    pub fee_owed_a: u64,
    pub fee_growth_checkpoint_b: u128,
    pub fee_owed_b: u64,
    pub reward_infos: [PositionRewardInfo; 3],
}
impl Position {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let whirlpool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let tick_lower_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let tick_upper_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let fee_growth_checkpoint_a: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fee_owed_a: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_growth_checkpoint_b: u128 = crate::borsh_de_or_default(&mut reader)?;
        let fee_owed_b: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reward_infos: [PositionRewardInfo; 3] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            whirlpool,
            position_mint,
            liquidity,
            tick_lower_index,
            tick_upper_index,
            fee_growth_checkpoint_a,
            fee_owed_a,
            fee_growth_checkpoint_b,
            fee_owed_b,
            reward_infos,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.whirlpool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_lower_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_upper_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_growth_checkpoint_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_owed_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_growth_checkpoint_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_owed_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reward_infos, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PositionAccount(pub Position);
impl PositionAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POSITION_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Position::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POSITION_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POSITION_BUNDLE_ACCOUNT_DISCM: [u8; 8] = [129, 169, 175, 65, 185, 95, 32, 100];
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
pub struct PositionBundle {
    pub position_bundle_mint: Pubkey,
    pub position_bitmap: [u8; 32],
}
impl PositionBundle {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let position_bundle_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let position_bitmap: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            position_bundle_mint,
            position_bitmap,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.position_bundle_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.position_bitmap, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PositionBundleAccount(pub PositionBundle);
impl PositionBundleAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POSITION_BUNDLE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(PositionBundle::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POSITION_BUNDLE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const TICK_ARRAY_ACCOUNT_DISCM: [u8; 8] = [69, 97, 189, 190, 110, 7, 66, 187];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct TickArray {
    pub start_tick_index: i32,
    #[serde(with = "crate::big_array_serde")]
    pub ticks: [Tick; 88],
    pub whirlpool: Pubkey,
}
impl TickArray {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let start_tick_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let ticks = <[Tick; 88] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let whirlpool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            start_tick_index,
            ticks,
            whirlpool,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.start_tick_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.ticks, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.whirlpool, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TickArrayAccount(pub TickArray);
impl TickArrayAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TICK_ARRAY_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(TickArray::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TICK_ARRAY_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const TOKEN_BADGE_ACCOUNT_DISCM: [u8; 8] = [116, 219, 204, 229, 249, 116, 255, 150];
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
pub struct TokenBadge {
    pub whirlpools_config: Pubkey,
    pub token_mint: Pubkey,
    pub attribute_require_non_transferable_position: bool,
}
impl TokenBadge {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let whirlpools_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let attribute_require_non_transferable_position: bool = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            whirlpools_config,
            token_mint,
            attribute_require_non_transferable_position,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.whirlpools_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.attribute_require_non_transferable_position,
            &mut writer,
        )?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TokenBadgeAccount(pub TokenBadge);
impl TokenBadgeAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TOKEN_BADGE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(TokenBadge::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TOKEN_BADGE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const WHIRLPOOL_ACCOUNT_DISCM: [u8; 8] = [63, 149, 209, 12, 225, 128, 99, 9];
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
pub struct Whirlpool {
    pub whirlpools_config: Pubkey,
    pub whirlpool_bump: [u8; 1],
    pub tick_spacing: u16,
    pub fee_tier_index_seed: [u8; 2],
    pub fee_rate: u16,
    pub protocol_fee_rate: u16,
    pub liquidity: u128,
    pub sqrt_price: u128,
    pub tick_current_index: i32,
    pub protocol_fee_owed_a: u64,
    pub protocol_fee_owed_b: u64,
    pub token_mint_a: Pubkey,
    pub token_vault_a: Pubkey,
    pub fee_growth_global_a: u128,
    pub token_mint_b: Pubkey,
    pub token_vault_b: Pubkey,
    pub fee_growth_global_b: u128,
    pub reward_last_updated_timestamp: u64,
    pub reward_infos: [WhirlpoolRewardInfo; 3],
}
impl Whirlpool {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let whirlpools_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let whirlpool_bump: [u8; 1] = crate::borsh_de_or_default(&mut reader)?;
        let tick_spacing: u16 = crate::borsh_de_or_default(&mut reader)?;
        let fee_tier_index_seed: [u8; 2] = crate::borsh_de_or_default(&mut reader)?;
        let fee_rate: u16 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee_rate: u16 = crate::borsh_de_or_default(&mut reader)?;
        let liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let sqrt_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        let tick_current_index: i32 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee_owed_a: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee_owed_b: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_mint_a: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_vault_a: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_growth_global_a: u128 = crate::borsh_de_or_default(&mut reader)?;
        let token_mint_b: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_vault_b: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_growth_global_b: u128 = crate::borsh_de_or_default(&mut reader)?;
        let reward_last_updated_timestamp: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let reward_infos: [WhirlpoolRewardInfo; 3] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            whirlpools_config,
            whirlpool_bump,
            tick_spacing,
            fee_tier_index_seed,
            fee_rate,
            protocol_fee_rate,
            liquidity,
            sqrt_price,
            tick_current_index,
            protocol_fee_owed_a,
            protocol_fee_owed_b,
            token_mint_a,
            token_vault_a,
            fee_growth_global_a,
            token_mint_b,
            token_vault_b,
            fee_growth_global_b,
            reward_last_updated_timestamp,
            reward_infos,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.whirlpools_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.whirlpool_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_spacing, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_tier_index_seed, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fee_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sqrt_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.tick_current_index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fee_owed_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fee_owed_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_mint_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_vault_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_growth_global_a, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_mint_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_vault_b, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_growth_global_b, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.reward_last_updated_timestamp,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.reward_infos, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct WhirlpoolAccount(pub Whirlpool);
impl WhirlpoolAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WHIRLPOOL_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Whirlpool::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WHIRLPOOL_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const WHIRLPOOLS_CONFIG_ACCOUNT_DISCM: [u8; 8] = [
    157, 20, 49, 224, 217, 87, 193, 254,
];
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
pub struct WhirlpoolsConfig {
    pub fee_authority: Pubkey,
    pub collect_protocol_fees_authority: Pubkey,
    pub reward_emissions_super_authority: Pubkey,
    pub default_protocol_fee_rate: u16,
    pub feature_flags: u16,
}
impl WhirlpoolsConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let fee_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let collect_protocol_fees_authority: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let reward_emissions_super_authority: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let default_protocol_fee_rate: u16 = crate::borsh_de_or_default(&mut reader)?;
        let feature_flags: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            fee_authority,
            collect_protocol_fees_authority,
            reward_emissions_super_authority,
            default_protocol_fee_rate,
            feature_flags,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.fee_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.collect_protocol_fees_authority,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.reward_emissions_super_authority,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.default_protocol_fee_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.feature_flags, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct WhirlpoolsConfigAccount(pub WhirlpoolsConfig);
impl WhirlpoolsConfigAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WHIRLPOOLS_CONFIG_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(WhirlpoolsConfig::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WHIRLPOOLS_CONFIG_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const WHIRLPOOLS_CONFIG_EXTENSION_ACCOUNT_DISCM: [u8; 8] = [
    2, 99, 215, 163, 240, 26, 153, 58,
];
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
pub struct WhirlpoolsConfigExtension {
    pub whirlpools_config: Pubkey,
    pub config_extension_authority: Pubkey,
    pub token_badge_authority: Pubkey,
}
impl WhirlpoolsConfigExtension {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let whirlpools_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let config_extension_authority: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let token_badge_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            whirlpools_config,
            config_extension_authority,
            token_badge_authority,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.whirlpools_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.config_extension_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_badge_authority, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct WhirlpoolsConfigExtensionAccount(pub WhirlpoolsConfigExtension);
impl WhirlpoolsConfigExtensionAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WHIRLPOOLS_CONFIG_EXTENSION_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(WhirlpoolsConfigExtension::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WHIRLPOOLS_CONFIG_EXTENSION_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
