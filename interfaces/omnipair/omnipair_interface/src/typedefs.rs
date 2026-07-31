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
pub struct AddLiquidityArgs {
    pub amount0_in: u64,
    pub amount1_in: u64,
    pub min_liquidity_out: u64,
}
impl AddLiquidityArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount0_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount1_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_liquidity_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            amount0_in,
            amount1_in,
            min_liquidity_out,
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
pub struct AdjustCollateralArgs {
    pub amount: u64,
}
impl AdjustCollateralArgs {
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
pub struct AdjustDebtArgs {
    pub amount: u64,
}
impl AdjustDebtArgs {
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
pub struct CreateRateModelArgs {
    pub target_util_start_bps: u64,
    pub target_util_end_bps: u64,
    pub half_life_ms: u64,
    pub min_rate_bps: u64,
    pub max_rate_bps: u64,
    pub initial_rate_bps: u64,
}
impl CreateRateModelArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let target_util_start_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let target_util_end_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let half_life_ms: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_rate_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_rate_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let initial_rate_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            target_util_start_bps,
            target_util_end_bps,
            half_life_ms,
            min_rate_bps,
            max_rate_bps,
            initial_rate_bps,
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
pub struct EmitValueArgs {
    pub amount: Option<u64>,
    pub token_mint: Option<Pubkey>,
    pub debt_amount: Option<u64>,
}
impl EmitValueArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let token_mint: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let debt_amount: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            amount,
            token_mint,
            debt_amount,
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
pub struct EventMetadata {
    pub signer: Pubkey,
    pub pair: Pubkey,
    pub slot: u64,
}
impl EventMetadata {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let signer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let pair: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { signer, pair, slot })
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
pub struct FlashloanArgs {
    pub amount0: u64,
    pub amount1: u64,
    pub data: Vec<u8>,
}
impl FlashloanArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount0: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount1: u64 = crate::borsh_de_or_default(&mut reader)?;
        let data: Vec<u8> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { amount0, amount1, data })
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
pub struct InitFutarchyAuthorityArgs {
    pub authority: Pubkey,
    pub swap_bps: u16,
    pub interest_bps: u16,
    pub futarchy_treasury: Pubkey,
    pub futarchy_treasury_bps: u16,
    pub buybacks_vault: Pubkey,
    pub buybacks_vault_bps: u16,
    pub team_treasury: Pubkey,
    pub team_treasury_bps: u16,
}
impl InitFutarchyAuthorityArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let swap_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let interest_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let futarchy_treasury: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let futarchy_treasury_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let buybacks_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let buybacks_vault_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let team_treasury: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let team_treasury_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            authority,
            swap_bps,
            interest_bps,
            futarchy_treasury,
            futarchy_treasury_bps,
            buybacks_vault,
            buybacks_vault_bps,
            team_treasury,
            team_treasury_bps,
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
pub struct InitializeAndBootstrapArgs {
    pub swap_fee_bps: u16,
    pub half_life: u64,
    pub fixed_cf_bps: Option<u16>,
    pub target_util_start_bps: Option<u64>,
    pub target_util_end_bps: Option<u64>,
    pub rate_half_life_ms: Option<u64>,
    pub min_rate_bps: Option<u64>,
    pub max_rate_bps: Option<u64>,
    pub initial_rate_bps: Option<u64>,
    pub params_hash: [u8; 32],
    pub version: u8,
    pub amount0_in: u64,
    pub amount1_in: u64,
    pub min_liquidity_out: u64,
    pub lp_name: String,
    pub lp_symbol: String,
    pub lp_uri: String,
}
impl InitializeAndBootstrapArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let swap_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let half_life: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fixed_cf_bps: Option<u16> = crate::borsh_de_or_default(&mut reader)?;
        let target_util_start_bps: Option<u64> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let target_util_end_bps: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let rate_half_life_ms: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let min_rate_bps: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let max_rate_bps: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let initial_rate_bps: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let params_hash: [u8; 32] = crate::borsh_de_or_default(&mut reader)?;
        let version: u8 = crate::borsh_de_or_default(&mut reader)?;
        let amount0_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let amount1_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_liquidity_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        let lp_name: String = crate::borsh_de_or_default(&mut reader)?;
        let lp_symbol: String = crate::borsh_de_or_default(&mut reader)?;
        let lp_uri: String = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            swap_fee_bps,
            half_life,
            fixed_cf_bps,
            target_util_start_bps,
            target_util_end_bps,
            rate_half_life_ms,
            min_rate_bps,
            max_rate_bps,
            initial_rate_bps,
            params_hash,
            version,
            amount0_in,
            amount1_in,
            min_liquidity_out,
            lp_name,
            lp_symbol,
            lp_uri,
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
pub struct LastPriceEMA {
    pub symmetric: u64,
    pub directional: u64,
}
impl LastPriceEMA {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let symmetric: u64 = crate::borsh_de_or_default(&mut reader)?;
        let directional: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { symmetric, directional })
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
pub enum PairViewKind {
    #[default]
    EmaPrice0Nad,
    EmaPrice1Nad,
    SpotPrice0Nad,
    SpotPrice1Nad,
    K,
    GetRates,
    GetBorrowLimitAndCfBpsForCollateral,
    Reserves,
    CashReserves,
    SwapQuote,
    SimulateLiquidationPrice,
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
pub struct RemoveLiquidityArgs {
    pub liquidity_in: u64,
    pub min_amount0_out: u64,
    pub min_amount1_out: u64,
}
impl RemoveLiquidityArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let liquidity_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_amount0_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_amount1_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            liquidity_in,
            min_amount0_out,
            min_amount1_out,
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
pub struct RevenueDistribution {
    pub futarchy_treasury_bps: u16,
    pub buybacks_vault_bps: u16,
    pub team_treasury_bps: u16,
}
impl RevenueDistribution {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let futarchy_treasury_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let buybacks_vault_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let team_treasury_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            futarchy_treasury_bps,
            buybacks_vault_bps,
            team_treasury_bps,
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
pub struct RevenueRecipients {
    pub futarchy_treasury: Pubkey,
    pub buybacks_vault: Pubkey,
    pub team_treasury: Pubkey,
}
impl RevenueRecipients {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let futarchy_treasury: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let buybacks_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let team_treasury: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            futarchy_treasury,
            buybacks_vault,
            team_treasury,
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
pub struct RevenueShare {
    pub swap_bps: u16,
    pub interest_bps: u16,
}
impl RevenueShare {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let swap_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let interest_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { swap_bps, interest_bps })
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
pub struct SetGlobalReduceOnlyArgs {
    pub reduce_only: bool,
}
impl SetGlobalReduceOnlyArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let reduce_only: bool = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { reduce_only })
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
pub struct SetPairReduceOnlyArgs {
    pub reduce_only: bool,
}
impl SetPairReduceOnlyArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let reduce_only: bool = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { reduce_only })
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
pub struct SwapArgs {
    pub amount_in: u64,
    pub min_amount_out: u64,
}
impl SwapArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_amount_out: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { amount_in, min_amount_out })
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
pub struct UpdateFutarchyAuthorityArgs {
    pub new_authority: Pubkey,
}
impl UpdateFutarchyAuthorityArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let new_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { new_authority })
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
pub struct UpdateProtocolRevenueArgs {
    pub swap_bps: Option<u16>,
    pub interest_bps: Option<u16>,
    pub revenue_distribution: Option<RevenueDistribution>,
}
impl UpdateProtocolRevenueArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let swap_bps: Option<u16> = crate::borsh_de_or_default(&mut reader)?;
        let interest_bps: Option<u16> = crate::borsh_de_or_default(&mut reader)?;
        let revenue_distribution: Option<RevenueDistribution> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            swap_bps,
            interest_bps,
            revenue_distribution,
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
pub struct UpdateRevenueRecipientsArgs {
    pub futarchy_treasury: Option<Pubkey>,
    pub buybacks_vault: Option<Pubkey>,
    pub team_treasury: Option<Pubkey>,
}
impl UpdateRevenueRecipientsArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let futarchy_treasury: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let buybacks_vault: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let team_treasury: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            futarchy_treasury,
            buybacks_vault,
            team_treasury,
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
pub enum UserPositionViewKind {
    #[default]
    UserDynamicBorrowLimit,
    UserDynamicCollateralFactorBps,
    UserLiquidationCfBps,
    UserDebtUtilizationBps,
    UserLiquidationPrice,
    UserDebtWithInterest,
    UserIsLiquidatable,
    UserCollateralValueWithImpact,
    UserLiquidationBorrowLimit,
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
pub struct VaultBumps {
    pub reserve0: u8,
    pub reserve1: u8,
    pub collateral0: u8,
    pub collateral1: u8,
}
impl VaultBumps {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let reserve0: u8 = crate::borsh_de_or_default(&mut reader)?;
        let reserve1: u8 = crate::borsh_de_or_default(&mut reader)?;
        let collateral0: u8 = crate::borsh_de_or_default(&mut reader)?;
        let collateral1: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            reserve0,
            reserve1,
            collateral0,
            collateral1,
        })
    }
}
