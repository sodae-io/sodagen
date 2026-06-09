use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const GLOBAL_CONFIG_ACCOUNT_DISCM: [u8; 8] = [149, 8, 156, 202, 160, 252, 176, 217];
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
pub struct GlobalConfig {
    pub epoch: u64,
    pub curve_type: u8,
    pub index: u16,
    pub migrate_fee: u64,
    pub trade_fee_rate: u64,
    pub max_share_fee_rate: u64,
    pub min_base_supply: u64,
    pub max_lock_rate: u64,
    pub min_base_sell_rate: u64,
    pub min_base_migrate_rate: u64,
    pub min_quote_fund_raising: u64,
    pub quote_mint: Pubkey,
    pub protocol_fee_owner: Pubkey,
    pub migrate_fee_owner: Pubkey,
    pub migrate_to_amm_wallet: Pubkey,
    pub migrate_to_cpswap_wallet: Pubkey,
    pub requires_platform_auth: u8,
    pub padding_alignment: [u8; 7],
    pub padding: [u64; 15],
}
impl GlobalConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let epoch: u64 = crate::borsh_de_or_default(&mut reader)?;
        let curve_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let migrate_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let trade_fee_rate: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_share_fee_rate: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_base_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_lock_rate: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_base_sell_rate: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_base_migrate_rate: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_quote_fund_raising: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee_owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let migrate_fee_owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let migrate_to_amm_wallet: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let migrate_to_cpswap_wallet: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let requires_platform_auth: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding_alignment: [u8; 7] = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u64; 15] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            epoch,
            curve_type,
            index,
            migrate_fee,
            trade_fee_rate,
            max_share_fee_rate,
            min_base_supply,
            max_lock_rate,
            min_base_sell_rate,
            min_base_migrate_rate,
            min_quote_fund_raising,
            quote_mint,
            protocol_fee_owner,
            migrate_fee_owner,
            migrate_to_amm_wallet,
            migrate_to_cpswap_wallet,
            requires_platform_auth,
            padding_alignment,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.epoch, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.curve_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.migrate_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.trade_fee_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_share_fee_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.min_base_supply, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_lock_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.min_base_sell_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.min_base_migrate_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.min_quote_fund_raising, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fee_owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.migrate_fee_owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.migrate_to_amm_wallet, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.migrate_to_cpswap_wallet, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.requires_platform_auth, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding_alignment, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct GlobalConfigAccount(pub GlobalConfig);
