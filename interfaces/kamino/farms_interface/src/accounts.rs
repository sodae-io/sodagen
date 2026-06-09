use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
pub const FARM_STATE_ACCOUNT_DISCM: [u8; 8] = [198, 102, 216, 74, 63, 66, 163, 190];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct FarmState {
    pub farm_admin: Pubkey,
    pub global_config: Pubkey,
    pub token: TokenInfo,
    pub reward_infos: [RewardInfo; 10],
    pub num_reward_tokens: u64,
    pub num_users: u64,
    pub total_staked_amount: u64,
    pub farm_vault: Pubkey,
    pub farm_vaults_authority: Pubkey,
    pub farm_vaults_authority_bump: u64,
    pub delegate_authority: Pubkey,
    pub time_unit: u8,
    pub is_farm_frozen: u8,
    pub is_farm_delegated: u8,
    pub is_reward_user_once_enabled: u8,
    pub is_harvesting_permissionless: u8,
    pub padding0: [u8; 3],
    pub withdraw_authority: Pubkey,
    pub deposit_warmup_period: u32,
    pub withdrawal_cooldown_period: u32,
    pub total_active_stake_scaled: u128,
    pub total_pending_stake_scaled: u128,
    pub total_pending_amount: u64,
    pub slashed_amount_current: u64,
    pub slashed_amount_cumulative: u64,
    pub slashed_amount_spill_address: Pubkey,
    pub locking_mode: u64,
    pub locking_start_timestamp: u64,
    pub locking_duration: u64,
    pub locking_early_withdrawal_penalty_bps: u64,
    pub deposit_cap_amount: u64,
    pub scope_prices: Pubkey,
    pub scope_oracle_price_id: u64,
    pub scope_oracle_max_age: u64,
    pub pending_farm_admin: Pubkey,
    pub strategy_id: Pubkey,
    pub delegated_rps_admin: Pubkey,
    pub vault_id: Pubkey,
    pub second_delegated_authority: Pubkey,
    #[serde(with = "crate::big_array_serde")]
    pub padding: [u64; 74],
}
impl FarmState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let farm_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let global_config: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let token = if reader.is_empty() {
            Default::default()
        } else {
            <TokenInfo>::deserialize(&mut reader)?
        };
        let reward_infos: [RewardInfo; 10] = crate::borsh_de_or_default(&mut reader)?;
        let num_reward_tokens: u64 = crate::borsh_de_or_default(&mut reader)?;
        let num_users: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_staked_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let farm_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let farm_vaults_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let farm_vaults_authority_bump: u64 = crate::borsh_de_or_default(&mut reader)?;
        let delegate_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let time_unit: u8 = crate::borsh_de_or_default(&mut reader)?;
        let is_farm_frozen: u8 = crate::borsh_de_or_default(&mut reader)?;
        let is_farm_delegated: u8 = crate::borsh_de_or_default(&mut reader)?;
        let is_reward_user_once_enabled: u8 = crate::borsh_de_or_default(&mut reader)?;
        let is_harvesting_permissionless: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding0: [u8; 3] = crate::borsh_de_or_default(&mut reader)?;
        let withdraw_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let deposit_warmup_period: u32 = crate::borsh_de_or_default(&mut reader)?;
        let withdrawal_cooldown_period: u32 = crate::borsh_de_or_default(&mut reader)?;
        let total_active_stake_scaled: u128 = crate::borsh_de_or_default(&mut reader)?;
        let total_pending_stake_scaled: u128 = crate::borsh_de_or_default(&mut reader)?;
        let total_pending_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let slashed_amount_current: u64 = crate::borsh_de_or_default(&mut reader)?;
        let slashed_amount_cumulative: u64 = crate::borsh_de_or_default(&mut reader)?;
        let slashed_amount_spill_address: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let locking_mode: u64 = crate::borsh_de_or_default(&mut reader)?;
        let locking_start_timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        let locking_duration: u64 = crate::borsh_de_or_default(&mut reader)?;
        let locking_early_withdrawal_penalty_bps: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let deposit_cap_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let scope_prices: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let scope_oracle_price_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        let scope_oracle_max_age: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pending_farm_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let strategy_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let delegated_rps_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let second_delegated_authority: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let padding = <[u64; 74] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            farm_admin,
            global_config,
            token,
            reward_infos,
            num_reward_tokens,
            num_users,
            total_staked_amount,
            farm_vault,
            farm_vaults_authority,
            farm_vaults_authority_bump,
            delegate_authority,
            time_unit,
            is_farm_frozen,
            is_farm_delegated,
            is_reward_user_once_enabled,
            is_harvesting_permissionless,
            padding0,
            withdraw_authority,
            deposit_warmup_period,
            withdrawal_cooldown_period,
            total_active_stake_scaled,
            total_pending_stake_scaled,
            total_pending_amount,
            slashed_amount_current,
            slashed_amount_cumulative,
            slashed_amount_spill_address,
            locking_mode,
            locking_start_timestamp,
            locking_duration,
            locking_early_withdrawal_penalty_bps,
            deposit_cap_amount,
            scope_prices,
            scope_oracle_price_id,
            scope_oracle_max_age,
            pending_farm_admin,
            strategy_id,
            delegated_rps_admin,
            vault_id,
            second_delegated_authority,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.farm_admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.global_config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.token, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reward_infos, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.num_reward_tokens, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.num_users, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_staked_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.farm_vault, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.farm_vaults_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.farm_vaults_authority_bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.delegate_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.time_unit, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.is_farm_frozen, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.is_farm_delegated, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.is_reward_user_once_enabled,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.is_harvesting_permissionless,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.padding0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.withdraw_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.deposit_warmup_period, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.withdrawal_cooldown_period, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_active_stake_scaled, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_pending_stake_scaled, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.total_pending_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.slashed_amount_current, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.slashed_amount_cumulative, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.slashed_amount_spill_address,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.locking_mode, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.locking_start_timestamp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.locking_duration, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.locking_early_withdrawal_penalty_bps,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.deposit_cap_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.scope_prices, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.scope_oracle_price_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.scope_oracle_max_age, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pending_farm_admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.strategy_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.delegated_rps_admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.vault_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.second_delegated_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct FarmStateAccount(pub FarmState);
