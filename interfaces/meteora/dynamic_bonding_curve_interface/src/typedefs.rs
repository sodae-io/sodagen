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
pub enum AccountsType {
    #[default]
    TransferHookBase,
    TransferHookBaseReferral,
}
impl TryFrom<u8> for AccountsType {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::TransferHookBase),
            1u8 => Ok(Self::TransferHookBaseReferral),
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
pub struct BaseFeeConfig {
    pub cliff_fee_numerator: u64,
    pub second_factor: u64,
    pub third_factor: u64,
    pub first_factor: u16,
    pub base_fee_mode: u8,
    pub padding_0: [u8; 5],
}
impl BaseFeeConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let cliff_fee_numerator: u64 = crate::borsh_de_or_default(&mut reader)?;
        let second_factor: u64 = crate::borsh_de_or_default(&mut reader)?;
        let third_factor: u64 = crate::borsh_de_or_default(&mut reader)?;
        let first_factor: u16 = crate::borsh_de_or_default(&mut reader)?;
        let base_fee_mode: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding_0: [u8; 5] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            cliff_fee_numerator,
            second_factor,
            third_factor,
            first_factor,
            base_fee_mode,
            padding_0,
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
pub struct BaseFeeParameters {
    pub cliff_fee_numerator: u64,
    pub first_factor: u16,
    pub second_factor: u64,
    pub third_factor: u64,
    pub base_fee_mode: u8,
}
impl BaseFeeParameters {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let cliff_fee_numerator: u64 = crate::borsh_de_or_default(&mut reader)?;
        let first_factor: u16 = crate::borsh_de_or_default(&mut reader)?;
        let second_factor: u64 = crate::borsh_de_or_default(&mut reader)?;
        let third_factor: u64 = crate::borsh_de_or_default(&mut reader)?;
        let base_fee_mode: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            cliff_fee_numerator,
            first_factor,
            second_factor,
            third_factor,
            base_fee_mode,
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
pub struct Config {
    pub pool_fees: PoolFees,
    pub activation_duration: u64,
    pub vault_config_key: Pubkey,
    pub pool_creator_authority: Pubkey,
    pub activation_type: u8,
    pub partner_fee_numerator: u64,
    #[serde(with = "crate::big_array_serde")]
    pub padding: [u8; 219],
}
impl Config {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_fees = if reader.is_empty() {
            Default::default()
        } else {
            <PoolFees>::deserialize(&mut reader)?
        };
        let activation_duration: u64 = crate::borsh_de_or_default(&mut reader)?;
        let vault_config_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_creator_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let activation_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let partner_fee_numerator: u64 = crate::borsh_de_or_default(&mut reader)?;
        let padding = <[u8; 219] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            pool_fees,
            activation_duration,
            vault_config_key,
            pool_creator_authority,
            activation_type,
            partner_fee_numerator,
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
pub struct ConfigParameters {
    pub pool_fees: PoolFeeParameters,
    pub collect_fee_mode: u8,
    pub migration_option: u8,
    pub activation_type: u8,
    pub token_type: u8,
    pub token_decimal: u8,
    pub partner_liquidity_percentage: u8,
    pub partner_permanent_locked_liquidity_percentage: u8,
    pub creator_liquidity_percentage: u8,
    pub creator_permanent_locked_liquidity_percentage: u8,
    pub migration_quote_threshold: u64,
    pub sqrt_start_price: u128,
    pub locked_vesting: LockedVestingParams,
    pub migration_fee_option: u8,
    pub token_supply: Option<TokenSupplyParams>,
    pub creator_trading_fee_percentage: u8,
    pub token_update_authority: u8,
    pub migration_fee: MigrationFee,
    pub migrated_pool_fee: MigratedPoolFee,
    pub pool_creation_fee: u64,
    pub partner_liquidity_vesting_info: LiquidityVestingInfoParams,
    pub creator_liquidity_vesting_info: LiquidityVestingInfoParams,
    pub migrated_pool_base_fee_mode: u8,
    pub migrated_pool_market_cap_fee_scheduler_params: MigratedPoolMarketCapFeeSchedulerParams,
    pub enable_first_swap_with_min_fee: bool,
    pub compounding_fee_bps: u16,
    pub padding: [u8; 2],
    pub curve: Vec<LiquidityDistributionParameters>,
}
impl ConfigParameters {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_fees = if reader.is_empty() {
            Default::default()
        } else {
            <PoolFeeParameters>::deserialize(&mut reader)?
        };
        let collect_fee_mode: u8 = crate::borsh_de_or_default(&mut reader)?;
        let migration_option: u8 = crate::borsh_de_or_default(&mut reader)?;
        let activation_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let token_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let token_decimal: u8 = crate::borsh_de_or_default(&mut reader)?;
        let partner_liquidity_percentage: u8 = crate::borsh_de_or_default(&mut reader)?;
        let partner_permanent_locked_liquidity_percentage: u8 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let creator_liquidity_percentage: u8 = crate::borsh_de_or_default(&mut reader)?;
        let creator_permanent_locked_liquidity_percentage: u8 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let migration_quote_threshold: u64 = crate::borsh_de_or_default(&mut reader)?;
        let sqrt_start_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        let locked_vesting = if reader.is_empty() {
            Default::default()
        } else {
            <LockedVestingParams>::deserialize(&mut reader)?
        };
        let migration_fee_option: u8 = crate::borsh_de_or_default(&mut reader)?;
        let token_supply: Option<TokenSupplyParams> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let creator_trading_fee_percentage: u8 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let token_update_authority: u8 = crate::borsh_de_or_default(&mut reader)?;
        let migration_fee = if reader.is_empty() {
            Default::default()
        } else {
            <MigrationFee>::deserialize(&mut reader)?
        };
        let migrated_pool_fee = if reader.is_empty() {
            Default::default()
        } else {
            <MigratedPoolFee>::deserialize(&mut reader)?
        };
        let pool_creation_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let partner_liquidity_vesting_info = if reader.is_empty() {
            Default::default()
        } else {
            <LiquidityVestingInfoParams>::deserialize(&mut reader)?
        };
        let creator_liquidity_vesting_info = if reader.is_empty() {
            Default::default()
        } else {
            <LiquidityVestingInfoParams>::deserialize(&mut reader)?
        };
        let migrated_pool_base_fee_mode: u8 = crate::borsh_de_or_default(&mut reader)?;
        let migrated_pool_market_cap_fee_scheduler_params = if reader.is_empty() {
            Default::default()
        } else {
            <MigratedPoolMarketCapFeeSchedulerParams>::deserialize(&mut reader)?
        };
        let enable_first_swap_with_min_fee: bool = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let compounding_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 2] = crate::borsh_de_or_default(&mut reader)?;
        let curve: Vec<LiquidityDistributionParameters> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            pool_fees,
            collect_fee_mode,
            migration_option,
            activation_type,
            token_type,
            token_decimal,
            partner_liquidity_percentage,
            partner_permanent_locked_liquidity_percentage,
            creator_liquidity_percentage,
            creator_permanent_locked_liquidity_percentage,
            migration_quote_threshold,
            sqrt_start_price,
            locked_vesting,
            migration_fee_option,
            token_supply,
            creator_trading_fee_percentage,
            token_update_authority,
            migration_fee,
            migrated_pool_fee,
            pool_creation_fee,
            partner_liquidity_vesting_info,
            creator_liquidity_vesting_info,
            migrated_pool_base_fee_mode,
            migrated_pool_market_cap_fee_scheduler_params,
            enable_first_swap_with_min_fee,
            compounding_fee_bps,
            padding,
            curve,
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
pub struct CreatePartnerMetadataParameters {
    #[serde(with = "crate::big_array_serde")]
    pub padding: [u8; 96],
    pub name: String,
    pub website: String,
    pub logo: String,
}
impl CreatePartnerMetadataParameters {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let padding = <[u8; 96] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        let website: String = crate::borsh_de_or_default(&mut reader)?;
        let logo: String = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            padding,
            name,
            website,
            logo,
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
pub struct CreateVirtualPoolMetadataParameters {
    #[serde(with = "crate::big_array_serde")]
    pub padding: [u8; 96],
    pub name: String,
    pub website: String,
    pub logo: String,
}
impl CreateVirtualPoolMetadataParameters {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let padding = <[u8; 96] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        let website: String = crate::borsh_de_or_default(&mut reader)?;
        let logo: String = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            padding,
            name,
            website,
            logo,
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
pub struct DynamicFeeConfig {
    pub initialized: u8,
    pub padding: [u8; 7],
    pub max_volatility_accumulator: u32,
    pub variable_fee_control: u32,
    pub bin_step: u16,
    pub filter_period: u16,
    pub decay_period: u16,
    pub reduction_factor: u16,
    pub padding2: [u8; 8],
    pub bin_step_u128: u128,
}
impl DynamicFeeConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let initialized: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 7] = crate::borsh_de_or_default(&mut reader)?;
        let max_volatility_accumulator: u32 = crate::borsh_de_or_default(&mut reader)?;
        let variable_fee_control: u32 = crate::borsh_de_or_default(&mut reader)?;
        let bin_step: u16 = crate::borsh_de_or_default(&mut reader)?;
        let filter_period: u16 = crate::borsh_de_or_default(&mut reader)?;
        let decay_period: u16 = crate::borsh_de_or_default(&mut reader)?;
        let reduction_factor: u16 = crate::borsh_de_or_default(&mut reader)?;
        let padding2: [u8; 8] = crate::borsh_de_or_default(&mut reader)?;
        let bin_step_u128: u128 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            initialized,
            padding,
            max_volatility_accumulator,
            variable_fee_control,
            bin_step,
            filter_period,
            decay_period,
            reduction_factor,
            padding2,
            bin_step_u128,
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
pub struct DynamicFeeParameters {
    pub bin_step: u16,
    pub bin_step_u128: u128,
    pub filter_period: u16,
    pub decay_period: u16,
    pub reduction_factor: u16,
    pub max_volatility_accumulator: u32,
    pub variable_fee_control: u32,
}
impl DynamicFeeParameters {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bin_step: u16 = crate::borsh_de_or_default(&mut reader)?;
        let bin_step_u128: u128 = crate::borsh_de_or_default(&mut reader)?;
        let filter_period: u16 = crate::borsh_de_or_default(&mut reader)?;
        let decay_period: u16 = crate::borsh_de_or_default(&mut reader)?;
        let reduction_factor: u16 = crate::borsh_de_or_default(&mut reader)?;
        let max_volatility_accumulator: u32 = crate::borsh_de_or_default(&mut reader)?;
        let variable_fee_control: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            bin_step,
            bin_step_u128,
            filter_period,
            decay_period,
            reduction_factor,
            max_volatility_accumulator,
            variable_fee_control,
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
pub struct InitializePoolParameters {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}
impl InitializePoolParameters {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        let symbol: String = crate::borsh_de_or_default(&mut reader)?;
        let uri: String = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { name, symbol, uri })
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
pub struct LiquidityDistributionConfig {
    pub sqrt_price: u128,
    pub liquidity: u128,
}
impl LiquidityDistributionConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let sqrt_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        let liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { sqrt_price, liquidity })
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
pub struct LiquidityDistributionParameters {
    pub sqrt_price: u128,
    pub liquidity: u128,
}
impl LiquidityDistributionParameters {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let sqrt_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        let liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { sqrt_price, liquidity })
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
pub struct LiquidityVestingInfo {
    pub is_initialized: u8,
    pub vesting_percentage: u8,
    pub padding: [u8; 2],
    pub bps_per_period: u16,
    pub number_of_periods: u16,
    pub frequency: u32,
    pub cliff_duration_from_migration_time: u32,
}
impl LiquidityVestingInfo {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let is_initialized: u8 = crate::borsh_de_or_default(&mut reader)?;
        let vesting_percentage: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 2] = crate::borsh_de_or_default(&mut reader)?;
        let bps_per_period: u16 = crate::borsh_de_or_default(&mut reader)?;
        let number_of_periods: u16 = crate::borsh_de_or_default(&mut reader)?;
        let frequency: u32 = crate::borsh_de_or_default(&mut reader)?;
        let cliff_duration_from_migration_time: u32 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            is_initialized,
            vesting_percentage,
            padding,
            bps_per_period,
            number_of_periods,
            frequency,
            cliff_duration_from_migration_time,
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
pub struct LiquidityVestingInfoParams {
    pub vesting_percentage: u8,
    pub bps_per_period: u16,
    pub number_of_periods: u16,
    pub cliff_duration_from_migration_time: u32,
    pub frequency: u32,
}
impl LiquidityVestingInfoParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let vesting_percentage: u8 = crate::borsh_de_or_default(&mut reader)?;
        let bps_per_period: u16 = crate::borsh_de_or_default(&mut reader)?;
        let number_of_periods: u16 = crate::borsh_de_or_default(&mut reader)?;
        let cliff_duration_from_migration_time: u32 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let frequency: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            vesting_percentage,
            bps_per_period,
            number_of_periods,
            cliff_duration_from_migration_time,
            frequency,
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
pub struct LockEscrow {
    pub pool: Pubkey,
    pub owner: Pubkey,
    pub escrow_vault: Pubkey,
    pub bump: u8,
    pub total_locked_amount: u64,
    pub lp_per_token: u128,
    pub unclaimed_fee_pending: u64,
    pub a_fee: u64,
    pub b_fee: u64,
}
impl LockEscrow {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let escrow_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let total_locked_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let lp_per_token: u128 = crate::borsh_de_or_default(&mut reader)?;
        let unclaimed_fee_pending: u64 = crate::borsh_de_or_default(&mut reader)?;
        let a_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let b_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool,
            owner,
            escrow_vault,
            bump,
            total_locked_amount,
            lp_per_token,
            unclaimed_fee_pending,
            a_fee,
            b_fee,
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
pub struct LockedVestingConfig {
    pub amount_per_period: u64,
    pub cliff_duration_from_migration_time: u64,
    pub frequency: u64,
    pub number_of_period: u64,
    pub cliff_unlock_amount: u64,
    pub padding: u64,
}
impl LockedVestingConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount_per_period: u64 = crate::borsh_de_or_default(&mut reader)?;
        let cliff_duration_from_migration_time: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let frequency: u64 = crate::borsh_de_or_default(&mut reader)?;
        let number_of_period: u64 = crate::borsh_de_or_default(&mut reader)?;
        let cliff_unlock_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let padding: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            amount_per_period,
            cliff_duration_from_migration_time,
            frequency,
            number_of_period,
            cliff_unlock_amount,
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
pub struct LockedVestingParams {
    pub amount_per_period: u64,
    pub cliff_duration_from_migration_time: u64,
    pub frequency: u64,
    pub number_of_period: u64,
    pub cliff_unlock_amount: u64,
}
impl LockedVestingParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount_per_period: u64 = crate::borsh_de_or_default(&mut reader)?;
        let cliff_duration_from_migration_time: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let frequency: u64 = crate::borsh_de_or_default(&mut reader)?;
        let number_of_period: u64 = crate::borsh_de_or_default(&mut reader)?;
        let cliff_unlock_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            amount_per_period,
            cliff_duration_from_migration_time,
            frequency,
            number_of_period,
            cliff_unlock_amount,
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
pub struct MigratedPoolFee {
    pub collect_fee_mode: u8,
    pub dynamic_fee: u8,
    pub pool_fee_bps: u16,
}
impl MigratedPoolFee {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let collect_fee_mode: u8 = crate::borsh_de_or_default(&mut reader)?;
        let dynamic_fee: u8 = crate::borsh_de_or_default(&mut reader)?;
        let pool_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            collect_fee_mode,
            dynamic_fee,
            pool_fee_bps,
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
pub struct MigratedPoolMarketCapFeeSchedulerParams {
    pub number_of_period: u16,
    pub sqrt_price_step_bps: u16,
    pub scheduler_expiration_duration: u32,
    pub reduction_factor: u64,
}
impl MigratedPoolMarketCapFeeSchedulerParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let number_of_period: u16 = crate::borsh_de_or_default(&mut reader)?;
        let sqrt_price_step_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let scheduler_expiration_duration: u32 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let reduction_factor: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            number_of_period,
            sqrt_price_step_bps,
            scheduler_expiration_duration,
            reduction_factor,
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
pub struct MigrationFee {
    pub fee_percentage: u8,
    pub creator_fee_percentage: u8,
}
impl MigrationFee {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let fee_percentage: u8 = crate::borsh_de_or_default(&mut reader)?;
        let creator_fee_percentage: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            fee_percentage,
            creator_fee_percentage,
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
pub struct PoolFeeParameters {
    pub base_fee: BaseFeeParameters,
    pub dynamic_fee: Option<DynamicFeeParameters>,
}
impl PoolFeeParameters {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let base_fee = if reader.is_empty() {
            Default::default()
        } else {
            <BaseFeeParameters>::deserialize(&mut reader)?
        };
        let dynamic_fee: Option<DynamicFeeParameters> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self { base_fee, dynamic_fee })
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
pub struct PoolFeesConfig {
    pub base_fee: BaseFeeConfig,
    pub dynamic_fee: DynamicFeeConfig,
}
impl PoolFeesConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let base_fee = if reader.is_empty() {
            Default::default()
        } else {
            <BaseFeeConfig>::deserialize(&mut reader)?
        };
        let dynamic_fee = if reader.is_empty() {
            Default::default()
        } else {
            <DynamicFeeConfig>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self { base_fee, dynamic_fee })
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
pub struct PoolMetrics {
    pub total_protocol_base_fee: u64,
    pub total_protocol_quote_fee: u64,
    pub total_trading_base_fee: u64,
    pub total_trading_quote_fee: u64,
}
impl PoolMetrics {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let total_protocol_base_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_protocol_quote_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_trading_base_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_trading_quote_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            total_protocol_base_fee,
            total_protocol_quote_fee,
            total_trading_base_fee,
            total_trading_quote_fee,
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
pub struct PoolState {
    pub volatility_tracker: VolatilityTracker,
    pub config: Pubkey,
    pub creator: Pubkey,
    pub base_mint: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub base_reserve: u64,
    pub quote_reserve: u64,
    pub protocol_base_fee: u64,
    pub protocol_quote_fee: u64,
    pub partner_base_fee: u64,
    pub partner_quote_fee: u64,
    pub sqrt_price: u128,
    pub activation_point: u64,
    pub pool_type: u8,
    pub is_migrated: u8,
    pub is_partner_withdraw_surplus: u8,
    pub is_protocol_withdraw_surplus: u8,
    pub migration_progress: u8,
    pub is_withdraw_leftover: u8,
    pub is_creator_withdraw_surplus: u8,
    pub migration_fee_withdraw_status: u8,
    pub metrics: PoolMetrics,
    pub finish_curve_timestamp: u64,
    pub creator_base_fee: u64,
    pub creator_quote_fee: u64,
    pub legacy_creation_fee_bits: u8,
    pub creation_fee_bits: u8,
    pub has_swap: u8,
    pub padding_0: [u8; 5],
    pub protocol_liquidity_migration_fee_bps: u16,
    pub padding_1: [u8; 6],
    pub protocol_migration_base_fee_amount: u64,
    pub protocol_migration_quote_fee_amount: u64,
    pub padding_2: [u64; 3],
}
impl PoolState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let volatility_tracker = if reader.is_empty() {
            Default::default()
        } else {
            <VolatilityTracker>::deserialize(&mut reader)?
        };
        let config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let quote_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_reserve: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_reserve: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_base_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_quote_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let partner_base_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let partner_quote_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let sqrt_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        let activation_point: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pool_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let is_migrated: u8 = crate::borsh_de_or_default(&mut reader)?;
        let is_partner_withdraw_surplus: u8 = crate::borsh_de_or_default(&mut reader)?;
        let is_protocol_withdraw_surplus: u8 = crate::borsh_de_or_default(&mut reader)?;
        let migration_progress: u8 = crate::borsh_de_or_default(&mut reader)?;
        let is_withdraw_leftover: u8 = crate::borsh_de_or_default(&mut reader)?;
        let is_creator_withdraw_surplus: u8 = crate::borsh_de_or_default(&mut reader)?;
        let migration_fee_withdraw_status: u8 = crate::borsh_de_or_default(&mut reader)?;
        let metrics = if reader.is_empty() {
            Default::default()
        } else {
            <PoolMetrics>::deserialize(&mut reader)?
        };
        let finish_curve_timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        let creator_base_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let creator_quote_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let legacy_creation_fee_bits: u8 = crate::borsh_de_or_default(&mut reader)?;
        let creation_fee_bits: u8 = crate::borsh_de_or_default(&mut reader)?;
        let has_swap: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding_0: [u8; 5] = crate::borsh_de_or_default(&mut reader)?;
        let protocol_liquidity_migration_fee_bps: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let padding_1: [u8; 6] = crate::borsh_de_or_default(&mut reader)?;
        let protocol_migration_base_fee_amount: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let protocol_migration_quote_fee_amount: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let padding_2: [u64; 3] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            volatility_tracker,
            config,
            creator,
            base_mint,
            base_vault,
            quote_vault,
            base_reserve,
            quote_reserve,
            protocol_base_fee,
            protocol_quote_fee,
            partner_base_fee,
            partner_quote_fee,
            sqrt_price,
            activation_point,
            pool_type,
            is_migrated,
            is_partner_withdraw_surplus,
            is_protocol_withdraw_surplus,
            migration_progress,
            is_withdraw_leftover,
            is_creator_withdraw_surplus,
            migration_fee_withdraw_status,
            metrics,
            finish_curve_timestamp,
            creator_base_fee,
            creator_quote_fee,
            legacy_creation_fee_bits,
            creation_fee_bits,
            has_swap,
            padding_0,
            protocol_liquidity_migration_fee_bps,
            padding_1,
            protocol_migration_base_fee_amount,
            protocol_migration_quote_fee_amount,
            padding_2,
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
pub struct RemainingAccountsSlice {
    pub accounts_type: AccountsType,
    pub length: u8,
}
impl RemainingAccountsSlice {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let accounts_type: AccountsType = crate::borsh_de_or_default(&mut reader)?;
        let length: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { accounts_type, length })
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
pub struct SwapParameters {
    pub amount_in: u64,
    pub minimum_amount_out: u64,
}
impl SwapParameters {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let minimum_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            amount_in,
            minimum_amount_out,
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
pub struct SwapParameters2 {
    pub amount_0: u64,
    pub amount_1: u64,
    pub swap_mode: u8,
}
impl SwapParameters2 {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount_0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let swap_mode: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            amount_0,
            amount_1,
            swap_mode,
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
pub struct SwapResult {
    pub actual_input_amount: u64,
    pub output_amount: u64,
    pub next_sqrt_price: u128,
    pub trading_fee: u64,
    pub protocol_fee: u64,
    pub referral_fee: u64,
}
impl SwapResult {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let actual_input_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let output_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let next_sqrt_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        let trading_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let referral_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            actual_input_amount,
            output_amount,
            next_sqrt_price,
            trading_fee,
            protocol_fee,
            referral_fee,
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
pub struct SwapResult2 {
    pub included_fee_input_amount: u64,
    pub excluded_fee_input_amount: u64,
    pub amount_left: u64,
    pub output_amount: u64,
    pub next_sqrt_price: u128,
    pub trading_fee: u64,
    pub protocol_fee: u64,
    pub referral_fee: u64,
}
impl SwapResult2 {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let included_fee_input_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let excluded_fee_input_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_left: u64 = crate::borsh_de_or_default(&mut reader)?;
        let output_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let next_sqrt_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        let trading_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let referral_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            included_fee_input_amount,
            excluded_fee_input_amount,
            amount_left,
            output_amount,
            next_sqrt_price,
            trading_fee,
            protocol_fee,
            referral_fee,
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
pub struct TokenSupplyParams {
    pub pre_migration_token_supply: u64,
    pub post_migration_token_supply: u64,
}
impl TokenSupplyParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pre_migration_token_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let post_migration_token_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pre_migration_token_supply,
            post_migration_token_supply,
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
pub struct TransferHookAccountsInfo {
    pub slices: Vec<RemainingAccountsSlice>,
}
impl TransferHookAccountsInfo {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let slices: Vec<RemainingAccountsSlice> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self { slices })
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
pub struct VolatilityTracker {
    pub last_update_timestamp: u64,
    pub padding: [u8; 8],
    pub sqrt_price_reference: u128,
    pub volatility_accumulator: u128,
    pub volatility_reference: u128,
}
impl VolatilityTracker {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let last_update_timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 8] = crate::borsh_de_or_default(&mut reader)?;
        let sqrt_price_reference: u128 = crate::borsh_de_or_default(&mut reader)?;
        let volatility_accumulator: u128 = crate::borsh_de_or_default(&mut reader)?;
        let volatility_reference: u128 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            last_update_timestamp,
            padding,
            sqrt_price_reference,
            volatility_accumulator,
            volatility_reference,
        })
    }
}
