use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;
#[allow(unused_imports)]
use crate::*;
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
    #[serde(with = "crate::big_array_serde")]
    pub padding1: [u64; 50],
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
        let padding1 = <[u64; 50] as borsh::BorshDeserialize>::deserialize_reader(
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
    pub pending_admin: Pubkey,
    pub fee_collector: Pubkey,
    #[serde(with = "crate::big_array_serde")]
    pub padding: [u8; 928],
}
impl GlobalConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let global_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pending_admin: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_collector: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let padding = <[u8; 928] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            global_admin,
            pending_admin,
            fee_collector,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.global_admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pending_admin, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.fee_collector, &mut writer)?;
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
pub const LENDING_MARKET_ACCOUNT_DISCM: [u8; 8] = [246, 114, 50, 98, 72, 157, 28, 120];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct LendingMarket {
    pub version: u64,
    pub bump_seed: u64,
    pub lending_market_owner: Pubkey,
    pub lending_market_owner_cached: Pubkey,
    pub quote_currency: [u8; 32],
    pub referral_fee_bps: u16,
    pub emergency_mode: u8,
    pub autodeleverage_enabled: u8,
    pub borrow_disabled: u8,
    pub price_refresh_trigger_to_max_age_pct: u8,
    pub liquidation_max_debt_close_factor_pct: u8,
    pub insolvency_risk_unhealthy_ltv_pct: u8,
    pub min_full_liquidation_value_threshold: u64,
    pub max_liquidatable_debt_market_value_at_once: u64,
    pub reserved0: [u8; 8],
    pub global_allowed_borrow_value: u64,
    pub emergency_council: Pubkey,
    pub reserved1: [u8; 8],
    pub elevation_groups: [ElevationGroup; 32],
    #[serde(with = "crate::big_array_serde")]
    pub elevation_group_padding: [u64; 90],
    pub min_net_value_in_obligation_sf: u128,
    pub min_value_skip_liquidation_ltv_checks: u64,
    pub name: [u8; 32],
    pub min_value_skip_liquidation_bf_checks: u64,
    pub individual_autodeleverage_margin_call_period_secs: u64,
    pub min_initial_deposit_amount: u64,
    pub obligation_order_execution_enabled: u8,
    pub immutable: u8,
    pub obligation_order_creation_enabled: u8,
    pub price_triggered_liquidation_disabled: u8,
    pub mature_reserve_debt_liquidation_enabled: u8,
    pub obligation_borrow_debt_term_liquidation_enabled: u8,
    pub borrow_order_creation_enabled: u8,
    pub borrow_order_execution_enabled: u8,
    pub proposer_authority: Pubkey,
    pub min_borrow_order_fill_value: u64,
    pub withdraw_ticket_issuance_enabled: u8,
    pub withdraw_ticket_redemption_enabled: u8,
    pub obligation_borrow_rollover_configuration_enabled: u8,
    pub obligation_borrow_migration_to_fixed_execution_enabled: u8,
    pub withdraw_ticket_cancellation_enabled: u8,
    pub padding2: [u8; 1],
    pub reserve_rewards_max_apr_bps: u16,
    pub min_withdraw_queued_liquidity_value: u64,
    pub fixed_term_rollover_window_duration_seconds: u64,
    pub open_term_rollover_window_duration_seconds: u64,
    pub min_partial_rollover_value: u64,
    pub term_based_full_liquidation_duration_secs: u64,
    pub permissioning_authority: Pubkey,
    pub permissioned_ops: u64,
    #[serde(with = "crate::big_array_serde")]
    pub padding1: [u64; 153],
}
impl LendingMarket {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let version: u64 = crate::borsh_de_or_default(&mut reader)?;
        let bump_seed: u64 = crate::borsh_de_or_default(&mut reader)?;
        let lending_market_owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let lending_market_owner_cached: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let quote_currency: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        let referral_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let emergency_mode: u8 = crate::borsh_de_or_default(&mut reader)?;
        let autodeleverage_enabled: u8 = crate::borsh_de_or_default(&mut reader)?;
        let borrow_disabled: u8 = crate::borsh_de_or_default(&mut reader)?;
        let price_refresh_trigger_to_max_age_pct: u8 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let liquidation_max_debt_close_factor_pct: u8 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let insolvency_risk_unhealthy_ltv_pct: u8 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let min_full_liquidation_value_threshold: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let max_liquidatable_debt_market_value_at_once: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let reserved0: [u8; 8] = crate::borsh_de_or_default(&mut reader)?;
        let global_allowed_borrow_value: u64 = crate::borsh_de_or_default(&mut reader)?;
        let emergency_council: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let reserved1: [u8; 8] = crate::borsh_de_or_default(&mut reader)?;
        let elevation_groups: [ElevationGroup; 32] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let elevation_group_padding = <[u64; 90] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let min_net_value_in_obligation_sf: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let min_value_skip_liquidation_ltv_checks: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let name: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        let min_value_skip_liquidation_bf_checks: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let individual_autodeleverage_margin_call_period_secs: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let min_initial_deposit_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let obligation_order_execution_enabled: u8 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let immutable: u8 = crate::borsh_de_or_default(&mut reader)?;
        let obligation_order_creation_enabled: u8 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let price_triggered_liquidation_disabled: u8 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let mature_reserve_debt_liquidation_enabled: u8 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let obligation_borrow_debt_term_liquidation_enabled: u8 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let borrow_order_creation_enabled: u8 = crate::borsh_de_or_default(&mut reader)?;
        let borrow_order_execution_enabled: u8 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let proposer_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let min_borrow_order_fill_value: u64 = crate::borsh_de_or_default(&mut reader)?;
        let withdraw_ticket_issuance_enabled: u8 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let withdraw_ticket_redemption_enabled: u8 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let obligation_borrow_rollover_configuration_enabled: u8 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let obligation_borrow_migration_to_fixed_execution_enabled: u8 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let withdraw_ticket_cancellation_enabled: u8 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let padding2: [u8; 1] = crate::borsh_de_or_default(&mut reader)?;
        let reserve_rewards_max_apr_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let min_withdraw_queued_liquidity_value: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let fixed_term_rollover_window_duration_seconds: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let open_term_rollover_window_duration_seconds: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let min_partial_rollover_value: u64 = crate::borsh_de_or_default(&mut reader)?;
        let term_based_full_liquidation_duration_secs: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let permissioning_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let permissioned_ops: u64 = crate::borsh_de_or_default(&mut reader)?;
        let padding1 = <[u64; 153] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            version,
            bump_seed,
            lending_market_owner,
            lending_market_owner_cached,
            quote_currency,
            referral_fee_bps,
            emergency_mode,
            autodeleverage_enabled,
            borrow_disabled,
            price_refresh_trigger_to_max_age_pct,
            liquidation_max_debt_close_factor_pct,
            insolvency_risk_unhealthy_ltv_pct,
            min_full_liquidation_value_threshold,
            max_liquidatable_debt_market_value_at_once,
            reserved0,
            global_allowed_borrow_value,
            emergency_council,
            reserved1,
            elevation_groups,
            elevation_group_padding,
            min_net_value_in_obligation_sf,
            min_value_skip_liquidation_ltv_checks,
            name,
            min_value_skip_liquidation_bf_checks,
            individual_autodeleverage_margin_call_period_secs,
            min_initial_deposit_amount,
            obligation_order_execution_enabled,
            immutable,
            obligation_order_creation_enabled,
            price_triggered_liquidation_disabled,
            mature_reserve_debt_liquidation_enabled,
            obligation_borrow_debt_term_liquidation_enabled,
            borrow_order_creation_enabled,
            borrow_order_execution_enabled,
            proposer_authority,
            min_borrow_order_fill_value,
            withdraw_ticket_issuance_enabled,
            withdraw_ticket_redemption_enabled,
            obligation_borrow_rollover_configuration_enabled,
            obligation_borrow_migration_to_fixed_execution_enabled,
            withdraw_ticket_cancellation_enabled,
            padding2,
            reserve_rewards_max_apr_bps,
            min_withdraw_queued_liquidity_value,
            fixed_term_rollover_window_duration_seconds,
            open_term_rollover_window_duration_seconds,
            min_partial_rollover_value,
            term_based_full_liquidation_duration_secs,
            permissioning_authority,
            permissioned_ops,
            padding1,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.version, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump_seed, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lending_market_owner, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.lending_market_owner_cached,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.quote_currency, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.referral_fee_bps, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.emergency_mode, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.autodeleverage_enabled, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.borrow_disabled, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.price_refresh_trigger_to_max_age_pct,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.liquidation_max_debt_close_factor_pct,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.insolvency_risk_unhealthy_ltv_pct,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.min_full_liquidation_value_threshold,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.max_liquidatable_debt_market_value_at_once,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.reserved0, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.global_allowed_borrow_value,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.emergency_council, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserved1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.elevation_groups, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.elevation_group_padding, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.min_net_value_in_obligation_sf,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.min_value_skip_liquidation_ltv_checks,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.name, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.min_value_skip_liquidation_bf_checks,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.individual_autodeleverage_margin_call_period_secs,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.min_initial_deposit_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.obligation_order_execution_enabled,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.immutable, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.obligation_order_creation_enabled,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.price_triggered_liquidation_disabled,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.mature_reserve_debt_liquidation_enabled,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.obligation_borrow_debt_term_liquidation_enabled,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.borrow_order_creation_enabled,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.borrow_order_execution_enabled,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.proposer_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.min_borrow_order_fill_value,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.withdraw_ticket_issuance_enabled,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.withdraw_ticket_redemption_enabled,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.obligation_borrow_rollover_configuration_enabled,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.obligation_borrow_migration_to_fixed_execution_enabled,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.withdraw_ticket_cancellation_enabled,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.padding2, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.reserve_rewards_max_apr_bps,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.min_withdraw_queued_liquidity_value,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.fixed_term_rollover_window_duration_seconds,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.open_term_rollover_window_duration_seconds,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.min_partial_rollover_value, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.term_based_full_liquidation_duration_secs,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.permissioning_authority, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.permissioned_ops, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding1, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LendingMarketAccount(pub LendingMarket);
impl LendingMarketAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_MARKET_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(LendingMarket::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_MARKET_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const OBLIGATION_ACCOUNT_DISCM: [u8; 8] = [168, 206, 141, 106, 88, 76, 172, 167];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct Obligation {
    pub tag: u64,
    pub last_update: LastUpdate,
    pub lending_market: Pubkey,
    pub owner: Pubkey,
    pub deposits: [ObligationCollateral; 8],
    pub lowest_reserve_deposit_liquidation_ltv: u64,
    pub deposited_value_sf: u128,
    pub borrows: [ObligationLiquidity; 5],
    pub borrow_factor_adjusted_debt_value_sf: u128,
    pub borrowed_assets_market_value_sf: u128,
    pub allowed_borrow_value_sf: u128,
    pub unhealthy_borrow_value_sf: u128,
    pub padding_deprecated_asset_tiers: [u8; 13],
    pub elevation_group: u8,
    pub num_of_obsolete_deposit_reserves: u8,
    pub has_debt: u8,
    pub referrer: Pubkey,
    pub borrowing_disabled: u8,
    pub autodeleverage_target_ltv_pct: u8,
    pub lowest_reserve_deposit_max_ltv_pct: u8,
    pub num_of_obsolete_borrow_reserves: u8,
    pub ownership_transfer_state: u8,
    pub reserved: [u8; 3],
    pub highest_borrow_factor_pct: u64,
    pub autodeleverage_margin_call_started_timestamp: u64,
    pub obligation_orders: [ObligationOrder; 2],
    pub borrow_order: BorrowOrder,
    pub pending_owner: Pubkey,
    #[serde(with = "crate::big_array_serde")]
    pub padding3: [u64; 69],
}
impl Obligation {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let tag: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_update = if reader.is_empty() {
            Default::default()
        } else {
            <LastUpdate>::deserialize(&mut reader)?
        };
        let lending_market: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let deposits: [ObligationCollateral; 8] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let lowest_reserve_deposit_liquidation_ltv: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let deposited_value_sf: u128 = crate::borsh_de_or_default(&mut reader)?;
        let borrows: [ObligationLiquidity; 5] = crate::borsh_de_or_default(&mut reader)?;
        let borrow_factor_adjusted_debt_value_sf: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let borrowed_assets_market_value_sf: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let allowed_borrow_value_sf: u128 = crate::borsh_de_or_default(&mut reader)?;
        let unhealthy_borrow_value_sf: u128 = crate::borsh_de_or_default(&mut reader)?;
        let padding_deprecated_asset_tiers: [u8; 13] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let elevation_group: u8 = crate::borsh_de_or_default(&mut reader)?;
        let num_of_obsolete_deposit_reserves: u8 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let has_debt: u8 = crate::borsh_de_or_default(&mut reader)?;
        let referrer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let borrowing_disabled: u8 = crate::borsh_de_or_default(&mut reader)?;
        let autodeleverage_target_ltv_pct: u8 = crate::borsh_de_or_default(&mut reader)?;
        let lowest_reserve_deposit_max_ltv_pct: u8 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let num_of_obsolete_borrow_reserves: u8 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let ownership_transfer_state: u8 = crate::borsh_de_or_default(&mut reader)?;
        let reserved: [u8; 3] = crate::borsh_de_or_default(&mut reader)?;
        let highest_borrow_factor_pct: u64 = crate::borsh_de_or_default(&mut reader)?;
        let autodeleverage_margin_call_started_timestamp: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let obligation_orders: [ObligationOrder; 2] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let borrow_order = if reader.is_empty() {
            Default::default()
        } else {
            <BorrowOrder>::deserialize(&mut reader)?
        };
        let pending_owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let padding3 = <[u64; 69] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            tag,
            last_update,
            lending_market,
            owner,
            deposits,
            lowest_reserve_deposit_liquidation_ltv,
            deposited_value_sf,
            borrows,
            borrow_factor_adjusted_debt_value_sf,
            borrowed_assets_market_value_sf,
            allowed_borrow_value_sf,
            unhealthy_borrow_value_sf,
            padding_deprecated_asset_tiers,
            elevation_group,
            num_of_obsolete_deposit_reserves,
            has_debt,
            referrer,
            borrowing_disabled,
            autodeleverage_target_ltv_pct,
            lowest_reserve_deposit_max_ltv_pct,
            num_of_obsolete_borrow_reserves,
            ownership_transfer_state,
            reserved,
            highest_borrow_factor_pct,
            autodeleverage_margin_call_started_timestamp,
            obligation_orders,
            borrow_order,
            pending_owner,
            padding3,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.tag, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_update, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lending_market, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.deposits, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.lowest_reserve_deposit_liquidation_ltv,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.deposited_value_sf, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.borrows, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.borrow_factor_adjusted_debt_value_sf,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.borrowed_assets_market_value_sf,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.allowed_borrow_value_sf, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.unhealthy_borrow_value_sf, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.padding_deprecated_asset_tiers,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.elevation_group, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.num_of_obsolete_deposit_reserves,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.has_debt, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.referrer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.borrowing_disabled, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.autodeleverage_target_ltv_pct,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.lowest_reserve_deposit_max_ltv_pct,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.num_of_obsolete_borrow_reserves,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.ownership_transfer_state, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserved, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.highest_borrow_factor_pct, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.autodeleverage_margin_call_started_timestamp,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.obligation_orders, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.borrow_order, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.pending_owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding3, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ObligationAccount(pub Obligation);
impl ObligationAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != OBLIGATION_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Obligation::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&OBLIGATION_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REFERRER_STATE_ACCOUNT_DISCM: [u8; 8] = [194, 81, 217, 103, 12, 19, 12, 66];
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
pub struct ReferrerState {
    pub short_url: Pubkey,
    pub owner: Pubkey,
}
impl ReferrerState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let short_url: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { short_url, owner })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.short_url, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ReferrerStateAccount(pub ReferrerState);
impl ReferrerStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REFERRER_STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(ReferrerState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REFERRER_STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REFERRER_TOKEN_STATE_ACCOUNT_DISCM: [u8; 8] = [
    39, 15, 208, 77, 32, 195, 105, 56,
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
pub struct ReferrerTokenState {
    pub referrer: Pubkey,
    pub mint: Pubkey,
    pub amount_unclaimed_sf: u128,
    pub amount_cumulative_sf: u128,
    pub bump: u64,
    pub padding: [u64; 31],
}
impl ReferrerTokenState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let referrer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount_unclaimed_sf: u128 = crate::borsh_de_or_default(&mut reader)?;
        let amount_cumulative_sf: u128 = crate::borsh_de_or_default(&mut reader)?;
        let bump: u64 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u64; 31] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            referrer,
            mint,
            amount_unclaimed_sf,
            amount_cumulative_sf,
            bump,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.referrer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.mint, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_unclaimed_sf, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.amount_cumulative_sf, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ReferrerTokenStateAccount(pub ReferrerTokenState);
impl ReferrerTokenStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REFERRER_TOKEN_STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(ReferrerTokenState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REFERRER_TOKEN_STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SHORT_URL_ACCOUNT_DISCM: [u8; 8] = [28, 89, 174, 25, 226, 124, 126, 212];
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
pub struct ShortUrl {
    pub referrer: Pubkey,
    pub short_url: String,
}
impl ShortUrl {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let referrer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let short_url: String = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { referrer, short_url })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.referrer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.short_url, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ShortUrlAccount(pub ShortUrl);
impl ShortUrlAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SHORT_URL_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(ShortUrl::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SHORT_URL_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const USER_METADATA_ACCOUNT_DISCM: [u8; 8] = [157, 214, 220, 235, 98, 135, 171, 28];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct UserMetadata {
    pub referrer: Pubkey,
    pub bump: u64,
    pub user_lookup_table: Pubkey,
    pub owner: Pubkey,
    #[serde(with = "crate::big_array_serde")]
    pub padding1: [u64; 51],
    #[serde(with = "crate::big_array_serde")]
    pub padding2: [u64; 64],
}
impl UserMetadata {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let referrer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let bump: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user_lookup_table: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let padding1 = <[u64; 51] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let padding2 = <[u64; 64] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            referrer,
            bump,
            user_lookup_table,
            owner,
            padding1,
            padding2,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.referrer, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.bump, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.user_lookup_table, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding1, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding2, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct UserMetadataAccount(pub UserMetadata);
impl UserMetadataAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != USER_METADATA_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(UserMetadata::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&USER_METADATA_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const RESERVE_ACCOUNT_DISCM: [u8; 8] = [43, 242, 204, 202, 26, 247, 59, 127];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct Reserve {
    pub version: u64,
    pub last_update: LastUpdate,
    pub lending_market: Pubkey,
    pub farm_collateral: Pubkey,
    pub farm_debt: Pubkey,
    pub liquidity: ReserveLiquidity,
    #[serde(with = "crate::big_array_serde")]
    pub reserve_liquidity_padding: [u64; 150],
    pub collateral: ReserveCollateral,
    #[serde(with = "crate::big_array_serde")]
    pub reserve_collateral_padding: [u64; 150],
    pub config: ReserveConfig,
    #[serde(with = "crate::big_array_serde")]
    pub config_padding: [u64; 112],
    pub borrowed_amount_outside_elevation_group: u64,
    pub borrowed_amounts_against_this_reserve_in_elevation_groups: [u64; 32],
    pub withdraw_queue: WithdrawQueue,
    #[serde(with = "crate::big_array_serde")]
    pub padding: [u64; 204],
}
impl Reserve {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let version: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_update = if reader.is_empty() {
            Default::default()
        } else {
            <LastUpdate>::deserialize(&mut reader)?
        };
        let lending_market: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let farm_collateral: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let farm_debt: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let liquidity = <ReserveLiquidity>::deserialize(&mut reader)?;
        let reserve_liquidity_padding = <[u64; 150] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let collateral = if reader.is_empty() {
            Default::default()
        } else {
            <ReserveCollateral>::deserialize(&mut reader)?
        };
        let reserve_collateral_padding = <[u64; 150] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let config = if reader.is_empty() {
            Default::default()
        } else {
            <ReserveConfig>::deserialize(&mut reader)?
        };
        let config_padding = <[u64; 112] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let borrowed_amount_outside_elevation_group: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let borrowed_amounts_against_this_reserve_in_elevation_groups: [u64; 32] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let withdraw_queue = if reader.is_empty() {
            Default::default()
        } else {
            <WithdrawQueue>::deserialize(&mut reader)?
        };
        let padding = <[u64; 204] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            version,
            last_update,
            lending_market,
            farm_collateral,
            farm_debt,
            liquidity,
            reserve_liquidity_padding,
            collateral,
            reserve_collateral_padding,
            config,
            config_padding,
            borrowed_amount_outside_elevation_group,
            borrowed_amounts_against_this_reserve_in_elevation_groups,
            withdraw_queue,
            padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.version, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.last_update, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.lending_market, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.farm_collateral, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.farm_debt, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.liquidity, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserve_liquidity_padding, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.collateral, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserve_collateral_padding, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.config, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.config_padding, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.borrowed_amount_outside_elevation_group,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(
            &self.borrowed_amounts_against_this_reserve_in_elevation_groups,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.withdraw_queue, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ReserveAccount(pub Reserve);
impl ReserveAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != RESERVE_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(Reserve::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&RESERVE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const WITHDRAW_TICKET_ACCOUNT_DISCM: [u8; 8] = [237, 23, 164, 58, 53, 248, 240, 94];
#[derive(
    Clone,
    Debug,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
pub struct WithdrawTicket {
    pub sequence_number: u64,
    pub owner: Pubkey,
    pub reserve: Pubkey,
    pub user_destination_liquidity_ta: Pubkey,
    pub queued_collateral_amount: u64,
    pub created_at_timestamp: u64,
    pub invalid: u8,
    pub progress_callback_type: u8,
    pub alignment_padding: [u8; 6],
    pub progress_callback_custom_accounts: [Pubkey; 2],
    #[serde(with = "crate::big_array_serde")]
    pub end_padding: [u64; 40],
}
impl WithdrawTicket {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let sequence_number: u64 = crate::borsh_de_or_default(&mut reader)?;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let reserve: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let user_destination_liquidity_ta: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let queued_collateral_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let created_at_timestamp: u64 = crate::borsh_de_or_default(&mut reader)?;
        let invalid: u8 = crate::borsh_de_or_default(&mut reader)?;
        let progress_callback_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let alignment_padding: [u8; 6] = crate::borsh_de_or_default(&mut reader)?;
        let progress_callback_custom_accounts: [Pubkey; 2] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let end_padding = <[u64; 40] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            sequence_number,
            owner,
            reserve,
            user_destination_liquidity_ta,
            queued_collateral_amount,
            created_at_timestamp,
            invalid,
            progress_callback_type,
            alignment_padding,
            progress_callback_custom_accounts,
            end_padding,
        })
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.sequence_number, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.owner, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.reserve, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.user_destination_liquidity_ta,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.queued_collateral_amount, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.created_at_timestamp, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.invalid, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.progress_callback_type, &mut writer)?;
        borsh::BorshSerialize::serialize(&self.alignment_padding, &mut writer)?;
        borsh::BorshSerialize::serialize(
            &self.progress_callback_custom_accounts,
            &mut writer,
        )?;
        borsh::BorshSerialize::serialize(&self.end_padding, &mut writer)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawTicketAccount(pub WithdrawTicket);
impl WithdrawTicketAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_TICKET_ACCOUNT_DISCM {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        Ok(Self(WithdrawTicket::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_TICKET_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
