use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const BONDING_CURVE_ACCOUNT_DISCM: [u8; 8] = [23, 183, 248, 55, 96, 216, 172, 96];
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
pub struct BondingCurve {
    pub virtual_token_reserves: u64,
    pub virtual_sol_reserves: u64,
    pub real_token_reserves: u64,
    pub real_sol_reserves: u64,
    pub token_total_supply: u64,
    pub complete: bool,
    pub creator: Pubkey,
    pub is_mayhem_mode: bool,
    pub is_cashback_coin: bool,
}
impl BondingCurve {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let virtual_token_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let virtual_sol_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let real_token_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let real_sol_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token_total_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let complete: bool = crate::borsh_de_or_default(&mut reader)?;
        let creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let is_mayhem_mode: bool = crate::borsh_de_or_default(&mut reader)?;
        let is_cashback_coin: bool = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            virtual_token_reserves,
            virtual_sol_reserves,
            real_token_reserves,
            real_sol_reserves,
            token_total_supply,
            complete,
            creator,
            is_mayhem_mode,
            is_cashback_coin,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.virtual_token_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.virtual_sol_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.real_token_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.real_sol_reserves, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token_total_supply, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.complete, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.is_mayhem_mode, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.is_cashback_coin, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BondingCurveAccount(pub BondingCurve);
impl BondingCurveAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BONDING_CURVE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(BondingCurve::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BONDING_CURVE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const FEE_CONFIG_ACCOUNT_DISCM: [u8; 8] = [143, 52, 146, 187, 219, 123, 76, 155];
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
pub struct FeeConfig {
    pub bump: u8,
    pub admin: Pubkey,
    pub flat_fees: Fees,
    pub fee_tiers: Vec<FeeTier>,
    pub stable_fee_tiers: Vec<FeeTier>,
}
impl FeeConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let flat_fees = if reader.is_empty() {
            Default::default()
        } else {
            <Fees>::deserialize(&mut reader)?
        };
        let fee_tiers: Vec<FeeTier> = crate::borsh_de_or_default(&mut reader)?;
        let stable_fee_tiers: Vec<FeeTier> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            bump,
            admin,
            flat_fees,
            fee_tiers,
            stable_fee_tiers,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.flat_fees, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_tiers, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stable_fee_tiers, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct FeeConfigAccount(pub FeeConfig);
impl FeeConfigAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FEE_CONFIG_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(FeeConfig::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FEE_CONFIG_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
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
    pub admin: Pubkey,
    pub lp_fee_basis_points: u64,
    pub protocol_fee_basis_points: u64,
    pub disable_flags: u8,
    pub protocol_fee_recipients: [Pubkey; 8],
    pub coin_creator_fee_basis_points: u64,
    pub admin_set_coin_creator_authority: Pubkey,
    pub whitelist_pda: Pubkey,
    pub reserved_fee_recipient: Pubkey,
    pub mayhem_mode_enabled: bool,
    pub reserved_fee_recipients: [Pubkey; 7],
    pub is_cashback_enabled: bool,
    pub buyback_fee_recipients: [Pubkey; 8],
    pub buyback_basis_points: u64,
    pub boost_authority: Pubkey,
    pub boost_enabled: bool,
}
impl GlobalConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lp_fee_basis_points: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee_basis_points: u64 = crate::borsh_de_or_default(&mut reader)?;
        let disable_flags: u8 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee_recipients: [Pubkey; 8] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let coin_creator_fee_basis_points: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let admin_set_coin_creator_authority: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let whitelist_pda: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let reserved_fee_recipient: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mayhem_mode_enabled: bool = crate::borsh_de_or_default(&mut reader)?;
        let reserved_fee_recipients: [Pubkey; 7] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let is_cashback_enabled: bool = crate::borsh_de_or_default(&mut reader)?;
        let buyback_fee_recipients: [Pubkey; 8] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let buyback_basis_points: u64 = crate::borsh_de_or_default(&mut reader)?;
        let boost_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let boost_enabled: bool = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            admin,
            lp_fee_basis_points,
            protocol_fee_basis_points,
            disable_flags,
            protocol_fee_recipients,
            coin_creator_fee_basis_points,
            admin_set_coin_creator_authority,
            whitelist_pda,
            reserved_fee_recipient,
            mayhem_mode_enabled,
            reserved_fee_recipients,
            is_cashback_enabled,
            buyback_fee_recipients,
            buyback_basis_points,
            boost_authority,
            boost_enabled,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_fee_basis_points, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fee_basis_points, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.disable_flags, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_fee_recipients, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.coin_creator_fee_basis_points,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.admin_set_coin_creator_authority,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.whitelist_pda, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserved_fee_recipient, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mayhem_mode_enabled, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserved_fee_recipients, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.is_cashback_enabled, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.buyback_fee_recipients, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.buyback_basis_points, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.boost_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.boost_enabled, &mut writer)?;
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
pub const GLOBAL_VOLUME_ACCUMULATOR_ACCOUNT_DISCM: [u8; 8] = [
    202, 42, 246, 43, 142, 190, 30, 255,
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
pub struct GlobalVolumeAccumulator {
    pub start_time: i64,
    pub end_time: i64,
    pub seconds_in_a_day: i64,
    pub mint: Pubkey,
    pub total_token_supply: [u64; 30],
    pub sol_volumes: [u64; 30],
}
impl GlobalVolumeAccumulator {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let start_time: i64 = crate::borsh_de_or_default(&mut reader)?;
        let end_time: i64 = crate::borsh_de_or_default(&mut reader)?;
        let seconds_in_a_day: i64 = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let total_token_supply: [u64; 30] = crate::borsh_de_or_default(&mut reader)?;
        let sol_volumes: [u64; 30] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            start_time,
            end_time,
            seconds_in_a_day,
            mint,
            total_token_supply,
            sol_volumes,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.start_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.end_time, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.seconds_in_a_day, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_token_supply, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sol_volumes, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct GlobalVolumeAccumulatorAccount(pub GlobalVolumeAccumulator);
impl GlobalVolumeAccumulatorAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != GLOBAL_VOLUME_ACCUMULATOR_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(GlobalVolumeAccumulator::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&GLOBAL_VOLUME_ACCUMULATOR_ACCOUNT_DISCM)?;
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
    pub pool_bump: u8,
    pub index: u16,
    pub creator: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub lp_mint: Pubkey,
    pub pool_base_token_account: Pubkey,
    pub pool_quote_token_account: Pubkey,
    pub lp_supply: u64,
    pub coin_creator: Pubkey,
    pub is_mayhem_mode: bool,
    pub is_cashback_coin: bool,
    pub virtual_quote_reserves: i128,
}
impl Pool {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let quote_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lp_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_base_token_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_quote_token_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lp_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let coin_creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let is_mayhem_mode: bool = crate::borsh_de_or_default(&mut reader)?;
        let is_cashback_coin: bool = crate::borsh_de_or_default(&mut reader)?;
        let virtual_quote_reserves: i128 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pool_bump,
            index,
            creator,
            base_mint,
            quote_mint,
            lp_mint,
            pool_base_token_account,
            pool_quote_token_account,
            lp_supply,
            coin_creator,
            is_mayhem_mode,
            is_cashback_coin,
            virtual_quote_reserves,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.pool_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.index, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.creator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.base_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.quote_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_base_token_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_quote_token_account, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_supply, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.coin_creator, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.is_mayhem_mode, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.is_cashback_coin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.virtual_quote_reserves, &mut writer)?;
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
pub const SHARING_CONFIG_ACCOUNT_DISCM: [u8; 8] = [216, 74, 9, 0, 56, 140, 93, 75];
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
pub struct SharingConfig {
    pub bump: u8,
    pub version: u8,
    pub status: ConfigStatus,
    pub mint: Pubkey,
    pub admin: Pubkey,
    pub admin_revoked: bool,
    pub shareholders: Vec<Shareholder>,
}
impl SharingConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let version: u8 = crate::borsh_de_or_default(&mut reader)?;
        let status: ConfigStatus = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let admin_revoked: bool = crate::borsh_de_or_default(&mut reader)?;
        let shareholders: Vec<Shareholder> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            bump,
            version,
            status,
            mint,
            admin,
            admin_revoked,
            shareholders,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.version, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.status, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.admin_revoked, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.shareholders, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SharingConfigAccount(pub SharingConfig);
impl SharingConfigAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SHARING_CONFIG_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(SharingConfig::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SHARING_CONFIG_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const USER_VOLUME_ACCUMULATOR_ACCOUNT_DISCM: [u8; 8] = [
    86, 255, 112, 14, 102, 53, 154, 250,
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
pub struct UserVolumeAccumulator {
    pub user: Pubkey,
    pub needs_claim: bool,
    pub total_unclaimed_tokens: u64,
    pub total_claimed_tokens: u64,
    pub current_sol_volume: u64,
    pub last_update_timestamp: i64,
    pub has_total_claimed_tokens: bool,
    pub cashback_earned: u64,
    pub total_cashback_claimed: u64,
}
impl UserVolumeAccumulator {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let user: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let needs_claim: bool = crate::borsh_de_or_default(&mut reader)?;
        let total_unclaimed_tokens: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_claimed_tokens: u64 = crate::borsh_de_or_default(&mut reader)?;
        let current_sol_volume: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_update_timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        let has_total_claimed_tokens: bool = crate::borsh_de_or_default(&mut reader)?;
        let cashback_earned: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_cashback_claimed: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            user,
            needs_claim,
            total_unclaimed_tokens,
            total_claimed_tokens,
            current_sol_volume,
            last_update_timestamp,
            has_total_claimed_tokens,
            cashback_earned,
            total_cashback_claimed,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.user, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.needs_claim, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_unclaimed_tokens, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_claimed_tokens, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.current_sol_volume, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_update_timestamp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.has_total_claimed_tokens, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.cashback_earned, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_cashback_claimed, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UserVolumeAccumulatorAccount(pub UserVolumeAccumulator);
impl UserVolumeAccumulatorAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != USER_VOLUME_ACCUMULATOR_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(UserVolumeAccumulator::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&USER_VOLUME_ACCUMULATOR_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
