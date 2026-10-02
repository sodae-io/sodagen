use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const CONFIG_ACCOUNT_DISCM: [u8; 8] = [155, 12, 170, 224, 30, 250, 204, 130];
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
pub struct Config {
    pub vault_config_key: Pubkey,
    pub pool_creator_authority: Pubkey,
    pub pool_fees: PoolFeesConfig,
    pub activation_type: u8,
    pub collect_fee_mode: u8,
    pub config_type: u8,
    pub padding_0: [u8; 5],
    pub index: u64,
    pub sqrt_min_price: u128,
    pub sqrt_max_price: u128,
    pub permission: u128,
    pub padding_1: [u64; 8],
}
impl Config {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let vault_config_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_creator_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_fees = if reader.is_empty() {
            Default::default()
        } else {
            <PoolFeesConfig>::deserialize(&mut reader)?
        };
        let activation_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let collect_fee_mode: u8 = crate::borsh_de_or_default(&mut reader)?;
        let config_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding_0: [u8; 5] = crate::borsh_de_or_default(&mut reader)?;
        let index: u64 = crate::borsh_de_or_default(&mut reader)?;
        let sqrt_min_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        let sqrt_max_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        let permission: u128 = crate::borsh_de_or_default(&mut reader)?;
        let padding_1: [u64; 8] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            vault_config_key,
            pool_creator_authority,
            pool_fees,
            activation_type,
            collect_fee_mode,
            config_type,
            padding_0,
            index,
            sqrt_min_price,
            sqrt_max_price,
            permission,
            padding_1,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.vault_config_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_creator_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_fees, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.activation_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collect_fee_mode, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.config_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sqrt_min_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sqrt_max_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.permission, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding_1, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConfigAccount(pub Config);
impl ConfigAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CONFIG_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Config::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CONFIG_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const OPERATOR_ACCOUNT_DISCM: [u8; 8] = [219, 31, 188, 145, 69, 139, 204, 117];
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
pub struct Operator {
    pub whitelisted_address: Pubkey,
    pub permission: u128,
    pub padding: [u64; 2],
}
impl Operator {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let whitelisted_address: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let permission: u128 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u64; 2] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            whitelisted_address,
            permission,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.whitelisted_address, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.permission, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct OperatorAccount(pub Operator);
impl OperatorAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != OPERATOR_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Operator::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&OPERATOR_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POD_ALIGNED_FEE_MARKET_CAP_SCHEDULER_ACCOUNT_DISCM: [u8; 8] = [
    251, 130, 208, 253, 245, 27, 145, 203,
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
pub struct PodAlignedFeeMarketCapScheduler {
    pub cliff_fee_numerator: u64,
    pub base_fee_mode: u8,
    pub padding: [u8; 5],
    pub number_of_period: u16,
    pub sqrt_price_step_bps: u32,
    pub scheduler_expiration_duration: u32,
    pub reduction_factor: u64,
}
impl PodAlignedFeeMarketCapScheduler {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let cliff_fee_numerator: u64 = crate::borsh_de_or_default(&mut reader)?;
        let base_fee_mode: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 5] = crate::borsh_de_or_default(&mut reader)?;
        let number_of_period: u16 = crate::borsh_de_or_default(&mut reader)?;
        let sqrt_price_step_bps: u32 = crate::borsh_de_or_default(&mut reader)?;
        let scheduler_expiration_duration: u32 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let reduction_factor: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            cliff_fee_numerator,
            base_fee_mode,
            padding,
            number_of_period,
            sqrt_price_step_bps,
            scheduler_expiration_duration,
            reduction_factor,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.cliff_fee_numerator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_fee_mode, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.number_of_period, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sqrt_price_step_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.scheduler_expiration_duration,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.reduction_factor, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PodAlignedFeeMarketCapSchedulerAccount(pub PodAlignedFeeMarketCapScheduler);
impl PodAlignedFeeMarketCapSchedulerAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POD_ALIGNED_FEE_MARKET_CAP_SCHEDULER_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(PodAlignedFeeMarketCapScheduler::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POD_ALIGNED_FEE_MARKET_CAP_SCHEDULER_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POD_ALIGNED_FEE_RATE_LIMITER_ACCOUNT_DISCM: [u8; 8] = [
    160, 219, 8, 251, 179, 7, 16, 117,
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
pub struct PodAlignedFeeRateLimiter {
    pub cliff_fee_numerator: u64,
    pub base_fee_mode: u8,
    pub padding: [u8; 5],
    pub fee_increment_bps: u16,
    pub max_limiter_duration: u32,
    pub max_fee_bps: u32,
    pub reference_amount: u64,
}
impl PodAlignedFeeRateLimiter {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let cliff_fee_numerator: u64 = crate::borsh_de_or_default(&mut reader)?;
        let base_fee_mode: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 5] = crate::borsh_de_or_default(&mut reader)?;
        let fee_increment_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let max_limiter_duration: u32 = crate::borsh_de_or_default(&mut reader)?;
        let max_fee_bps: u32 = crate::borsh_de_or_default(&mut reader)?;
        let reference_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            cliff_fee_numerator,
            base_fee_mode,
            padding,
            fee_increment_bps,
            max_limiter_duration,
            max_fee_bps,
            reference_amount,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.cliff_fee_numerator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_fee_mode, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_increment_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_limiter_duration, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_fee_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reference_amount, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PodAlignedFeeRateLimiterAccount(pub PodAlignedFeeRateLimiter);
impl PodAlignedFeeRateLimiterAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POD_ALIGNED_FEE_RATE_LIMITER_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(PodAlignedFeeRateLimiter::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POD_ALIGNED_FEE_RATE_LIMITER_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POD_ALIGNED_FEE_TIME_SCHEDULER_ACCOUNT_DISCM: [u8; 8] = [
    239, 132, 138, 213, 67, 154, 130, 70,
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
pub struct PodAlignedFeeTimeScheduler {
    pub cliff_fee_numerator: u64,
    pub base_fee_mode: u8,
    pub padding: [u8; 5],
    pub number_of_period: u16,
    pub period_frequency: u64,
    pub reduction_factor: u64,
}
impl PodAlignedFeeTimeScheduler {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let cliff_fee_numerator: u64 = crate::borsh_de_or_default(&mut reader)?;
        let base_fee_mode: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 5] = crate::borsh_de_or_default(&mut reader)?;
        let number_of_period: u16 = crate::borsh_de_or_default(&mut reader)?;
        let period_frequency: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reduction_factor: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            cliff_fee_numerator,
            base_fee_mode,
            padding,
            number_of_period,
            period_frequency,
            reduction_factor,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.cliff_fee_numerator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_fee_mode, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.number_of_period, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.period_frequency, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reduction_factor, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PodAlignedFeeTimeSchedulerAccount(pub PodAlignedFeeTimeScheduler);
impl PodAlignedFeeTimeSchedulerAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POD_ALIGNED_FEE_TIME_SCHEDULER_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(PodAlignedFeeTimeScheduler::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POD_ALIGNED_FEE_TIME_SCHEDULER_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POOL_ACCOUNT_DISCM: [u8; 8] = [241, 154, 109, 4, 17, 177, 109, 188];
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
pub struct Pool {
    pub pool_fees: PoolFeesStruct,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub whitelisted_vault: Pubkey,
    pub padding_0: [u8; 32],
    pub liquidity: u128,
    pub padding_1: u128,
    pub protocol_a_fee: u64,
    pub protocol_b_fee: u64,
    pub dead_liquidity_fee_checkpoint: u64,
    pub padding_2: [u8; 8],
    pub sqrt_min_price: u128,
    pub sqrt_max_price: u128,
    pub sqrt_price: u128,
    pub activation_point: u64,
    pub activation_type: u8,
    pub pool_status: u8,
    pub token_a_flag: u8,
    pub token_b_flag: u8,
    pub collect_fee_mode: u8,
    pub pool_type: u8,
    pub fee_version: u8,
    pub padding_3: u8,
    pub fee_a_per_liquidity: [u8; 32],
    pub fee_b_per_liquidity: [u8; 32],
    pub permanent_lock_liquidity: u128,
    pub metrics: PoolMetrics,
    pub creator: Pubkey,
    pub token_a_amount: u64,
    pub token_b_amount: u64,
    pub layout_version: u8,
    pub padding_4: [u8; 7],
    pub padding_5: [u64; 3],
    pub reward_infos: [RewardInfo; 2],
}
impl Pool {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_fees = if reader.is_empty() {
            Default::default()
        } else {
            <PoolFeesStruct>::deserialize(&mut reader)?
        };
        let token_a_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_b_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_a_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_b_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let whitelisted_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let padding_0: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        let liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let padding_1: u128 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_a_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_b_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let dead_liquidity_fee_checkpoint: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let padding_2: [u8; 8] = crate::borsh_de_or_default(&mut reader)?;
        let sqrt_min_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        let sqrt_max_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        let sqrt_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        let activation_point: u64 = crate::borsh_de_or_default(&mut reader)?;
        let activation_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let pool_status: u8 = crate::borsh_de_or_default(&mut reader)?;
        let token_a_flag: u8 = crate::borsh_de_or_default(&mut reader)?;
        let token_b_flag: u8 = crate::borsh_de_or_default(&mut reader)?;
        let collect_fee_mode: u8 = crate::borsh_de_or_default(&mut reader)?;
        let pool_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let fee_version: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding_3: u8 = crate::borsh_de_or_default(&mut reader)?;
        let fee_a_per_liquidity: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        let fee_b_per_liquidity: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        let permanent_lock_liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let metrics = if reader.is_empty() {
            Default::default()
        } else {
            <PoolMetrics>::deserialize(&mut reader)?
        };
        let creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_a_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_b_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let layout_version: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding_4: [u8; 7] = crate::borsh_de_or_default(&mut reader)?;
        let padding_5: [u64; 3] = crate::borsh_de_or_default(&mut reader)?;
        let reward_infos: [RewardInfo; 2] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool_fees,
            token_a_mint,
            token_b_mint,
            token_a_vault,
            token_b_vault,
            whitelisted_vault,
            padding_0,
            liquidity,
            padding_1,
            protocol_a_fee,
            protocol_b_fee,
            dead_liquidity_fee_checkpoint,
            padding_2,
            sqrt_min_price,
            sqrt_max_price,
            sqrt_price,
            activation_point,
            activation_type,
            pool_status,
            token_a_flag,
            token_b_flag,
            collect_fee_mode,
            pool_type,
            fee_version,
            padding_3,
            fee_a_per_liquidity,
            fee_b_per_liquidity,
            permanent_lock_liquidity,
            metrics,
            creator,
            token_a_amount,
            token_b_amount,
            layout_version,
            padding_4,
            padding_5,
            reward_infos,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_fees, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_a_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_b_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_a_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_b_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.whitelisted_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_a_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_b_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.dead_liquidity_fee_checkpoint,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.padding_2, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sqrt_min_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sqrt_max_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sqrt_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.activation_point, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.activation_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_status, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_a_flag, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_b_flag, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collect_fee_mode, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_version, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding_3, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_a_per_liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_b_per_liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.permanent_lock_liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.metrics, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_a_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_b_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.layout_version, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding_4, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding_5, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reward_infos, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PoolAccount(pub Pool);
impl PoolAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POOL_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Pool::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POOL_ACCOUNT_DISCM)?;
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
    pub pool: Pubkey,
    pub nft_mint: Pubkey,
    pub fee_a_per_token_checkpoint: [u8; 32],
    pub fee_b_per_token_checkpoint: [u8; 32],
    pub fee_a_pending: u64,
    pub fee_b_pending: u64,
    pub unlocked_liquidity: u128,
    pub vested_liquidity: u128,
    pub permanent_locked_liquidity: u128,
    pub metrics: PositionMetrics,
    pub reward_infos: [UserRewardInfo; 2],
    pub inner_vesting: InnerVesting,
    pub delegate_permission: u32,
    pub padding: [u8; 12],
}
impl Position {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let nft_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_a_per_token_checkpoint: [u8; 32] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let fee_b_per_token_checkpoint: [u8; 32] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let fee_a_pending: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_b_pending: u64 = crate::borsh_de_or_default(&mut reader)?;
        let unlocked_liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let vested_liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let permanent_locked_liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let metrics = if reader.is_empty() {
            Default::default()
        } else {
            <PositionMetrics>::deserialize(&mut reader)?
        };
        let reward_infos: [UserRewardInfo; 2] = crate::borsh_de_or_default(&mut reader)?;
        let inner_vesting = if reader.is_empty() {
            Default::default()
        } else {
            <InnerVesting>::deserialize(&mut reader)?
        };
        let delegate_permission: u32 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 12] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool,
            nft_mint,
            fee_a_per_token_checkpoint,
            fee_b_per_token_checkpoint,
            fee_a_pending,
            fee_b_pending,
            unlocked_liquidity,
            vested_liquidity,
            permanent_locked_liquidity,
            metrics,
            reward_infos,
            inner_vesting,
            delegate_permission,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.nft_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_a_per_token_checkpoint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_b_per_token_checkpoint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_a_pending, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_b_pending, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.unlocked_liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vested_liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.permanent_locked_liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.metrics, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reward_infos, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.inner_vesting, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.delegate_permission, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
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
pub const TOKEN_BADGE_ACCOUNT_DISCM: [u8; 8] = [116, 219, 204, 229, 249, 116, 255, 150];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct TokenBadge {
    pub token_mint: Pubkey,
    #[serde(with = "crate::big_array_serde")]
    pub padding: [u8; 128],
}
impl TokenBadge {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let token_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let padding = <[u8; 128] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self { token_mint, padding })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.token_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
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
pub const VESTING_ACCOUNT_DISCM: [u8; 8] = [100, 149, 66, 138, 95, 200, 128, 241];
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
pub struct Vesting {
    pub position: Pubkey,
    pub inner_vesting: InnerVesting,
    pub padding2: [u128; 4],
}
impl Vesting {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let position: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let inner_vesting = if reader.is_empty() {
            Default::default()
        } else {
            <InnerVesting>::deserialize(&mut reader)?
        };
        let padding2: [u128; 4] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            position,
            inner_vesting,
            padding2,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.position, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.inner_vesting, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding2, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct VestingAccount(pub Vesting);
impl VestingAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != VESTING_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Vesting::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&VESTING_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