impl GlobalConfigAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != GLOBAL_CONFIG_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(GlobalConfig::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&GLOBAL_CONFIG_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const PLATFORM_CONFIG_ACCOUNT_DISCM: [u8; 8] = [160, 78, 128, 0, 248, 83, 230, 160];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct PlatformConfig {
    pub epoch: u64,
    pub platform_fee_wallet: Pubkey,
    pub platform_nft_wallet: Pubkey,
    pub platform_scale: u64,
    pub creator_scale: u64,
    pub burn_scale: u64,
    pub fee_rate: u64,
    #[serde(with = "crate::big_array_serde")]
    pub name: [u8; 64],
    #[serde(with = "crate::big_array_serde")]
    pub web: [u8; 256],
    #[serde(with = "crate::big_array_serde")]
    pub img: [u8; 256],
    pub cpswap_config: Pubkey,
    pub creator_fee_rate: u64,
    pub transfer_fee_extension_auth: Pubkey,
    pub platform_vesting_wallet: Pubkey,
    pub platform_vesting_scale: u64,
    pub platform_cp_creator: Pubkey,
    #[serde(with = "crate::big_array_serde")]
    pub padding: [u8; 108],
    pub curve_params: Vec<PlatformCurveParam>,
}
impl PlatformConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let epoch: u64 = crate::borsh_de_or_default(&mut reader)?;
        let platform_fee_wallet: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let platform_nft_wallet: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let platform_scale: u64 = crate::borsh_de_or_default(&mut reader)?;
        let creator_scale: u64 = crate::borsh_de_or_default(&mut reader)?;
        let burn_scale: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_rate: u64 = crate::borsh_de_or_default(&mut reader)?;
        let name = <[u8; 64] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let web = <[u8; 256] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let img = <[u8; 256] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let cpswap_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let creator_fee_rate: u64 = crate::borsh_de_or_default(&mut reader)?;
        let transfer_fee_extension_auth: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let platform_vesting_wallet: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let platform_vesting_scale: u64 = crate::borsh_de_or_default(&mut reader)?;
        let platform_cp_creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let padding = <[u8; 108] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let curve_params: Vec<PlatformCurveParam> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            epoch,
            platform_fee_wallet,
            platform_nft_wallet,
            platform_scale,
            creator_scale,
            burn_scale,
            fee_rate,
            name,
            web,
            img,
            cpswap_config,
            creator_fee_rate,
            transfer_fee_extension_auth,
            platform_vesting_wallet,
            platform_vesting_scale,
            platform_cp_creator,
            padding,
            curve_params,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.epoch, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.platform_fee_wallet, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.platform_nft_wallet, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.platform_scale, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator_scale, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.burn_scale, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.name, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.web, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.img, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.cpswap_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator_fee_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.transfer_fee_extension_auth,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.platform_vesting_wallet, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.platform_vesting_scale, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.platform_cp_creator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.curve_params, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PlatformConfigAccount(pub PlatformConfig);
impl PlatformConfigAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PLATFORM_CONFIG_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(PlatformConfig::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PLATFORM_CONFIG_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const PLATFORM_GLOBAL_ACCESS_ACCOUNT_DISCM: [u8; 8] = [
    174, 147, 132, 240, 137, 219, 243, 16,
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
pub struct PlatformGlobalAccess {
    pub bump: u8,
    pub global_config: Pubkey,
    pub platform_config: Pubkey,
    pub padding: [u64; 8],
}
impl PlatformGlobalAccess {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let global_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let platform_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u64; 8] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            bump,
            global_config,
            platform_config,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.global_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.platform_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PlatformGlobalAccessAccount(pub PlatformGlobalAccess);
impl PlatformGlobalAccessAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PLATFORM_GLOBAL_ACCESS_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(PlatformGlobalAccess::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PLATFORM_GLOBAL_ACCESS_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const POOL_STATE_ACCOUNT_DISCM: [u8; 8] = [247, 237, 227, 245, 215, 195, 222, 70];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct PoolState {
    pub epoch: u64,
    pub auth_bump: u8,
    pub status: u8,
    pub base_decimals: u8,
    pub quote_decimals: u8,
    pub migrate_type: u8,
    pub supply: u64,
    pub total_base_sell: u64,
    pub virtual_base: u64,
    pub virtual_quote: u64,
    pub real_base: u64,
    pub real_quote: u64,
    pub total_quote_fund_raising: u64,
    pub quote_protocol_fee: u64,
    pub platform_fee: u64,
    pub migrate_fee: u64,
    pub vesting_schedule: VestingSchedule,
    pub global_config: Pubkey,
    pub platform_config: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub creator: Pubkey,
    pub token_program_flag: u8,
    pub amm_creator_fee_on: AmmCreatorFeeOn,
    pub platform_vesting_share: u64,
    #[serde(with = "crate::big_array_serde")]
    pub padding: [u8; 54],
}
impl PoolState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let epoch: u64 = crate::borsh_de_or_default(&mut reader)?;
        let auth_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let status: u8 = crate::borsh_de_or_default(&mut reader)?;
        let base_decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let quote_decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let migrate_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_base_sell: u64 = crate::borsh_de_or_default(&mut reader)?;
        let virtual_base: u64 = crate::borsh_de_or_default(&mut reader)?;
        let virtual_quote: u64 = crate::borsh_de_or_default(&mut reader)?;
        let real_base: u64 = crate::borsh_de_or_default(&mut reader)?;
        let real_quote: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_quote_fund_raising: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_protocol_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let platform_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let migrate_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let vesting_schedule = if reader.is_empty() {
            Default::default()
        } else {
            <VestingSchedule>::deserialize(&mut reader)?
        };
        let global_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let platform_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let quote_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let quote_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token_program_flag: u8 = crate::borsh_de_or_default(&mut reader)?;
        let amm_creator_fee_on: AmmCreatorFeeOn = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let platform_vesting_share: u64 = crate::borsh_de_or_default(&mut reader)?;
        let padding = <[u8; 54] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            epoch,
            auth_bump,
            status,
            base_decimals,
            quote_decimals,
            migrate_type,
            supply,
            total_base_sell,
            virtual_base,
            virtual_quote,
            real_base,
            real_quote,
            total_quote_fund_raising,
            quote_protocol_fee,
            platform_fee,
            migrate_fee,
            vesting_schedule,
            global_config,
            platform_config,
            base_mint,
            quote_mint,
            base_vault,
            quote_vault,
            creator,
            token_program_flag,
            amm_creator_fee_on,
            platform_vesting_share,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.epoch, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.auth_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.status, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_decimals, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_decimals, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.migrate_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.supply, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_base_sell, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.virtual_base, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.virtual_quote, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.real_base, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.real_quote, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_quote_fund_raising, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_protocol_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.platform_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.migrate_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vesting_schedule, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.global_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.platform_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_program_flag, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amm_creator_fee_on, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.platform_vesting_share, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PoolStateAccount(pub PoolState);
impl PoolStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POOL_STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(PoolState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POOL_STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const VESTING_RECORD_ACCOUNT_DISCM: [u8; 8] = [106, 243, 221, 205, 230, 126, 85, 83];
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
pub struct VestingRecord {
    pub epoch: u64,
    pub pool: Pubkey,
    pub beneficiary: Pubkey,
    pub claimed_amount: u64,
    pub token_share_amount: u64,
    pub padding: [u64; 8],
}
impl VestingRecord {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let epoch: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let beneficiary: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let claimed_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_share_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u64; 8] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            epoch,
            pool,
            beneficiary,
            claimed_amount,
            token_share_amount,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.epoch, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.beneficiary, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.claimed_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_share_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct VestingRecordAccount(pub VestingRecord);
impl VestingRecordAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != VESTING_RECORD_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(VestingRecord::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&VESTING_RECORD_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
