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
pub struct BankAccounting {
    pub yielding_tvl: u64,
    pub total_issued_supply: u64,
    pub max_yielding_tvl: u64,
    pub _padding1: [u64; 13],
}
impl BankAccounting {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let yielding_tvl: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_issued_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_yielding_tvl: u64 = crate::borsh_de_or_default(&mut reader)?;
        let _padding1: [u64; 13] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            yielding_tvl,
            total_issued_supply,
            max_yielding_tvl,
            _padding1,
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
pub struct BankConfig {
    pub creator: Pubkey,
    pub bank_manager: Pubkey,
    pub risk_manager: Pubkey,
    pub _padding2: [u64; 5],
}
impl BankConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let bank_manager: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let risk_manager: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let _padding2: [u64; 5] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            creator,
            bank_manager,
            risk_manager,
            _padding2,
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
pub struct BankMint {
    pub pubkey: Pubkey,
    pub price: u64,
    pub decimals: u8,
    pub _padding1: [u8; 7],
    pub _padding2: [u64; 8],
}
impl BankMint {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pubkey: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let _padding1: [u8; 7] = crate::borsh_de_or_default(&mut reader)?;
        let _padding2: [u64; 8] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pubkey,
            price,
            decimals,
            _padding1,
            _padding2,
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
pub struct BankStatus {
    pub is_halted: PodBool,
    pub is_halted_deposit: PodBool,
    pub is_halted_withdrawal: PodBool,
    pub _padding1: [u8; 5],
    pub _padding2: [u64; 4],
}
impl BankStatus {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let is_halted = if reader.is_empty() {
            Default::default()
        } else {
            <PodBool>::deserialize(&mut reader)?
        };
        let is_halted_deposit = if reader.is_empty() {
            Default::default()
        } else {
            <PodBool>::deserialize(&mut reader)?
        };
        let is_halted_withdrawal = if reader.is_empty() {
            Default::default()
        } else {
            <PodBool>::deserialize(&mut reader)?
        };
        let _padding1: [u8; 5] = crate::borsh_de_or_default(&mut reader)?;
        let _padding2: [u64; 4] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            is_halted,
            is_halted_deposit,
            is_halted_withdrawal,
            _padding1,
            _padding2,
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
pub enum FeeType {
    #[default]
    MintBurn,
    Performance,
    Unstake,
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
pub enum LendingPlatform {
    #[default]
    Marginfi,
    Kamino,
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
pub enum LendingType {
    #[default]
    LendingMarket,
    Vault,
    Multiply,
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
pub struct OracleConfig {
    pub price_max_ts_gap: u64,
    pub price_max_gap_bps: u16,
    pub yielding_mint_decimals: u8,
    pub _padding1: [u8; 5],
    pub _padding2: [u64; 4],
}
impl OracleConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let price_max_ts_gap: u64 = crate::borsh_de_or_default(&mut reader)?;
        let price_max_gap_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let yielding_mint_decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let _padding1: [u8; 5] = crate::borsh_de_or_default(&mut reader)?;
        let _padding2: [u64; 4] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            price_max_ts_gap,
            price_max_gap_bps,
            yielding_mint_decimals,
            _padding1,
            _padding2,
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
pub struct OracleData {
    pub oracle_registry: [Pubkey; 8],
    pub oracle_prices: [u64; 8],
    pub oracle_pending_yield: [u64; 8],
    pub oracle_veto: [u8; 8],
    pub _padding1: [u8; 8],
    pub oracle_last_ts: [u64; 8],
    pub _padding: [u64; 16],
}
impl OracleData {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let oracle_registry: [Pubkey; 8] = crate::borsh_de_or_default(&mut reader)?;
        let oracle_prices: [u64; 8] = crate::borsh_de_or_default(&mut reader)?;
        let oracle_pending_yield: [u64; 8] = crate::borsh_de_or_default(&mut reader)?;
        let oracle_veto: [u8; 8] = crate::borsh_de_or_default(&mut reader)?;
        let _padding1: [u8; 8] = crate::borsh_de_or_default(&mut reader)?;
        let oracle_last_ts: [u64; 8] = crate::borsh_de_or_default(&mut reader)?;
        let _padding: [u64; 16] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            oracle_registry,
            oracle_prices,
            oracle_pending_yield,
            oracle_veto,
            _padding1,
            oracle_last_ts,
            _padding,
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
pub struct OracleResult {
    pub yielding_token_price: u64,
    pub yielding_pending_amount: u64,
    pub update_ts: u64,
    pub start_marker_unix_seconds: u64,
    pub highest_confirmed_price: u64,
    pub _padding1: [u64; 10],
}
impl OracleResult {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let yielding_token_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let yielding_pending_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let update_ts: u64 = crate::borsh_de_or_default(&mut reader)?;
        let start_marker_unix_seconds: u64 = crate::borsh_de_or_default(&mut reader)?;
        let highest_confirmed_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let _padding1: [u64; 10] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            yielding_token_price,
            yielding_pending_amount,
            update_ts,
            start_marker_unix_seconds,
            highest_confirmed_price,
            _padding1,
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
pub struct PodBool {
    pub field_0: u8,
}
impl PodBool {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let field_0: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { field_0 })
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
pub struct TeamAccounting {
    pub net_yielding_to_deposit_back: u64,
    pub mint_burn_fees: u64,
    pub performance_fees: u64,
    pub unstake_fees: u64,
    pub _padding2: [u64; 14],
}
impl TeamAccounting {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let net_yielding_to_deposit_back: u64 = crate::borsh_de_or_default(&mut reader)?;
        let mint_burn_fees: u64 = crate::borsh_de_or_default(&mut reader)?;
        let performance_fees: u64 = crate::borsh_de_or_default(&mut reader)?;
        let unstake_fees: u64 = crate::borsh_de_or_default(&mut reader)?;
        let _padding2: [u64; 14] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            net_yielding_to_deposit_back,
            mint_burn_fees,
            performance_fees,
            unstake_fees,
            _padding2,
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
pub struct TeamDetails {
    pub bank: Pubkey,
    pub vault: Pubkey,
    pub creator: Pubkey,
    pub yielding_fees_ata: Pubkey,
    pub fee_gatherer: Pubkey,
    pub yield_manager: Pubkey,
    pub _padding: [u64; 12],
}
impl TeamDetails {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bank: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let yielding_fees_ata: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_gatherer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let yield_manager: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let _padding: [u64; 12] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            bank,
            vault,
            creator,
            yielding_fees_ata,
            fee_gatherer,
            yield_manager,
            _padding,
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
pub struct TrancheAccounting {
    pub shares: u64,
    pub staked_amount: u64,
    pub virtual_value: u64,
    pub cumulative_loss_absorbed: u64,
    pub last_update_ts: i64,
    pub share_price: u64,
    pub protocol_fees_accrued: u64,
    pub has_reached_recovery_threshold: u8,
    pub _padding1: [u8; 7],
    pub _padding2: [u64; 8],
}
impl TrancheAccounting {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        let staked_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let virtual_value: u64 = crate::borsh_de_or_default(&mut reader)?;
        let cumulative_loss_absorbed: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_update_ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let share_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fees_accrued: u64 = crate::borsh_de_or_default(&mut reader)?;
        let has_reached_recovery_threshold: u8 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let _padding1: [u8; 7] = crate::borsh_de_or_default(&mut reader)?;
        let _padding2: [u64; 8] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            shares,
            staked_amount,
            virtual_value,
            cumulative_loss_absorbed,
            last_update_ts,
            share_price,
            protocol_fees_accrued,
            has_reached_recovery_threshold,
            _padding1,
            _padding2,
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
pub struct TrancheConfig {
    pub mint: Pubkey,
    pub share_mint: Pubkey,
    pub share_price_decimals: u8,
    pub _padding1: [u8; 1],
    pub base_leverage_factor: u16,
    pub leverage_slope: u16,
    pub target_percent_staked_bps: u16,
    pub early_unstake_fee_bps: u16,
    pub standard_unstake_fee_bps: u16,
    pub low_stake_threshold_bps: u16,
    pub low_stake_fee_bps: u16,
    pub target_lockup_duration_secs: i64,
    pub recovery_yield_bps: u16,
    pub recovery_threshold_bps: u16,
    pub bootstrap_recovery_yield_bps: u16,
    pub low_stake_fee_enabled: PodBool,
    pub _padding2: [u8; 1],
    pub _padding3: [u64; 13],
}
impl TrancheConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let share_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let share_price_decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let _padding1: [u8; 1] = crate::borsh_de_or_default(&mut reader)?;
        let base_leverage_factor: u16 = crate::borsh_de_or_default(&mut reader)?;
        let leverage_slope: u16 = crate::borsh_de_or_default(&mut reader)?;
        let target_percent_staked_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let early_unstake_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let standard_unstake_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let low_stake_threshold_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let low_stake_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let target_lockup_duration_secs: i64 = crate::borsh_de_or_default(&mut reader)?;
        let recovery_yield_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let recovery_threshold_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let bootstrap_recovery_yield_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let low_stake_fee_enabled = if reader.is_empty() {
            Default::default()
        } else {
            <PodBool>::deserialize(&mut reader)?
        };
        let _padding2: [u8; 1] = crate::borsh_de_or_default(&mut reader)?;
        let _padding3: [u64; 13] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            mint,
            share_mint,
            share_price_decimals,
            _padding1,
            base_leverage_factor,
            leverage_slope,
            target_percent_staked_bps,
            early_unstake_fee_bps,
            standard_unstake_fee_bps,
            low_stake_threshold_bps,
            low_stake_fee_bps,
            target_lockup_duration_secs,
            recovery_yield_bps,
            recovery_threshold_bps,
            bootstrap_recovery_yield_bps,
            low_stake_fee_enabled,
            _padding2,
            _padding3,
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
pub struct VaultAccounting {
    pub yielding_reserve_amount: u64,
    pub external_yielding_amount: u64,
    pub yielding_tvl_cap: u64,
    pub _legacy_padding1: u64,
    pub _legacy_padding2: u64,
    pub yielding_tvl: u64,
    pub reserve_at_last_fee_extraction: u64,
    pub _padding: [u64; 11],
}
impl VaultAccounting {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let yielding_reserve_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let external_yielding_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let yielding_tvl_cap: u64 = crate::borsh_de_or_default(&mut reader)?;
        let _legacy_padding1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let _legacy_padding2: u64 = crate::borsh_de_or_default(&mut reader)?;
        let yielding_tvl: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reserve_at_last_fee_extraction: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let _padding: [u64; 11] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            yielding_reserve_amount,
            external_yielding_amount,
            yielding_tvl_cap,
            _legacy_padding1,
            _legacy_padding2,
            yielding_tvl,
            reserve_at_last_fee_extraction,
            _padding,
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
pub struct VaultConfig {
    pub creator: Pubkey,
    pub bank: Pubkey,
    pub team_account: Pubkey,
    pub oracle_state: Pubkey,
    pub risk_manager: Pubkey,
    pub yielding_token_mint: Pubkey,
    pub yielding_vault_ata: Pubkey,
    pub yielding_mint_decimals: u8,
    pub _padding1: [u8; 7],
    pub fee_controller: Pubkey,
    pub permissioned_users: [Pubkey; 3],
    pub _padding2: u16,
    pub performance_fee_bps: u16,
    pub minting_fee_bps: u16,
    pub burning_fee_bps: u16,
    pub lp_account_index: u16,
    pub lp_third_party_id: u16,
    pub lending_platform: LendingPlatform,
    pub lending_type: LendingType,
    pub _padding4: [u8; 2],
    pub _padding: [u64; 11],
}
impl VaultConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let bank: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let team_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let oracle_state: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let risk_manager: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let yielding_token_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let yielding_vault_ata: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let yielding_mint_decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let _padding1: [u8; 7] = crate::borsh_de_or_default(&mut reader)?;
        let fee_controller: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let permissioned_users: [Pubkey; 3] = crate::borsh_de_or_default(&mut reader)?;
        let _padding2: u16 = crate::borsh_de_or_default(&mut reader)?;
        let performance_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let minting_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let burning_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let lp_account_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let lp_third_party_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let lending_platform: LendingPlatform = crate::borsh_de_or_default(&mut reader)?;
        let lending_type: LendingType = crate::borsh_de_or_default(&mut reader)?;
        let _padding4: [u8; 2] = crate::borsh_de_or_default(&mut reader)?;
        let _padding: [u64; 11] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            creator,
            bank,
            team_account,
            oracle_state,
            risk_manager,
            yielding_token_mint,
            yielding_vault_ata,
            yielding_mint_decimals,
            _padding1,
            fee_controller,
            permissioned_users,
            _padding2,
            performance_fee_bps,
            minting_fee_bps,
            burning_fee_bps,
            lp_account_index,
            lp_third_party_id,
            lending_platform,
            lending_type,
            _padding4,
            _padding,
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
pub struct VaultConfigData {
    pub yield_manager: Option<Pubkey>,
    pub risk_manager: Option<Pubkey>,
    pub fee_controller: Option<Pubkey>,
    pub permissioned_users: Option<[Pubkey; 3]>,
}
impl VaultConfigData {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let yield_manager: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let risk_manager: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let fee_controller: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let permissioned_users: Option<[Pubkey; 3]> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            yield_manager,
            risk_manager,
            fee_controller,
            permissioned_users,
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
pub struct VaultManagement {
    pub _legacy_padding1: [u64; 16],
    pub _legacy_padding2: [i64; 16],
    pub _legacy_padding3: [Pubkey; 16],
    pub new_index: u8,
    pub _padding1: [u8; 7],
    pub _padding: [u64; 12],
}
impl VaultManagement {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let _legacy_padding1: [u64; 16] = crate::borsh_de_or_default(&mut reader)?;
        let _legacy_padding2: [i64; 16] = crate::borsh_de_or_default(&mut reader)?;
        let _legacy_padding3: [Pubkey; 16] = crate::borsh_de_or_default(&mut reader)?;
        let new_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let _padding1: [u8; 7] = crate::borsh_de_or_default(&mut reader)?;
        let _padding: [u64; 12] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            _legacy_padding1,
            _legacy_padding2,
            _legacy_padding3,
            new_index,
            _padding1,
            _padding,
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
pub struct VaultStatus {
    pub is_halted: PodBool,
    pub is_halted_deposit: PodBool,
    pub is_halted_withdrawal: PodBool,
    pub losses_accepted: PodBool,
    pub _padding1: [u8; 4],
    pub _padding: [u64; 12],
}
impl VaultStatus {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let is_halted = if reader.is_empty() {
            Default::default()
        } else {
            <PodBool>::deserialize(&mut reader)?
        };
        let is_halted_deposit = if reader.is_empty() {
            Default::default()
        } else {
            <PodBool>::deserialize(&mut reader)?
        };
        let is_halted_withdrawal = if reader.is_empty() {
            Default::default()
        } else {
            <PodBool>::deserialize(&mut reader)?
        };
        let losses_accepted = if reader.is_empty() {
            Default::default()
        } else {
            <PodBool>::deserialize(&mut reader)?
        };
        let _padding1: [u8; 4] = crate::borsh_de_or_default(&mut reader)?;
        let _padding: [u64; 12] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            is_halted,
            is_halted_deposit,
            is_halted_withdrawal,
            losses_accepted,
            _padding1,
            _padding,
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
pub enum VaultType {
    #[default]
    YieldBearing,
    CollateralizedExternalYield,
    UnCollateralizedExternalYield,
    AtomicLending,
}
