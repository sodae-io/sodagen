use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const HYLO_ACCOUNT_DISCM: [u8; 8] = [114, 161, 169, 210, 204, 175, 149, 174];
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
pub struct Hylo {
    pub admin: Pubkey,
    pub treasury: Pubkey,
    pub lst_registry: Pubkey,
    pub stablecoin_mint: Pubkey,
    pub levercoin_mint: Pubkey,
    pub pause_authority: Pubkey,
    pub stablecoin_mint_bump: u8,
    pub stablecoin_auth_bump: u8,
    pub levercoin_mint_bump: u8,
    pub levercoin_auth_bump: u8,
    pub registry_auth_bump: u8,
    pub total_sol_cache_bump: u8,
    pub oracle_interval_secs: u64,
    pub stablecoin_fees: StablecoinFees,
    pub levercoin_fees: LevercoinFees,
    pub total_sol_cache: TotalSolCache,
    pub yield_harvest_cache: HarvestCache,
    pub yield_harvest_config: YieldHarvestConfig,
    pub stablecoin_mint_threshold: UFixValue64,
    pub _unused_1: UFixValue64,
    pub oracle_conf_tolerance: UFixValue64,
    pub sol_usd_oracle: Pubkey,
    pub lst_swap_fee: UFixValue64,
    pub virtual_stablecoin: VirtualStablecoin,
    pub lst_buy_curve_config: RebalanceCurveConfig,
    pub lst_sell_curve_config: RebalanceCurveConfig,
    pub protocol_paused: bool,
    pub lst_pair_paused: bool,
    pub _unused_2: UFixValue64,
    pub pool_drawdown: PoolDrawdown,
    pub _reserved: [u8; 13],
}
impl Hylo {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let treasury: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lst_registry: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let stablecoin_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let levercoin_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pause_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let stablecoin_mint_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let stablecoin_auth_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let levercoin_mint_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let levercoin_auth_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let registry_auth_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let total_sol_cache_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let oracle_interval_secs: u64 = crate::borsh_de_or_default(&mut reader)?;
        let stablecoin_fees = if reader.is_empty() {
            Default::default()
        } else {
            <StablecoinFees>::deserialize(&mut reader)?
        };
        let levercoin_fees = if reader.is_empty() {
            Default::default()
        } else {
            <LevercoinFees>::deserialize(&mut reader)?
        };
        let total_sol_cache = if reader.is_empty() {
            Default::default()
        } else {
            <TotalSolCache>::deserialize(&mut reader)?
        };
        let yield_harvest_cache = if reader.is_empty() {
            Default::default()
        } else {
            <HarvestCache>::deserialize(&mut reader)?
        };
        let yield_harvest_config = if reader.is_empty() {
            Default::default()
        } else {
            <YieldHarvestConfig>::deserialize(&mut reader)?
        };
        let stablecoin_mint_threshold = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let _unused_1 = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let oracle_conf_tolerance = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let sol_usd_oracle: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lst_swap_fee = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let virtual_stablecoin = if reader.is_empty() {
            Default::default()
        } else {
            <VirtualStablecoin>::deserialize(&mut reader)?
        };
        let lst_buy_curve_config = if reader.is_empty() {
            Default::default()
        } else {
            <RebalanceCurveConfig>::deserialize(&mut reader)?
        };
        let lst_sell_curve_config = if reader.is_empty() {
            Default::default()
        } else {
            <RebalanceCurveConfig>::deserialize(&mut reader)?
        };
        let protocol_paused: bool = crate::borsh_de_or_default(&mut reader)?;
        let lst_pair_paused: bool = crate::borsh_de_or_default(&mut reader)?;
        let _unused_2 = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let pool_drawdown = if reader.is_empty() {
            Default::default()
        } else {
            <PoolDrawdown>::deserialize(&mut reader)?
        };
        let _reserved: [u8; 13] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            admin,
            treasury,
            lst_registry,
            stablecoin_mint,
            levercoin_mint,
            pause_authority,
            stablecoin_mint_bump,
            stablecoin_auth_bump,
            levercoin_mint_bump,
            levercoin_auth_bump,
            registry_auth_bump,
            total_sol_cache_bump,
            oracle_interval_secs,
            stablecoin_fees,
            levercoin_fees,
            total_sol_cache,
            yield_harvest_cache,
            yield_harvest_config,
            stablecoin_mint_threshold,
            _unused_1,
            oracle_conf_tolerance,
            sol_usd_oracle,
            lst_swap_fee,
            virtual_stablecoin,
            lst_buy_curve_config,
            lst_sell_curve_config,
            protocol_paused,
            lst_pair_paused,
            _unused_2,
            pool_drawdown,
            _reserved,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.treasury, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lst_registry, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stablecoin_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.levercoin_mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pause_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stablecoin_mint_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stablecoin_auth_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.levercoin_mint_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.levercoin_auth_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.registry_auth_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_sol_cache_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.oracle_interval_secs, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stablecoin_fees, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.levercoin_fees, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_sol_cache, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.yield_harvest_cache, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.yield_harvest_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.stablecoin_mint_threshold, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._unused_1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.oracle_conf_tolerance, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.sol_usd_oracle, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lst_swap_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.virtual_stablecoin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lst_buy_curve_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lst_sell_curve_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.protocol_paused, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lst_pair_paused, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._unused_2, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_drawdown, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._reserved, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct HyloAccount(pub Hylo);
impl HyloAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != HYLO_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Hylo::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&HYLO_ACCOUNT_DISCM)?;
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
    pub _dead_admin: Pubkey,
    pub pool_auth_bump: u8,
    pub lp_token_auth_bump: u8,
    pub lp_token_mint_bump: u8,
    pub withdrawal_fee: UFixValue64,
    pub paused: bool,
    pub withdrawal_limiter: WithdrawalLimiter,
    pub deposit_limiter: DepositLimiter,
    pub _reserved: [u8; 19],
}
impl PoolConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let _dead_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pool_auth_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let lp_token_auth_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let lp_token_mint_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let withdrawal_fee = if reader.is_empty() {
            Default::default()
        } else {
            <UFixValue64>::deserialize(&mut reader)?
        };
        let paused: bool = crate::borsh_de_or_default(&mut reader)?;
        let withdrawal_limiter = if reader.is_empty() {
            Default::default()
        } else {
            <WithdrawalLimiter>::deserialize(&mut reader)?
        };
        let deposit_limiter = if reader.is_empty() {
            Default::default()
        } else {
            <DepositLimiter>::deserialize(&mut reader)?
        };
        let _reserved: [u8; 19] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            _dead_admin,
            pool_auth_bump,
            lp_token_auth_bump,
            lp_token_mint_bump,
            withdrawal_fee,
            paused,
            withdrawal_limiter,
            deposit_limiter,
            _reserved,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self._dead_admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pool_auth_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_token_auth_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lp_token_mint_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.withdrawal_fee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.paused, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.withdrawal_limiter, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.deposit_limiter, &mut writer)?;
        borsh::BorshSerialize::serialize(&self._reserved, &mut writer)?;
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
