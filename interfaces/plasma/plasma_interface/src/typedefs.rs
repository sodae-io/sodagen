use borsh::{BorshDeserialize, BorshSerialize};
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
pub struct SwapResult {
    pub side: Side,
    pub base_matched: u64,
    pub quote_matched: u64,
    pub base_matched_as_limit_order: u64,
    pub quote_matched_as_limit_order: u64,
    pub base_matched_as_swap: u64,
    pub quote_matched_as_swap: u64,
    pub fee_in_quote: u64,
}
impl SwapResult {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let side: Side = crate::borsh_de_or_default(&mut reader)?;
        let base_matched: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_matched: u64 = crate::borsh_de_or_default(&mut reader)?;
        let base_matched_as_limit_order: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_matched_as_limit_order: u64 = crate::borsh_de_or_default(&mut reader)?;
        let base_matched_as_swap: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_matched_as_swap: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_in_quote: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            side,
            base_matched,
            quote_matched,
            base_matched_as_limit_order,
            quote_matched_as_limit_order,
            base_matched_as_swap,
            quote_matched_as_swap,
            fee_in_quote,
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
pub struct TokenParams {
    pub decimals: u32,
    pub vault_bump: u32,
    pub mint_key: Pubkey,
    pub vault_key: Pubkey,
}
impl TokenParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let decimals: u32 = crate::borsh_de_or_default(&mut reader)?;
        let vault_bump: u32 = crate::borsh_de_or_default(&mut reader)?;
        let mint_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault_key: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            decimals,
            vault_bump,
            mint_key,
            vault_key,
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
pub struct ProtocolFeeRecipient {
    pub recipient: Pubkey,
    pub shares: u64,
    pub total_accumulated_quote_fees: u64,
    pub collected_quote_fees: u64,
}
impl ProtocolFeeRecipient {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let recipient: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_accumulated_quote_fees: u64 = crate::borsh_de_or_default(&mut reader)?;
        let collected_quote_fees: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            recipient,
            shares,
            total_accumulated_quote_fees,
            collected_quote_fees,
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
pub struct ProtocolFeeRecipients {
    pub recipients: [ProtocolFeeRecipient; 3],
    pub padding: [u64; 12],
}
impl ProtocolFeeRecipients {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let recipients: [ProtocolFeeRecipient; 3] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let padding: [u64; 12] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { recipients, padding })
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
pub struct PoolHeader {
    pub sequence_number: u64,
    pub base_params: TokenParams,
    pub quote_params: TokenParams,
    pub fee_recipients: ProtocolFeeRecipients,
    pub swap_sequence_number: u64,
    pub padding: [u64; 12],
}
impl PoolHeader {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let sequence_number: u64 = crate::borsh_de_or_default(&mut reader)?;
        let base_params = if reader.is_empty() {
            Default::default()
        } else {
            <TokenParams>::deserialize(&mut reader)?
        };
        let quote_params = if reader.is_empty() {
            Default::default()
        } else {
            <TokenParams>::deserialize(&mut reader)?
        };
        let fee_recipients = if reader.is_empty() {
            Default::default()
        } else {
            <ProtocolFeeRecipients>::deserialize(&mut reader)?
        };
        let swap_sequence_number: u64 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u64; 12] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            sequence_number,
            base_params,
            quote_params,
            fee_recipients,
            swap_sequence_number,
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
pub struct Amm {
    pub fee_in_bps: u32,
    pub protocol_allocation_in_pct: u32,
    pub lp_vesting_window: u64,
    pub reward_factor: u128,
    pub total_lp_shares: u64,
    pub slot_snapshot: u64,
    pub base_reserves_snapshot: u64,
    pub quote_reserves_snapshot: u64,
    pub base_reserves: u64,
    pub quote_reserves: u64,
    pub cumulative_quote_lp_fees: u64,
    pub cumulative_quote_protocol_fees: u64,
}
impl Amm {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let fee_in_bps: u32 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_allocation_in_pct: u32 = crate::borsh_de_or_default(&mut reader)?;
        let lp_vesting_window: u64 = crate::borsh_de_or_default(&mut reader)?;
        let reward_factor: u128 = crate::borsh_de_or_default(&mut reader)?;
        let total_lp_shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        let slot_snapshot: u64 = crate::borsh_de_or_default(&mut reader)?;
        let base_reserves_snapshot: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_reserves_snapshot: u64 = crate::borsh_de_or_default(&mut reader)?;
        let base_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_reserves: u64 = crate::borsh_de_or_default(&mut reader)?;
        let cumulative_quote_lp_fees: u64 = crate::borsh_de_or_default(&mut reader)?;
        let cumulative_quote_protocol_fees: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            fee_in_bps,
            protocol_allocation_in_pct,
            lp_vesting_window,
            reward_factor,
            total_lp_shares,
            slot_snapshot,
            base_reserves_snapshot,
            quote_reserves_snapshot,
            base_reserves,
            quote_reserves,
            cumulative_quote_lp_fees,
            cumulative_quote_protocol_fees,
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
pub struct LpPosition {
    pub reward_factor_snapshot: u128,
    pub lp_shares: u64,
    pub withdrawable_lp_shares: u64,
    pub uncollected_fees: u64,
    pub collected_fees: u64,
    pub pending_shares_to_vest: PendingSharesToVest,
}
impl LpPosition {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let reward_factor_snapshot: u128 = crate::borsh_de_or_default(&mut reader)?;
        let lp_shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        let withdrawable_lp_shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        let uncollected_fees: u64 = crate::borsh_de_or_default(&mut reader)?;
        let collected_fees: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pending_shares_to_vest = if reader.is_empty() {
            Default::default()
        } else {
            <PendingSharesToVest>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            reward_factor_snapshot,
            lp_shares,
            withdrawable_lp_shares,
            uncollected_fees,
            collected_fees,
            pending_shares_to_vest,
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
pub struct PendingSharesToVest {
    pub deposit_slot: u64,
    pub lp_shares_to_vest: u64,
}
impl PendingSharesToVest {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let deposit_slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        let lp_shares_to_vest: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            deposit_slot,
            lp_shares_to_vest,
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
pub struct InitializePoolIxParams {
    pub lp_fee_in_bps: u64,
    pub protocol_lp_fee_allocation_in_pct: u64,
    pub fee_recipients_params: [ProtocolFeeRecipientParams; 3],
    pub num_slots_to_vest_lp_shares: Option<u64>,
}
impl InitializePoolIxParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let lp_fee_in_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_lp_fee_allocation_in_pct: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let fee_recipients_params: [ProtocolFeeRecipientParams; 3] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let num_slots_to_vest_lp_shares: Option<u64> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            lp_fee_in_bps,
            protocol_lp_fee_allocation_in_pct,
            fee_recipients_params,
            num_slots_to_vest_lp_shares,
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
pub struct AddLiquidityIxParams {
    pub desired_base_amount_in: u64,
    pub desired_quote_amount_in: u64,
    pub initial_lp_shares: Option<u64>,
}
impl AddLiquidityIxParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let desired_base_amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let desired_quote_amount_in: u64 = crate::borsh_de_or_default(&mut reader)?;
        let initial_lp_shares: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            desired_base_amount_in,
            desired_quote_amount_in,
            initial_lp_shares,
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
pub struct RemoveLiquidityIxParams {
    pub lp_shares: u64,
}
impl RemoveLiquidityIxParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let lp_shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { lp_shares })
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
pub struct SwapIxParams {
    pub side: Side,
    pub swap_type: SwapType,
}
impl SwapIxParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let side: Side = crate::borsh_de_or_default(&mut reader)?;
        let swap_type = <SwapType as borsh::BorshDeserialize>::deserialize_reader(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self { side, swap_type })
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
pub struct RenounceLiquidityIxParams {
    pub allow_fee_withdrawal: bool,
}
impl RenounceLiquidityIxParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let allow_fee_withdrawal: bool = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { allow_fee_withdrawal })
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
pub struct PlasmaEventHeader {
    pub sequence_number: u64,
    pub slot: u64,
    pub timestamp: i64,
    pub pool: Pubkey,
    pub signer: Pubkey,
    pub base_decimals: u8,
    pub quote_decimals: u8,
}
impl PlasmaEventHeader {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let sequence_number: u64 = crate::borsh_de_or_default(&mut reader)?;
        let slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        let timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        let pool: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let signer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let quote_decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            sequence_number,
            slot,
            timestamp,
            pool,
            signer,
            base_decimals,
            quote_decimals,
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
pub struct SwapEvent {
    pub swap_sequence_number: u64,
    pub pre_base_liquidity: u64,
    pub pre_quote_liquidity: u64,
    pub post_base_liquidity: u64,
    pub post_quote_liquidity: u64,
    pub snapshot_base_liquidity: u64,
    pub snapshot_quote_liquidity: u64,
    pub swap_result: SwapResult,
}
impl SwapEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let swap_sequence_number: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pre_base_liquidity: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pre_quote_liquidity: u64 = crate::borsh_de_or_default(&mut reader)?;
        let post_base_liquidity: u64 = crate::borsh_de_or_default(&mut reader)?;
        let post_quote_liquidity: u64 = crate::borsh_de_or_default(&mut reader)?;
        let snapshot_base_liquidity: u64 = crate::borsh_de_or_default(&mut reader)?;
        let snapshot_quote_liquidity: u64 = crate::borsh_de_or_default(&mut reader)?;
        let swap_result = if reader.is_empty() {
            Default::default()
        } else {
            <SwapResult>::deserialize(&mut reader)?
        };
        *__buf = reader;
        Ok(Self {
            swap_sequence_number,
            pre_base_liquidity,
            pre_quote_liquidity,
            post_base_liquidity,
            post_quote_liquidity,
            snapshot_base_liquidity,
            snapshot_quote_liquidity,
            swap_result,
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
pub struct AddLiquidityEvent {
    pub pool_total_lp_shares: u64,
    pub pool_total_base_liquidity: u64,
    pub pool_total_quote_liquitidy: u64,
    pub snapshot_base_liquidity: u64,
    pub snapshot_quote_liquidity: u64,
    pub user_lp_shares_received: u64,
    pub user_lp_shares_available: u64,
    pub user_lp_shares_locked: u64,
    pub user_lp_shares_unlocked_for_withdrawal: u64,
    pub user_base_deposited: u64,
    pub user_quote_deposited: u64,
    pub user_total_withdrawable_base: u64,
    pub user_total_withdrawable_quote: u64,
}
impl AddLiquidityEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_total_lp_shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pool_total_base_liquidity: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pool_total_quote_liquitidy: u64 = crate::borsh_de_or_default(&mut reader)?;
        let snapshot_base_liquidity: u64 = crate::borsh_de_or_default(&mut reader)?;
        let snapshot_quote_liquidity: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user_lp_shares_received: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user_lp_shares_available: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user_lp_shares_locked: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user_lp_shares_unlocked_for_withdrawal: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let user_base_deposited: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user_quote_deposited: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user_total_withdrawable_base: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user_total_withdrawable_quote: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            pool_total_lp_shares,
            pool_total_base_liquidity,
            pool_total_quote_liquitidy,
            snapshot_base_liquidity,
            snapshot_quote_liquidity,
            user_lp_shares_received,
            user_lp_shares_available,
            user_lp_shares_locked,
            user_lp_shares_unlocked_for_withdrawal,
            user_base_deposited,
            user_quote_deposited,
            user_total_withdrawable_base,
            user_total_withdrawable_quote,
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
pub struct RemoveLiquidityEvent {
    pub pool_total_lp_shares: u64,
    pub pool_total_base_liquidity: u64,
    pub pool_total_quote_liquitidy: u64,
    pub snapshot_base_liquidity: u64,
    pub snapshot_quote_liquidity: u64,
    pub user_lp_shares_burned: u64,
    pub user_lp_shares_available: u64,
    pub user_lp_shares_locked: u64,
    pub user_lp_shares_unlocked_for_withdrawal: u64,
    pub user_base_withdrawn: u64,
    pub user_quote_withdrawn: u64,
    pub user_total_withdrawable_base: u64,
    pub user_total_withdrawable_quote: u64,
}
impl RemoveLiquidityEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let pool_total_lp_shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pool_total_base_liquidity: u64 = crate::borsh_de_or_default(&mut reader)?;
        let pool_total_quote_liquitidy: u64 = crate::borsh_de_or_default(&mut reader)?;
        let snapshot_base_liquidity: u64 = crate::borsh_de_or_default(&mut reader)?;
        let snapshot_quote_liquidity: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user_lp_shares_burned: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user_lp_shares_available: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user_lp_shares_locked: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user_lp_shares_unlocked_for_withdrawal: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let user_base_withdrawn: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user_quote_withdrawn: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user_total_withdrawable_base: u64 = crate::borsh_de_or_default(&mut reader)?;
        let user_total_withdrawable_quote: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            pool_total_lp_shares,
            pool_total_base_liquidity,
            pool_total_quote_liquitidy,
            snapshot_base_liquidity,
            snapshot_quote_liquidity,
            user_lp_shares_burned,
            user_lp_shares_available,
            user_lp_shares_locked,
            user_lp_shares_unlocked_for_withdrawal,
            user_base_withdrawn,
            user_quote_withdrawn,
            user_total_withdrawable_base,
            user_total_withdrawable_quote,
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
pub struct RenounceLiquidityEvent {
    pub allow_fee_withdrawal: bool,
}
impl RenounceLiquidityEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let allow_fee_withdrawal: bool = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { allow_fee_withdrawal })
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
pub struct InitializeLpPositionEvent {
    pub owner: Pubkey,
}
impl InitializeLpPositionEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let owner: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { owner })
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
pub struct ProtocolFeeRecipientParams {
    pub recipient: Pubkey,
    pub shares: u64,
}
impl ProtocolFeeRecipientParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let recipient: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let shares: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { recipient, shares })
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
pub struct InitializePoolEvent {
    pub lp_fee_in_bps: u64,
    pub protocol_fee_in_pct: u64,
    pub fee_recipient_params: [ProtocolFeeRecipientParams; 3],
}
impl InitializePoolEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let lp_fee_in_bps: u64 = crate::borsh_de_or_default(&mut reader)?;
        let protocol_fee_in_pct: u64 = crate::borsh_de_or_default(&mut reader)?;
        let fee_recipient_params: [ProtocolFeeRecipientParams; 3] = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            lp_fee_in_bps,
            protocol_fee_in_pct,
            fee_recipient_params,
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
pub struct WithdrawLpFeesEvent {
    pub fees_withdrawn: u64,
}
impl WithdrawLpFeesEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let fees_withdrawn: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { fees_withdrawn })
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
pub struct WithdrawProtocolFeesEvent {
    pub protocol_fee_recipient: Pubkey,
    pub fees_withdrawn: u64,
}
impl WithdrawProtocolFeesEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let protocol_fee_recipient: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fees_withdrawn: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            protocol_fee_recipient,
            fees_withdrawn,
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
pub enum Side {
    #[default]
    Buy,
    Sell,
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
pub enum SwapType {
    ExactIn { amount_in: u64, min_amount_out: u64 },
    ExactOut { amount_out: u64, max_amount_in: u64 },
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
pub enum PlasmaEvent {
    Swap { header: PlasmaEventHeader, event: SwapEvent },
    AddLiquidity { header: PlasmaEventHeader, event: AddLiquidityEvent },
    RemoveLiquidity { header: PlasmaEventHeader, event: RemoveLiquidityEvent },
    RenounceLiquidity { header: PlasmaEventHeader, event: RenounceLiquidityEvent },
    WithdrawLpFees { header: PlasmaEventHeader, event: WithdrawLpFeesEvent },
    InitializeLpPosition { header: PlasmaEventHeader, event: InitializeLpPositionEvent },
    InitializePool { header: PlasmaEventHeader, event: InitializePoolEvent },
    WithdrawProtocolFees { header: PlasmaEventHeader, event: WithdrawProtocolFeesEvent },
}
