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
pub struct ChestStrategyDepositArgs {
    pub amount: u64,
    pub fail_on_low_balance: bool,
    pub preserve_recorded_balance: bool,
}
impl ChestStrategyDepositArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fail_on_low_balance: bool = crate::borsh_de_or_default(&mut reader)?;
        let preserve_recorded_balance: bool = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            amount,
            fail_on_low_balance,
            preserve_recorded_balance,
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
pub struct ChestStrategyInitArgs {
    pub name: String,
}
impl ChestStrategyInitArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { name })
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
pub struct ChestStrategyRequestWithdrawArgs {
    pub asset_amount: u64,
}
impl ChestStrategyRequestWithdrawArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let asset_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { asset_amount })
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
pub struct ChestStrategyWithdrawArgs {
    pub fail_on_low_balance: bool,
    pub preserve_recorded_balance: bool,
}
impl ChestStrategyWithdrawArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let fail_on_low_balance: bool = crate::borsh_de_or_default(&mut reader)?;
        let preserve_recorded_balance: bool = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            fail_on_low_balance,
            preserve_recorded_balance,
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
pub struct ClendSupplyStrategyDepositArgs {
    pub amount: u64,
}
impl ClendSupplyStrategyDepositArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { amount })
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
pub struct ClendSupplyStrategyInitArgs {
    pub name: String,
}
impl ClendSupplyStrategyInitArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { name })
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
pub struct ClendSupplyStrategyWithdrawArgs {
    pub amount: u64,
}
impl ClendSupplyStrategyWithdrawArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { amount })
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
pub struct DriftInsuranceFundStrategyInitArgs {
    pub name: String,
    pub drift_market_index: u16,
}
impl DriftInsuranceFundStrategyInitArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        let drift_market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { name, drift_market_index })
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
pub struct DriftInsuranceFundStrategyStakeArgs {
    pub amount: u64,
}
impl DriftInsuranceFundStrategyStakeArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { amount })
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
pub struct DriftInsuranceFundStrategyUnstakeArgs {
    pub amount: u64,
}
impl DriftInsuranceFundStrategyUnstakeArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { amount })
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
pub struct DriftSupplyStrategyDepositArgs {
    pub amount: u64,
}
impl DriftSupplyStrategyDepositArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { amount })
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
pub struct DriftSupplyStrategyInitArgs {
    pub name: String,
    pub drift_sub_account_id: u16,
    pub drift_market_index: u16,
}
impl DriftSupplyStrategyInitArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        let drift_sub_account_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let drift_market_index: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            name,
            drift_sub_account_id,
            drift_market_index,
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
pub struct DriftSupplyStrategyWithdrawArgs {
    pub amount: u64,
}
impl DriftSupplyStrategyWithdrawArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { amount })
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
pub struct InitVaultArgs {
    pub redemption_fee_bps: u16,
    pub management_fee_bps: u16,
    pub performance_fee_bps: u16,
}
impl InitVaultArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let redemption_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let management_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let performance_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            redemption_fee_bps,
            management_fee_bps,
            performance_fee_bps,
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
pub struct IssueArgs {
    pub amount: u64,
}
impl IssueArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { amount })
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
pub struct KlendSupplyStrategyDepositArgs {
    pub amount: u64,
}
impl KlendSupplyStrategyDepositArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { amount })
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
pub struct KlendSupplyStrategyInitArgs {
    pub name: String,
}
impl KlendSupplyStrategyInitArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { name })
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
pub struct KlendSupplyStrategyWithdrawArgs {
    pub amount: u64,
}
impl KlendSupplyStrategyWithdrawArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { amount })
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
pub struct MangoSupplyStrategyDepositArgs {
    pub amount: u64,
}
impl MangoSupplyStrategyDepositArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { amount })
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
pub struct MangoSupplyStrategyInitArgs {
    pub name: String,
}
impl MangoSupplyStrategyInitArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { name })
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
pub struct MangoSupplyStrategyWithdrawArgs {
    pub amount: u64,
}
impl MangoSupplyStrategyWithdrawArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { amount })
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
pub struct MarginfiSupplyStrategyDepositArgs {
    pub amount: u64,
}
impl MarginfiSupplyStrategyDepositArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { amount })
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
pub struct MarginfiSupplyStrategyInitArgs {
    pub name: String,
}
impl MarginfiSupplyStrategyInitArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { name })
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
pub struct MarginfiSupplyStrategyWithdrawArgs {
    pub amount: u64,
}
impl MarginfiSupplyStrategyWithdrawArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { amount })
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
pub struct PauseVaultArgs {
    pub paused: bool,
}
impl PauseVaultArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let paused: bool = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { paused })
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
pub struct RedeemArgs {
    pub amount: u64,
}
impl RedeemArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { amount })
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
pub struct SolendSupplyStrategyDepositArgs {
    pub amount: u64,
}
impl SolendSupplyStrategyDepositArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { amount })
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
pub struct SolendSupplyStrategyInitArgs {
    pub name: String,
}
impl SolendSupplyStrategyInitArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { name })
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
pub struct SolendSupplyStrategyWithdrawArgs {
    pub amount: u64,
}
impl SolendSupplyStrategyWithdrawArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { amount })
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
pub struct UpdateAssetArgs {
    pub asset_id: u16,
    pub asset_new_oracle: Pubkey,
}
impl UpdateAssetArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let asset_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let asset_new_oracle: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { asset_id, asset_new_oracle })
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
pub struct UpdateFeeArgs {
    pub new_redemption_fee_bps: Option<u16>,
    pub new_management_fee_bps: Option<u16>,
    pub new_performance_fee_bps: Option<u16>,
}
impl UpdateFeeArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let new_redemption_fee_bps: Option<u16> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let new_management_fee_bps: Option<u16> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let new_performance_fee_bps: Option<u16> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            new_redemption_fee_bps,
            new_management_fee_bps,
            new_performance_fee_bps,
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
pub struct UpdateStrategyArgs {
    pub new_strategy_type: StrategyType,
}
impl UpdateStrategyArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let new_strategy_type = <StrategyType as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self { new_strategy_type })
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
pub struct Asset {
    pub asset_id: u16,
    pub mint: Pubkey,
    pub decimals: u8,
    pub ata: Pubkey,
    pub oracle: Pubkey,
}
impl Asset {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let asset_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let ata: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let oracle: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            asset_id,
            mint,
            decimals,
            ata,
            oracle,
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
pub struct StrategyRecord {
    pub strategy_id: u16,
    pub asset_id: u16,
    pub balance: u64,
    pub net_earnings: i64,
}
impl StrategyRecord {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let strategy_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let asset_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let net_earnings: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            strategy_id,
            asset_id,
            balance,
            net_earnings,
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
pub struct StrategyMetadata {
    pub name: String,
    pub strategy_id: u16,
    pub asset_mint: Pubkey,
    pub vault: Pubkey,
}
impl StrategyMetadata {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let name: String = crate::borsh_de_or_default(&mut reader)?;
        let strategy_id: u16 = crate::borsh_de_or_default(&mut reader)?;
        let asset_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            name,
            strategy_id,
            asset_mint,
            vault,
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
pub struct Fee {
    pub redemption_fee_bps: u16,
    pub redemption_fee_accumulated: u64,
    pub management_fee_bps: u16,
    pub management_fee_last_update: i64,
    pub management_fee_accumulated: u64,
    pub performance_fee_bps: u16,
}
impl Fee {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let redemption_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let redemption_fee_accumulated: u64 = crate::borsh_de_or_default(&mut reader)?;
        let management_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let management_fee_last_update: i64 = crate::borsh_de_or_default(&mut reader)?;
        let management_fee_accumulated: u64 = crate::borsh_de_or_default(&mut reader)?;
        let performance_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            redemption_fee_bps,
            redemption_fee_accumulated,
            management_fee_bps,
            management_fee_last_update,
            management_fee_accumulated,
            performance_fee_bps,
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
pub struct IssueEvent {
    pub depositor: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
    pub shares_minted: u64,
    pub management_fee: u64,
    pub tvl: u128,
}
impl IssueEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let depositor: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let shares_minted: u64 = crate::borsh_de_or_default(&mut reader)?;
        let management_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let tvl: u128 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            depositor,
            mint,
            amount,
            shares_minted,
            management_fee,
            tvl,
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
pub struct RedeemEvent {
    pub withdrawer: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
    pub shares_burned: u64,
    pub redemption_fee: u64,
    pub management_fee: u64,
    pub tvl: u128,
}
impl RedeemEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let withdrawer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let shares_burned: u64 = crate::borsh_de_or_default(&mut reader)?;
        let redemption_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let management_fee: u64 = crate::borsh_de_or_default(&mut reader)?;
        let tvl: u128 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            withdrawer,
            mint,
            amount,
            shares_burned,
            redemption_fee,
            management_fee,
            tvl,
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
pub struct StrategyDepositEvent {
    pub strategy: Pubkey,
    pub mint: Pubkey,
    pub deposit_amount: u64,
    pub net_earnings: i64,
    pub strategy_balance: u64,
}
impl StrategyDepositEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let strategy: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let deposit_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let net_earnings: i64 = crate::borsh_de_or_default(&mut reader)?;
        let strategy_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            strategy,
            mint,
            deposit_amount,
            net_earnings,
            strategy_balance,
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
pub struct StrategyWithdrawEvent {
    pub strategy: Pubkey,
    pub mint: Pubkey,
    pub withdraw_amount: u64,
    pub net_earnings: i64,
    pub strategy_balance: u64,
}
impl StrategyWithdrawEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let strategy: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let withdraw_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let net_earnings: i64 = crate::borsh_de_or_default(&mut reader)?;
        let strategy_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            strategy,
            mint,
            withdraw_amount,
            net_earnings,
            strategy_balance,
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
pub struct StrategyClaimEmissionsEvent {
    pub strategy: Pubkey,
    pub mint: Pubkey,
}
impl StrategyClaimEmissionsEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let strategy: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { strategy, mint })
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
pub struct DistributeVaultFeesEvent {
    pub management_fee_amount_distributed: u64,
    pub performance_fee_amount_distributed: u64,
    pub redemption_fee_amount_distributed: u64,
}
impl DistributeVaultFeesEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let management_fee_amount_distributed: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let performance_fee_amount_distributed: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let redemption_fee_amount_distributed: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            management_fee_amount_distributed,
            performance_fee_amount_distributed,
            redemption_fee_amount_distributed,
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
pub struct UpdateSwitchboardOraclePriceEvent {
    pub vault_nav: i128,
    pub vault_tvl: u128,
    pub shares_supply: u64,
}
impl UpdateSwitchboardOraclePriceEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let vault_nav: i128 = crate::borsh_de_or_default(&mut reader)?;
        let vault_tvl: u128 = crate::borsh_de_or_default(&mut reader)?;
        let shares_supply: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            vault_nav,
            vault_tvl,
            shares_supply,
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
pub enum StrategyType {
    MarginfiSupply {
        account: Pubkey,
        group: Pubkey,
        bank: Pubkey,
        bank_liquidity_vault: Pubkey,
        bank_liquidity_vault_authority: Pubkey,
        oracle: Pubkey,
    },
    KlendSupply {
        reserve: Pubkey,
        reserve_collateral_mint: Pubkey,
        reserve_liquidity_supply: Pubkey,
        reserve_destination_deposit_collateral: Pubkey,
        reserve_farm_state: Pubkey,
        lending_market: Pubkey,
        oracle: Pubkey,
        scope_prices: Pubkey,
    },
    SolendSupply {
        reserve: Pubkey,
        reserve_collateral_mint: Pubkey,
        reserve_liquidity_supply: Pubkey,
        deposit_collateral_ata: Pubkey,
        lending_market: Pubkey,
        lending_market_authority: Pubkey,
        pyth_oracle: Pubkey,
        switchboard_oracle: Pubkey,
    },
    MangoSupply {
        group: Pubkey,
        account: Pubkey,
        bank: Pubkey,
        vault: Pubkey,
        pyth_oracle: Pubkey,
        switchboard_oracle: Pubkey,
    },
    DriftSupply {
        state: Pubkey,
        signer: Pubkey,
        spot_market: Pubkey,
        spot_market_vault: Pubkey,
        perp_market: Pubkey,
        spot_pyth_oracle: Pubkey,
        perp_pyth_oracle: Pubkey,
        sub_account_id: u16,
        market_index: u16,
    },
    DriftInsuranceFund {
        state: Pubkey,
        spot_market: Pubkey,
        spot_market_vault: Pubkey,
        market_index: u16,
    },
    Chest {
        chest: Pubkey,
        coin: Pubkey,
        drift_vault: Pubkey,
        coin_token_account: Pubkey,
        asset_destination: Pubkey,
        redemption_request: Option<Pubkey>,
        spot_market_index: u16,
    },
    ClendSupply {
        account: Pubkey,
        group: Pubkey,
        bank: Pubkey,
        bank_liquidity_vault: Pubkey,
        bank_liquidity_vault_authority: Pubkey,
        oracle: Pubkey,
    },
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
pub enum StrategyTypeSelection {
    #[default]
    MarginfiSupply,
    KlendSupply,
    SolendSupply,
    MangoSupply,
    DriftSupply,
    DriftInsuranceFund,
    Chest,
    ClendSupply,
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
pub enum CarrotEvent {
    Issue(IssueEvent),
    Redeem(RedeemEvent),
    StrategyDeposit(StrategyDepositEvent),
    StrategyWithdraw(StrategyWithdrawEvent),
    StrategyClaimEmissions(StrategyClaimEmissionsEvent),
    DistributeVaultFees(DistributeVaultFeesEvent),
    UpdateSwitchboardOraclePrice(UpdateSwitchboardOraclePriceEvent),
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
pub enum RoundingMode {
    #[default]
    RoundUp,
    RoundDown,
    Avg,
}
