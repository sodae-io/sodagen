use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const FUTARCHY_AUTHORITY_ACCOUNT_DISCM: [u8; 8] = [
    175, 247, 160, 182, 140, 128, 211, 226,
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
pub struct FutarchyAuthority {
    pub version: u8,
    pub authority: Pubkey,
    pub recipients: RevenueRecipients,
    pub revenue_share: RevenueShare,
    pub revenue_distribution: RevenueDistribution,
    pub global_reduce_only: bool,
    pub bump: u8,
}
impl FutarchyAuthority {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let version: u8 = crate::borsh_de_or_default(&mut reader)?;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let recipients = if reader.is_empty() {
            Default::default()
        } else {
            <RevenueRecipients>::deserialize(&mut reader)?
        };
        let revenue_share = if reader.is_empty() {
            Default::default()
        } else {
            <RevenueShare>::deserialize(&mut reader)?
        };
        let revenue_distribution = if reader.is_empty() {
            Default::default()
        } else {
            <RevenueDistribution>::deserialize(&mut reader)?
        };
        let global_reduce_only: bool = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            version,
            authority,
            recipients,
            revenue_share,
            revenue_distribution,
            global_reduce_only,
            bump,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.version, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.recipients, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.revenue_share, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.revenue_distribution, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.global_reduce_only, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct FutarchyAuthorityAccount(pub FutarchyAuthority);
impl FutarchyAuthorityAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FUTARCHY_AUTHORITY_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(FutarchyAuthority::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FUTARCHY_AUTHORITY_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const PAIR_ACCOUNT_DISCM: [u8; 8] = [85, 72, 49, 176, 182, 228, 141, 82];
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
pub struct Pair {
    pub token0: Pubkey,
    pub token1: Pubkey,
    pub lp_mint: Pubkey,
    pub rate_model: Pubkey,
    pub swap_fee_bps: u16,
    pub half_life: u64,
    pub fixed_cf_bps: Option<u16>,
    pub reserve0: u64,
    pub reserve1: u64,
    pub cash_reserve0: u64,
    pub cash_reserve1: u64,
    pub last_price0_ema: LastPriceEMA,
    pub last_price1_ema: LastPriceEMA,
    pub last_update: u64,
    pub last_rate0: u64,
    pub last_rate1: u64,
    pub total_debt0: u64,
    pub total_debt1: u64,
    pub total_debt0_shares: u128,
    pub total_debt1_shares: u128,
    pub total_supply: u64,
    pub total_collateral0: u64,
    pub total_collateral1: u64,
    pub token0_decimals: u8,
    pub token1_decimals: u8,
    pub params_hash: [u8; 32],
    pub version: u8,
    pub bump: u8,
    pub vault_bumps: VaultBumps,
    pub reduce_only: bool,
}
impl Pair {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let token0: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token1: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lp_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let rate_model: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let swap_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let half_life: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fixed_cf_bps: Option<u16> = crate::borsh_de_or_default(&mut reader)?;
        let reserve0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reserve1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let cash_reserve0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let cash_reserve1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_price0_ema = if reader.is_empty() {
            Default::default()
        } else {
            <LastPriceEMA>::deserialize(&mut reader)?
        };
        let last_price1_ema = if reader.is_empty() {
            Default::default()
        } else {
            <LastPriceEMA>::deserialize(&mut reader)?
        };
        let last_update: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_rate0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_rate1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_debt0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_debt1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_debt0_shares: u128 = crate::borsh_de_or_default(&mut reader)?;
        let total_debt1_shares: u128 = crate::borsh_de_or_default(&mut reader)?;
        let total_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_collateral0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_collateral1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let token0_decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let token1_decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let params_hash: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        let version: u8 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let vault_bumps = if reader.is_empty() {
            Default::default()
        } else {
            <VaultBumps>::deserialize(&mut reader)?
        };
        let reduce_only: bool = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            token0,
            token1,
            lp_mint,
            rate_model,
            swap_fee_bps,
            half_life,
            fixed_cf_bps,
            reserve0,
            reserve1,
            cash_reserve0,
            cash_reserve1,
            last_price0_ema,
            last_price1_ema,
            last_update,
            last_rate0,
            last_rate1,
            total_debt0,
            total_debt1,
            total_debt0_shares,
            total_debt1_shares,
            total_supply,
            total_collateral0,
            total_collateral1,
            token0_decimals,
            token1_decimals,
            params_hash,
            version,
            bump,
            vault_bumps,
            reduce_only,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.token0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.rate_model, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.swap_fee_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.half_life, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fixed_cf_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserve0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserve1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.cash_reserve0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.cash_reserve1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_price0_ema, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_price1_ema, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_update, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_rate0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_rate1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_debt0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_debt1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_debt0_shares, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_debt1_shares, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_supply, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_collateral0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_collateral1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token0_decimals, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token1_decimals, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.params_hash, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.version, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault_bumps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reduce_only, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PairAccount(pub Pair);
impl PairAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PAIR_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Pair::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PAIR_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const RATE_MODEL_ACCOUNT_DISCM: [u8; 8] = [94, 3, 203, 219, 107, 137, 4, 162];
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
pub struct RateModel {
    pub exp_rate: u64,
    pub target_util_start: u64,
    pub target_util_end: u64,
    pub half_life_ms: u64,
    pub min_rate: u64,
    pub max_rate: u64,
    pub initial_rate: u64,
}
impl RateModel {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let exp_rate: u64 = crate::borsh_de_or_default(&mut reader)?;
        let target_util_start: u64 = crate::borsh_de_or_default(&mut reader)?;
        let target_util_end: u64 = crate::borsh_de_or_default(&mut reader)?;
        let half_life_ms: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_rate: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_rate: u64 = crate::borsh_de_or_default(&mut reader)?;
        let initial_rate: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            exp_rate,
            target_util_start,
            target_util_end,
            half_life_ms,
            min_rate,
            max_rate,
            initial_rate,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.exp_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.target_util_start, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.target_util_end, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.half_life_ms, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.min_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.max_rate, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.initial_rate, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RateModelAccount(pub RateModel);
impl RateModelAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != RATE_MODEL_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(RateModel::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&RATE_MODEL_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const USER_POSITION_ACCOUNT_DISCM: [u8; 8] = [251, 248, 209, 245, 83, 234, 17, 27];
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
pub struct UserPosition {
    pub owner: Pubkey,
    pub pair: Pubkey,
    pub collateral0_liquidation_cf_bps: u16,
    pub collateral1_liquidation_cf_bps: u16,
    pub collateral0: u64,
    pub collateral1: u64,
    pub debt0_shares: u128,
    pub debt1_shares: u128,
    pub bump: u8,
}
impl UserPosition {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pair: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let collateral0_liquidation_cf_bps: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let collateral1_liquidation_cf_bps: u16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let collateral0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let collateral1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let debt0_shares: u128 = crate::borsh_de_or_default(&mut reader)?;
        let debt1_shares: u128 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            owner,
            pair,
            collateral0_liquidation_cf_bps,
            collateral1_liquidation_cf_bps,
            collateral0,
            collateral1,
            debt0_shares,
            debt1_shares,
            bump,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pair, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.collateral0_liquidation_cf_bps,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.collateral1_liquidation_cf_bps,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.collateral0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.debt0_shares, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.debt1_shares, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UserPositionAccount(pub UserPosition);
impl UserPositionAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != USER_POSITION_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(UserPosition::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&USER_POSITION_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
