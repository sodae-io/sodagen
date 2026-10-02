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
pub struct APYConfig {
    pub fixed_apy_bps: u16,
    pub max_apy_bps: u16,
    pub padding0: [u8; 4],
    pub anchor_share_price: u64,
    pub anchor_ts: i64,
    pub last_settled_ts: i64,
    pub accrued_apy_balance: i64,
    pub padding1: [u64; 32],
}
impl APYConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let fixed_apy_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let max_apy_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let padding0: [u8; 4] = crate::borsh_de_or_default(&mut reader)?;
        let anchor_share_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let anchor_ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let last_settled_ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let accrued_apy_balance: i64 = crate::borsh_de_or_default(&mut reader)?;
        let padding1: [u64; 32] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            fixed_apy_bps,
            max_apy_bps,
            padding0,
            anchor_share_price,
            anchor_ts,
            last_settled_ts,
            accrued_apy_balance,
            padding1,
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
pub struct AssetPriceOracleConfigArgs {
    pub price_oracle_type: PriceOracleType,
    pub initial_price: u64,
    pub price_oracle_account: Pubkey,
    pub pyth_max_age_secs: u64,
}
impl AssetPriceOracleConfigArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let price_oracle_type: PriceOracleType = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let initial_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let price_oracle_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pyth_max_age_secs: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            price_oracle_type,
            initial_price,
            price_oracle_account,
            pyth_max_age_secs,
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
pub struct BankAccounting {
    pub yielding_tvl: u64,
    pub total_issued_supply: u64,
    pub max_yielding_tvl: u64,
    pub padding1: [u64; 13],
}
impl BankAccounting {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let yielding_tvl: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_issued_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_yielding_tvl: u64 = crate::borsh_de_or_default(&mut reader)?;
        let padding1: [u64; 13] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            yielding_tvl,
            total_issued_supply,
            max_yielding_tvl,
            padding1,
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
    pub padding2: [u64; 5],
}
impl BankConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let creator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let bank_manager: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let risk_manager: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let padding2: [u64; 5] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            creator,
            bank_manager,
            risk_manager,
            padding2,
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
    pub padding1: [u8; 7],
    pub padding2: [u64; 8],
}
impl BankMint {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pubkey: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding1: [u8; 7] = crate::borsh_de_or_default(&mut reader)?;
        let padding2: [u64; 8] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            pubkey,
            price,
            decimals,
            padding1,
            padding2,
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
    pub is_halted: VaultPodBool,
    pub is_halted_deposit: VaultPodBool,
    pub is_halted_withdrawal: VaultPodBool,
    pub padding1: [u8; 5],
    pub padding2: [u64; 4],
}
impl BankStatus {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let is_halted = if reader.is_empty() {
            Default::default()
        } else {
            <VaultPodBool>::deserialize(&mut reader)?
        };
        let is_halted_deposit = if reader.is_empty() {
            Default::default()
        } else {
            <VaultPodBool>::deserialize(&mut reader)?
        };
        let is_halted_withdrawal = if reader.is_empty() {
            Default::default()
        } else {
            <VaultPodBool>::deserialize(&mut reader)?
        };
        let padding1: [u8; 5] = crate::borsh_de_or_default(&mut reader)?;
        let padding2: [u64; 4] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            is_halted,
            is_halted_deposit,
            is_halted_withdrawal,
            padding1,
            padding2,
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
pub enum ConsensusOracleUpdate {
    #[default]
    Nav,
    Asset { mint: Pubkey },
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
pub struct CpiMapping {
    pub indices: Vec<u8>,
    pub lengths: Vec<u8>,
}
impl CpiMapping {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let indices: Vec<u8> = crate::borsh_de_or_default(&mut reader)?;
        let lengths: Vec<u8> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { indices, lengths })
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
pub struct CpiRefs {
    pub accounts: CpiMapping,
    pub types: Vec<u8>,
    pub args: Vec<u8>,
}
impl CpiRefs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let accounts = if reader.is_empty() {
            Default::default()
        } else {
            <CpiMapping>::deserialize(&mut reader)?
        };
        let types: Vec<u8> = crate::borsh_de_or_default(&mut reader)?;
        let args: Vec<u8> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { accounts, types, args })
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
pub struct CreateAssetHoldingArgs {
    pub price_oracle_type: u8,
    pub price_oracle_account: Pubkey,
}
impl CreateAssetHoldingArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let price_oracle_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let price_oracle_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            price_oracle_type,
            price_oracle_account,
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
pub struct ExternalLiquiditySlot {
    #[serde(with = "crate::big_array_serde")]
    pub data: [u8; 256],
}
impl ExternalLiquiditySlot {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let data = <[u8; 256] as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self { data })
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
pub enum ExternalLiquiditySource {
    #[default]
    None,
    Marginfi,
}
impl TryFrom<u8> for ExternalLiquiditySource {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::None),
            1u8 => Ok(Self::Marginfi),
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
pub enum FeeType {
    #[default]
    MintBurn,
    Performance,
    Unstake,
}
impl TryFrom<u8> for FeeType {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::MintBurn),
            1u8 => Ok(Self::Performance),
            2u8 => Ok(Self::Unstake),
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
pub struct InstructionRefs {
    pub cpi: CpiRefs,
    pub tracked: Vec<u8>,
}
impl InstructionRefs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let cpi = if reader.is_empty() {
            Default::default()
        } else {
            <CpiRefs>::deserialize(&mut reader)?
        };
        let tracked: Vec<u8> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { cpi, tracked })
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
pub struct JupiterSwapArgs {
    pub destination_price_oracle: AssetPriceOracleConfigArgs,
}
impl JupiterSwapArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let destination_price_oracle = if reader.is_empty() {
            Default::default()
        } else {
            <AssetPriceOracleConfigArgs>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self { destination_price_oracle })
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
pub enum LendingPlatform {
    #[default]
    Marginfi,
    Kamino,
}
impl TryFrom<u8> for LendingPlatform {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Marginfi),
            1u8 => Ok(Self::Kamino),
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
pub enum LendingType {
    #[default]
    LendingMarket,
    Vault,
    Multiply,
}
impl TryFrom<u8> for LendingType {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::LendingMarket),
            1u8 => Ok(Self::Vault),
            2u8 => Ok(Self::Multiply),
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
pub struct MigrateLegacyVaultArgs {
    pub migrate_amount: Option<u64>,
}
impl MigrateLegacyVaultArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let migrate_amount: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { migrate_amount })
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
pub enum MigratedMintKind {
    #[default]
    Main,
    Senior,
    Junior,
}
impl TryFrom<u8> for MigratedMintKind {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Main),
            1u8 => Ok(Self::Senior),
            2u8 => Ok(Self::Junior),
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
pub struct OracleConfig {
    pub price_max_ts_gap: u64,
    pub price_max_gap_bps: u16,
    pub yielding_mint_decimals: u8,
    pub padding1: [u8; 5],
    pub padding2: [u64; 4],
}
impl OracleConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let price_max_ts_gap: u64 = crate::borsh_de_or_default(&mut reader)?;
        let price_max_gap_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let yielding_mint_decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding1: [u8; 5] = crate::borsh_de_or_default(&mut reader)?;
        let padding2: [u64; 4] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            price_max_ts_gap,
            price_max_gap_bps,
            yielding_mint_decimals,
            padding1,
            padding2,
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
    pub padding1: [u8; 8],
    pub oracle_last_ts: [u64; 8],
    pub padding: [u64; 16],
}
impl OracleData {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let oracle_registry: [Pubkey; 8] = crate::borsh_de_or_default(&mut reader)?;
        let oracle_prices: [u64; 8] = crate::borsh_de_or_default(&mut reader)?;
        let oracle_pending_yield: [u64; 8] = crate::borsh_de_or_default(&mut reader)?;
        let oracle_veto: [u8; 8] = crate::borsh_de_or_default(&mut reader)?;
        let padding1: [u8; 8] = crate::borsh_de_or_default(&mut reader)?;
        let oracle_last_ts: [u64; 8] = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u64; 16] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            oracle_registry,
            oracle_prices,
            oracle_pending_yield,
            oracle_veto,
            padding1,
            oracle_last_ts,
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
pub struct OracleResult {
    pub yielding_token_price: u64,
    pub yielding_pending_amount: u64,
    pub update_ts: u64,
    pub start_marker_unix_seconds: u64,
    pub highest_confirmed_price: u64,
    pub padding1: [u64; 10],
}
impl OracleResult {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let yielding_token_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let yielding_pending_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let update_ts: u64 = crate::borsh_de_or_default(&mut reader)?;
        let start_marker_unix_seconds: u64 = crate::borsh_de_or_default(&mut reader)?;
        let highest_confirmed_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let padding1: [u64; 10] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            yielding_token_price,
            yielding_pending_amount,
            update_ts,
            start_marker_unix_seconds,
            highest_confirmed_price,
            padding1,
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
pub enum PriceOracleType {
    #[default]
    None,
    ConsensusOracle,
    Pyth,
    PerenaVault,
}
impl TryFrom<u8> for PriceOracleType {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::None),
            1u8 => Ok(Self::ConsensusOracle),
            2u8 => Ok(Self::Pyth),
            3u8 => Ok(Self::PerenaVault),
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
pub struct ProtocolInteractionArgs {
    pub external_liquidity_index: u8,
    pub affected_asset_mint: Option<Pubkey>,
}
impl ProtocolInteractionArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let external_liquidity_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let affected_asset_mint: Option<Pubkey> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            external_liquidity_index,
            affected_asset_mint,
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
pub struct SetAssetPriceOracleArgs {
    pub mint: Pubkey,
    pub price_oracle_type: PriceOracleType,
    pub price_oracle_account: Pubkey,
}
impl SetAssetPriceOracleArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let price_oracle_type: PriceOracleType = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let price_oracle_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            mint,
            price_oracle_type,
            price_oracle_account,
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
pub struct SetExternalLiquidityArgs {
    pub index: u8,
    pub source: ExternalLiquiditySource,
    pub user_account: Pubkey,
}
impl SetExternalLiquidityArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let source: ExternalLiquiditySource = crate::borsh_de_or_default(&mut reader)?;
        let user_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            index,
            source,
            user_account,
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
pub struct SetVaultConfigArgs {
    pub manager: Option<Pubkey>,
    pub hw_manager: Option<Pubkey>,
    pub cb_trigger: Option<Pubkey>,
    pub fulfiller: Option<Pubkey>,
    pub fee_collector: Option<Pubkey>,
    pub mint_fee_bps: Option<u16>,
    pub burn_fee_bps: Option<u16>,
    pub performance_fee_bps: Option<u16>,
    pub fixed_apy_bps: Option<u16>,
    pub max_apy_bps: Option<u16>,
    pub tvl_limit: Option<u64>,
    pub losses_enabled: Option<bool>,
    pub price_staleness_threshold_secs: Option<i64>,
    pub new_curator: Option<Pubkey>,
    pub user_burn_limit: Option<RollingLimitConfig>,
    pub manager_pull_limit: Option<RollingLimitConfig>,
}
impl SetVaultConfigArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let manager: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let hw_manager: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let cb_trigger: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let fulfiller: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let fee_collector: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let mint_fee_bps: Option<u16> = crate::borsh_de_or_default(&mut reader)?;
        let burn_fee_bps: Option<u16> = crate::borsh_de_or_default(&mut reader)?;
        let performance_fee_bps: Option<u16> = crate::borsh_de_or_default(&mut reader)?;
        let fixed_apy_bps: Option<u16> = crate::borsh_de_or_default(&mut reader)?;
        let max_apy_bps: Option<u16> = crate::borsh_de_or_default(&mut reader)?;
        let tvl_limit: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let losses_enabled: Option<bool> = crate::borsh_de_or_default(&mut reader)?;
        let price_staleness_threshold_secs: Option<i64> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let new_curator: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let user_burn_limit: Option<RollingLimitConfig> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let manager_pull_limit: Option<RollingLimitConfig> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            manager,
            hw_manager,
            cb_trigger,
            fulfiller,
            fee_collector,
            mint_fee_bps,
            burn_fee_bps,
            performance_fee_bps,
            fixed_apy_bps,
            max_apy_bps,
            tvl_limit,
            losses_enabled,
            price_staleness_threshold_secs,
            new_curator,
            user_burn_limit,
            manager_pull_limit,
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
pub struct TeamAccounting {
    pub net_yielding_to_deposit_back: u64,
    pub mint_burn_fees: u64,
    pub performance_fees: u64,
    pub unstake_fees: u64,
    pub withdraw_to_invest_limit: VaultRollingWithdrawalLimit,
    pub padding2: [u64; 11],
}
impl TeamAccounting {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let net_yielding_to_deposit_back: u64 = crate::borsh_de_or_default(&mut reader)?;
        let mint_burn_fees: u64 = crate::borsh_de_or_default(&mut reader)?;
        let performance_fees: u64 = crate::borsh_de_or_default(&mut reader)?;
        let unstake_fees: u64 = crate::borsh_de_or_default(&mut reader)?;
        let withdraw_to_invest_limit = if reader.is_empty() {
            Default::default()
        } else {
            <VaultRollingWithdrawalLimit>::deserialize(&mut reader)?
        };
        let padding2: [u64; 11] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            net_yielding_to_deposit_back,
            mint_burn_fees,
            performance_fees,
            unstake_fees,
            withdraw_to_invest_limit,
            padding2,
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
    pub padding: [u64; 12],
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
        let padding: [u64; 12] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            bank,
            vault,
            creator,
            yielding_fees_ata,
            fee_gatherer,
            yield_manager,
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
pub struct TimelockState {
    pub time_created_ts: u64,
    pub touch_cooldown_secs: u64,
    pub cleanup_cooldown_secs: u64,
    pub enabled: CommonPodBool,
    pub padding1: [u8; 7],
}
impl TimelockState {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let time_created_ts: u64 = crate::borsh_de_or_default(&mut reader)?;
        let touch_cooldown_secs: u64 = crate::borsh_de_or_default(&mut reader)?;
        let cleanup_cooldown_secs: u64 = crate::borsh_de_or_default(&mut reader)?;
        let enabled = if reader.is_empty() {
            Default::default()
        } else {
            <CommonPodBool>::deserialize(&mut reader)?
        };
        let padding1: [u8; 7] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            time_created_ts,
            touch_cooldown_secs,
            cleanup_cooldown_secs,
            enabled,
            padding1,
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
pub enum TokenProgram {
    #[default]
    Spl,
    Token2022,
}
impl TryFrom<u8> for TokenProgram {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Spl),
            1u8 => Ok(Self::Token2022),
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
pub struct TrancheAccounting {
    pub shares: u64,
    pub staked_amount: u64,
    pub virtual_value: u64,
    pub cumulative_loss_absorbed: u64,
    pub last_update_ts: i64,
    pub share_price: u64,
    pub protocol_fees_accrued: u64,
    pub has_reached_recovery_threshold: u8,
    pub padding1: [u8; 7],
    pub padding2: [u64; 8],
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
        let padding1: [u8; 7] = crate::borsh_de_or_default(&mut reader)?;
        let padding2: [u64; 8] = crate::borsh_de_or_default(&mut reader)?;
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
            padding1,
            padding2,
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
    pub padding1: [u8; 1],
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
    pub low_stake_fee_enabled: VaultPodBool,
    pub padding2: [u8; 1],
    pub padding3: [u64; 13],
}
impl TrancheConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let share_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let share_price_decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding1: [u8; 1] = crate::borsh_de_or_default(&mut reader)?;
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
            <VaultPodBool>::deserialize(&mut reader)?
        };
        let padding2: [u8; 1] = crate::borsh_de_or_default(&mut reader)?;
        let padding3: [u64; 13] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            mint,
            share_mint,
            share_price_decimals,
            padding1,
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
            padding2,
            padding3,
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
pub enum TrancheKind {
    #[default]
    Junior,
    Senior,
}
impl TryFrom<u8> for TrancheKind {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::Junior),
            1u8 => Ok(Self::Senior),
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
pub struct TransferMigratedMintAuthorityArgs {
    pub kind: MigratedMintKind,
}
impl TransferMigratedMintAuthorityArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let kind: MigratedMintKind = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { kind })
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
pub struct UpdateAssetPriceArgs {
    pub mint: Pubkey,
    pub max_age_secs: u64,
}
impl UpdateAssetPriceArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let max_age_secs: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { mint, max_age_secs })
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
pub struct UpdateConsensusOracleArgs {
    pub updates: Vec<ConsensusAssetUpdate>,
}
impl UpdateConsensusOracleArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let updates: Vec<ConsensusAssetUpdate> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self { updates })
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
pub struct UpdateConsensusSignersArgs {
    pub signers: Vec<Pubkey>,
}
impl UpdateConsensusSignersArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let signers: Vec<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { signers })
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
    pub max_burn_for_yielding_limit: Option<u64>,
    pub max_withdraw_to_invest_limit: Option<u64>,
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
        let max_burn_for_yielding_limit: Option<u64> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let max_withdraw_to_invest_limit: Option<u64> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            yield_manager,
            risk_manager,
            fee_controller,
            permissioned_users,
            max_burn_for_yielding_limit,
            max_withdraw_to_invest_limit,
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
pub struct VaultCreateTrancheStateArgs {
    pub senior_fixed_apy_bps: u16,
    pub early_unstake_fee_bps: u16,
    pub standard_unstake_fee_bps: u16,
    pub target_lockup_duration_secs: i64,
    pub junior_deposit_cap: u64,
    pub for_migration: bool,
}
impl VaultCreateTrancheStateArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let senior_fixed_apy_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let early_unstake_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let standard_unstake_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let target_lockup_duration_secs: i64 = crate::borsh_de_or_default(&mut reader)?;
        let junior_deposit_cap: u64 = crate::borsh_de_or_default(&mut reader)?;
        let for_migration: bool = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            senior_fixed_apy_bps,
            early_unstake_fee_bps,
            standard_unstake_fee_bps,
            target_lockup_duration_secs,
            junior_deposit_cap,
            for_migration,
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
pub struct VaultCreateVaultArgs {
    pub vault_id: u16,
    pub hw_manager: Pubkey,
    pub cb_trigger: Pubkey,
    pub fee_collector: Pubkey,
    pub base_asset_mint: Pubkey,
    pub mint_fee_bps: u16,
    pub burn_fee_bps: u16,
    pub performance_fee_bps: u16,
    pub fixed_apy_bps: u16,
    pub max_apy_bps: u16,
    pub tvl_limit: u64,
    pub losses_enabled: bool,
    pub initial_mint_share_price: Option<u64>,
    pub base_price_oracle_type: u8,
    pub base_price: u64,
    pub base_price_oracle_account: Pubkey,
    pub for_migration: bool,
}
impl VaultCreateVaultArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let vault_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let hw_manager: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let cb_trigger: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_collector: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_asset_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let burn_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let performance_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let fixed_apy_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let max_apy_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let tvl_limit: u64 = crate::borsh_de_or_default(&mut reader)?;
        let losses_enabled: bool = crate::borsh_de_or_default(&mut reader)?;
        let initial_mint_share_price: Option<u64> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let base_price_oracle_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let base_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let base_price_oracle_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let for_migration: bool = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            vault_id,
            hw_manager,
            cb_trigger,
            fee_collector,
            base_asset_mint,
            mint_fee_bps,
            burn_fee_bps,
            performance_fee_bps,
            fixed_apy_bps,
            max_apy_bps,
            tvl_limit,
            losses_enabled,
            initial_mint_share_price,
            base_price_oracle_type,
            base_price,
            base_price_oracle_account,
            for_migration,
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
pub struct VaultFees {
    pub mint_fee_bps: u16,
    pub burn_fee_bps: u16,
    pub performance_fee_bps: u16,
    pub protocol_fee_bps: u16,
    pub padding2: [u64; 32],
}
impl VaultFees {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mint_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let burn_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let performance_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let padding2: [u64; 32] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            mint_fee_bps,
            burn_fee_bps,
            performance_fee_bps,
            protocol_fee_bps,
            padding2,
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
pub struct VaultHolding {
    pub mint: Pubkey,
    pub price: u64,
    pub last_update_ts: i64,
    pub decimals: u8,
    pub token_program: TokenProgram,
    pub price_oracle_type: PriceOracleType,
    pub is_base: CommonPodBool,
    pub padding1: [u8; 4],
    pub price_oracle_account: Pubkey,
    pub local_amount: u64,
    pub external_amount: u64,
    pub timelock: TimelockState,
    pub last_rebalance_ts: i64,
    pub padding2: [u64; 5],
}
impl VaultHolding {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_update_ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let token_program: TokenProgram = crate::borsh_de_or_default(&mut reader)?;
        let price_oracle_type: PriceOracleType = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let is_base = if reader.is_empty() {
            Default::default()
        } else {
            <CommonPodBool>::deserialize(&mut reader)?
        };
        let padding1: [u8; 4] = crate::borsh_de_or_default(&mut reader)?;
        let price_oracle_account: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let local_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let external_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let timelock = if reader.is_empty() {
            Default::default()
        } else {
            <TimelockState>::deserialize(&mut reader)?
        };
        let last_rebalance_ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let padding2: [u64; 5] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            mint,
            price,
            last_update_ts,
            decimals,
            token_program,
            price_oracle_type,
            is_base,
            padding1,
            price_oracle_account,
            local_amount,
            external_amount,
            timelock,
            last_rebalance_ts,
            padding2,
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
    pub legacy_padding1: [u64; 16],
    pub legacy_padding2: [i64; 16],
    pub legacy_padding3: [Pubkey; 16],
    pub new_index: u8,
    pub padding1: [u8; 7],
    pub padding: [u64; 12],
}
impl VaultManagement {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let legacy_padding1: [u64; 16] = crate::borsh_de_or_default(&mut reader)?;
        let legacy_padding2: [i64; 16] = crate::borsh_de_or_default(&mut reader)?;
        let legacy_padding3: [Pubkey; 16] = crate::borsh_de_or_default(&mut reader)?;
        let new_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding1: [u8; 7] = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u64; 12] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            legacy_padding1,
            legacy_padding2,
            legacy_padding3,
            new_index,
            padding1,
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
pub enum VaultOracleType {
    #[default]
    ConsensusOracle,
}
impl TryFrom<u8> for VaultOracleType {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::ConsensusOracle),
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
pub struct VaultReallocationArgs {
    pub amount: u64,
    pub asset_price_oracle: AssetPriceOracleConfigArgs,
}
impl VaultReallocationArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let asset_price_oracle = if reader.is_empty() {
            Default::default()
        } else {
            <AssetPriceOracleConfigArgs>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self { amount, asset_price_oracle })
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
pub struct VaultRoles {
    pub curator: Pubkey,
    pub manager: Pubkey,
    pub hw_manager: Pubkey,
    pub cb_trigger: Pubkey,
    pub fulfiller: Pubkey,
    pub fee_collector: Pubkey,
    pub padding1: [Pubkey; 10],
}
impl VaultRoles {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let curator: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let manager: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let hw_manager: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let cb_trigger: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fulfiller: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_collector: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let padding1: [Pubkey; 10] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            curator,
            manager,
            hw_manager,
            cb_trigger,
            fulfiller,
            fee_collector,
            padding1,
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
pub struct PendingVaultRoleUpdates {
    pub updates: [PendingVaultRoleUpdate; 2],
    pub format_tag: [u8; 8],
    pub padding1: [u64; 12],
    pub pending_fields: u8,
    pub padding2: [u8; 7],
}
impl PendingVaultRoleUpdates {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let updates: [PendingVaultRoleUpdate; 2] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let format_tag: [u8; 8] = crate::borsh_de_or_default(&mut reader)?;
        let padding1: [u64; 12] = crate::borsh_de_or_default(&mut reader)?;
        let pending_fields: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding2: [u8; 7] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            updates,
            format_tag,
            padding1,
            pending_fields,
            padding2,
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
    pub is_halted: VaultPodBool,
    pub is_halted_deposit: VaultPodBool,
    pub is_halted_withdrawal: VaultPodBool,
    pub losses_accepted: VaultPodBool,
    pub is_migrated: VaultPodBool,
    pub padding1: [u8; 3],
    pub padding: [u64; 12],
}
impl VaultStatus {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let is_halted = if reader.is_empty() {
            Default::default()
        } else {
            <VaultPodBool>::deserialize(&mut reader)?
        };
        let is_halted_deposit = if reader.is_empty() {
            Default::default()
        } else {
            <VaultPodBool>::deserialize(&mut reader)?
        };
        let is_halted_withdrawal = if reader.is_empty() {
            Default::default()
        } else {
            <VaultPodBool>::deserialize(&mut reader)?
        };
        let losses_accepted = if reader.is_empty() {
            Default::default()
        } else {
            <VaultPodBool>::deserialize(&mut reader)?
        };
        let is_migrated = if reader.is_empty() {
            Default::default()
        } else {
            <VaultPodBool>::deserialize(&mut reader)?
        };
        let padding1: [u8; 3] = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u64; 12] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            is_halted,
            is_halted_deposit,
            is_halted_withdrawal,
            losses_accepted,
            is_migrated,
            padding1,
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
pub struct VaultTrancheAccounting {
    pub value: u64,
    pub total_supply: u64,
    pub share_price: u64,
    pub last_update_ts: i64,
    pub cumulative_loss_absorbed: u64,
    pub protocol_fees_accrued: u64,
    pub hwm_share_price: u64,
    pub padding1: [u64; 13],
}
impl VaultTrancheAccounting {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let value: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let share_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_update_ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let cumulative_loss_absorbed: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fees_accrued: u64 = crate::borsh_de_or_default(&mut reader)?;
        let hwm_share_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let padding1: [u64; 13] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            value,
            total_supply,
            share_price,
            last_update_ts,
            cumulative_loss_absorbed,
            protocol_fees_accrued,
            hwm_share_price,
            padding1,
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
pub struct VaultTrancheConfig {
    pub junior_mint: Pubkey,
    pub senior_mint: Pubkey,
    pub senior_fixed_apy_bps: u16,
    pub early_unstake_fee_bps: u16,
    pub standard_unstake_fee_bps: u16,
    pub junior_mint_token_program: TokenProgram,
    pub senior_mint_token_program: TokenProgram,
    pub target_lockup_duration_secs: i64,
    pub padding2: [u64; 14],
}
impl VaultTrancheConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let junior_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let senior_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let senior_fixed_apy_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let early_unstake_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let standard_unstake_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let junior_mint_token_program: TokenProgram = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let senior_mint_token_program: TokenProgram = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let target_lockup_duration_secs: i64 = crate::borsh_de_or_default(&mut reader)?;
        let padding2: [u64; 14] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            junior_mint,
            senior_mint,
            senior_fixed_apy_bps,
            early_unstake_fee_bps,
            standard_unstake_fee_bps,
            junior_mint_token_program,
            senior_mint_token_program,
            target_lockup_duration_secs,
            padding2,
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
pub struct VaultUpdateTrancheConfigArgs {
    pub senior_fixed_apy_bps: Option<u16>,
    pub early_unstake_fee_bps: Option<u16>,
    pub standard_unstake_fee_bps: Option<u16>,
    pub target_lockup_duration_secs: Option<i64>,
}
impl VaultUpdateTrancheConfigArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let senior_fixed_apy_bps: Option<u16> = crate::borsh_de_or_default(&mut reader)?;
        let early_unstake_fee_bps: Option<u16> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let standard_unstake_fee_bps: Option<u16> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let target_lockup_duration_secs: Option<i64> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            senior_fixed_apy_bps,
            early_unstake_fee_bps,
            standard_unstake_fee_bps,
            target_lockup_duration_secs,
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
pub struct WithdrawalQueueHeader {
    pub bump: u8,
    pub queue_id: u8,
    pub padding: [u8; 6],
}
impl WithdrawalQueueHeader {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let queue_id: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 6] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { bump, queue_id, padding })
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
pub struct WithdrawalQueueMints {
    pub input_mint: Pubkey,
    pub output_mint: Pubkey,
}
impl WithdrawalQueueMints {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let input_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let output_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { input_mint, output_mint })
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
pub struct WithdrawalQueueParties {
    pub owner: Pubkey,
    pub vault: Pubkey,
    pub payer: Pubkey,
}
impl WithdrawalQueueParties {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let payer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { owner, vault, payer })
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
pub struct WithdrawalQueueRequest {
    pub input_amount: u64,
    pub request_ts: i64,
    pub expiry_ts: i64,
    pub padding: [u64; 12],
}
impl WithdrawalQueueRequest {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let input_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let request_ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let expiry_ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u64; 12] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            input_amount,
            request_ts,
            expiry_ts,
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
pub struct BankinecoRollingWithdrawalLimit {
    pub window_start_ts: u64,
    pub amount_in_window: u64,
    pub window_limit: u64,
}
impl BankinecoRollingWithdrawalLimit {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let window_start_ts: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_in_window: u64 = crate::borsh_de_or_default(&mut reader)?;
        let window_limit: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            window_start_ts,
            amount_in_window,
            window_limit,
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
pub struct BankinecoVaultAccounting {
    pub yielding_reserve_amount: u64,
    pub external_yielding_amount: u64,
    pub yielding_tvl_cap: u64,
    pub legacy_padding1: u64,
    pub legacy_padding2: u64,
    pub yielding_tvl: u64,
    pub reserve_at_last_fee_extraction: u64,
    pub burn_for_yielding_limit: BankinecoRollingWithdrawalLimit,
    pub padding: [u64; 8],
}
impl BankinecoVaultAccounting {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let yielding_reserve_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let external_yielding_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let yielding_tvl_cap: u64 = crate::borsh_de_or_default(&mut reader)?;
        let legacy_padding1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let legacy_padding2: u64 = crate::borsh_de_or_default(&mut reader)?;
        let yielding_tvl: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reserve_at_last_fee_extraction: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let burn_for_yielding_limit = if reader.is_empty() {
            Default::default()
        } else {
            <BankinecoRollingWithdrawalLimit>::deserialize(&mut reader)?
        };
        let padding: [u64; 8] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            yielding_reserve_amount,
            external_yielding_amount,
            yielding_tvl_cap,
            legacy_padding1,
            legacy_padding2,
            yielding_tvl,
            reserve_at_last_fee_extraction,
            burn_for_yielding_limit,
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
pub struct BankinecoVaultConfig {
    pub creator: Pubkey,
    pub bank: Pubkey,
    pub team_account: Pubkey,
    pub oracle_state: Pubkey,
    pub risk_manager: Pubkey,
    pub yielding_token_mint: Pubkey,
    pub yielding_vault_ata: Pubkey,
    pub yielding_mint_decimals: u8,
    pub padding1: [u8; 7],
    pub fee_controller: Pubkey,
    pub permissioned_users: [Pubkey; 3],
    pub padding2: u16,
    pub performance_fee_bps: u16,
    pub minting_fee_bps: u16,
    pub burning_fee_bps: u16,
    pub lp_account_index: u16,
    pub lp_third_party_id: u16,
    pub lending_platform: LendingPlatform,
    pub lending_type: LendingType,
    pub padding4: [u8; 2],
    pub padding: [u64; 11],
}
impl BankinecoVaultConfig {
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
        let padding1: [u8; 7] = crate::borsh_de_or_default(&mut reader)?;
        let fee_controller: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let permissioned_users: [Pubkey; 3] = crate::borsh_de_or_default(&mut reader)?;
        let padding2: u16 = crate::borsh_de_or_default(&mut reader)?;
        let performance_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let minting_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let burning_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let lp_account_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let lp_third_party_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let lending_platform: LendingPlatform = crate::borsh_de_or_default(&mut reader)?;
        let lending_type: LendingType = crate::borsh_de_or_default(&mut reader)?;
        let padding4: [u8; 2] = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u64; 11] = crate::borsh_de_or_default(&mut reader)?;
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
            padding1,
            fee_controller,
            permissioned_users,
            padding2,
            performance_fee_bps,
            minting_fee_bps,
            burning_fee_bps,
            lp_account_index,
            lp_third_party_id,
            lending_platform,
            lending_type,
            padding4,
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
pub struct BankinecoPodBool {
    pub field_0: u8,
}
impl BankinecoPodBool {
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
pub enum BankinecoVaultType {
    #[default]
    YieldBearing,
    CollateralizedExternalYield,
    UnCollateralizedExternalYield,
    AtomicLending,
}
impl TryFrom<u8> for BankinecoVaultType {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::YieldBearing),
            1u8 => Ok(Self::CollateralizedExternalYield),
            2u8 => Ok(Self::UnCollateralizedExternalYield),
            3u8 => Ok(Self::AtomicLending),
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
pub struct CommonVaultAccounting {
    pub tvl: u64,
    pub total_mint_supply: u64,
    pub mint_share_price: u64,
    pub last_update_ts: i64,
    pub hwm_share_price: u64,
    pub hwm_updated_ts: i64,
    pub user_burn_limit: CommonRollingWithdrawalLimit,
    pub manager_pull_limit: CommonRollingWithdrawalLimit,
    pub padding1: [u64; 32],
    pub padding2: [u64; 24],
}
impl CommonVaultAccounting {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let tvl: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_mint_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        let mint_share_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_update_ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let hwm_share_price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let hwm_updated_ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let user_burn_limit = if reader.is_empty() {
            Default::default()
        } else {
            <CommonRollingWithdrawalLimit>::deserialize(&mut reader)?
        };
        let manager_pull_limit = if reader.is_empty() {
            Default::default()
        } else {
            <CommonRollingWithdrawalLimit>::deserialize(&mut reader)?
        };
        let padding1: [u64; 32] = crate::borsh_de_or_default(&mut reader)?;
        let padding2: [u64; 24] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            tvl,
            total_mint_supply,
            mint_share_price,
            last_update_ts,
            hwm_share_price,
            hwm_updated_ts,
            user_burn_limit,
            manager_pull_limit,
            padding1,
            padding2,
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
pub struct CommonVaultConfig {
    pub reserved_vault_type: u8,
    pub padding1: [u8; 7],
    pub reserved_strict_asset_mint: Pubkey,
    pub asset_decimals: u8,
    pub padding2: [u8; 7],
    pub fees: VaultFees,
    pub apy: APYConfig,
    pub tvl_limit: u64,
    pub price_staleness_threshold_secs: i64,
    pub padding3: [u64; 22],
}
impl CommonVaultConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let reserved_vault_type: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding1: [u8; 7] = crate::borsh_de_or_default(&mut reader)?;
        let reserved_strict_asset_mint: Pubkey = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let asset_decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding2: [u8; 7] = crate::borsh_de_or_default(&mut reader)?;
        let fees = if reader.is_empty() {
            Default::default()
        } else {
            <VaultFees>::deserialize(&mut reader)?
        };
        let apy = if reader.is_empty() {
            Default::default()
        } else {
            <APYConfig>::deserialize(&mut reader)?
        };
        let tvl_limit: u64 = crate::borsh_de_or_default(&mut reader)?;
        let price_staleness_threshold_secs: i64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let padding3: [u64; 22] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            reserved_vault_type,
            padding1,
            reserved_strict_asset_mint,
            asset_decimals,
            padding2,
            fees,
            apy,
            tvl_limit,
            price_staleness_threshold_secs,
            padding3,
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
pub struct CommonRollingWithdrawalLimit {
    pub window_start_ts: i64,
    pub window_duration_seconds: u64,
    pub amount_in_window: u64,
    pub window_limit: u64,
    pub padding1: [u64; 8],
}
impl CommonRollingWithdrawalLimit {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let window_start_ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        let window_duration_seconds: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_in_window: u64 = crate::borsh_de_or_default(&mut reader)?;
        let window_limit: u64 = crate::borsh_de_or_default(&mut reader)?;
        let padding1: [u64; 8] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            window_start_ts,
            window_duration_seconds,
            amount_in_window,
            window_limit,
            padding1,
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
pub struct CommonPodBool {
    pub field_0: u8,
}
impl CommonPodBool {
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
pub enum CommonVaultType {
    #[default]
    ProgramManaged,
    ExternallyManaged,
}
impl TryFrom<u8> for CommonVaultType {
    type Error = std::io::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Self::ProgramManaged),
            1u8 => Ok(Self::ExternallyManaged),
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
pub struct SetProtocolFeeArgs {
    pub protocol_fee_bps: u16,
}
impl SetProtocolFeeArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let protocol_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { protocol_fee_bps })
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
pub struct ConsensusAssetUpdate {
    pub holding_index: u8,
    pub mint: Option<Pubkey>,
    pub price: u64,
    pub external_amount: u64,
}
impl ConsensusAssetUpdate {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let holding_index: u8 = crate::borsh_de_or_default(&mut reader)?;
        let mint: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let price: u64 = crate::borsh_de_or_default(&mut reader)?;
        let external_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            holding_index,
            mint,
            price,
            external_amount,
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
pub struct InitializeVaultRolesArgs {
    pub manager: Pubkey,
    pub fulfiller: Pubkey,
}
impl InitializeVaultRolesArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let manager: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fulfiller: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { manager, fulfiller })
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
pub struct ManagerRedepositAssetArgs {
    pub deposit_amount: u64,
    pub external_amount: u64,
}
impl ManagerRedepositAssetArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let deposit_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let external_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            deposit_amount,
            external_amount,
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
pub struct PendingVaultRoleUpdate {
    pub role: u8,
    pub padding1: [u8; 7],
    pub value: Pubkey,
    pub effective_ts: i64,
}
impl PendingVaultRoleUpdate {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let role: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding1: [u8; 7] = crate::borsh_de_or_default(&mut reader)?;
        let value: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let effective_ts: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            role,
            padding1,
            value,
            effective_ts,
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
pub struct RollingLimitConfig {
    pub window_limit: u64,
    pub window_duration_seconds: u64,
}
impl RollingLimitConfig {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let window_limit: u64 = crate::borsh_de_or_default(&mut reader)?;
        let window_duration_seconds: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            window_limit,
            window_duration_seconds,
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
pub struct VaultRollingWithdrawalLimit {
    pub window_start_ts: u64,
    pub amount_in_window: u64,
    pub window_limit: u64,
}
impl VaultRollingWithdrawalLimit {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let window_start_ts: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount_in_window: u64 = crate::borsh_de_or_default(&mut reader)?;
        let window_limit: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            window_start_ts,
            amount_in_window,
            window_limit,
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
pub struct VaultVaultAccounting {
    pub yielding_reserve_amount: u64,
    pub external_yielding_amount: u64,
    pub yielding_tvl_cap: u64,
    pub legacy_padding1: u64,
    pub legacy_padding2: u64,
    pub yielding_tvl: u64,
    pub reserve_at_last_fee_extraction: u64,
    pub burn_for_yielding_limit: VaultRollingWithdrawalLimit,
    pub padding: [u64; 8],
}
impl VaultVaultAccounting {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let yielding_reserve_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let external_yielding_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let yielding_tvl_cap: u64 = crate::borsh_de_or_default(&mut reader)?;
        let legacy_padding1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let legacy_padding2: u64 = crate::borsh_de_or_default(&mut reader)?;
        let yielding_tvl: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reserve_at_last_fee_extraction: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let burn_for_yielding_limit = if reader.is_empty() {
            Default::default()
        } else {
            <VaultRollingWithdrawalLimit>::deserialize(&mut reader)?
        };
        let padding: [u64; 8] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            yielding_reserve_amount,
            external_yielding_amount,
            yielding_tvl_cap,
            legacy_padding1,
            legacy_padding2,
            yielding_tvl,
            reserve_at_last_fee_extraction,
            burn_for_yielding_limit,
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
pub struct VaultVaultConfig {
    pub creator: Pubkey,
    pub bank: Pubkey,
    pub team_account: Pubkey,
    pub oracle_state: Pubkey,
    pub risk_manager: Pubkey,
    pub yielding_token_mint: Pubkey,
    pub yielding_vault_ata: Pubkey,
    pub yielding_mint_decimals: u8,
    pub padding1: [u8; 7],
    pub fee_controller: Pubkey,
    pub permissioned_users: [Pubkey; 3],
    pub padding2: u16,
    pub performance_fee_bps: u16,
    pub minting_fee_bps: u16,
    pub burning_fee_bps: u16,
    pub lp_account_index: u16,
    pub lp_third_party_id: u16,
    pub lending_platform: LendingPlatform,
    pub lending_type: LendingType,
    pub padding4: [u8; 2],
    pub padding: [u64; 11],
}
impl VaultVaultConfig {
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
        let padding1: [u8; 7] = crate::borsh_de_or_default(&mut reader)?;
        let fee_controller: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let permissioned_users: [Pubkey; 3] = crate::borsh_de_or_default(&mut reader)?;
        let padding2: u16 = crate::borsh_de_or_default(&mut reader)?;
        let performance_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let minting_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let burning_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let lp_account_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let lp_third_party_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let lending_platform: LendingPlatform = crate::borsh_de_or_default(&mut reader)?;
        let lending_type: LendingType = crate::borsh_de_or_default(&mut reader)?;
        let padding4: [u8; 2] = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u64; 11] = crate::borsh_de_or_default(&mut reader)?;
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
            padding1,
            fee_controller,
            permissioned_users,
            padding2,
            performance_fee_bps,
            minting_fee_bps,
            burning_fee_bps,
            lp_account_index,
            lp_third_party_id,
            lending_platform,
            lending_type,
            padding4,
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
pub struct VaultPodBool {
    pub field_0: u8,
}
impl VaultPodBool {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let field_0: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { field_0 })
    }
}
