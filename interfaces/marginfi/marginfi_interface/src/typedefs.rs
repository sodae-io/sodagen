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
pub struct AccountEventHeader {
    pub signer: Option<Pubkey>,
    pub marginfi_account: Pubkey,
    pub marginfi_account_authority: Pubkey,
    pub marginfi_group: Pubkey,
}
impl AccountEventHeader {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let signer: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let marginfi_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let marginfi_account_authority: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let marginfi_group: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            signer,
            marginfi_account,
            marginfi_account_authority,
            marginfi_group,
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
pub struct Balance {
    pub active: u8,
    pub bank_pk: Pubkey,
    pub bank_asset_tag: u8,
    pub tag: u16,
    pub _pad0: [u8; 4],
    pub asset_shares: WrappedI80F48,
    pub liability_shares: WrappedI80F48,
    pub emissions_outstanding: WrappedI80F48,
    pub last_update: u64,
    pub _padding: [u64; 1],
}
impl Balance {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let active: u8 = crate::borsh_de_or_default(&mut reader)?;
        let bank_pk: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let bank_asset_tag: u8 = crate::borsh_de_or_default(&mut reader)?;
        let tag: u16 = crate::borsh_de_or_default(&mut reader)?;
        let _pad0: [u8; 4] = crate::borsh_de_or_default(&mut reader)?;
        let asset_shares = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let liability_shares = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let emissions_outstanding = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let last_update: u64 = crate::borsh_de_or_default(&mut reader)?;
        let _padding: [u64; 1] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            active,
            bank_pk,
            bank_asset_tag,
            tag,
            _pad0,
            asset_shares,
            liability_shares,
            emissions_outstanding,
            last_update,
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
pub struct BankCache {
    pub base_rate: u32,
    pub lending_rate: u32,
    pub borrowing_rate: u32,
    pub interest_accumulated_for: u32,
    pub accumulated_since_last_update: WrappedI80F48,
    pub last_oracle_price: WrappedI80F48,
    pub last_oracle_price_timestamp: i64,
    pub last_oracle_price_confidence: WrappedI80F48,
    pub liq_cache_flags: u8,
    pub _padding: [u8; 23],
    pub liquidation_price_rt: WrappedI80F48,
    pub liquidation_price_rt_confidence: WrappedI80F48,
    pub liquidation_price_twap: WrappedI80F48,
    pub liquidation_price_twap_confidence: WrappedI80F48,
}
impl BankCache {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let base_rate: u32 = crate::borsh_de_or_default(&mut reader)?;
        let lending_rate: u32 = crate::borsh_de_or_default(&mut reader)?;
        let borrowing_rate: u32 = crate::borsh_de_or_default(&mut reader)?;
        let interest_accumulated_for: u32 = crate::borsh_de_or_default(&mut reader)?;
        let accumulated_since_last_update = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let last_oracle_price = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let last_oracle_price_timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        let last_oracle_price_confidence = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let liq_cache_flags: u8 = crate::borsh_de_or_default(&mut reader)?;
        let _padding: [u8; 23] = crate::borsh_de_or_default(&mut reader)?;
        let liquidation_price_rt = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let liquidation_price_rt_confidence = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let liquidation_price_twap = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let liquidation_price_twap_confidence = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            base_rate,
            lending_rate,
            borrowing_rate,
            interest_accumulated_for,
            accumulated_since_last_update,
            last_oracle_price,
            last_oracle_price_timestamp,
            last_oracle_price_confidence,
            liq_cache_flags,
            _padding,
            liquidation_price_rt,
            liquidation_price_rt_confidence,
            liquidation_price_twap,
            liquidation_price_twap_confidence,
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
    pub asset_weight_init: WrappedI80F48,
    pub asset_weight_maint: WrappedI80F48,
    pub liability_weight_init: WrappedI80F48,
    pub liability_weight_maint: WrappedI80F48,
    pub deposit_limit: u64,
    pub interest_rate_config: InterestRateConfig,
    pub operational_state: BankOperationalState,
    pub oracle_setup: OracleSetup,
    pub oracle_keys: [Pubkey; 5],
    pub _pad0: [u8; 6],
    pub borrow_limit: u64,
    pub risk_tier: RiskTier,
    pub asset_tag: u8,
    pub config_flags: u8,
    pub _pad1: [u8; 5],
    pub total_asset_value_init_limit: u64,
    pub oracle_max_age: u16,
    pub _padding0: [u8; 2],
    pub oracle_max_confidence: u32,
    pub fixed_price: WrappedI80F48,
    pub _padding1: [u8; 16],
}
impl BankConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let asset_weight_init = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let asset_weight_maint = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let liability_weight_init = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let liability_weight_maint = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let deposit_limit: u64 = crate::borsh_de_or_default(&mut reader)?;
        let interest_rate_config = if reader.is_empty() {
            Default::default()
        } else {
            <InterestRateConfig>::deserialize(&mut reader)?
        };
        let operational_state: BankOperationalState = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let oracle_setup: OracleSetup = crate::borsh_de_or_default(&mut reader)?;
        let oracle_keys: [Pubkey; 5] = crate::borsh_de_or_default(&mut reader)?;
        let _pad0: [u8; 6] = crate::borsh_de_or_default(&mut reader)?;
        let borrow_limit: u64 = crate::borsh_de_or_default(&mut reader)?;
        let risk_tier: RiskTier = crate::borsh_de_or_default(&mut reader)?;
        let asset_tag: u8 = crate::borsh_de_or_default(&mut reader)?;
        let config_flags: u8 = crate::borsh_de_or_default(&mut reader)?;
        let _pad1: [u8; 5] = crate::borsh_de_or_default(&mut reader)?;
        let total_asset_value_init_limit: u64 = crate::borsh_de_or_default(&mut reader)?;
        let oracle_max_age: u16 = crate::borsh_de_or_default(&mut reader)?;
        let _padding0: [u8; 2] = crate::borsh_de_or_default(&mut reader)?;
        let oracle_max_confidence: u32 = crate::borsh_de_or_default(&mut reader)?;
        let fixed_price = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let _padding1: [u8; 16] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            asset_weight_init,
            asset_weight_maint,
            liability_weight_init,
            liability_weight_maint,
            deposit_limit,
            interest_rate_config,
            operational_state,
            oracle_setup,
            oracle_keys,
            _pad0,
            borrow_limit,
            risk_tier,
            asset_tag,
            config_flags,
            _pad1,
            total_asset_value_init_limit,
            oracle_max_age,
            _padding0,
            oracle_max_confidence,
            fixed_price,
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
pub struct BankConfigCompact {
    pub asset_weight_init: WrappedI80F48,
    pub asset_weight_maint: WrappedI80F48,
    pub liability_weight_init: WrappedI80F48,
    pub liability_weight_maint: WrappedI80F48,
    pub deposit_limit: u64,
    pub interest_rate_config: InterestRateConfigCompact,
    pub operational_state: BankOperationalState,
    pub borrow_limit: u64,
    pub risk_tier: RiskTier,
    pub asset_tag: u8,
    pub config_flags: u8,
    pub _pad0: [u8; 5],
    pub total_asset_value_init_limit: u64,
    pub oracle_max_age: u16,
    pub oracle_max_confidence: u32,
}
impl BankConfigCompact {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let asset_weight_init = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let asset_weight_maint = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let liability_weight_init = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let liability_weight_maint = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let deposit_limit: u64 = crate::borsh_de_or_default(&mut reader)?;
        let interest_rate_config = if reader.is_empty() {
            Default::default()
        } else {
            <InterestRateConfigCompact>::deserialize(&mut reader)?
        };
        let operational_state: BankOperationalState = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let borrow_limit: u64 = crate::borsh_de_or_default(&mut reader)?;
        let risk_tier: RiskTier = crate::borsh_de_or_default(&mut reader)?;
        let asset_tag: u8 = crate::borsh_de_or_default(&mut reader)?;
        let config_flags: u8 = crate::borsh_de_or_default(&mut reader)?;
        let _pad0: [u8; 5] = crate::borsh_de_or_default(&mut reader)?;
        let total_asset_value_init_limit: u64 = crate::borsh_de_or_default(&mut reader)?;
        let oracle_max_age: u16 = crate::borsh_de_or_default(&mut reader)?;
        let oracle_max_confidence: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            asset_weight_init,
            asset_weight_maint,
            liability_weight_init,
            liability_weight_maint,
            deposit_limit,
            interest_rate_config,
            operational_state,
            borrow_limit,
            risk_tier,
            asset_tag,
            config_flags,
            _pad0,
            total_asset_value_init_limit,
            oracle_max_age,
            oracle_max_confidence,
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
pub struct BankConfigOpt {
    pub asset_weight_init: Option<WrappedI80F48>,
    pub asset_weight_maint: Option<WrappedI80F48>,
    pub liability_weight_init: Option<WrappedI80F48>,
    pub liability_weight_maint: Option<WrappedI80F48>,
    pub deposit_limit: Option<u64>,
    pub borrow_limit: Option<u64>,
    pub operational_state: Option<BankOperationalState>,
    pub interest_rate_config: Option<InterestRateConfigOpt>,
    pub risk_tier: Option<RiskTier>,
    pub asset_tag: Option<u8>,
    pub total_asset_value_init_limit: Option<u64>,
    pub oracle_max_confidence: Option<u32>,
    pub oracle_max_age: Option<u16>,
    pub permissionless_bad_debt_settlement: Option<bool>,
    pub freeze_settings: Option<bool>,
    pub tokenless_repayments_allowed: Option<bool>,
}
impl BankConfigOpt {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let asset_weight_init: Option<WrappedI80F48> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let asset_weight_maint: Option<WrappedI80F48> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let liability_weight_init: Option<WrappedI80F48> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let liability_weight_maint: Option<WrappedI80F48> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let deposit_limit: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let borrow_limit: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let operational_state: Option<BankOperationalState> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let interest_rate_config: Option<InterestRateConfigOpt> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let risk_tier: Option<RiskTier> = crate::borsh_de_or_default(&mut reader)?;
        let asset_tag: Option<u8> = crate::borsh_de_or_default(&mut reader)?;
        let total_asset_value_init_limit: Option<u64> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let oracle_max_confidence: Option<u32> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let oracle_max_age: Option<u16> = crate::borsh_de_or_default(&mut reader)?;
        let permissionless_bad_debt_settlement: Option<bool> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let freeze_settings: Option<bool> = crate::borsh_de_or_default(&mut reader)?;
        let tokenless_repayments_allowed: Option<bool> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            asset_weight_init,
            asset_weight_maint,
            liability_weight_init,
            liability_weight_maint,
            deposit_limit,
            borrow_limit,
            operational_state,
            interest_rate_config,
            risk_tier,
            asset_tag,
            total_asset_value_init_limit,
            oracle_max_confidence,
            oracle_max_age,
            permissionless_bad_debt_settlement,
            freeze_settings,
            tokenless_repayments_allowed,
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
pub enum BankOperationalState {
    #[default]
    Paused,
    Operational,
    ReduceOnly,
    KilledByBankruptcy,
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
pub struct BankRateLimiter {
    pub hourly: RateLimitWindow,
    pub daily: RateLimitWindow,
}
impl BankRateLimiter {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let hourly = if reader.is_empty() {
            Default::default()
        } else {
            <RateLimitWindow>::deserialize(&mut reader)?
        };
        let daily = if reader.is_empty() {
            Default::default()
        } else {
            <RateLimitWindow>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self { hourly, daily })
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
pub struct DriftConfigCompact {
    pub oracle: Pubkey,
    pub asset_weight_init: WrappedI80F48,
    pub asset_weight_maint: WrappedI80F48,
    pub deposit_limit: u64,
    pub oracle_setup: OracleSetup,
    pub operational_state: BankOperationalState,
    pub risk_tier: RiskTier,
    pub config_flags: u8,
    pub total_asset_value_init_limit: u64,
    pub oracle_max_age: u16,
    pub oracle_max_confidence: u32,
}
impl DriftConfigCompact {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let oracle: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let asset_weight_init = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let asset_weight_maint = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let deposit_limit: u64 = crate::borsh_de_or_default(&mut reader)?;
        let oracle_setup: OracleSetup = crate::borsh_de_or_default(&mut reader)?;
        let operational_state: BankOperationalState = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let risk_tier: RiskTier = crate::borsh_de_or_default(&mut reader)?;
        let config_flags: u8 = crate::borsh_de_or_default(&mut reader)?;
        let total_asset_value_init_limit: u64 = crate::borsh_de_or_default(&mut reader)?;
        let oracle_max_age: u16 = crate::borsh_de_or_default(&mut reader)?;
        let oracle_max_confidence: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            oracle,
            asset_weight_init,
            asset_weight_maint,
            deposit_limit,
            oracle_setup,
            operational_state,
            risk_tier,
            config_flags,
            total_asset_value_init_limit,
            oracle_max_age,
            oracle_max_confidence,
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
pub struct EmodeConfig {
    pub entries: [EmodeEntry; 10],
}
impl EmodeConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let entries: [EmodeEntry; 10] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { entries })
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
pub struct EmodeEntry {
    pub collateral_bank_emode_tag: u16,
    pub flags: u8,
    pub pad0: [u8; 5],
    pub asset_weight_init: WrappedI80F48,
    pub asset_weight_maint: WrappedI80F48,
}
impl EmodeEntry {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let collateral_bank_emode_tag: u16 = crate::borsh_de_or_default(&mut reader)?;
        let flags: u8 = crate::borsh_de_or_default(&mut reader)?;
        let pad0: [u8; 5] = crate::borsh_de_or_default(&mut reader)?;
        let asset_weight_init = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let asset_weight_maint = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            collateral_bank_emode_tag,
            flags,
            pad0,
            asset_weight_init,
            asset_weight_maint,
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
pub struct EmodeSettings {
    pub emode_tag: u16,
    pub pad0: [u8; 6],
    pub timestamp: i64,
    pub flags: u64,
    pub emode_config: EmodeConfig,
}
impl EmodeSettings {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let emode_tag: u16 = crate::borsh_de_or_default(&mut reader)?;
        let pad0: [u8; 6] = crate::borsh_de_or_default(&mut reader)?;
        let timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        let flags: u64 = crate::borsh_de_or_default(&mut reader)?;
        let emode_config = if reader.is_empty() {
            Default::default()
        } else {
            <EmodeConfig>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            emode_tag,
            pad0,
            timestamp,
            flags,
            emode_config,
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
pub struct ExecuteOrderBalanceRecord {
    pub bank: Pubkey,
    pub is_asset: u8,
    pub _pad0: [u8; 5],
    pub tag: u16,
    pub shares: WrappedI80F48,
}
impl ExecuteOrderBalanceRecord {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bank: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let is_asset: u8 = crate::borsh_de_or_default(&mut reader)?;
        let _pad0: [u8; 5] = crate::borsh_de_or_default(&mut reader)?;
        let tag: u16 = crate::borsh_de_or_default(&mut reader)?;
        let shares = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            bank,
            is_asset,
            _pad0,
            tag,
            shares,
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
pub struct FeeStateCache {
    pub global_fee_wallet: Pubkey,
    pub program_fee_fixed: WrappedI80F48,
    pub program_fee_rate: WrappedI80F48,
    pub last_update: i64,
}
impl FeeStateCache {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let global_fee_wallet: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let program_fee_fixed = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let program_fee_rate = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let last_update: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            global_fee_wallet,
            program_fee_fixed,
            program_fee_rate,
            last_update,
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
pub struct GroupEventHeader {
    pub signer: Option<Pubkey>,
    pub marginfi_group: Pubkey,
}
impl GroupEventHeader {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let signer: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let marginfi_group: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { signer, marginfi_group })
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
pub struct GroupRateLimiter {
    pub hourly: RateLimitWindow,
    pub daily: RateLimitWindow,
}
impl GroupRateLimiter {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let hourly = if reader.is_empty() {
            Default::default()
        } else {
            <RateLimitWindow>::deserialize(&mut reader)?
        };
        let daily = if reader.is_empty() {
            Default::default()
        } else {
            <RateLimitWindow>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self { hourly, daily })
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
pub struct HealthCache {
    pub asset_value: WrappedI80F48,
    pub liability_value: WrappedI80F48,
    pub asset_value_maint: WrappedI80F48,
    pub liability_value_maint: WrappedI80F48,
    pub asset_value_equity: WrappedI80F48,
    pub liability_value_equity: WrappedI80F48,
    pub timestamp: i64,
    pub flags: u32,
    pub mrgn_err: u32,
    pub prices: [[u8; 8]; 16],
    pub internal_err: u32,
    pub err_index: u8,
    pub program_version: u8,
    pub pad0: [u8; 2],
    pub internal_liq_err: u32,
    pub internal_bankruptcy_err: u32,
    pub reserved0: [u8; 32],
    pub reserved1: [u8; 16],
}
impl HealthCache {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let asset_value = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let liability_value = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let asset_value_maint = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let liability_value_maint = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let asset_value_equity = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let liability_value_equity = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        let flags: u32 = crate::borsh_de_or_default(&mut reader)?;
        let mrgn_err: u32 = crate::borsh_de_or_default(&mut reader)?;
        let prices: [[u8; 8]; 16] = crate::borsh_de_or_default(&mut reader)?;
        let internal_err: u32 = crate::borsh_de_or_default(&mut reader)?;
        let err_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let program_version: u8 = crate::borsh_de_or_default(&mut reader)?;
        let pad0: [u8; 2] = crate::borsh_de_or_default(&mut reader)?;
        let internal_liq_err: u32 = crate::borsh_de_or_default(&mut reader)?;
        let internal_bankruptcy_err: u32 = crate::borsh_de_or_default(&mut reader)?;
        let reserved0: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        let reserved1: [u8; 16] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            asset_value,
            liability_value,
            asset_value_maint,
            liability_value_maint,
            asset_value_equity,
            liability_value_equity,
            timestamp,
            flags,
            mrgn_err,
            prices,
            internal_err,
            err_index,
            program_version,
            pad0,
            internal_liq_err,
            internal_bankruptcy_err,
            reserved0,
            reserved1,
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
pub struct InterestRateConfig {
    pub optimal_utilization_rate: WrappedI80F48,
    pub plateau_interest_rate: WrappedI80F48,
    pub max_interest_rate: WrappedI80F48,
    pub insurance_fee_fixed_apr: WrappedI80F48,
    pub insurance_ir_fee: WrappedI80F48,
    pub protocol_fixed_fee_apr: WrappedI80F48,
    pub protocol_ir_fee: WrappedI80F48,
    pub protocol_origination_fee: WrappedI80F48,
    pub zero_util_rate: u32,
    pub hundred_util_rate: u32,
    pub points: [RatePoint; 5],
    pub curve_type: u8,
    pub _pad0: [u8; 7],
    pub _padding1: [u8; 32],
    pub _padding2: [u8; 16],
    pub _padding3: [u8; 8],
}
impl InterestRateConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let optimal_utilization_rate = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let plateau_interest_rate = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let max_interest_rate = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let insurance_fee_fixed_apr = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let insurance_ir_fee = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let protocol_fixed_fee_apr = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let protocol_ir_fee = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let protocol_origination_fee = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let zero_util_rate: u32 = crate::borsh_de_or_default(&mut reader)?;
        let hundred_util_rate: u32 = crate::borsh_de_or_default(&mut reader)?;
        let points: [RatePoint; 5] = crate::borsh_de_or_default(&mut reader)?;
        let curve_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let _pad0: [u8; 7] = crate::borsh_de_or_default(&mut reader)?;
        let _padding1: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        let _padding2: [u8; 16] = crate::borsh_de_or_default(&mut reader)?;
        let _padding3: [u8; 8] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            optimal_utilization_rate,
            plateau_interest_rate,
            max_interest_rate,
            insurance_fee_fixed_apr,
            insurance_ir_fee,
            protocol_fixed_fee_apr,
            protocol_ir_fee,
            protocol_origination_fee,
            zero_util_rate,
            hundred_util_rate,
            points,
            curve_type,
            _pad0,
            _padding1,
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
pub struct InterestRateConfigCompact {
    pub insurance_fee_fixed_apr: WrappedI80F48,
    pub insurance_ir_fee: WrappedI80F48,
    pub protocol_fixed_fee_apr: WrappedI80F48,
    pub protocol_ir_fee: WrappedI80F48,
    pub protocol_origination_fee: WrappedI80F48,
    pub zero_util_rate: u32,
    pub hundred_util_rate: u32,
    pub points: [RatePoint; 5],
}
impl InterestRateConfigCompact {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let insurance_fee_fixed_apr = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let insurance_ir_fee = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let protocol_fixed_fee_apr = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let protocol_ir_fee = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let protocol_origination_fee = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let zero_util_rate: u32 = crate::borsh_de_or_default(&mut reader)?;
        let hundred_util_rate: u32 = crate::borsh_de_or_default(&mut reader)?;
        let points: [RatePoint; 5] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            insurance_fee_fixed_apr,
            insurance_ir_fee,
            protocol_fixed_fee_apr,
            protocol_ir_fee,
            protocol_origination_fee,
            zero_util_rate,
            hundred_util_rate,
            points,
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
pub struct InterestRateConfigOpt {
    pub insurance_fee_fixed_apr: Option<WrappedI80F48>,
    pub insurance_ir_fee: Option<WrappedI80F48>,
    pub protocol_fixed_fee_apr: Option<WrappedI80F48>,
    pub protocol_ir_fee: Option<WrappedI80F48>,
    pub protocol_origination_fee: Option<WrappedI80F48>,
    pub zero_util_rate: Option<u32>,
    pub hundred_util_rate: Option<u32>,
    pub points: Option<[RatePoint; 5]>,
}
impl InterestRateConfigOpt {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let insurance_fee_fixed_apr: Option<WrappedI80F48> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let insurance_ir_fee: Option<WrappedI80F48> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let protocol_fixed_fee_apr: Option<WrappedI80F48> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let protocol_ir_fee: Option<WrappedI80F48> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let protocol_origination_fee: Option<WrappedI80F48> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let zero_util_rate: Option<u32> = crate::borsh_de_or_default(&mut reader)?;
        let hundred_util_rate: Option<u32> = crate::borsh_de_or_default(&mut reader)?;
        let points: Option<[RatePoint; 5]> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            insurance_fee_fixed_apr,
            insurance_ir_fee,
            protocol_fixed_fee_apr,
            protocol_ir_fee,
            protocol_origination_fee,
            zero_util_rate,
            hundred_util_rate,
            points,
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
pub struct JuplendConfigCompact {
    pub oracle: Pubkey,
    pub asset_weight_init: WrappedI80F48,
    pub asset_weight_maint: WrappedI80F48,
    pub deposit_limit: u64,
    pub oracle_setup: OracleSetup,
    pub risk_tier: RiskTier,
    pub config_flags: u8,
    pub total_asset_value_init_limit: u64,
    pub oracle_max_age: u16,
    pub oracle_max_confidence: u32,
}
impl JuplendConfigCompact {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let oracle: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let asset_weight_init = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let asset_weight_maint = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let deposit_limit: u64 = crate::borsh_de_or_default(&mut reader)?;
        let oracle_setup: OracleSetup = crate::borsh_de_or_default(&mut reader)?;
        let risk_tier: RiskTier = crate::borsh_de_or_default(&mut reader)?;
        let config_flags: u8 = crate::borsh_de_or_default(&mut reader)?;
        let total_asset_value_init_limit: u64 = crate::borsh_de_or_default(&mut reader)?;
        let oracle_max_age: u16 = crate::borsh_de_or_default(&mut reader)?;
        let oracle_max_confidence: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            oracle,
            asset_weight_init,
            asset_weight_maint,
            deposit_limit,
            oracle_setup,
            risk_tier,
            config_flags,
            total_asset_value_init_limit,
            oracle_max_age,
            oracle_max_confidence,
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
pub struct KaminoConfigCompact {
    pub oracle: Pubkey,
    pub asset_weight_init: WrappedI80F48,
    pub asset_weight_maint: WrappedI80F48,
    pub deposit_limit: u64,
    pub oracle_setup: OracleSetup,
    pub operational_state: BankOperationalState,
    pub risk_tier: RiskTier,
    pub config_flags: u8,
    pub total_asset_value_init_limit: u64,
    pub oracle_max_age: u16,
    pub oracle_max_confidence: u32,
}
impl KaminoConfigCompact {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let oracle: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let asset_weight_init = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let asset_weight_maint = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let deposit_limit: u64 = crate::borsh_de_or_default(&mut reader)?;
        let oracle_setup: OracleSetup = crate::borsh_de_or_default(&mut reader)?;
        let operational_state: BankOperationalState = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let risk_tier: RiskTier = crate::borsh_de_or_default(&mut reader)?;
        let config_flags: u8 = crate::borsh_de_or_default(&mut reader)?;
        let total_asset_value_init_limit: u64 = crate::borsh_de_or_default(&mut reader)?;
        let oracle_max_age: u16 = crate::borsh_de_or_default(&mut reader)?;
        let oracle_max_confidence: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            oracle,
            asset_weight_init,
            asset_weight_maint,
            deposit_limit,
            oracle_setup,
            operational_state,
            risk_tier,
            config_flags,
            total_asset_value_init_limit,
            oracle_max_age,
            oracle_max_confidence,
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
pub struct LendingAccount {
    pub balances: [Balance; 16],
    pub last_tag_used: u16,
    pub _pad1: [u8; 6],
    pub _padding: [u64; 7],
}
impl LendingAccount {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let balances: [Balance; 16] = crate::borsh_de_or_default(&mut reader)?;
        let last_tag_used: u16 = crate::borsh_de_or_default(&mut reader)?;
        let _pad1: [u8; 6] = crate::borsh_de_or_default(&mut reader)?;
        let _padding: [u64; 7] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            balances,
            last_tag_used,
            _pad1,
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
pub struct LiquidationBalances {
    pub liquidatee_asset_balance: f64,
    pub liquidatee_liability_balance: f64,
    pub liquidator_asset_balance: f64,
    pub liquidator_liability_balance: f64,
}
impl LiquidationBalances {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let liquidatee_asset_balance: f64 = crate::borsh_de_or_default(&mut reader)?;
        let liquidatee_liability_balance: f64 = crate::borsh_de_or_default(&mut reader)?;
        let liquidator_asset_balance: f64 = crate::borsh_de_or_default(&mut reader)?;
        let liquidator_liability_balance: f64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            liquidatee_asset_balance,
            liquidatee_liability_balance,
            liquidator_asset_balance,
            liquidator_liability_balance,
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
pub struct LiquidationCache {
    pub asset_value_maint: WrappedI80F48,
    pub liability_value_maint: WrappedI80F48,
    pub asset_value_equity: WrappedI80F48,
    pub liability_value_equity: WrappedI80F48,
    pub _placeholder: u64,
    pub _reserved0: [u8; 32],
}
impl LiquidationCache {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let asset_value_maint = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let liability_value_maint = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let asset_value_equity = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let liability_value_equity = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let _placeholder: u64 = crate::borsh_de_or_default(&mut reader)?;
        let _reserved0: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            asset_value_maint,
            liability_value_maint,
            asset_value_equity,
            liability_value_equity,
            _placeholder,
            _reserved0,
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
pub struct LiquidationEntry {
    pub asset_amount_seized: [u8; 8],
    pub liab_amount_repaid: [u8; 8],
    pub placeholder0: u64,
    pub timestamp: i64,
    pub _reserved0: [u8; 16],
}
impl LiquidationEntry {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let asset_amount_seized: [u8; 8] = crate::borsh_de_or_default(&mut reader)?;
        let liab_amount_repaid: [u8; 8] = crate::borsh_de_or_default(&mut reader)?;
        let placeholder0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        let _reserved0: [u8; 16] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            asset_amount_seized,
            liab_amount_repaid,
            placeholder0,
            timestamp,
            _reserved0,
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
pub struct MinimalObligationCollateral {
    pub deposit_reserve: Pubkey,
    pub deposited_amount: u64,
    pub market_value_sf: [u8; 16],
    pub borrowed_amount_against_this_collateral_in_elevation_group: u64,
    pub padding: [u64; 9],
}
impl MinimalObligationCollateral {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let deposit_reserve: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let deposited_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let market_value_sf: [u8; 16] = crate::borsh_de_or_default(&mut reader)?;
        let borrowed_amount_against_this_collateral_in_elevation_group: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let padding: [u64; 9] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            deposit_reserve,
            deposited_amount,
            market_value_sf,
            borrowed_amount_against_this_collateral_in_elevation_group,
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
pub enum OracleSetup {
    #[default]
    None,
    PythLegacy,
    SwitchboardV2,
    PythPushOracle,
    SwitchboardPull,
    StakedWithPythPush,
    KaminoPythPush,
    KaminoSwitchboardPull,
    Fixed,
    DriftPythPull,
    DriftSwitchboardPull,
    SolendPythPull,
    SolendSwitchboardPull,
    FixedKamino,
    FixedDrift,
    JuplendPythPull,
    JuplendSwitchboardPull,
    FixedJuplend,
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
pub enum OrderTrigger {
    StopLoss { threshold: WrappedI80F48, max_slippage: u32 },
    TakeProfit { threshold: WrappedI80F48, max_slippage: u32 },
    Both { stop_loss: WrappedI80F48, take_profit: WrappedI80F48, max_slippage: u32 },
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
pub enum OrderTriggerType {
    #[default]
    StopLoss,
    TakeProfit,
    Both,
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
pub struct PanicState {
    pub pause_flags: u8,
    pub daily_pause_count: u8,
    pub consecutive_pause_count: u8,
    pub _reserved: [u8; 5],
    pub pause_start_timestamp: i64,
    pub last_daily_reset_timestamp: i64,
    pub _reserved_space: [u8; 8],
}
impl PanicState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pause_flags: u8 = crate::borsh_de_or_default(&mut reader)?;
        let daily_pause_count: u8 = crate::borsh_de_or_default(&mut reader)?;
        let consecutive_pause_count: u8 = crate::borsh_de_or_default(&mut reader)?;
        let _reserved: [u8; 5] = crate::borsh_de_or_default(&mut reader)?;
        let pause_start_timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        let last_daily_reset_timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        let _reserved_space: [u8; 8] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pause_flags,
            daily_pause_count,
            consecutive_pause_count,
            _reserved,
            pause_start_timestamp,
            last_daily_reset_timestamp,
            _reserved_space,
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
pub struct PanicStateCache {
    pub pause_flags: u8,
    pub _reserved: [u8; 7],
    pub pause_start_timestamp: i64,
    pub last_cache_update: i64,
}
impl PanicStateCache {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pause_flags: u8 = crate::borsh_de_or_default(&mut reader)?;
        let _reserved: [u8; 7] = crate::borsh_de_or_default(&mut reader)?;
        let pause_start_timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        let last_cache_update: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pause_flags,
            _reserved,
            pause_start_timestamp,
            last_cache_update,
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
pub struct RateLimitWindow {
    pub max_outflow: u64,
    pub window_duration: u64,
    pub window_start: i64,
    pub prev_window_outflow: i64,
    pub cur_window_outflow: i64,
}
impl RateLimitWindow {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let max_outflow: u64 = crate::borsh_de_or_default(&mut reader)?;
        let window_duration: u64 = crate::borsh_de_or_default(&mut reader)?;
        let window_start: i64 = crate::borsh_de_or_default(&mut reader)?;
        let prev_window_outflow: i64 = crate::borsh_de_or_default(&mut reader)?;
        let cur_window_outflow: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            max_outflow,
            window_duration,
            window_start,
            prev_window_outflow,
            cur_window_outflow,
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
pub struct RatePoint {
    pub util: u32,
    pub rate: u32,
}
impl RatePoint {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let util: u32 = crate::borsh_de_or_default(&mut reader)?;
        let rate: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { util, rate })
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
pub enum RiskTier {
    #[default]
    Collateral,
    Isolated,
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
pub struct SolendConfigCompact {
    pub oracle: Pubkey,
    pub asset_weight_init: WrappedI80F48,
    pub asset_weight_maint: WrappedI80F48,
    pub deposit_limit: u64,
    pub oracle_setup: OracleSetup,
    pub operational_state: BankOperationalState,
    pub risk_tier: RiskTier,
    pub config_flags: u8,
    pub total_asset_value_init_limit: u64,
    pub oracle_max_age: u16,
    pub oracle_max_confidence: u32,
}
impl SolendConfigCompact {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let oracle: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let asset_weight_init = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let asset_weight_maint = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let deposit_limit: u64 = crate::borsh_de_or_default(&mut reader)?;
        let oracle_setup: OracleSetup = crate::borsh_de_or_default(&mut reader)?;
        let operational_state: BankOperationalState = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let risk_tier: RiskTier = crate::borsh_de_or_default(&mut reader)?;
        let config_flags: u8 = crate::borsh_de_or_default(&mut reader)?;
        let total_asset_value_init_limit: u64 = crate::borsh_de_or_default(&mut reader)?;
        let oracle_max_age: u16 = crate::borsh_de_or_default(&mut reader)?;
        let oracle_max_confidence: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            oracle,
            asset_weight_init,
            asset_weight_maint,
            deposit_limit,
            oracle_setup,
            operational_state,
            risk_tier,
            config_flags,
            total_asset_value_init_limit,
            oracle_max_age,
            oracle_max_confidence,
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
pub enum SpotBalanceType {
    #[default]
    Deposit,
    Borrow,
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
pub struct SpotPosition {
    pub scaled_balance: u64,
    pub open_bids: i64,
    pub open_asks: i64,
    pub cumulative_deposits: i64,
    pub market_index: u16,
    pub balance_type: SpotBalanceType,
    pub open_orders: u8,
    pub padding: [u8; 4],
}
impl SpotPosition {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let scaled_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let open_bids: i64 = crate::borsh_de_or_default(&mut reader)?;
        let open_asks: i64 = crate::borsh_de_or_default(&mut reader)?;
        let cumulative_deposits: i64 = crate::borsh_de_or_default(&mut reader)?;
        let market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let balance_type: SpotBalanceType = crate::borsh_de_or_default(&mut reader)?;
        let open_orders: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 4] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            scaled_balance,
            open_bids,
            open_asks,
            cumulative_deposits,
            market_index,
            balance_type,
            open_orders,
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
pub struct StakedSettingsConfig {
    pub oracle: Pubkey,
    pub asset_weight_init: WrappedI80F48,
    pub asset_weight_maint: WrappedI80F48,
    pub deposit_limit: u64,
    pub total_asset_value_init_limit: u64,
    pub oracle_max_age: u16,
    pub risk_tier: RiskTier,
}
impl StakedSettingsConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let oracle: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let asset_weight_init = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let asset_weight_maint = if reader.is_empty() {
            Default::default()
        } else {
            <WrappedI80F48>::deserialize(&mut reader)?
        };
        let deposit_limit: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_asset_value_init_limit: u64 = crate::borsh_de_or_default(&mut reader)?;
        let oracle_max_age: u16 = crate::borsh_de_or_default(&mut reader)?;
        let risk_tier: RiskTier = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            oracle,
            asset_weight_init,
            asset_weight_maint,
            deposit_limit,
            total_asset_value_init_limit,
            oracle_max_age,
            risk_tier,
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
pub struct StakedSettingsEditConfig {
    pub oracle: Option<Pubkey>,
    pub asset_weight_init: Option<WrappedI80F48>,
    pub asset_weight_maint: Option<WrappedI80F48>,
    pub deposit_limit: Option<u64>,
    pub total_asset_value_init_limit: Option<u64>,
    pub oracle_max_age: Option<u16>,
    pub risk_tier: Option<RiskTier>,
}
impl StakedSettingsEditConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let oracle: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let asset_weight_init: Option<WrappedI80F48> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let asset_weight_maint: Option<WrappedI80F48> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let deposit_limit: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let total_asset_value_init_limit: Option<u64> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let oracle_max_age: Option<u16> = crate::borsh_de_or_default(&mut reader)?;
        let risk_tier: Option<RiskTier> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            oracle,
            asset_weight_init,
            asset_weight_maint,
            deposit_limit,
            total_asset_value_init_limit,
            oracle_max_age,
            risk_tier,
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
pub enum UserStatus {
    #[default]
    Active,
    BeingLiquidated,
    Bankrupt,
    ReduceOnly,
    AdvancedLp,
    ProtectedMakerOrders,
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
pub struct WithdrawWindowCache {
    pub daily_limit: u32,
    pub withdrawn_today: u32,
    pub last_daily_reset_timestamp: i64,
}
impl WithdrawWindowCache {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let daily_limit: u32 = crate::borsh_de_or_default(&mut reader)?;
        let withdrawn_today: u32 = crate::borsh_de_or_default(&mut reader)?;
        let last_daily_reset_timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            daily_limit,
            withdrawn_today,
            last_daily_reset_timestamp,
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
pub struct WrappedI80F48 {
    pub value: [u8; 16],
}
impl WrappedI80F48 {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let value: [u8; 16] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { value })
    }
}
