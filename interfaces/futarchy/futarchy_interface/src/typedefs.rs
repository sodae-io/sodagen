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
pub struct CommonFields {
    pub slot: u64,
    pub unix_timestamp: i64,
    pub dao_seq_num: u64,
}
impl CommonFields {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        let unix_timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        let dao_seq_num: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            slot,
            unix_timestamp,
            dao_seq_num,
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
pub struct AdminApproveMultisigProposalArgs {
    pub transaction_index: u64,
}
impl AdminApproveMultisigProposalArgs {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let transaction_index: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { transaction_index })
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
pub struct ConditionalSwapParams {
    pub market: Market,
    pub swap_type: SwapType,
    pub input_amount: u64,
    pub min_output_amount: u64,
}
impl ConditionalSwapParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let market: Market = crate::borsh_de_or_default(&mut reader)?;
        let swap_type: SwapType = crate::borsh_de_or_default(&mut reader)?;
        let input_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_output_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            market,
            swap_type,
            input_amount,
            min_output_amount,
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
pub struct InitializeDaoParams {
    pub twap_initial_observation: u128,
    pub twap_max_observation_change_per_update: u128,
    pub twap_start_delay_seconds: u32,
    pub min_quote_futarchic_liquidity: u64,
    pub min_base_futarchic_liquidity: u64,
    pub base_to_stake: u64,
    pub pass_threshold_bps: u16,
    pub seconds_per_proposal: u32,
    pub nonce: u64,
    pub initial_spending_limit: Option<InitialSpendingLimit>,
    pub team_sponsored_pass_threshold_bps: i16,
    pub team_address: Pubkey,
}
impl InitializeDaoParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let twap_initial_observation: u128 = crate::borsh_de_or_default(&mut reader)?;
        let twap_max_observation_change_per_update: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let twap_start_delay_seconds: u32 = crate::borsh_de_or_default(&mut reader)?;
        let min_quote_futarchic_liquidity: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let min_base_futarchic_liquidity: u64 = crate::borsh_de_or_default(&mut reader)?;
        let base_to_stake: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pass_threshold_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let seconds_per_proposal: u32 = crate::borsh_de_or_default(&mut reader)?;
        let nonce: u64 = crate::borsh_de_or_default(&mut reader)?;
        let initial_spending_limit: Option<InitialSpendingLimit> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let team_sponsored_pass_threshold_bps: i16 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let team_address: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            twap_initial_observation,
            twap_max_observation_change_per_update,
            twap_start_delay_seconds,
            min_quote_futarchic_liquidity,
            min_base_futarchic_liquidity,
            base_to_stake,
            pass_threshold_bps,
            seconds_per_proposal,
            nonce,
            initial_spending_limit,
            team_sponsored_pass_threshold_bps,
            team_address,
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
pub struct InitiateVaultSpendOptimisticProposalParams {
    pub amount: u64,
}
impl InitiateVaultSpendOptimisticProposalParams {
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
pub struct ProvideLiquidityParams {
    pub quote_amount: u64,
    pub max_base_amount: u64,
    pub min_liquidity: u128,
    pub position_authority: Pubkey,
}
impl ProvideLiquidityParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let quote_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let max_base_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let position_authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            quote_amount,
            max_base_amount,
            min_liquidity,
            position_authority,
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
pub struct SpotSwapParams {
    pub input_amount: u64,
    pub swap_type: SwapType,
    pub min_output_amount: u64,
}
impl SpotSwapParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let input_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let swap_type: SwapType = crate::borsh_de_or_default(&mut reader)?;
        let min_output_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            input_amount,
            swap_type,
            min_output_amount,
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
pub struct StakeToProposalParams {
    pub amount: u64,
}
impl StakeToProposalParams {
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
pub struct UnstakeFromProposalParams {
    pub amount: u64,
}
impl UnstakeFromProposalParams {
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
pub struct UpdateDaoParams {
    pub pass_threshold_bps: Option<u16>,
    pub seconds_per_proposal: Option<u32>,
    pub twap_initial_observation: Option<u128>,
    pub twap_max_observation_change_per_update: Option<u128>,
    pub twap_start_delay_seconds: Option<u32>,
    pub min_quote_futarchic_liquidity: Option<u64>,
    pub min_base_futarchic_liquidity: Option<u64>,
    pub base_to_stake: Option<u64>,
    pub team_sponsored_pass_threshold_bps: Option<i16>,
    pub team_address: Option<Pubkey>,
    pub is_optimistic_governance_enabled: Option<bool>,
}
impl UpdateDaoParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pass_threshold_bps: Option<u16> = crate::borsh_de_or_default(&mut reader)?;
        let seconds_per_proposal: Option<u32> = crate::borsh_de_or_default(&mut reader)?;
        let twap_initial_observation: Option<u128> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let twap_max_observation_change_per_update: Option<u128> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let twap_start_delay_seconds: Option<u32> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let min_quote_futarchic_liquidity: Option<u64> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let min_base_futarchic_liquidity: Option<u64> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let base_to_stake: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let team_sponsored_pass_threshold_bps: Option<i16> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let team_address: Option<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        let is_optimistic_governance_enabled: Option<bool> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            pass_threshold_bps,
            seconds_per_proposal,
            twap_initial_observation,
            twap_max_observation_change_per_update,
            twap_start_delay_seconds,
            min_quote_futarchic_liquidity,
            min_base_futarchic_liquidity,
            base_to_stake,
            team_sponsored_pass_threshold_bps,
            team_address,
            is_optimistic_governance_enabled,
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
pub struct WithdrawLiquidityParams {
    pub liquidity_to_withdraw: u128,
    pub min_base_amount: u64,
    pub min_quote_amount: u64,
}
impl WithdrawLiquidityParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let liquidity_to_withdraw: u128 = crate::borsh_de_or_default(&mut reader)?;
        let min_base_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        let min_quote_amount: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            liquidity_to_withdraw,
            min_base_amount,
            min_quote_amount,
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
pub struct OptimisticProposal {
    pub squads_proposal: Pubkey,
    pub enqueued_timestamp: i64,
}
impl OptimisticProposal {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let squads_proposal: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let enqueued_timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            squads_proposal,
            enqueued_timestamp,
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
pub struct InitialSpendingLimit {
    pub amount_per_month: u64,
    pub members: Vec<Pubkey>,
}
impl InitialSpendingLimit {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount_per_month: u64 = crate::borsh_de_or_default(&mut reader)?;
        let members: Vec<Pubkey> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { amount_per_month, members })
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
pub struct FutarchyAmm {
    pub state: PoolState,
    pub total_liquidity: u128,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub amm_base_vault: Pubkey,
    pub amm_quote_vault: Pubkey,
}
impl FutarchyAmm {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let state = <PoolState as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        let total_liquidity: u128 = crate::borsh_de_or_default(&mut reader)?;
        let base_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let quote_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amm_base_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let amm_quote_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            state,
            total_liquidity,
            base_mint,
            quote_mint,
            amm_base_vault,
            amm_quote_vault,
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
pub struct TwapOracle {
    pub aggregator: u128,
    pub last_updated_timestamp: i64,
    pub created_at_timestamp: i64,
    pub last_price: u128,
    pub last_observation: u128,
    pub max_observation_change_per_update: u128,
    pub initial_observation: u128,
    pub start_delay_seconds: u32,
}
impl TwapOracle {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let aggregator: u128 = crate::borsh_de_or_default(&mut reader)?;
        let last_updated_timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        let created_at_timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        let last_price: u128 = crate::borsh_de_or_default(&mut reader)?;
        let last_observation: u128 = crate::borsh_de_or_default(&mut reader)?;
        let max_observation_change_per_update: u128 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let initial_observation: u128 = crate::borsh_de_or_default(&mut reader)?;
        let start_delay_seconds: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            aggregator,
            last_updated_timestamp,
            created_at_timestamp,
            last_price,
            last_observation,
            max_observation_change_per_update,
            initial_observation,
            start_delay_seconds,
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
pub struct Pool {
    pub oracle: TwapOracle,
    pub quote_reserves: u64,
    pub base_reserves: u64,
    pub quote_protocol_fee_balance: u64,
    pub base_protocol_fee_balance: u64,
}
impl Pool {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let oracle = if reader.is_empty() {
            Default::default()
        } else {
            <TwapOracle>::deserialize(&mut reader)?
        };
        let quote_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let base_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_protocol_fee_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let base_protocol_fee_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            oracle,
            quote_reserves,
            base_reserves,
            quote_protocol_fee_balance,
            base_protocol_fee_balance,
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
pub enum PoolState {
    Spot { spot: Pool },
    Futarchy { spot: Pool, pass: Pool, fail: Pool },
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
pub enum Market {
    #[default]
    Spot,
    Pass,
    Fail,
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
pub enum SwapType {
    #[default]
    Buy,
    Sell,
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
pub enum Token {
    #[default]
    Base,
    Quote,
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
pub enum ProposalState {
    Draft { amount_staked: u64 },
    Pending,
    Passed,
    Failed,
    Removed,
}