impl FarmStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FARM_STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(FarmState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FARM_STATE_ACCOUNT_DISCM)?;
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
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct GlobalConfig {
    pub global_admin: Pubkey,
    pub treasury_fee_bps: u64,
    pub treasury_vaults_authority: Pubkey,
    pub treasury_vaults_authority_bump: u64,
    pub pending_global_admin: Pubkey,
    #[serde(with = "crate::big_array_serde")]
    pub padding1: [u128; 126],
}
impl GlobalConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let global_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let treasury_fee_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let treasury_vaults_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let treasury_vaults_authority_bump: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let pending_global_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let padding1 = <[u128; 126] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            global_admin,
            treasury_fee_bps,
            treasury_vaults_authority,
            treasury_vaults_authority_bump,
            pending_global_admin,
            padding1,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.global_admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.treasury_fee_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.treasury_vaults_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.treasury_vaults_authority_bump,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.pending_global_admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding1, &mut writer)?;
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
pub const USER_STATE_ACCOUNT_DISCM: [u8; 8] = [72, 177, 85, 249, 76, 167, 186, 126];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct UserState {
    pub user_id: u64,
    pub farm_state: Pubkey,
    pub owner: Pubkey,
    pub is_farm_delegated: u8,
    pub padding0: [u8; 7],
    pub rewards_tally_scaled: [u128; 10],
    pub rewards_issued_unclaimed: [u64; 10],
    pub last_claim_ts: [u64; 10],
    pub active_stake_scaled: u128,
    pub pending_deposit_stake_scaled: u128,
    pub pending_deposit_stake_ts: u64,
    pub pending_withdrawal_unstake_scaled: u128,
    pub pending_withdrawal_unstake_ts: u64,
    pub bump: u64,
    pub delegatee: Pubkey,
    pub last_stake_ts: u64,
    pub rewards_issued_cumulative: [u64; 10],
    #[serde(with = "crate::big_array_serde")]
    pub padding1: [u64; 40],
}
impl UserState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let user_id: u64 = crate::borsh_de_or_default(&mut reader)?;
        let farm_state: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let is_farm_delegated: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding0: [u8; 7] = crate::borsh_de_or_default(&mut reader)?;
        let rewards_tally_scaled: [u128; 10] = crate::borsh_de_or_default(&mut reader)?;
        let rewards_issued_unclaimed: [u64; 10] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let last_claim_ts: [u64; 10] = crate::borsh_de_or_default(&mut reader)?;
        let active_stake_scaled: u128 = crate::borsh_de_or_default(&mut reader)?;
        let pending_deposit_stake_scaled: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let pending_deposit_stake_ts: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pending_withdrawal_unstake_scaled: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let pending_withdrawal_unstake_ts: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let bump: u64 = crate::borsh_de_or_default(&mut reader)?;
        let delegatee: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let last_stake_ts: u64 = crate::borsh_de_or_default(&mut reader)?;
        let rewards_issued_cumulative: [u64; 10] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let padding1 = <[u64; 40] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            user_id,
            farm_state,
            owner,
            is_farm_delegated,
            padding0,
            rewards_tally_scaled,
            rewards_issued_unclaimed,
            last_claim_ts,
            active_stake_scaled,
            pending_deposit_stake_scaled,
            pending_deposit_stake_ts,
            pending_withdrawal_unstake_scaled,
            pending_withdrawal_unstake_ts,
            bump,
            delegatee,
            last_stake_ts,
            rewards_issued_cumulative,
            padding1,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.user_id, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.farm_state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.is_farm_delegated, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding0, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.rewards_tally_scaled, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.rewards_issued_unclaimed, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_claim_ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.active_stake_scaled, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.pending_deposit_stake_scaled,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.pending_deposit_stake_ts, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.pending_withdrawal_unstake_scaled,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.pending_withdrawal_unstake_ts,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.delegatee, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_stake_ts, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.rewards_issued_cumulative, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding1, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UserStateAccount(pub UserState);
impl UserStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != USER_STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(UserState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&USER_STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const ORACLE_PRICES_ACCOUNT_DISCM: [u8; 8] = [89, 128, 118, 221, 6, 72, 180, 146];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct OraclePrices {
    pub oracle_mappings: Pubkey,
    #[serde(with = "crate::big_array_serde")]
    pub prices: [DatedPrice; 512],
}
impl OraclePrices {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let oracle_mappings: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let prices = <[DatedPrice; 512] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self { oracle_mappings, prices })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.oracle_mappings, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.prices, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct OraclePricesAccount(pub OraclePrices);
impl OraclePricesAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ORACLE_PRICES_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(OraclePrices::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ORACLE_PRICES_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
