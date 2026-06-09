use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const CLAIM_FEE_OPERATOR_ACCOUNT_DISCM: [u8; 8] = [
    166, 48, 134, 86, 34, 200, 188, 150,
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
pub struct ClaimFeeOperator {
    pub operator: Pubkey,
    #[serde(with = "crate::big_array_serde")]
    pub _padding: [u8; 128],
}
impl ClaimFeeOperator {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let operator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let _padding = <[u8; 128] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self { operator, _padding })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.operator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimFeeOperatorAccount(pub ClaimFeeOperator);
impl ClaimFeeOperatorAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_FEE_OPERATOR_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(ClaimFeeOperator::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_FEE_OPERATOR_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CONFIG_ACCOUNT_DISCM: [u8; 8] = [155, 12, 170, 224, 30, 250, 204, 130];
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
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_fees, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.activation_duration, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault_config_key, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_creator_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.activation_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.partner_fee_numerator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
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
pub const LOCK_ESCROW_ACCOUNT_DISCM: [u8; 8] = [190, 106, 121, 6, 200, 182, 21, 75];
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
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.escrow_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_locked_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_per_token, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.unclaimed_fee_pending, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.a_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.b_fee, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LockEscrowAccount(pub LockEscrow);
impl LockEscrowAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOCK_ESCROW_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(LockEscrow::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOCK_ESCROW_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const METEORA_DAMM_MIGRATION_METADATA_ACCOUNT_DISCM: [u8; 8] = [
    17, 155, 141, 215, 207, 4, 133, 156,
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
pub struct MeteoraDammMigrationMetadata {
    pub virtual_pool: Pubkey,
    pub padding_0: [u8; 32],
    pub partner: Pubkey,
    pub lp_mint: Pubkey,
    pub partner_locked_liquidity: u64,
    pub partner_liquidity: u64,
    pub creator_locked_liquidity: u64,
    pub creator_liquidity: u64,
    pub _padding_0: u8,
    pub creator_locked_status: u8,
    pub partner_locked_status: u8,
    pub creator_claim_status: u8,
    pub partner_claim_status: u8,
    #[serde(with = "crate::big_array_serde")]
    pub _padding: [u8; 107],
}
impl MeteoraDammMigrationMetadata {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let virtual_pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let padding_0: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        let partner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lp_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let partner_locked_liquidity: u64 = crate::borsh_de_or_default(&mut reader)?;
        let partner_liquidity: u64 = crate::borsh_de_or_default(&mut reader)?;
        let creator_locked_liquidity: u64 = crate::borsh_de_or_default(&mut reader)?;
        let creator_liquidity: u64 = crate::borsh_de_or_default(&mut reader)?;
        let _padding_0: u8 = crate::borsh_de_or_default(&mut reader)?;
        let creator_locked_status: u8 = crate::borsh_de_or_default(&mut reader)?;
        let partner_locked_status: u8 = crate::borsh_de_or_default(&mut reader)?;
        let creator_claim_status: u8 = crate::borsh_de_or_default(&mut reader)?;
        let partner_claim_status: u8 = crate::borsh_de_or_default(&mut reader)?;
        let _padding = <[u8; 107] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            virtual_pool,
            padding_0,
            partner,
            lp_mint,
            partner_locked_liquidity,
            partner_liquidity,
            creator_locked_liquidity,
            creator_liquidity,
            _padding_0,
            creator_locked_status,
            partner_locked_status,
            creator_claim_status,
            partner_claim_status,
            _padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.virtual_pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.partner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.partner_locked_liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.partner_liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator_locked_liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator_liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._padding_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator_locked_status, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.partner_locked_status, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator_claim_status, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.partner_claim_status, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MeteoraDammMigrationMetadataAccount(pub MeteoraDammMigrationMetadata);
impl MeteoraDammMigrationMetadataAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != METEORA_DAMM_MIGRATION_METADATA_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(MeteoraDammMigrationMetadata::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&METEORA_DAMM_MIGRATION_METADATA_ACCOUNT_DISCM)?;
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
pub const PARTNER_METADATA_ACCOUNT_DISCM: [u8; 8] = [68, 68, 130, 19, 16, 209, 98, 156];
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
pub struct PartnerMetadata {
    pub fee_claimer: Pubkey,
    pub padding: [u128; 6],
    pub name: String,
    pub website: String,
    pub logo: String,
}
impl PartnerMetadata {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let fee_claimer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u128; 6] = crate::borsh_de_or_default(&mut reader)?;
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        let website: String = crate::borsh_de_or_default(&mut reader)?;
        let logo: String = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            fee_claimer,
            padding,
            name,
            website,
            logo,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.fee_claimer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.name, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.website, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.logo, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PartnerMetadataAccount(pub PartnerMetadata);
impl PartnerMetadataAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PARTNER_METADATA_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(PartnerMetadata::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PARTNER_METADATA_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POOL_CONFIG_ACCOUNT_DISCM: [u8; 8] = [26, 108, 14, 123, 116, 230, 129, 43];
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
pub struct PoolConfig {
    pub quote_mint: Pubkey,
    pub fee_claimer: Pubkey,
    pub leftover_receiver: Pubkey,
    pub pool_fees: PoolFeesConfig,
    pub partner_liquidity_vesting_info: LiquidityVestingInfo,
    pub creator_liquidity_vesting_info: LiquidityVestingInfo,
    pub padding_0: [u8; 14],
    pub padding_1: u16,
    pub collect_fee_mode: u8,
    pub migration_option: u8,
    pub activation_type: u8,
    pub token_decimal: u8,
    pub version: u8,
    pub token_type: u8,
    pub quote_token_flag: u8,
    pub partner_permanent_locked_liquidity_percentage: u8,
    pub partner_liquidity_percentage: u8,
    pub creator_permanent_locked_liquidity_percentage: u8,
    pub creator_liquidity_percentage: u8,
    pub migration_fee_option: u8,
    pub fixed_token_supply_flag: u8,
    pub creator_trading_fee_percentage: u8,
    pub token_update_authority: u8,
    pub migration_fee_percentage: u8,
    pub creator_migration_fee_percentage: u8,
    pub padding_2: [u8; 7],
    pub swap_base_amount: u64,
    pub migration_quote_threshold: u64,
    pub migration_base_threshold: u64,
    pub migration_sqrt_price: u128,
    pub locked_vesting_config: LockedVestingConfig,
    pub pre_migration_token_supply: u64,
    pub post_migration_token_supply: u64,
    pub migrated_collect_fee_mode: u8,
    pub migrated_dynamic_fee: u8,
    pub migrated_pool_fee_bps: u16,
    pub migrated_pool_base_fee_mode: u8,
    pub enable_first_swap_with_min_fee: u8,
    pub migrated_compounding_fee_bps: u16,
    pub pool_creation_fee: u64,
    pub migrated_pool_base_fee_bytes: [u8; 16],
    pub sqrt_start_price: u128,
    pub curve: [LiquidityDistributionConfig; 20],
}
impl PoolConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let quote_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_claimer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let leftover_receiver: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_fees = if reader.is_empty() {
            Default::default()
        } else {
            <PoolFeesConfig>::deserialize(&mut reader)?
        };
        let partner_liquidity_vesting_info = if reader.is_empty() {
            Default::default()
        } else {
            <LiquidityVestingInfo>::deserialize(&mut reader)?
        };
        let creator_liquidity_vesting_info = if reader.is_empty() {
            Default::default()
        } else {
            <LiquidityVestingInfo>::deserialize(&mut reader)?
        };
        let padding_0: [u8; 14] = crate::borsh_de_or_default(&mut reader)?;
        let padding_1: u16 = crate::borsh_de_or_default(&mut reader)?;
        let collect_fee_mode: u8 = crate::borsh_de_or_default(&mut reader)?;
        let migration_option: u8 = crate::borsh_de_or_default(&mut reader)?;
        let activation_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let token_decimal: u8 = crate::borsh_de_or_default(&mut reader)?;
        let version: u8 = crate::borsh_de_or_default(&mut reader)?;
        let token_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let quote_token_flag: u8 = crate::borsh_de_or_default(&mut reader)?;
        let partner_permanent_locked_liquidity_percentage: u8 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let partner_liquidity_percentage: u8 = crate::borsh_de_or_default(&mut reader)?;
        let creator_permanent_locked_liquidity_percentage: u8 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let creator_liquidity_percentage: u8 = crate::borsh_de_or_default(&mut reader)?;
        let migration_fee_option: u8 = crate::borsh_de_or_default(&mut reader)?;
        let fixed_token_supply_flag: u8 = crate::borsh_de_or_default(&mut reader)?;
        let creator_trading_fee_percentage: u8 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let token_update_authority: u8 = crate::borsh_de_or_default(&mut reader)?;
        let migration_fee_percentage: u8 = crate::borsh_de_or_default(&mut reader)?;
        let creator_migration_fee_percentage: u8 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let padding_2: [u8; 7] = crate::borsh_de_or_default(&mut reader)?;
        let swap_base_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let migration_quote_threshold: u64 = crate::borsh_de_or_default(&mut reader)?;
        let migration_base_threshold: u64 = crate::borsh_de_or_default(&mut reader)?;
        let migration_sqrt_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        let locked_vesting_config = if reader.is_empty() {
            Default::default()
        } else {
            <LockedVestingConfig>::deserialize(&mut reader)?
        };
        let pre_migration_token_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let post_migration_token_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let migrated_collect_fee_mode: u8 = crate::borsh_de_or_default(&mut reader)?;
        let migrated_dynamic_fee: u8 = crate::borsh_de_or_default(&mut reader)?;
        let migrated_pool_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let migrated_pool_base_fee_mode: u8 = crate::borsh_de_or_default(&mut reader)?;
        let enable_first_swap_with_min_fee: u8 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let migrated_compounding_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let pool_creation_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let migrated_pool_base_fee_bytes: [u8; 16] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let sqrt_start_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        let curve: [LiquidityDistributionConfig; 20] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            quote_mint,
            fee_claimer,
            leftover_receiver,
            pool_fees,
            partner_liquidity_vesting_info,
            creator_liquidity_vesting_info,
            padding_0,
            padding_1,
            collect_fee_mode,
            migration_option,
            activation_type,
            token_decimal,
            version,
            token_type,
            quote_token_flag,
            partner_permanent_locked_liquidity_percentage,
            partner_liquidity_percentage,
            creator_permanent_locked_liquidity_percentage,
            creator_liquidity_percentage,
            migration_fee_option,
            fixed_token_supply_flag,
            creator_trading_fee_percentage,
            token_update_authority,
            migration_fee_percentage,
            creator_migration_fee_percentage,
            padding_2,
            swap_base_amount,
            migration_quote_threshold,
            migration_base_threshold,
            migration_sqrt_price,
            locked_vesting_config,
            pre_migration_token_supply,
            post_migration_token_supply,
            migrated_collect_fee_mode,
            migrated_dynamic_fee,
            migrated_pool_fee_bps,
            migrated_pool_base_fee_mode,
            enable_first_swap_with_min_fee,
            migrated_compounding_fee_bps,
            pool_creation_fee,
            migrated_pool_base_fee_bytes,
            sqrt_start_price,
            curve,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.quote_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_claimer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.leftover_receiver, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_fees, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.partner_liquidity_vesting_info,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.creator_liquidity_vesting_info,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.padding_0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collect_fee_mode, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.migration_option, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.activation_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_decimal, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.version, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_token_flag, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.partner_permanent_locked_liquidity_percentage,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.partner_liquidity_percentage,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.creator_permanent_locked_liquidity_percentage,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.creator_liquidity_percentage,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.migration_fee_option, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fixed_token_supply_flag, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.creator_trading_fee_percentage,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.token_update_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.migration_fee_percentage, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.creator_migration_fee_percentage,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.padding_2, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.swap_base_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.migration_quote_threshold, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.migration_base_threshold, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.migration_sqrt_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.locked_vesting_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pre_migration_token_supply, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.post_migration_token_supply,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.migrated_collect_fee_mode, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.migrated_dynamic_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.migrated_pool_fee_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.migrated_pool_base_fee_mode,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.enable_first_swap_with_min_fee,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.migrated_compounding_fee_bps,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.pool_creation_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.migrated_pool_base_fee_bytes,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.sqrt_start_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.curve, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PoolConfigAccount(pub PoolConfig);
impl PoolConfigAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POOL_CONFIG_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(PoolConfig::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POOL_CONFIG_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const VIRTUAL_POOL_ACCOUNT_DISCM: [u8; 8] = [213, 224, 5, 209, 98, 69, 119, 92];
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
pub struct VirtualPool {
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
    pub _padding_0: [u8; 5],
    pub protocol_liquidity_migration_fee_bps: u16,
    pub _padding_1: [u8; 6],
    pub protocol_migration_base_fee_amount: u64,
    pub protocol_migration_quote_fee_amount: u64,
    pub _padding_2: [u64; 3],
}
impl VirtualPool {
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
        let _padding_0: [u8; 5] = crate::borsh_de_or_default(&mut reader)?;
        let protocol_liquidity_migration_fee_bps: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let _padding_1: [u8; 6] = crate::borsh_de_or_default(&mut reader)?;
        let protocol_migration_base_fee_amount: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let protocol_migration_quote_fee_amount: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let _padding_2: [u64; 3] = crate::borsh_de_or_default(&mut reader)?;
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
            _padding_0,
            protocol_liquidity_migration_fee_bps,
            _padding_1,
            protocol_migration_base_fee_amount,
            protocol_migration_quote_fee_amount,
            _padding_2,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.volatility_tracker, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_reserve, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_reserve, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_base_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_quote_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.partner_base_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.partner_quote_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sqrt_price, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.activation_point, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.is_migrated, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.is_partner_withdraw_surplus,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.is_protocol_withdraw_surplus,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.migration_progress, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.is_withdraw_leftover, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.is_creator_withdraw_surplus,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.migration_fee_withdraw_status,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.metrics, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.finish_curve_timestamp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator_base_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator_quote_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.legacy_creation_fee_bits, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creation_fee_bits, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.has_swap, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._padding_0, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.protocol_liquidity_migration_fee_bps,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self._padding_1, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.protocol_migration_base_fee_amount,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.protocol_migration_quote_fee_amount,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self._padding_2, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct VirtualPoolAccount(pub VirtualPool);
impl VirtualPoolAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != VIRTUAL_POOL_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(VirtualPool::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&VIRTUAL_POOL_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const VIRTUAL_POOL_METADATA_ACCOUNT_DISCM: [u8; 8] = [
    217, 37, 82, 250, 43, 47, 228, 254,
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
pub struct VirtualPoolMetadata {
    pub virtual_pool: Pubkey,
    pub padding: [u128; 6],
    pub name: String,
    pub website: String,
    pub logo: String,
}
impl VirtualPoolMetadata {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let virtual_pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u128; 6] = crate::borsh_de_or_default(&mut reader)?;
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        let website: String = crate::borsh_de_or_default(&mut reader)?;
        let logo: String = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            virtual_pool,
            padding,
            name,
            website,
            logo,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.virtual_pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.name, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.website, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.logo, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct VirtualPoolMetadataAccount(pub VirtualPoolMetadata);
impl VirtualPoolMetadataAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != VIRTUAL_POOL_METADATA_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(VirtualPoolMetadata::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&VIRTUAL_POOL_METADATA_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
