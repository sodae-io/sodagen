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
pub struct CancelOrderParams {
    pub order_sequence_number: u64,
    pub order_index_hint: Option<u32>,
}
impl CancelOrderParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let order_sequence_number: u64 = crate::borsh_de_or_default(&mut reader)?;
        let order_index_hint: Option<u32> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            order_sequence_number,
            order_index_hint,
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
pub struct PlaceOrderParams {
    pub base_atoms: u64,
    pub price_mantissa: u32,
    pub price_exponent: i8,
    pub is_bid: bool,
    pub last_valid_slot: u32,
    pub order_type: OrderType,
}
impl PlaceOrderParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let base_atoms: u64 = crate::borsh_de_or_default(&mut reader)?;
        let price_mantissa: u32 = crate::borsh_de_or_default(&mut reader)?;
        let price_exponent: i8 = crate::borsh_de_or_default(&mut reader)?;
        let is_bid: bool = crate::borsh_de_or_default(&mut reader)?;
        let last_valid_slot: u32 = crate::borsh_de_or_default(&mut reader)?;
        let order_type: OrderType = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            base_atoms,
            price_mantissa,
            price_exponent,
            is_bid,
            last_valid_slot,
            order_type,
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
pub struct BatchUpdateParams {
    pub trader_index_hint: Option<u32>,
    pub cancels: Vec<CancelOrderParams>,
    pub orders: Vec<PlaceOrderParams>,
}
impl BatchUpdateParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let trader_index_hint: Option<u32> = crate::borsh_de_or_default(&mut reader)?;
        let cancels: Vec<CancelOrderParams> = crate::borsh_de_or_default(&mut reader)?;
        let orders: Vec<PlaceOrderParams> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            trader_index_hint,
            cancels,
            orders,
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
pub struct DepositParams {
    pub amount_atoms: u64,
}
impl DepositParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount_atoms: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { amount_atoms })
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
pub struct GlobalCleanParams {
    pub order_index: u32,
}
impl GlobalCleanParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let order_index: u32 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { order_index })
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
pub struct GlobalDepositParams {
    pub amount_atoms: u64,
}
impl GlobalDepositParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount_atoms: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { amount_atoms })
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
pub struct GlobalEvictParams {
    pub amount_atoms: u64,
}
impl GlobalEvictParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount_atoms: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { amount_atoms })
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
pub struct GlobalWithdrawParams {
    pub amount_atoms: u64,
}
impl GlobalWithdrawParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount_atoms: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self { amount_atoms })
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
pub struct SwapParams {
    pub in_atoms: u64,
    pub out_atoms: u64,
    pub is_base_in: bool,
    pub is_exact_in: bool,
}
impl SwapParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let in_atoms: u64 = crate::borsh_de_or_default(&mut reader)?;
        let out_atoms: u64 = crate::borsh_de_or_default(&mut reader)?;
        let is_base_in: bool = crate::borsh_de_or_default(&mut reader)?;
        let is_exact_in: bool = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            in_atoms,
            out_atoms,
            is_base_in,
            is_exact_in,
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
pub struct WithdrawParams {
    pub amount_atoms: u64,
    pub trader_index_hint: Option<u32>,
}
impl WithdrawParams {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let amount_atoms: u64 = crate::borsh_de_or_default(&mut reader)?;
        let trader_index_hint: Option<u32> = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            amount_atoms,
            trader_index_hint,
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
pub struct ClaimedSeat {
    pub trader: Pubkey,
    pub base_withdrawable_balance: u64,
    pub quote_withdrawable_balance: u64,
    pub quote_volume: u64,
    pub padding: [u8; 8],
}
impl ClaimedSeat {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let trader: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_withdrawable_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_withdrawable_balance: u64 = crate::borsh_de_or_default(&mut reader)?;
        let quote_volume: u64 = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 8] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            trader,
            base_withdrawable_balance,
            quote_withdrawable_balance,
            quote_volume,
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
pub struct GlobalFixed {
    pub discriminant: u64,
    pub mint: Pubkey,
    pub vault: Pubkey,
    pub global_traders_root_index: u32,
    pub global_deposits_root_index: u32,
    pub global_deposits_max_index: u32,
    pub free_list_head_index: u32,
    pub num_bytes_allocated: u32,
    pub vault_bump: u8,
    pub global_bump: u8,
    pub num_seats_claimed: u16,
}
impl GlobalFixed {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let discriminant: u64 = crate::borsh_de_or_default(&mut reader)?;
        let mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let global_traders_root_index: u32 = crate::borsh_de_or_default(&mut reader)?;
        let global_deposits_root_index: u32 = crate::borsh_de_or_default(&mut reader)?;
        let global_deposits_max_index: u32 = crate::borsh_de_or_default(&mut reader)?;
        let free_list_head_index: u32 = crate::borsh_de_or_default(&mut reader)?;
        let num_bytes_allocated: u32 = crate::borsh_de_or_default(&mut reader)?;
        let vault_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let global_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let num_seats_claimed: u16 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            discriminant,
            mint,
            vault,
            global_traders_root_index,
            global_deposits_root_index,
            global_deposits_max_index,
            free_list_head_index,
            num_bytes_allocated,
            vault_bump,
            global_bump,
            num_seats_claimed,
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
pub struct GlobalTrader {
    pub trader: Pubkey,
    pub deposit_index: u32,
    pub padding: u32,
    pub padding2: u64,
}
impl GlobalTrader {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let trader: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let deposit_index: u32 = crate::borsh_de_or_default(&mut reader)?;
        let padding: u32 = crate::borsh_de_or_default(&mut reader)?;
        let padding2: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            trader,
            deposit_index,
            padding,
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
pub struct GlobalDeposit {
    pub trader: Pubkey,
    pub balance_atoms: u64,
    pub padding: u64,
}
impl GlobalDeposit {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let trader: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let balance_atoms: u64 = crate::borsh_de_or_default(&mut reader)?;
        let padding: u64 = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            trader,
            balance_atoms,
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
pub struct MarketFixed {
    pub discriminant: u64,
    pub version: u8,
    pub base_mint_decimals: u8,
    pub quote_mint_decimals: u8,
    pub base_vault_bump: u8,
    pub quote_vault_bump: u8,
    pub padding1: [u8; 3],
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub order_sequence_number: u64,
    pub num_bytes_allocated: u32,
    pub bids_root_index: u32,
    pub bids_best_index: u32,
    pub asks_root_index: u32,
    pub asks_best_index: u32,
    pub claimed_seats_root_index: u32,
    pub free_list_head_index: u32,
    pub padding2: [u32; 1],
    pub quote_volume: u64,
    pub padding3: [u64; 8],
}
impl MarketFixed {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let discriminant: u64 = crate::borsh_de_or_default(&mut reader)?;
        let version: u8 = crate::borsh_de_or_default(&mut reader)?;
        let base_mint_decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let quote_mint_decimals: u8 = crate::borsh_de_or_default(&mut reader)?;
        let base_vault_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let quote_vault_bump: u8 = crate::borsh_de_or_default(&mut reader)?;
        let padding1: [u8; 3] = crate::borsh_de_or_default(&mut reader)?;
        let base_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let quote_mint: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let base_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let quote_vault: Pubkey = crate::borsh_de_or_default(&mut reader)?;
        let order_sequence_number: u64 = crate::borsh_de_or_default(&mut reader)?;
        let num_bytes_allocated: u32 = crate::borsh_de_or_default(&mut reader)?;
        let bids_root_index: u32 = crate::borsh_de_or_default(&mut reader)?;
        let bids_best_index: u32 = crate::borsh_de_or_default(&mut reader)?;
        let asks_root_index: u32 = crate::borsh_de_or_default(&mut reader)?;
        let asks_best_index: u32 = crate::borsh_de_or_default(&mut reader)?;
        let claimed_seats_root_index: u32 = crate::borsh_de_or_default(&mut reader)?;
        let free_list_head_index: u32 = crate::borsh_de_or_default(&mut reader)?;
        let padding2: [u32; 1] = crate::borsh_de_or_default(&mut reader)?;
        let quote_volume: u64 = crate::borsh_de_or_default(&mut reader)?;
        let padding3: [u64; 8] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            discriminant,
            version,
            base_mint_decimals,
            quote_mint_decimals,
            base_vault_bump,
            quote_vault_bump,
            padding1,
            base_mint,
            quote_mint,
            base_vault,
            quote_vault,
            order_sequence_number,
            num_bytes_allocated,
            bids_root_index,
            bids_best_index,
            asks_root_index,
            asks_best_index,
            claimed_seats_root_index,
            free_list_head_index,
            padding2,
            quote_volume,
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
pub struct RestingOrder {
    pub price: u128,
    pub num_base_atoms: u64,
    pub sequence_number: u64,
    pub trader_index: u32,
    pub last_valid_slot: u32,
    pub is_bid: bool,
    pub order_type: OrderType,
    pub padding: [u8; 22],
}
impl RestingOrder {
    pub fn deserialize(__buf: &mut &[u8]) -> std::io::Result<Self> {
        let mut reader: &[u8] = *__buf;
        let price: u128 = crate::borsh_de_or_default(&mut reader)?;
        let num_base_atoms: u64 = crate::borsh_de_or_default(&mut reader)?;
        let sequence_number: u64 = crate::borsh_de_or_default(&mut reader)?;
        let trader_index: u32 = crate::borsh_de_or_default(&mut reader)?;
        let last_valid_slot: u32 = crate::borsh_de_or_default(&mut reader)?;
        let is_bid: bool = crate::borsh_de_or_default(&mut reader)?;
        let order_type: OrderType = crate::borsh_de_or_default(&mut reader)?;
        let padding: [u8; 22] = crate::borsh_de_or_default(&mut reader)?;
        *__buf = reader;
        Ok(Self {
            price,
            num_base_atoms,
            sequence_number,
            trader_index,
            last_valid_slot,
            is_bid,
            order_type,
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
pub enum OrderType {
    #[default]
    Limit,
    ImmediateOrCancel,
    PostOnly,
    Global,
    Reverse,
    ReverseTight,
}
