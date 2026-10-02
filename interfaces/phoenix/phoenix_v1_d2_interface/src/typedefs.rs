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
pub struct MarketSizeParams {
    pub bids_size: u64,
    pub asks_size: u64,
    pub num_seats: u64,
}
impl MarketSizeParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bids_size: u64 = crate::borsh_de_or_default(&mut reader)?;
        let asks_size: u64 = crate::borsh_de_or_default(&mut reader)?;
        let num_seats: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            bids_size,
            asks_size,
            num_seats,
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
pub struct Seat {
    pub discriminant: u64,
    pub market: Pubkey,
    pub trader: Pubkey,
    pub approval_status: u64,
    pub padding: [u64; 6],
}
impl Seat {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let discriminant: u64 = crate::borsh_de_or_default(&mut reader)?;
        let market: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let trader: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let approval_status: u64 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u64; 6] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            discriminant,
            market,
            trader,
            approval_status,
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
pub struct AuditLogHeader {
    pub instruction: u8,
    pub sequence_number: u64,
    pub timestamp: i64,
    pub slot: u64,
    pub market: Pubkey,
    pub signer: Pubkey,
    pub total_events: u16,
}
impl AuditLogHeader {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let instruction: u8 = crate::borsh_de_or_default(&mut reader)?;
        let sequence_number: u64 = crate::borsh_de_or_default(&mut reader)?;
        let timestamp: i64 = crate::borsh_de_or_default(&mut reader)?;
        let slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        let market: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let signer: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let total_events: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            instruction,
            sequence_number,
            timestamp,
            slot,
            market,
            signer,
            total_events,
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
pub struct FillEvent {
    pub index: u16,
    pub maker_id: Pubkey,
    pub order_sequence_number: u64,
    pub price_in_ticks: u64,
    pub base_lots_filled: u64,
    pub base_lots_remaining: u64,
}
impl FillEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let maker_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let order_sequence_number: u64 = crate::borsh_de_or_default(&mut reader)?;
        let price_in_ticks: u64 = crate::borsh_de_or_default(&mut reader)?;
        let base_lots_filled: u64 = crate::borsh_de_or_default(&mut reader)?;
        let base_lots_remaining: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            index,
            maker_id,
            order_sequence_number,
            price_in_ticks,
            base_lots_filled,
            base_lots_remaining,
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
pub struct ReduceEvent {
    pub index: u16,
    pub order_sequence_number: u64,
    pub price_in_ticks: u64,
    pub base_lots_removed: u64,
    pub base_lots_remaining: u64,
}
impl ReduceEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let order_sequence_number: u64 = crate::borsh_de_or_default(&mut reader)?;
        let price_in_ticks: u64 = crate::borsh_de_or_default(&mut reader)?;
        let base_lots_removed: u64 = crate::borsh_de_or_default(&mut reader)?;
        let base_lots_remaining: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            index,
            order_sequence_number,
            price_in_ticks,
            base_lots_removed,
            base_lots_remaining,
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
pub struct PlaceEvent {
    pub index: u16,
    pub order_sequence_number: u64,
    pub client_order_id: u128,
    pub price_in_ticks: u64,
    pub base_lots_placed: u64,
}
impl PlaceEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let order_sequence_number: u64 = crate::borsh_de_or_default(&mut reader)?;
        let client_order_id: u128 = crate::borsh_de_or_default(&mut reader)?;
        let price_in_ticks: u64 = crate::borsh_de_or_default(&mut reader)?;
        let base_lots_placed: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            index,
            order_sequence_number,
            client_order_id,
            price_in_ticks,
            base_lots_placed,
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
pub struct EvictEvent {
    pub index: u16,
    pub maker_id: Pubkey,
    pub order_sequence_number: u64,
    pub price_in_ticks: u64,
    pub base_lots_evicted: u64,
}
impl EvictEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let maker_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let order_sequence_number: u64 = crate::borsh_de_or_default(&mut reader)?;
        let price_in_ticks: u64 = crate::borsh_de_or_default(&mut reader)?;
        let base_lots_evicted: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            index,
            maker_id,
            order_sequence_number,
            price_in_ticks,
            base_lots_evicted,
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
pub struct FillSummaryEvent {
    pub index: u16,
    pub client_order_id: u128,
    pub total_base_lots_filled: u64,
    pub total_quote_lots_filled: u64,
    pub total_fee_in_quote_lots: u64,
}
impl FillSummaryEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let client_order_id: u128 = crate::borsh_de_or_default(&mut reader)?;
        let total_base_lots_filled: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_quote_lots_filled: u64 = crate::borsh_de_or_default(&mut reader)?;
        let total_fee_in_quote_lots: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            index,
            client_order_id,
            total_base_lots_filled,
            total_quote_lots_filled,
            total_fee_in_quote_lots,
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
pub struct FeeEvent {
    pub index: u16,
    pub fees_collected_in_quote_lots: u64,
}
impl FeeEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let fees_collected_in_quote_lots: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            index,
            fees_collected_in_quote_lots,
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
pub struct TimeInForceEvent {
    pub index: u16,
    pub order_sequence_number: u64,
    pub last_valid_slot: u64,
    pub last_valid_unix_timestamp_in_seconds: u64,
}
impl TimeInForceEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let order_sequence_number: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_valid_slot: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_valid_unix_timestamp_in_seconds: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            index,
            order_sequence_number,
            last_valid_slot,
            last_valid_unix_timestamp_in_seconds,
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
pub struct ExpiredOrderEvent {
    pub index: u16,
    pub maker_id: Pubkey,
    pub order_sequence_number: u64,
    pub price_in_ticks: u64,
    pub base_lots_removed: u64,
}
impl ExpiredOrderEvent {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let index: u16 = crate::borsh_de_or_default(&mut reader)?;
        let maker_id: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let order_sequence_number: u64 = crate::borsh_de_or_default(&mut reader)?;
        let price_in_ticks: u64 = crate::borsh_de_or_default(&mut reader)?;
        let base_lots_removed: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            index,
            maker_id,
            order_sequence_number,
            price_in_ticks,
            base_lots_removed,
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
pub struct CancelUpToParams {
    pub side: Side,
    pub tick_limit: Option<u64>,
    pub num_orders_to_search: Option<u32>,
    pub num_orders_to_cancel: Option<u32>,
}
impl CancelUpToParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let side: Side = crate::borsh_de_or_default(&mut reader)?;
        let tick_limit: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let num_orders_to_search: Option<u32> = crate::borsh_de_or_default(&mut reader)?;
        let num_orders_to_cancel: Option<u32> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            side,
            tick_limit,
            num_orders_to_search,
            num_orders_to_cancel,
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
pub struct CancelMultipleOrdersByIdParams {
    pub orders: Vec<CancelOrderParams>,
}
impl CancelMultipleOrdersByIdParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let orders: Vec<CancelOrderParams> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { orders })
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
pub struct DepositParams {
    pub quote_lots_to_deposit: u64,
    pub base_lots_to_deposit: u64,
}
impl DepositParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let quote_lots_to_deposit: u64 = crate::borsh_de_or_default(&mut reader)?;
        let base_lots_to_deposit: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            quote_lots_to_deposit,
            base_lots_to_deposit,
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
pub struct InitializeParams {
    pub market_size_params: MarketSizeParams,
    pub num_quote_lots_per_quote_unit: u64,
    pub tick_size_in_quote_lots_per_base_unit: u64,
    pub num_base_lots_per_base_unit: u64,
    pub taker_fee_bps: u16,
    pub fee_collector: Pubkey,
    pub raw_base_units_per_base_unit: Option<u32>,
}
impl InitializeParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let market_size_params = if reader.is_empty() {
            Default::default()
        } else {
            <MarketSizeParams>::deserialize(&mut reader)?
        };
        let num_quote_lots_per_quote_unit: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let tick_size_in_quote_lots_per_base_unit: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let num_base_lots_per_base_unit: u64 = crate::borsh_de_or_default(&mut reader)?;
        let taker_fee_bps: u16 = crate::borsh_de_or_default(&mut reader)?;
        let fee_collector: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let raw_base_units_per_base_unit: Option<u32> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            market_size_params,
            num_quote_lots_per_quote_unit,
            tick_size_in_quote_lots_per_base_unit,
            num_base_lots_per_base_unit,
            taker_fee_bps,
            fee_collector,
            raw_base_units_per_base_unit,
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
pub struct MultipleOrderPacket {
    pub bids: Vec<CondensedOrder>,
    pub asks: Vec<CondensedOrder>,
    pub client_order_id: Option<u128>,
    pub failed_multiple_limit_order_behavior: FailedMultipleLimitOrderBehavior,
}
impl MultipleOrderPacket {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let bids: Vec<CondensedOrder> = crate::borsh_de_or_default(&mut reader)?;
        let asks: Vec<CondensedOrder> = crate::borsh_de_or_default(&mut reader)?;
        let client_order_id: Option<u128> = crate::borsh_de_or_default(&mut reader)?;
        let failed_multiple_limit_order_behavior: FailedMultipleLimitOrderBehavior = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            bids,
            asks,
            client_order_id,
            failed_multiple_limit_order_behavior,
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
pub struct CondensedOrder {
    pub price_in_ticks: u64,
    pub size_in_base_lots: u64,
    pub last_valid_slot: Option<u64>,
    pub last_valid_unix_timestamp_in_seconds: Option<u64>,
}
impl CondensedOrder {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let price_in_ticks: u64 = crate::borsh_de_or_default(&mut reader)?;
        let size_in_base_lots: u64 = crate::borsh_de_or_default(&mut reader)?;
        let last_valid_slot: Option<u64> = crate::borsh_de_or_default(&mut reader)?;
        let last_valid_unix_timestamp_in_seconds: Option<u64> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            price_in_ticks,
            size_in_base_lots,
            last_valid_slot,
            last_valid_unix_timestamp_in_seconds,
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
pub struct CancelOrderParams {
    pub side: Side,
    pub price_in_ticks: u64,
    pub order_sequence_number: u64,
}
impl CancelOrderParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let side: Side = crate::borsh_de_or_default(&mut reader)?;
        let price_in_ticks: u64 = crate::borsh_de_or_default(&mut reader)?;
        let order_sequence_number: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            side,
            price_in_ticks,
            order_sequence_number,
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
pub struct ReduceOrderParams {
    pub base_params: CancelOrderParams,
    pub size: u64,
}
impl ReduceOrderParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let base_params = if reader.is_empty() {
            Default::default()
        } else {
            <CancelOrderParams>::deserialize(&mut reader)?
        };
        let size: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { base_params, size })
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
pub struct WithdrawParams {
    pub quote_lots_to_withdraw: Option<u64>,
    pub base_lots_to_withdraw: Option<u64>,
}
impl WithdrawParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let quote_lots_to_withdraw: Option<u64> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let base_lots_to_withdraw: Option<u64> = crate::borsh_de_or_default(
            &mut reader,
        )?;
        *__buf = reader;
        Ok(Self {
            quote_lots_to_withdraw,
            base_lots_to_withdraw,
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
pub struct MarketHeader {
    pub discriminant: u64,
    pub status: u64,
    pub market_size_params: MarketSizeParams,
    pub base_params: TokenParams,
    pub base_lot_size: u64,
    pub quote_params: TokenParams,
    pub quote_lot_size: u64,
    pub tick_size_in_quote_atoms_per_base_unit: u64,
    pub authority: Pubkey,
    pub fee_recipient: Pubkey,
    pub market_sequence_number: u64,
    pub successor: Pubkey,
    pub raw_base_units_per_base_unit: u32,
    pub padding1: u32,
    pub padding2: [u64; 32],
}
impl MarketHeader {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let discriminant: u64 = crate::borsh_de_or_default(&mut reader)?;
        let status: u64 = crate::borsh_de_or_default(&mut reader)?;
        let market_size_params = if reader.is_empty() {
            Default::default()
        } else {
            <MarketSizeParams>::deserialize(&mut reader)?
        };
        let base_params = if reader.is_empty() {
            Default::default()
        } else {
            <TokenParams>::deserialize(&mut reader)?
        };
        let base_lot_size: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_params = if reader.is_empty() {
            Default::default()
        } else {
            <TokenParams>::deserialize(&mut reader)?
        };
        let quote_lot_size: u64 = crate::borsh_de_or_default(&mut reader)?;
        let tick_size_in_quote_atoms_per_base_unit: u64 = crate::borsh_de_or_default(
            &mut reader,
        )?;
        let authority: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let fee_recipient: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let market_sequence_number: u64 = crate::borsh_de_or_default(&mut reader)?;
        let successor: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let raw_base_units_per_base_unit: u32 = crate::borsh_de_or_default(&mut reader)?;
        let padding1: u32 = crate::borsh_de_or_default(&mut reader)?;
        let padding2: [u64; 32] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            discriminant,
            status,
            market_size_params,
            base_params,
            base_lot_size,
            quote_params,
            quote_lot_size,
            tick_size_in_quote_atoms_per_base_unit,
            authority,
            fee_recipient,
            market_sequence_number,
            successor,
            raw_base_units_per_base_unit,
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
pub struct FIFOOrderId {
    pub price_in_ticks: Ticks,
    pub order_sequence_number: u64,
}
impl FIFOOrderId {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let price_in_ticks = if reader.is_empty() {
            Default::default()
        } else {
            <Ticks>::deserialize(&mut reader)?
        };
        let order_sequence_number: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            price_in_ticks,
            order_sequence_number,
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
pub enum PhoenixMarketEvent {
    #[default]
    Uninitialized,
    Header(AuditLogHeader),
    Fill(FillEvent),
    Place(PlaceEvent),
    Reduce(ReduceEvent),
    Evict(EvictEvent),
    FillSummary(FillSummaryEvent),
    Fee(FeeEvent),
    TimeInForce(TimeInForceEvent),
    ExpiredOrder(ExpiredOrderEvent),
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
pub enum FailedMultipleLimitOrderBehavior {
    #[default]
    FailOnInsufficientFundsAndAmendOnCross,
    FailOnInsufficientFundsAndFailOnCross,
    SkipOnInsufficientFundsAndAmendOnCross,
    SkipOnInsufficientFundsAndFailOnCross,
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
pub enum MarketStatus {
    #[default]
    Uninitialized,
    Active,
    PostOnly,
    Paused,
    Closed,
    Tombstoned,
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
pub enum SeatApprovalStatus {
    #[default]
    NotApproved,
    Approved,
    Retired,
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
pub enum OrderPacket {
    PostOnly {
        side: Side,
        price_in_ticks: u64,
        num_base_lots: u64,
        client_order_id: u128,
        reject_post_only: bool,
        use_only_deposited_funds: bool,
        last_valid_slot: Option<u64>,
        last_valid_unix_timestamp_in_seconds: Option<u64>,
        fail_silently_on_insufficient_funds: bool,
    },
    Limit {
        side: Side,
        price_in_ticks: u64,
        num_base_lots: u64,
        self_trade_behavior: SelfTradeBehavior,
        match_limit: Option<u64>,
        client_order_id: u128,
        use_only_deposited_funds: bool,
        last_valid_slot: Option<u64>,
        last_valid_unix_timestamp_in_seconds: Option<u64>,
        fail_silently_on_insufficient_funds: bool,
    },
    ImmediateOrCancel {
        side: Side,
        price_in_ticks: Option<u64>,
        num_base_lots: u64,
        num_quote_lots: u64,
        min_base_lots_to_fill: u64,
        min_quote_lots_to_fill: u64,
        self_trade_behavior: SelfTradeBehavior,
        match_limit: Option<u64>,
        client_order_id: u128,
        use_only_deposited_funds: bool,
        last_valid_slot: Option<u64>,
        last_valid_unix_timestamp_in_seconds: Option<u64>,
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
pub enum Side {
    #[default]
    Bid,
    Ask,
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
pub enum SelfTradeBehavior {
    #[default]
    Abort,
    CancelProvide,
    DecrementTake,
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
pub struct Ticks {
    pub inner: u64,
}
impl Ticks {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let inner: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { inner })
    }
}
